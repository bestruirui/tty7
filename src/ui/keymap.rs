use gpui::{App, Global, KeyBinding, Keystroke, Modifiers};

use crate::core::actions::*;
use crate::core::config::{Config, KeybindingOverride};
use crate::terminal::view::{
    AlternatePaste, ClearScrollback, CopyRaw, CopyText, FindInTerminal, FindNext, FindPrevious,
    PasteText, ToggleComposer,
};
use crate::ui::i18n::{L10nKey, t, t_fmt};
use crate::ui::settings::humanize_action;
use crate::ui::theme::set_menus;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum CommandGroup {
    TabsPanes,
    Workspaces,
    View,
    Terminal,
    Ssh,
    Agents,
    Application,
}

impl CommandGroup {
    pub(crate) const ORDER: [CommandGroup; 7] = [
        CommandGroup::TabsPanes,
        CommandGroup::Workspaces,
        CommandGroup::View,
        CommandGroup::Terminal,
        CommandGroup::Ssh,
        CommandGroup::Agents,
        CommandGroup::Application,
    ];

    pub(crate) fn title(self) -> &'static str {
        match self {
            CommandGroup::TabsPanes => t(L10nKey::CmdGroupTabsPanes),
            CommandGroup::Workspaces => t(L10nKey::CmdGroupWorkspaces),
            CommandGroup::View => t(L10nKey::CmdGroupView),
            CommandGroup::Terminal => t(L10nKey::CmdGroupTerminal),
            CommandGroup::Ssh => t(L10nKey::CmdGroupSsh),
            CommandGroup::Agents => t(L10nKey::CmdGroupAgents),
            CommandGroup::Application => t(L10nKey::CmdGroupApplication),
        }
    }
}

/// The bindings that were already in the keymap before tty7 put its own
/// there. gpui-component installs a table of its own from `gpui::component::
/// init` — the `Input` context's editing keys, the list and menu navigation,
/// the escape that closes a dialog — and it has no entry point to install
/// them a second time. A rebuild replaces the whole map, so it has to lay
/// these back down first, in the order they were added, or the map that comes
/// out has no backspace in any text field in the app (#548).
struct BaseBindings(Vec<KeyBinding>);
impl Global for BaseBindings {}

pub fn init(cx: &mut App) {
    if cx.try_global::<BaseBindings>().is_none() {
        let base: Vec<KeyBinding> = cx.key_bindings().borrow().bindings().cloned().collect();
        cx.set_global(BaseBindings(base));
    }
    rebuild_keymap(cx);
    cx.on_action(|_: &Quit, cx: &mut App| cx.quit());
    // On the app rather than on a window, for the same reason `Quit` is: this
    // is the one action in the table whose whole point is the state where no
    // window is there to dispatch it. `show_tray_icon` is on by default, so
    // closing the last window retires tty7 to the tray instead of quitting —
    // process alive, nothing on screen — and a `NewWindow` that only exists on
    // a window's render root is dead in exactly the state a New Window chord
    // is for. `Tty7App`'s own listener still wins wherever there is a window:
    // gpui runs the window's bubble phase first and returns before the global
    // one, so the two never both fire.
    cx.on_action(|_: &NewWindow, cx: &mut App| crate::ui::windows::open(cx, None));
    set_menus(cx);
}

/// The fixed bindings every keymap carries, whatever the config says. These
/// are the terminal's own keys (Tab/BackTab as input), the font-size step
/// that has no settings row, and the modal-context guards the dispatch path
/// needs — none of them come from `effective_bindings`, so a rebuild that
/// only replayed the config would drop them.
fn fixed_bindings(cx: &App) -> Vec<KeyBinding> {
    let mut bindings = vec![
        KeyBinding::load(
            "secondary-+",
            Box::new(IncreaseFontSize),
            None,
            false,
            None,
            cx,
        )
        .expect("valid font-size shortcut"),
        KeyBinding::new("tab", SendTab, Some("Terminal")),
        KeyBinding::new("shift-tab", SendBackTab, Some("Terminal")),
    ];
    // The switcher's footer tells you Tab is the way across to the tab column
    // once a query is in the box. gpui-component's Root binds `tab` to its
    // focus walker, actions are dispatched before key listeners, and the panel
    // was therefore losing every Tab to the ellipsis button in its own header —
    // ring and all. A binding on the panel's own context is deeper in the
    // dispatch path, so it wins.
    bindings.push(KeyBinding::new("tab", SwitcherAcross, Some("Switcher")));
    bindings.push(KeyBinding::new(
        "shift-tab",
        SwitcherAcrossBack,
        Some("Switcher"),
    ));
    bindings
}

/// Replaces the keymap with the full set: the bindings tty7 inherited, then
/// the ones the live config asks for, then the fixed ones.
///
/// gpui's `Keymap::add_bindings` only ever pushes — it never dedups and
/// nothing retires a binding — so rebinding by *appending* (the old `rebind`)
/// leaked one full table per call, and every keystroke walks the whole map.
/// Clearing first makes a rebind O(map) instead of O(map × calls), and makes
/// it safe to drive from the config watcher: a hand edit that only reorders
/// or re-comments the file reloads to an identical triple and never reaches
/// here, while a real change rebuilds once (#548). The order is the order
/// `init` bound them in, and gpui reads the map back to front, so tty7's
/// bindings still win over the inherited ones.
fn rebuild_keymap(cx: &mut App) {
    let effective = effective_bindings(cx);
    let mut bindings: Vec<KeyBinding> = cx
        .try_global::<BaseBindings>()
        .map(|base| base.0.clone())
        .unwrap_or_default();
    bindings.extend(action_bindings(&effective, cx));
    bindings.extend(fixed_bindings(cx));
    cx.clear_key_bindings();
    cx.bind_keys(bindings);
}

pub fn rebind(cx: &mut App) {
    rebuild_keymap(cx);
    set_menus(cx);
}

/// The three config fields `effective_bindings` reads, as one comparable
/// value. The config watcher reloads on every write — including the app's
/// own `save()`, which fires on a sidebar drag — so it compares the triple
/// before and after and only rebinds when one of these actually moved;
/// otherwise each save would rebuild the keymap for nothing (#548).
pub(crate) fn keybinding_config(cx: &App) -> (Vec<(String, KeybindingOverride)>, String, String) {
    let cfg = cx.global::<Config>();
    let mut overrides: Vec<(String, KeybindingOverride)> = cfg
        .keybindings
        .iter()
        .map(|(a, k)| (a.clone(), k.clone()))
        .collect();
    // A HashMap's order is not stable across reloads, and a reordered map is
    // not a changed binding set.
    overrides.sort();
    (overrides, cfg.keybinding_preset.clone(), cfg.prefix.clone())
}

const PASTE_ALT_DEFAULT: &str = "shift-insert";

fn paste_text_default() -> &'static str {
    per_platform("", "ctrl-shift-v")
}

/// Windows and Linux paste with Ctrl+V everywhere else in the desktop, so the
/// terminal answers it too — but only off the alternate screen, where the key
/// is a control code a full-screen program is waiting for (#677). It is a
/// binding of its own rather than a second keystroke on `PasteText` so that it
/// can carry that narrower context, and so that a user who wants the Windows
/// Terminal behaviour back can say so: `"AlternatePaste": ""` hands Ctrl+V to
/// the shell at the prompt as well, and `"PasteText": "ctrl-v"` pastes with it
/// on every screen.
fn alternate_paste_default() -> &'static str {
    per_platform("", "ctrl-v")
}

fn extra_defaults() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![("PasteText", paste_text_default(), PASTE_ALT_DEFAULT)]
}

fn extra_keystrokes(effective: &[(String, String)]) -> Vec<(&'static str, &'static str)> {
    extra_defaults()
        .into_iter()
        .filter(|(action, primary, _)| {
            !primary.is_empty() && effective.iter().any(|(a, k)| a == action && k == primary)
        })
        .map(|(action, _, extra)| (action, extra))
        .collect()
}

pub(crate) fn extra_bindings(cx: &App) -> Vec<(String, String)> {
    extra_keystrokes(&effective_bindings(cx))
        .into_iter()
        .map(|(a, k)| (a.to_string(), k.to_string()))
        .collect()
}

/// Installs `key` for `action`, and alongside it the chord the platform will
/// actually deliver when the two are spelled differently — `secondary-shift-]`
/// is pressed as `secondary-}`, and only the second ever reaches the
/// dispatcher. Both go in rather than one replacing the other, so a spec that
/// was already live stays live whatever the backend does with Shift (#750).
///
/// Answers whether the action was known at all, which is the caller's cue to
/// warn about a name it cannot bind.
fn push_binding(bindings: &mut Vec<KeyBinding>, action: &str, key: &str, cx: &App) -> bool {
    let Some(binding) = make_binding(action, key, cx) else {
        return false;
    };
    bindings.push(binding);
    if let Some(folded) = folded_spec(key)
        && let Some(alias) = make_binding(action, &folded, cx)
    {
        bindings.push(alias);
    }
    true
}

fn action_bindings(effective: &[(String, String)], cx: &App) -> Vec<KeyBinding> {
    let mut bindings = Vec::new();
    for (action, key) in extra_keystrokes(effective) {
        push_binding(&mut bindings, action, key, cx);
    }
    for (action, key) in effective {
        if key.is_empty() {
            continue;
        }
        if !keystroke_is_valid(key) {
            log::warn!("ignoring keybinding for '{action}': invalid keystroke '{key}'");
            continue;
        }
        // Said once, and the binding still installs: a chord the user asked
        // for by name is the user's to spend, the way the tmux preset spends
        // Ctrl+B. The invariant this guards is that no *default* spends one
        // without saying so — `no_default_binding_sits_on_a_terminal_control_code`
        // is the half of it that fails a build. A single chord only, since a
        // prefix like `ctrl-b n` is that choice made deliberately.
        // Over both spellings that reach the keymap, because the fold is what
        // decides which of them the shell actually loses. `steals_a_control_code`
        // only ever recognises a bare Ctrl, so it waves `ctrl-shift-2` through —
        // and then the fold installs `ctrl-@` beside it, which is NUL. The chord
        // was dead before it was folded and stole nothing; now that it is live
        // the warning has to follow it (#750). At most one of the two can trip:
        // a written chord that folds carries Shift, and one that carries Shift
        // is never a control code.
        let folded = folded_spec(key);
        for chord in [Some(key.as_str()), folded.as_deref()]
            .into_iter()
            .flatten()
        {
            if !chord.contains(' ')
                && steals_a_control_code(chord)
                && !control_code_binding_allowed(action, chord)
            {
                log::warn!(
                    "keybinding '{chord}' for '{action}' takes a control code away from the shell"
                );
            }
        }
        if !push_binding(&mut bindings, action, key, cx) {
            log::warn!("ignoring keybinding: unknown action '{action}'");
        }
    }
    bindings
}

fn parse_keystroke(source: &str) -> Result<Keystroke, gpui::InvalidKeystrokeError> {
    Keystroke::parse_with_os(
        source,
        if cfg!(target_os = "macos") {
            gpui::OperatingSystem::Mac
        } else if cfg!(target_os = "windows") {
            gpui::OperatingSystem::Windows
        } else {
            gpui::OperatingSystem::Linux
        },
    )
}

/// Whether a chord is one the terminal owes the PTY as a control code.
///
/// Ctrl and nothing else, over the keys that carry a C0 byte: the alphabet,
/// `@ [ \ ] ^ _ / ?`, the digits 2..8 and Space — `ctrl_c0` in
/// `terminal::input` is the table this mirrors. A binding sitting on one of
/// these does not merely shadow the shell, it deletes a byte the program on
/// the far end is waiting for — Ctrl+D is EOF, Ctrl+W deletes a word, Ctrl+^
/// is vim's alternate file.
///
/// The backtick is in the set without being in that table — it is held over
/// from when this rule lived inside the defaults test. It errs the safe way:
/// a chord that encodes nothing gets a warning it did not strictly earn,
/// which is cheaper than a default quietly eating one that does.
fn steals_a_control_code(chord: &str) -> bool {
    let Ok(ks) = parse_keystroke(chord) else {
        return false;
    };
    let m = &ks.modifiers;
    if !m.control || m.alt || m.shift || m.platform || m.function {
        return false;
    }
    ks.key == "space"
        || ks.key.len() == 1
            && ks.key.chars().next().is_some_and(|c| {
                c.is_ascii_alphabetic() || "[]\\`^_/?@".contains(c) || ('2'..='8').contains(&c)
            })
}

/// The bindings allowed to sit on a control code anyway.
fn control_code_binding_allowed(_action: &str, chord: &str) -> bool {
    cfg!(not(target_os = "macos")) && chord == "ctrl-v"
}

fn per_platform(mac: &'static str, other: &'static str) -> &'static str {
    if cfg!(target_os = "macos") {
        mac
    } else {
        other
    }
}

pub(crate) fn default_bindings() -> Vec<(&'static str, &'static str)> {
    let mut bindings = shipped_bindings();
    // One slot per agent, unbound: each takes a key the same way any other
    // action does. Right after the other agent actions, so the Keybindings page
    // keeps them together.
    let at = bindings
        .iter()
        .position(|(action, _)| *action == "NewAgentTab")
        .map_or(bindings.len(), |i| i + 1);
    bindings.splice(
        at..at,
        crate::ui::agent_launch::launch_action_names()
            .iter()
            .map(|(_, name)| (name.as_str(), "")),
    );
    bindings
}

fn shipped_bindings() -> Vec<(&'static str, &'static str)> {
    vec![
        ("NewTab", per_platform("secondary-t", "secondary-shift-t")),
        ("NewWorkspace", "secondary-shift-n"),
        // Cmd+N is what "New Window" means on macOS, and nothing else here
        // claims it. Off macOS both chords the convention offers are gone:
        // Ctrl+N is a C0 byte the shell is owed, which
        // `no_default_binding_sits_on_a_terminal_control_code` fails a build
        // over, and Ctrl+Shift+N — where that same test says window actions
        // belong — has been `NewWorkspace` for far longer than this action has
        // existed. Minting an unguessable third chord would be worse than
        // shipping unbound: the Keybindings page carries this, so a key is one
        // line of config away.
        ("NewWindow", per_platform("secondary-n", "")),
        // Closing the window is how tty7 is put away with every shell still
        // running, and Cmd+Q — the only chord a Mac user reaches for — stops
        // the server instead. Cmd+Shift+W is what "Close Window" means on
        // macOS and is free here. Off macOS that chord is `CloseActiveTab`.
        ("CloseWindow", per_platform("secondary-shift-w", "")),
        (
            "CloseActiveTab",
            per_platform("secondary-w", "secondary-shift-w"),
        ),
        ("RenameTab", ""),
        ("CloseOtherTabs", ""),
        ("CloseTabsToTheRight", ""),
        ("CopyWorkingDirectory", ""),
        ("MarkTabUnread", ""),
        ("HibernateTab", ""),
        ("ForkAgentSession", ""),
        ("ForkAgentSessionRight", ""),
        ("ForkAgentSessionLeft", ""),
        ("ForkAgentSessionDown", ""),
        ("ForkAgentSessionUp", ""),
        ("CopyAgentSessionId", ""),
        // ⌘⇧A for "agent" on macOS, where nothing else in the table or the
        // terminal holds it. Off macOS Ctrl+Shift+A is select-all in every
        // Linux terminal, so it ships
        // unbound there rather than on a chord nobody would guess.
        ("NewAgentTab", per_platform("secondary-shift-a", "")),
        // U for "up next": the agent that has waited on you longest. Free in
        // the table on both sides, and a letter under Ctrl+Shift off macOS,
        // where window actions live. GNOME's IBus claims Ctrl+Shift+U for
        // Unicode entry while it is the active input method, and wins there:
        // that user rebinds this one, the price every Ctrl+Shift letter pays
        // to some input method or other.
        ("FocusNextAgentNeedingAttention", "secondary-shift-u"),
        ("StopWorkspace", ""),
        ("DeleteWorkspace", ""),
        ("RenameWorkspace", ""),
        ("ToggleSwitcher", "secondary-shift-o"),
        (
            "SplitRight",
            per_platform("secondary-d", "secondary-shift-d"),
        ),
        (
            "SplitDown",
            per_platform("secondary-shift-d", "secondary-alt-shift-d"),
        ),
        (
            "FocusNextPane",
            per_platform("secondary-]", "secondary-shift-]"),
        ),
        (
            "FocusPrevPane",
            per_platform("secondary-[", "secondary-shift-["),
        ),
        (
            "FocusPaneLeft",
            per_platform("secondary-alt-left", "alt-left"),
        ),
        (
            "FocusPaneRight",
            per_platform("secondary-alt-right", "alt-right"),
        ),
        ("FocusPaneUp", per_platform("secondary-alt-up", "alt-up")),
        (
            "FocusPaneDown",
            per_platform("secondary-alt-down", "alt-down"),
        ),
        ("ResizePaneLeft", ""),
        ("ResizePaneRight", ""),
        ("ResizePaneUp", ""),
        ("ResizePaneDown", ""),
        ("SwapPaneNext", ""),
        ("SwapPanePrev", ""),
        // Ctrl+Tab opens the most-recently-used switcher (a quick tap lands
        // on the tab you were just in); these step to the neighbouring tab
        // in strip order with no popup (#867). ⌘⇧] / ⌘⇧[ is iTerm2's and
        // Safari's chord. Off macOS Ctrl+Shift+] is FocusNextPane, so they
        // take Ctrl+PgDn / Ctrl+PgUp, the chord GNOME Terminal, xfce4-terminal
        // and every browser use for the same thing.
        ("NextTab", "ctrl-tab"),
        ("PrevTab", "ctrl-shift-tab"),
        (
            "SelectNextTab",
            per_platform("secondary-shift-]", "ctrl-pagedown"),
        ),
        (
            "SelectPrevTab",
            per_platform("secondary-shift-[", "ctrl-pageup"),
        ),
        ("MoveTabLeft", ""),
        ("MoveTabRight", ""),
        ("ActivateTab1", per_platform("secondary-1", "alt-1")),
        ("ActivateTab2", per_platform("secondary-2", "alt-2")),
        ("ActivateTab3", per_platform("secondary-3", "alt-3")),
        ("ActivateTab4", per_platform("secondary-4", "alt-4")),
        ("ActivateTab5", per_platform("secondary-5", "alt-5")),
        ("ActivateTab6", per_platform("secondary-6", "alt-6")),
        ("ActivateTab7", per_platform("secondary-7", "alt-7")),
        ("ActivateTab8", per_platform("secondary-8", "alt-8")),
        ("ActivateTab9", per_platform("secondary-9", "alt-9")),
        ("SelectWorkspace1", ""),
        ("SelectWorkspace2", ""),
        ("SelectWorkspace3", ""),
        ("SelectWorkspace4", ""),
        ("SelectWorkspace5", ""),
        ("SelectWorkspace6", ""),
        ("SelectWorkspace7", ""),
        ("SelectWorkspace8", ""),
        ("SelectWorkspace9", ""),
        ("IncreaseFontSize", "secondary-="),
        ("DecreaseFontSize", "secondary--"),
        ("ResetFontSize", "secondary-0"),
        (
            "ReopenClosedTab",
            per_platform("secondary-shift-t", "alt-shift-t"),
        ),
        ("ToggleMaximizePane", "secondary-shift-enter"),
        // The one default that sits on a bare function key, and it stays
        // there: F11 is the fullscreen chord Windows Terminal, GNOME Terminal
        // and konsole all train their users on, and no shell binds it — the
        // keys PSReadLine actually wants are F3, F7 and F8. gpui matches this
        // binding before the pane's key handler runs, so the terminal never
        // sees a bare F11; Shift/Ctrl/Alt+F11 are not bound and still reach
        // the PTY as `\E[23;<mods>~`, and the whole chord is one line of
        // config away from being retired. See
        // `f11_is_the_only_default_on_a_bare_function_key`.
        ("ToggleFullscreen", per_platform("secondary-enter", "f11")),
        ("ToggleTabSidebar", ""),
        (
            "ToggleLeftPanel",
            per_platform("secondary-b", "secondary-shift-b"),
        ),
        (
            "ToggleRightPanel",
            per_platform("secondary-j", "secondary-shift-j"),
        ),
        (
            "FindInTerminal",
            if cfg!(target_os = "macos") {
                "secondary-f"
            } else {
                "ctrl-shift-f"
            },
        ),
        (
            "FindNext",
            if cfg!(target_os = "macos") {
                "secondary-g"
            } else {
                "f3"
            },
        ),
        (
            "FindPrevious",
            if cfg!(target_os = "macos") {
                "secondary-shift-g"
            } else {
                "shift-f3"
            },
        ),
        (
            "ClearScrollback",
            per_platform("secondary-k", "secondary-shift-k"),
        ),
        // ⌘I for "input". Nothing in the table holds it, and no shell or agent
        // reads Cmd at all. Off macOS Ctrl+I is Tab, so the chord takes Shift
        // like every other terminal command there.
        (
            "ToggleComposer",
            per_platform("secondary-i", "secondary-shift-i"),
        ),
        ("CopyText", per_platform("", "ctrl-shift-c")),
        ("CopyRaw", "cmd-alt-c"),
        ("PasteText", paste_text_default()),
        ("AlternatePaste", alternate_paste_default()),
        ("OpenSettings", "secondary-,"),
        (
            "ShowKeyboardShortcuts",
            if cfg!(target_os = "macos") {
                "secondary-/"
            } else {
                ""
            },
        ),
        ("About", ""),
        ("CheckForUpdates", ""),
        ("OpenDocumentation", ""),
        ("OpenDiscord", ""),
        ("ReportIssue", ""),
        (
            "HideApp",
            if cfg!(target_os = "macos") {
                "secondary-h"
            } else {
                ""
            },
        ),
        (
            "HideOthers",
            if cfg!(target_os = "macos") {
                "secondary-alt-h"
            } else {
                ""
            },
        ),
        ("ShowAll", ""),
        (
            "MinimizeWindow",
            if cfg!(target_os = "macos") {
                "secondary-m"
            } else {
                ""
            },
        ),
        ("ZoomWindow", ""),
        ("ShowSshForwards", ""),
        ("OpenSshProfiles", ""),
        ("RestartSshSession", "secondary-shift-r"),
        ("Quit", per_platform("secondary-q", "secondary-shift-q")),
    ]
}

/// Where an action sits on the Keybindings page, and what it is called there.
pub(crate) fn action_entry(action: &str) -> (CommandGroup, String) {
    authored_entry(action).unwrap_or_else(|| {
        // An action with no entry still renders, under Application and with its
        // name split on capitals — a new action shows up on the page rather
        // than disappearing from it. `every_action_has_an_authored_name` is
        // what keeps that from being how the page looks.
        (CommandGroup::Application, humanize_action(action))
    })
}

fn authored_entry(action: &str) -> Option<(CommandGroup, String)> {
    // The numbered families are one row each in nine copies; a templated label
    // beats nine hand-written strings per locale.
    if let Some(n) = action.strip_prefix("ActivateTab") {
        return Some((
            CommandGroup::TabsPanes,
            t_fmt(L10nKey::KeybindGoToTab, &[("n", n)]),
        ));
    }
    if let Some(agent) = crate::ui::agent_launch::agent_for_launch_action(action) {
        return Some((
            CommandGroup::Agents,
            t_fmt(
                L10nKey::AppCmdAgentLaunchTitle,
                &[("name", agent.display_name())],
            ),
        ));
    }
    if let Some(n) = action.strip_prefix("SelectWorkspace") {
        return Some((
            CommandGroup::Workspaces,
            t_fmt(L10nKey::KeybindGoToWorkspace, &[("n", n)]),
        ));
    }
    Some(match action {
        "NewTab" => (CommandGroup::TabsPanes, t(L10nKey::CmdNewTab).to_string()),
        "CloseActiveTab" => (
            CommandGroup::TabsPanes,
            t(L10nKey::CmdClosePaneTab).to_string(),
        ),
        "RenameTab" => (
            CommandGroup::TabsPanes,
            t(L10nKey::CmdRenameTab).to_string(),
        ),
        "CloseOtherTabs" => (
            CommandGroup::TabsPanes,
            t(L10nKey::CmdCloseOtherTabs).to_string(),
        ),
        "CloseTabsToTheRight" => (
            CommandGroup::TabsPanes,
            t(L10nKey::CmdCloseTabsToTheRight).to_string(),
        ),
        "CopyWorkingDirectory" => (
            CommandGroup::TabsPanes,
            t(L10nKey::CmdCopyWorkingDirectory).to_string(),
        ),
        "MarkTabUnread" => (
            CommandGroup::TabsPanes,
            t(L10nKey::CmdMarkTabAsUnread).to_string(),
        ),
        "HibernateTab" => (
            CommandGroup::TabsPanes,
            t(L10nKey::CmdHibernateTab).to_string(),
        ),
        "ReopenClosedTab" => (
            CommandGroup::TabsPanes,
            t(L10nKey::CmdReopenClosedTab).to_string(),
        ),
        "SplitRight" => (
            CommandGroup::TabsPanes,
            t(L10nKey::CmdSplitRight).to_string(),
        ),
        "SplitDown" => (
            CommandGroup::TabsPanes,
            t(L10nKey::CmdSplitDown).to_string(),
        ),
        "ToggleMaximizePane" => (CommandGroup::TabsPanes, t(L10nKey::CmdZoomPane).to_string()),
        "FocusNextPane" => (CommandGroup::TabsPanes, t(L10nKey::CmdNextPane).to_string()),
        "FocusPrevPane" => (
            CommandGroup::TabsPanes,
            t(L10nKey::CmdPreviousPane).to_string(),
        ),
        "FocusPaneLeft" => (
            CommandGroup::TabsPanes,
            t(L10nKey::CmdFocusPaneLeft).to_string(),
        ),
        "FocusPaneRight" => (
            CommandGroup::TabsPanes,
            t(L10nKey::CmdFocusPaneRight).to_string(),
        ),
        "FocusPaneUp" => (
            CommandGroup::TabsPanes,
            t(L10nKey::CmdFocusPaneUp).to_string(),
        ),
        "FocusPaneDown" => (
            CommandGroup::TabsPanes,
            t(L10nKey::CmdFocusPaneDown).to_string(),
        ),
        "ResizePaneLeft" => (
            CommandGroup::TabsPanes,
            t(L10nKey::CmdResizePaneLeft).to_string(),
        ),
        "ResizePaneRight" => (
            CommandGroup::TabsPanes,
            t(L10nKey::CmdResizePaneRight).to_string(),
        ),
        "ResizePaneUp" => (
            CommandGroup::TabsPanes,
            t(L10nKey::CmdResizePaneUp).to_string(),
        ),
        "ResizePaneDown" => (
            CommandGroup::TabsPanes,
            t(L10nKey::CmdResizePaneDown).to_string(),
        ),
        "SwapPaneNext" => (
            CommandGroup::TabsPanes,
            t(L10nKey::CmdSwapPaneNext).to_string(),
        ),
        "SwapPanePrev" => (
            CommandGroup::TabsPanes,
            t(L10nKey::CmdSwapPanePrevious).to_string(),
        ),
        // `NextTab` / `PrevTab` keep their names — `Config::keybindings` is
        // keyed by them — but what they do is open the MRU switcher, so that
        // is what the page calls them. "Next Tab" belongs to the action that
        // actually goes to the next tab.
        "NextTab" => (
            CommandGroup::TabsPanes,
            t(L10nKey::CmdRecentTabSwitcher).to_string(),
        ),
        "PrevTab" => (
            CommandGroup::TabsPanes,
            t(L10nKey::CmdRecentTabSwitcherReverse).to_string(),
        ),
        "SelectNextTab" => (CommandGroup::TabsPanes, t(L10nKey::CmdNextTab).to_string()),
        "SelectPrevTab" => (
            CommandGroup::TabsPanes,
            t(L10nKey::CmdPreviousTab).to_string(),
        ),
        "MoveTabLeft" => (
            CommandGroup::TabsPanes,
            t(L10nKey::CmdMoveTabLeft).to_string(),
        ),
        "MoveTabRight" => (
            CommandGroup::TabsPanes,
            t(L10nKey::CmdMoveTabRight).to_string(),
        ),
        "NewWorkspace" => (
            CommandGroup::Workspaces,
            t(L10nKey::CmdNewWorkspace).to_string(),
        ),
        "RenameWorkspace" => (
            CommandGroup::Workspaces,
            t(L10nKey::CmdRenameWorkspace).to_string(),
        ),
        "StopWorkspace" => (
            CommandGroup::Workspaces,
            t(L10nKey::CmdStopWorkspace).to_string(),
        ),
        "DeleteWorkspace" => (
            CommandGroup::Workspaces,
            t(L10nKey::CmdDeleteWorkspace).to_string(),
        ),
        "ToggleSwitcher" => (
            CommandGroup::Workspaces,
            t(L10nKey::CmdSwitchWorkspace).to_string(),
        ),
        "IncreaseFontSize" => (
            CommandGroup::View,
            t(L10nKey::AppMenuIncreaseFontSize).to_string(),
        ),
        "DecreaseFontSize" => (
            CommandGroup::View,
            t(L10nKey::AppMenuDecreaseFontSize).to_string(),
        ),
        "ResetFontSize" => (CommandGroup::View, t(L10nKey::CmdResetFontSize).to_string()),
        "ToggleFullscreen" => (
            CommandGroup::View,
            t(L10nKey::CmdEnterFullScreen).to_string(),
        ),
        "ToggleTabSidebar" => (
            CommandGroup::View,
            t(L10nKey::AppMenuTabBarPosition).to_string(),
        ),
        "ToggleLeftPanel" => (
            CommandGroup::View,
            t(L10nKey::AppMenuLeftSidebar).to_string(),
        ),
        "ToggleRightPanel" => (
            CommandGroup::View,
            t(L10nKey::AppMenuRightPanel).to_string(),
        ),
        "FindInTerminal" => (
            CommandGroup::Terminal,
            t(L10nKey::CmdFindInTerminal).to_string(),
        ),
        "FindNext" => (CommandGroup::Terminal, t(L10nKey::CmdFindNext).to_string()),
        "ToggleComposer" => (
            CommandGroup::Terminal,
            t(L10nKey::CmdToggleComposer).to_string(),
        ),
        "FindPrevious" => (
            CommandGroup::Terminal,
            t(L10nKey::CmdFindPrevious).to_string(),
        ),
        "ClearScrollback" => (
            CommandGroup::Terminal,
            t(L10nKey::CmdClearScrollback).to_string(),
        ),
        "CopyText" => (CommandGroup::Terminal, t(L10nKey::CmdCopy).to_string()),
        "CopyRaw" => (CommandGroup::Terminal, t(L10nKey::CmdCopyRaw).to_string()),
        "PasteText" => (CommandGroup::Terminal, t(L10nKey::CmdPaste).to_string()),
        "AlternatePaste" => (
            CommandGroup::Terminal,
            t(L10nKey::CmdAlternatePaste).to_string(),
        ),
        "OpenSshProfiles" => (
            CommandGroup::Ssh,
            t(L10nKey::CmdSshManageProfiles).to_string(),
        ),
        "RestartSshSession" => (CommandGroup::Ssh, t(L10nKey::CmdSshReconnect).to_string()),
        "ShowSshForwards" => (
            CommandGroup::Ssh,
            t(L10nKey::CmdSshPortForwarding).to_string(),
        ),
        "ForkAgentSession" => (CommandGroup::Agents, t(L10nKey::CmdForkSession).to_string()),
        "ForkAgentSessionRight" => (
            CommandGroup::Agents,
            t(L10nKey::KeybindForkSessionRight).to_string(),
        ),
        "ForkAgentSessionLeft" => (
            CommandGroup::Agents,
            t(L10nKey::KeybindForkSessionLeft).to_string(),
        ),
        "ForkAgentSessionDown" => (
            CommandGroup::Agents,
            t(L10nKey::KeybindForkSessionDown).to_string(),
        ),
        "ForkAgentSessionUp" => (
            CommandGroup::Agents,
            t(L10nKey::KeybindForkSessionUp).to_string(),
        ),
        "CopyAgentSessionId" => (
            CommandGroup::Agents,
            t(L10nKey::CmdCopySessionId).to_string(),
        ),
        "NewAgentTab" => (CommandGroup::Agents, t(L10nKey::CmdNewAgentTab).to_string()),
        "FocusNextAgentNeedingAttention" => (
            CommandGroup::Agents,
            t(L10nKey::CmdNextAgentNeedingAttention).to_string(),
        ),
        "NewWindow" => (
            CommandGroup::Application,
            t(L10nKey::CmdNewWindow).to_string(),
        ),
        "CloseWindow" => (
            CommandGroup::Application,
            t(L10nKey::CmdCloseWindow).to_string(),
        ),
        "OpenSettings" => (
            CommandGroup::Application,
            t(L10nKey::CmdSettings).to_string(),
        ),
        "ShowKeyboardShortcuts" => (
            CommandGroup::Application,
            t(L10nKey::CmdKeyboardShortcuts).to_string(),
        ),
        "About" => (
            CommandGroup::Application,
            t(L10nKey::CmdAboutTty7).to_string(),
        ),
        "CheckForUpdates" => (
            CommandGroup::Application,
            t(L10nKey::CmdCheckForUpdates).to_string(),
        ),
        "OpenDocumentation" => (
            CommandGroup::Application,
            t(L10nKey::CmdDocumentation).to_string(),
        ),
        "OpenDiscord" => (
            CommandGroup::Application,
            t(L10nKey::CmdJoinDiscord).to_string(),
        ),
        "ReportIssue" => (
            CommandGroup::Application,
            t(L10nKey::CmdReportIssue).to_string(),
        ),
        "HideApp" => (
            CommandGroup::Application,
            t(L10nKey::AppMenuHideApp).to_string(),
        ),
        "HideOthers" => (
            CommandGroup::Application,
            t(L10nKey::AppMenuHideOthers).to_string(),
        ),
        "ShowAll" => (
            CommandGroup::Application,
            t(L10nKey::AppMenuShowAll).to_string(),
        ),
        "MinimizeWindow" => (
            CommandGroup::Application,
            t(L10nKey::AppMenuMinimize).to_string(),
        ),
        "ZoomWindow" => (
            CommandGroup::Application,
            t(L10nKey::AppMenuZoom).to_string(),
        ),
        "Quit" => (
            CommandGroup::Application,
            t(L10nKey::CmdQuitTty7).to_string(),
        ),
        _ => return None,
    })
}

/// One action and the chords it answers to, split by where they came from:
/// the ones it ships with — its default, or the preset's in its place — and
/// the ones `config.json` names.
///
/// Kept apart because they install apart. gpui resolves two bindings on one
/// chord to the one added last, and a chord the user asked for by name has to
/// be that one, wherever its action sits in the table.
struct ActionChords {
    action: String,
    shipped: Vec<String>,
    configured: Vec<String>,
}

fn resolve_chords(
    preset: &str,
    prefix: &str,
    overrides: &std::collections::HashMap<String, KeybindingOverride>,
) -> Vec<ActionChords> {
    let mut resolved: Vec<ActionChords> = default_bindings()
        .into_iter()
        .map(|(action, key)| ActionChords {
            action: action.to_string(),
            shipped: [key]
                .into_iter()
                .filter(|k| !k.is_empty())
                .map(str::to_string)
                .collect(),
            configured: Vec::new(),
        })
        .collect();
    // A preset is a scheme rather than an addition: its chord takes the
    // default's place.
    for (action, key) in preset_bindings(preset, prefix) {
        if let Some(slot) = resolved.iter_mut().find(|s| s.action == action) {
            slot.shipped = vec![key];
        }
    }
    for (action, value) in overrides {
        let Some(slot) = resolved.iter_mut().find(|s| s.action == *action) else {
            // A hand-edited config.json with a typo used to vanish into this
            // branch. The Keybindings page lists every name that works.
            log::warn!("keybinding for unknown action {action:?} ignored");
            continue;
        };
        match value {
            // `""` has always been how a config retires a chord — the docs
            // spell `"AlternatePaste": ""`, and Settings wrote it for an action
            // whose chord was taken — so it goes on unbinding the action.
            KeybindingOverride::Add(key) if key.is_empty() => {
                slot.shipped.clear();
            }
            // A chord beside the ones the action has, not in place of them
            // (#868): `"NextTab": "cmd-shift-]"` is a second way to switch tabs,
            // and Ctrl+Tab going dead because of it was the bug.
            KeybindingOverride::Add(key) => {
                if !slot.shipped.iter().any(|k| same_chord(k, key)) {
                    slot.configured.push(key.clone());
                }
            }
            KeybindingOverride::Exact(keys) => {
                slot.shipped.clear();
                for key in keys {
                    if !key.is_empty() && !slot.configured.iter().any(|k| same_chord(k, key)) {
                        slot.configured.push(key.clone());
                    }
                }
            }
        }
    }
    resolved
}

/// Every chord as an `(action, chord)` pair, in the order the keymap is built
/// from: all the shipped chords, then all the configured ones, so that a
/// configured chord landing on another action's default is the binding that
/// wins it. An action with no chord at all has no pair.
fn flatten_chords(resolved: &[ActionChords]) -> Vec<(String, String)> {
    let pairs = |pick: fn(&ActionChords) -> &Vec<String>| {
        resolved
            .iter()
            .flat_map(move |s| pick(s).iter().map(|k| (s.action.clone(), k.clone())))
    };
    pairs(|s| &s.shipped)
        .chain(pairs(|s| &s.configured))
        .collect()
}

pub(crate) fn effective_bindings(cx: &App) -> Vec<(String, String)> {
    let cfg = cx.global::<Config>();
    flatten_chords(&resolve_chords(
        &cfg.keybinding_preset,
        &cfg.prefix,
        &cfg.keybindings,
    ))
}

/// Every action in table order with all of its chords, shipped first — an
/// empty list for an action that has none. What a page listing the actions
/// reads; the keymap reads [`effective_bindings`].
pub(crate) fn effective_chords(cx: &App) -> Vec<(String, Vec<String>)> {
    let cfg = cx.global::<Config>();
    resolve_chords(&cfg.keybinding_preset, &cfg.prefix, &cfg.keybindings)
        .into_iter()
        .map(|mut s| {
            s.shipped.append(&mut s.configured);
            (s.action, s.shipped)
        })
        .collect()
}

fn preset_bindings(preset: &str, prefix: &str) -> Vec<(String, String)> {
    match preset {
        "tmux" => tmux_preset(prefix),
        _ => Vec::new(),
    }
}

fn tmux_preset(prefix: &str) -> Vec<(String, String)> {
    let p = |key: &str| format!("{prefix} {key}");
    [
        ("NewTab", p("c")),
        ("CloseActiveTab", p("x")),
        ("SplitRight", p("%")),
        ("SplitDown", p("\"")),
        ("FocusPaneLeft", p("left")),
        ("FocusPaneRight", p("right")),
        ("FocusPaneUp", p("up")),
        ("FocusPaneDown", p("down")),
        ("ResizePaneLeft", p("ctrl-left")),
        ("ResizePaneRight", p("ctrl-right")),
        ("ResizePaneUp", p("ctrl-up")),
        ("ResizePaneDown", p("ctrl-down")),
        ("SwapPanePrev", p("{")),
        ("SwapPaneNext", p("}")),
        ("ToggleMaximizePane", p("z")),
        ("FocusNextPane", p("o")),
        ("FocusPrevPane", p(";")),
        // tmux's `next-window` / `previous-window`: straight to the
        // neighbour, no chooser. The MRU switcher it used to open never
        // committed here — the modifier it waits on is not held (#867).
        ("SelectNextTab", p("n")),
        ("SelectPrevTab", p("p")),
        ("ActivateTab1", p("1")),
        ("ActivateTab2", p("2")),
        ("ActivateTab3", p("3")),
        ("ActivateTab4", p("4")),
        ("ActivateTab5", p("5")),
        ("ActivateTab6", p("6")),
        ("ActivateTab7", p("7")),
        ("ActivateTab8", p("8")),
        ("ActivateTab9", p("9")),
    ]
    .into_iter()
    .map(|(a, k)| (a.to_string(), k))
    .collect()
}

pub(crate) fn effective_key(action: &str, cx: &App) -> Option<String> {
    effective_bindings(cx)
        .into_iter()
        .find(|(a, _)| a == action)
        .map(|(_, k)| k)
        .filter(|k| !k.is_empty())
}

/// The modifier hold that raises the tab-number badges — the one the tab
/// activation chords are really on.
///
/// It used to be assumed: a bare secondary, ⌘ on macOS and Ctrl everywhere
/// else. That is one platform right and one wrong. Off macOS the nine ship on
/// Alt, so holding Alt raised nothing while Ctrl, which activates no tab there
/// at all, lit up every number; and any rebind — `"ActivateTab1": "alt-1"` on
/// macOS is the same complaint the other way round — moved the chords out from
/// under a hint that stayed where it was. Read from `ActivateTab1`, the chord
/// the first badge is drawn for, and matched against the held modifiers
/// exactly, so a wrong modifier, or Shift added mid-hold, takes them down.
///
/// `None` when there is nothing to advertise: an unbound or retired
/// `ActivateTab1`, a chord with no modifier (a bare `1` is not a hold), a key
/// that is not the digit the badge would draw (`alt-q` does activate tab 1, but
/// holding Alt would then be promising a number pad no key answers), or the
/// tmux preset's two-chord `prefix 1` — a sequence has no hold to watch, and
/// raising the badges on the bare prefix would point at half a chord.
pub(crate) fn tab_badge_hold(cx: &App) -> Option<Modifiers> {
    let spec = effective_key("ActivateTab1", cx)?;
    let mut chords = spec.split_whitespace();
    let (chord, None) = (chords.next()?, chords.next()) else {
        return None;
    };
    let ks = Keystroke::parse_with_os(chord, cx.platform_info().os).ok()?;
    (ks.key == "1" && ks.modifiers.modified()).then_some(ks.modifiers)
}

pub(crate) fn spec_from_keystroke(ks: &Keystroke) -> Option<String> {
    if matches!(
        ks.key.as_str(),
        "shift" | "control" | "alt" | "platform" | "function" | "cmd" | "ctrl"
    ) {
        return None;
    }
    let m = &ks.modifiers;
    let mut parts: Vec<&str> = Vec::new();
    #[cfg(target_os = "macos")]
    {
        if m.platform {
            parts.push("secondary");
        }
        if m.control {
            parts.push("ctrl");
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        if m.control {
            parts.push("secondary");
        }
        if m.platform {
            parts.push("cmd");
        }
    }
    if m.alt {
        parts.push("alt");
    }
    if m.shift {
        parts.push("shift");
    }
    if m.function {
        parts.push("fn");
    }
    let mut spec = String::new();
    for part in parts {
        spec.push_str(part);
        spec.push('-');
    }
    spec.push_str(&ks.key);
    Some(spec)
}

/// The glyph a US keyboard prints on the shifted half of every key that is not
/// a letter — `shift-]` types `}`, `shift-2` types `@`.
const SHIFTED_GLYPHS: [(char, char); 21] = [
    ('`', '~'),
    ('1', '!'),
    ('2', '@'),
    ('3', '#'),
    ('4', '$'),
    ('5', '%'),
    ('6', '^'),
    ('7', '&'),
    ('8', '*'),
    ('9', '('),
    ('0', ')'),
    ('-', '_'),
    ('=', '+'),
    ('[', '{'),
    (']', '}'),
    ('\\', '|'),
    (';', ':'),
    ('\'', '"'),
    (',', '<'),
    ('.', '>'),
    ('/', '?'),
];

/// Rewrites one chord into the shape the platform actually delivers, or
/// returns `None` when it is already that shape.
///
/// Every gpui backend carries Shift as a modifier for letters only. Over the
/// digits and the punctuation it spends the modifier on the character instead
/// and clears the flag: that is the `chars_with_shift` branch on macOS,
/// `need_to_convert_to_shifted_key` in the Windows mapper, and Linux's "we
/// only include the shift for upper-case letters by convention". So the chord
/// written `secondary-shift-]` arrives as `secondary-}` and the two never
/// meet — the binding installs and is unreachable, which is why Ctrl+Shift+]
/// stopped cycling panes off macOS and why hand-editing a config to that
/// spelling changed nothing (#750).
///
/// Folding the spec the same way on the lookup side closes the round trip.
/// The recorder already writes `secondary-}`, because that is what
/// `spec_from_keystroke` was handed; a default or a config written the other
/// way now installs the very same binding, so both spellings keep working and
/// nobody's config comes undone. `secondary-shift-}`, which people also reach
/// for, folds onto it too — the flag is simply redundant there.
///
/// The table is US-layout, and a chord it does not know is left alone rather
/// than guessed at. It cannot be layout-aware from here: `?` is Shift+ß on a
/// German keyboard and Shift+, on a French one, and the only mapper that knows
/// which is gpui's. `make_binding` disables key equivalents when calling
/// `KeyBinding::load`; the mapper exists on Windows only,
/// where `MacKeyboardMapper` does no shift folding at all. So off a US layout
/// the fold is a widening that may not land: `ctrl-shift-4` picks up `ctrl-$`,
/// which on AZERTY is a key of its own. Harmless, because the fold only ever
/// adds — the spec as written stays bound whatever the layout — but it is why
/// this is not the last word on #750.
///
/// The fold is *added* to a spec's bindings, never substituted for them,
/// because the convention is not universal. Windows leaves Shift intact on
/// the numpad's `/ * + -`: `need_to_convert_to_shifted_key` lists the OEM
/// keys and `VK_0..VK_9` but none of `VK_DIVIDE`/`VK_MULTIPLY`/`VK_ADD`/
/// `VK_SUBTRACT`, so `get_keystroke_key` falls through to `get_key_from_vkey`
/// and Ctrl+Shift+numpad-/ really does arrive as `ctrl-shift-/`. Substituting
/// `ctrl-?` for it would unbind a chord that works today — the same class of
/// bug this is fixing. macOS clears the flag there anyway (the numpad glyph
/// is the same shifted or not, so `chars_with_shift` takes the branch that
/// drops it) and Linux names those keys `divide`/`multiply`/`add`/`subtract`,
/// which are not one character and never reach this.
fn fold_shift_into_glyph(chord: &str) -> Option<String> {
    let mut ks = parse_keystroke(chord).ok()?;
    if !ks.modifiers.shift {
        return None;
    }
    let mut chars = ks.key.chars();
    let (Some(key), None) = (chars.next(), chars.next()) else {
        return None;
    };
    if let Some((_, shifted)) = SHIFTED_GLYPHS.iter().find(|(plain, _)| *plain == key) {
        ks.key = shifted.to_string();
    } else if !SHIFTED_GLYPHS.iter().any(|(_, shifted)| *shifted == key) {
        return None;
    }
    ks.modifiers.shift = false;
    spec_from_keystroke(&ks)
}

/// `spec` with every chord folded, or `None` when it is already the shape the
/// platform delivers and there is nothing to add.
pub(crate) fn folded_spec(spec: &str) -> Option<String> {
    let mut folded = String::with_capacity(spec.len());
    let mut changed = false;
    for chord in spec.split_whitespace() {
        if !folded.is_empty() {
            folded.push(' ');
        }
        match fold_shift_into_glyph(chord) {
            Some(chord) => {
                folded.push_str(&chord);
                changed = true;
            }
            None => folded.push_str(chord),
        }
    }
    changed.then_some(folded)
}

/// Whether two specs claim the same keystroke — spelled the same way, or one
/// the fold of the other. `secondary-shift-]` and `secondary-}` are one chord
/// written twice, and a rebind that cannot see that leaves both installed on
/// it, where which one fires is arbitrary (#750).
pub(crate) fn same_chord(a: &str, b: &str) -> bool {
    if a == b {
        return true;
    }
    let (folded_a, folded_b) = (folded_spec(a), folded_spec(b));
    folded_a.as_deref() == Some(b)
        || folded_b.as_deref() == Some(a)
        || (folded_a.is_some() && folded_a == folded_b)
}

pub(crate) fn key_chords(spec: &str) -> Vec<Vec<String>> {
    spec.split_whitespace().map(key_tokens).collect()
}

/// How to write the "secondary" modifier for this platform — ⌘ on macOS, Ctrl
/// everywhere else. Anything spelling a shortcut out in the UI needs this
/// rather than a literal ⌘.
pub(crate) fn secondary_glyph() -> &'static str {
    if cfg!(target_os = "macos") {
        "⌘"
    } else {
        "Ctrl"
    }
}

pub(crate) fn key_tokens(spec: &str) -> Vec<String> {
    #[cfg(target_os = "macos")]
    const MODS: [(&str, &str); 6] = [
        ("secondary", "⌘"),
        ("cmd", "⌘"),
        ("ctrl", "⌃"),
        ("alt", "⌥"),
        ("shift", "⇧"),
        ("fn", "fn"),
    ];
    #[cfg(not(target_os = "macos"))]
    const MODS: [(&str, &str); 6] = [
        ("secondary", "Ctrl"),
        ("cmd", "Win"),
        ("ctrl", "Ctrl"),
        ("alt", "Alt"),
        ("shift", "Shift"),
        ("fn", "Fn"),
    ];
    let mut rest = spec;
    let mut tokens = Vec::new();
    'outer: loop {
        for (name, glyph) in MODS {
            let prefix = format!("{name}-");
            if let Some(stripped) = rest.strip_prefix(&prefix) {
                if !stripped.is_empty() {
                    tokens.push(glyph.to_string());
                    rest = stripped;
                    continue 'outer;
                }
            }
        }
        break;
    }
    // macOS names modifiers in one fixed order — ⌃ ⌥ ⇧ ⌘ — whatever order a
    // binding was written in; `secondary-shift-o` read `⌘ ⇧ O` here while the
    // menu bar said `⇧⌘O` for the same key.
    #[cfg(target_os = "macos")]
    tokens.sort_by_key(|glyph| {
        ["fn", "⌃", "⌥", "⇧", "⌘"]
            .iter()
            .position(|g| g == glyph)
            .unwrap_or(usize::MAX)
    });
    tokens.push(key_glyph(rest));
    tokens
}

fn key_glyph(key: &str) -> String {
    match key {
        // The glyph every hint in the app already draws for Return; this one
        // alone said `⏎`, so the same key looked different in a row's
        // shortcut and in the footer under it.
        "enter" | "return" => "↵".into(),
        "tab" => "⇥".into(),
        "space" => "Space".into(),
        "escape" | "esc" => "⎋".into(),
        "backspace" => "⌫".into(),
        "up" => "↑".into(),
        "down" => "↓".into(),
        "left" => "←".into(),
        "right" => "→".into(),
        "-" => "−".into(),
        other => other.to_uppercase(),
    }
}

fn keystroke_is_valid(s: &str) -> bool {
    let mut any = false;
    for token in s.split_whitespace() {
        any = true;
        if parse_keystroke(token).is_err() {
            return false;
        }
    }
    any
}

fn action_context(action: &str) -> Option<&'static str> {
    match action {
        "FindInTerminal" | "FindNext" | "FindPrevious" | "ClearScrollback" | "CopyText"
        | "CopyRaw" | "PasteText" | "ToggleComposer" => Some("Terminal"),
        // `alt_screen` is declared by the pane whenever a full-screen program
        // owns the grid, so this binding is simply absent there and Ctrl+V
        // carries on to the PTY as SYN (#677).
        "AlternatePaste" => Some("Terminal && !alt_screen"),
        _ => None,
    }
}

fn make_binding(action: &str, keystroke: &str, cx: &App) -> Option<KeyBinding> {
    let handler: Box<dyn gpui::Action> =
        if let Some(agent) = crate::ui::agent_launch::agent_for_launch_action(action) {
            Box::new(LaunchAgent { agent })
        } else {
            match action {
                "NewTab" => Box::new(NewTab),
                "NewWorkspace" => Box::new(NewWorkspace),
                "StopWorkspace" => Box::new(StopWorkspace),
                "DeleteWorkspace" => Box::new(DeleteWorkspace),
                "RenameWorkspace" => Box::new(RenameWorkspace),
                "ToggleSwitcher" => Box::new(ToggleSwitcher),
                "NewWindow" => Box::new(NewWindow),
                "CloseWindow" => Box::new(CloseWindow),
                "CloseActiveTab" => Box::new(CloseActiveTab),
                "RenameTab" => Box::new(RenameTab),
                "CloseOtherTabs" => Box::new(CloseOtherTabs),
                "CloseTabsToTheRight" => Box::new(CloseTabsToTheRight),
                "CopyWorkingDirectory" => Box::new(CopyWorkingDirectory),
                "MarkTabUnread" => Box::new(MarkTabUnread),
                "HibernateTab" => Box::new(HibernateTab),
                "ForkAgentSession" => Box::new(ForkAgentSession),
                "ForkAgentSessionRight" => Box::new(ForkAgentSessionRight),
                "ForkAgentSessionLeft" => Box::new(ForkAgentSessionLeft),
                "ForkAgentSessionDown" => Box::new(ForkAgentSessionDown),
                "ForkAgentSessionUp" => Box::new(ForkAgentSessionUp),
                "CopyAgentSessionId" => Box::new(CopyAgentSessionId),
                "NewAgentTab" => Box::new(NewAgentTab),
                "FocusNextAgentNeedingAttention" => Box::new(FocusNextAgentNeedingAttention),
                "SplitRight" => Box::new(SplitRight),
                "SplitDown" => Box::new(SplitDown),
                "FocusNextPane" => Box::new(FocusNextPane),
                "FocusPrevPane" => Box::new(FocusPrevPane),
                "FocusPaneLeft" => Box::new(FocusPaneLeft),
                "FocusPaneRight" => Box::new(FocusPaneRight),
                "FocusPaneUp" => Box::new(FocusPaneUp),
                "FocusPaneDown" => Box::new(FocusPaneDown),
                "ResizePaneLeft" => Box::new(ResizePaneLeft),
                "ResizePaneRight" => Box::new(ResizePaneRight),
                "ResizePaneUp" => Box::new(ResizePaneUp),
                "ResizePaneDown" => Box::new(ResizePaneDown),
                "SwapPaneNext" => Box::new(SwapPaneNext),
                "SwapPanePrev" => Box::new(SwapPanePrev),
                "NextTab" => Box::new(NextTab),
                "PrevTab" => Box::new(PrevTab),
                "SelectNextTab" => Box::new(SelectNextTab),
                "SelectPrevTab" => Box::new(SelectPrevTab),
                "MoveTabLeft" => Box::new(MoveTabLeft),
                "MoveTabRight" => Box::new(MoveTabRight),
                "ActivateTab1" => Box::new(ActivateTab1),
                "ActivateTab2" => Box::new(ActivateTab2),
                "ActivateTab3" => Box::new(ActivateTab3),
                "ActivateTab4" => Box::new(ActivateTab4),
                "ActivateTab5" => Box::new(ActivateTab5),
                "ActivateTab6" => Box::new(ActivateTab6),
                "ActivateTab7" => Box::new(ActivateTab7),
                "ActivateTab8" => Box::new(ActivateTab8),
                "ActivateTab9" => Box::new(ActivateTab9),
                "SelectWorkspace1" => Box::new(SelectWorkspace1),
                "SelectWorkspace2" => Box::new(SelectWorkspace2),
                "SelectWorkspace3" => Box::new(SelectWorkspace3),
                "SelectWorkspace4" => Box::new(SelectWorkspace4),
                "SelectWorkspace5" => Box::new(SelectWorkspace5),
                "SelectWorkspace6" => Box::new(SelectWorkspace6),
                "SelectWorkspace7" => Box::new(SelectWorkspace7),
                "SelectWorkspace8" => Box::new(SelectWorkspace8),
                "SelectWorkspace9" => Box::new(SelectWorkspace9),
                "IncreaseFontSize" => Box::new(IncreaseFontSize),
                "DecreaseFontSize" => Box::new(DecreaseFontSize),
                "ResetFontSize" => Box::new(ResetFontSize),
                "ReopenClosedTab" => Box::new(ReopenClosedTab),
                "ToggleMaximizePane" => Box::new(ToggleMaximizePane),
                "ToggleFullscreen" => Box::new(ToggleFullscreen),
                "ToggleTabSidebar" => Box::new(ToggleTabSidebar),
                "ToggleLeftPanel" => Box::new(ToggleLeftPanel),
                "ToggleRightPanel" => Box::new(ToggleRightPanel),
                "FindInTerminal" => Box::new(FindInTerminal),
                "FindNext" => Box::new(FindNext),
                "ToggleComposer" => Box::new(ToggleComposer),
                "FindPrevious" => Box::new(FindPrevious),
                "ClearScrollback" => Box::new(ClearScrollback),
                "CopyText" => Box::new(CopyText),
                "CopyRaw" => Box::new(CopyRaw),
                "PasteText" => Box::new(PasteText),
                "AlternatePaste" => Box::new(AlternatePaste),
                "OpenSettings" => Box::new(OpenSettings),
                "ShowKeyboardShortcuts" => Box::new(ShowKeyboardShortcuts),
                "About" => Box::new(About),
                "CheckForUpdates" => Box::new(CheckForUpdates),
                "OpenDocumentation" => Box::new(OpenDocumentation),
                "OpenDiscord" => Box::new(OpenDiscord),
                "ReportIssue" => Box::new(ReportIssue),
                "HideApp" => Box::new(HideApp),
                "HideOthers" => Box::new(HideOthers),
                "ShowAll" => Box::new(ShowAll),
                "MinimizeWindow" => Box::new(MinimizeWindow),
                "ZoomWindow" => Box::new(ZoomWindow),
                "ShowSshForwards" => Box::new(ShowSshForwards),
                "OpenSshProfiles" => Box::new(OpenSshProfiles),
                "RestartSshSession" => Box::new(RestartSshSession),
                "Quit" => Box::new(Quit),
                _ => return None,
            }
        };
    let context = action_context(action).map(|context| {
        gpui::KeyBindingContextPredicate::parse(context)
            .expect("valid action context")
            .into()
    });
    KeyBinding::load(keystroke, handler, context, false, None, cx).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::Action as _;

    /// The actions a keymap built from `action_bindings` dispatches for `keys`
    /// typed in `context`, in precedence order — the same lookup gpui performs
    /// on a real keypress.
    ///
    /// Bindings are asserted through this rather than through a mirror of the
    /// table, so a chord the app would not really install, or would install in
    /// another context, cannot pass.
    fn dispatched(effective: &[(String, String)], keys: &str, context: &str) -> Vec<&'static str> {
        let input: Vec<Keystroke> = keys
            .split(' ')
            .map(|k| parse_keystroke(k).expect("the typed keystroke parses"))
            .collect();
        dispatched_keystrokes(effective, &input, context)
    }

    /// The same lookup for input a spec cannot spell — the shifted-glyph shape
    /// a backend hands over for `secondary-shift-]` has no written form that
    /// `Keystroke::parse` turns back into it.
    fn dispatched_keystrokes(
        effective: &[(String, String)],
        input: &[Keystroke],
        context: &str,
    ) -> Vec<&'static str> {
        let cx = gpui::TestAppContext::single();
        cx.read(|cx| {
            let mut keymap = gpui::Keymap::default();
            keymap.add_bindings(action_bindings(effective, cx));
            let context = [gpui::KeyContext::parse(context).expect("the context parses")];
            keymap
                .bindings_for_input(input, &context)
                .0
                .iter()
                .map(|b| b.action().name())
                .collect()
        })
    }

    /// What every gpui backend delivers when the secondary modifier is held
    /// over a key whose shifted half is `glyph`: the glyph, and no shift flag —
    /// the modifier is already spent on the character.
    fn delivered_with_secondary(glyph: &str) -> Keystroke {
        Keystroke {
            modifiers: gpui::Modifiers {
                control: cfg!(not(target_os = "macos")),
                platform: cfg!(target_os = "macos"),
                shift: false,
                alt: false,
                function: false,
            },
            key: glyph.to_string(),
            key_char: None,
        }
    }

    #[test]
    fn every_action_has_an_authored_name() {
        crate::ui::i18n::set_locale("en");
        let missing: Vec<&str> = default_bindings()
            .into_iter()
            .map(|(a, _)| a)
            .filter(|a| authored_entry(a).is_none())
            .collect();
        assert!(
            missing.is_empty(),
            "these actions would render on the Keybindings page as a CamelCase \
             split of their internal name, in English, in a three-locale app: {missing:?}"
        );
    }

    #[test]
    fn the_page_speaks_the_same_words_as_the_rest_of_the_app() {
        crate::ui::i18n::set_locale("en");
        assert_eq!(action_entry("ToggleMaximizePane").1, "Zoom Pane");
        assert_eq!(action_entry("CloseActiveTab").1, "Close Pane / Tab");
        assert_eq!(action_entry("NewWindow").1, "New Window");
        assert_eq!(action_entry("CloseWindow").1, "Close Window");
        assert_eq!(action_entry("ClearScrollback").1, "Clear Scrollback");
        assert_eq!(action_entry("ToggleSwitcher").1, "Switch Workspace…");
        // The numbered families are templated, not nine strings per locale.
        assert_eq!(action_entry("ActivateTab3").1, "Go to Tab 3");
        assert_eq!(action_entry("SelectWorkspace7").1, "Go to Workspace 7");
        // And they land in the section they belong to.
        assert_eq!(action_entry("ActivateTab3").0, CommandGroup::TabsPanes);
        assert_eq!(action_entry("SelectWorkspace7").0, CommandGroup::Workspaces);
        assert_eq!(action_entry("ForkAgentSessionUp").0, CommandGroup::Agents);
    }

    #[test]
    fn new_window_ships_a_chord_only_where_one_is_free() {
        let mut effective: Vec<(String, String)> = default_bindings()
            .into_iter()
            .map(|(a, k)| (a.to_string(), k.to_string()))
            .collect();
        let default = effective
            .iter()
            .find(|(action, _)| action == "NewWindow")
            .map(|(_, key)| key.clone())
            .expect("NewWindow has to be listed here or it cannot be bound at all");
        assert_eq!(
            default,
            if cfg!(target_os = "macos") {
                "secondary-n"
            } else {
                ""
            },
            "macOS gets Cmd+N; off macOS this ships unbound on purpose"
        );

        // Ask the keymap rather than the table. A default can look bound and
        // dispatch nothing — gpui folds shift into the punctuation glyph, so
        // `secondary-shift-]` reached no action at all off macOS (#750). An
        // exact match is also the conflict check: any other action holding
        // this chord would show up in the list.
        if !default.is_empty() {
            assert_eq!(
                dispatched(&effective, &default, "Terminal"),
                vec![NewWindow::name_for_type()],
                "{default} must reach NewWindow, and nothing else may answer it"
            );
        }

        // Unbound still has to mean bindable. `set_binding` only writes into
        // slots this table already has, and `make_binding` is what turns the
        // name back into a dispatchable binding; miss either and the action
        // sits on the Keybindings page, takes a key, and does nothing — which
        // is the whole complaint in #710, not just the missing default.
        effective
            .iter_mut()
            .find(|(action, _)| action == "NewWindow")
            .expect("found once already")
            .1 = "ctrl-alt-shift-n".to_string();
        assert_eq!(
            dispatched(&effective, "ctrl-alt-shift-n", "Terminal"),
            vec![NewWindow::name_for_type()],
            "a chord the user assigns to NewWindow has to reach it"
        );
    }

    #[cfg(target_os = "macos")]
    const SECONDARY: &str = "⌘";
    #[cfg(not(target_os = "macos"))]
    const SECONDARY: &str = "Ctrl";
    #[cfg(target_os = "macos")]
    const SHIFT: &str = "⇧";
    #[cfg(not(target_os = "macos"))]
    const SHIFT: &str = "Shift";
    #[cfg(target_os = "macos")]
    const CTRL: &str = "⌃";
    #[cfg(not(target_os = "macos"))]
    const CTRL: &str = "Ctrl";

    #[test]
    fn key_tokens_maps_modifiers_to_glyphs() {
        assert_eq!(key_tokens("secondary-t"), vec![SECONDARY, "T"]);
        #[cfg(target_os = "macos")]
        assert_eq!(key_tokens("secondary-shift-d"), vec![SHIFT, SECONDARY, "D"]);
        #[cfg(not(target_os = "macos"))]
        assert_eq!(key_tokens("secondary-shift-d"), vec![SECONDARY, SHIFT, "D"]);
        assert_eq!(key_tokens("secondary-enter"), vec![SECONDARY, "↵"]);
    }

    #[test]
    fn key_tokens_keeps_the_minus_key_as_the_final_token() {
        assert_eq!(key_tokens("secondary--"), vec![SECONDARY, "−"]);
        assert_eq!(key_tokens("secondary-="), vec![SECONDARY, "="]);
        assert_eq!(key_tokens("secondary-,"), vec![SECONDARY, ","]);
    }

    #[test]
    fn key_chords_splits_a_sequence_into_keycap_groups() {
        assert_eq!(
            key_chords("ctrl-b n"),
            vec![
                vec![CTRL.to_string(), "B".to_string()],
                vec!["N".to_string()]
            ]
        );
        assert_eq!(key_chords("secondary-t"), vec![vec![SECONDARY, "T"]]);
    }

    #[test]
    fn every_default_action_has_a_binding_builder_or_is_unbound() {
        let cx = gpui::TestAppContext::single();
        cx.read(|cx| {
            for (action, key) in default_bindings() {
                if !key.is_empty() {
                    assert!(
                        keystroke_is_valid(key),
                        "default keystroke for {action} is invalid: {key:?}"
                    );
                    assert!(
                        make_binding(action, key, cx).is_some(),
                        "no make_binding arm for action {action}"
                    );
                }
            }
        })
    }

    #[test]
    fn every_action_has_a_binding_arm() {
        let cx = gpui::TestAppContext::single();
        cx.read(|cx| {
            // The sibling test above only reaches actions that ship with a default
            // keystroke, which leaves the unbound ones — the majority — free to be
            // listed in `default_bindings` with no `make_binding` arm behind them.
            // Nothing surfaces that: the action shows up in Settings, the user
            // assigns a key, and the key silently does nothing.
            for (action, _) in default_bindings() {
                assert!(
                    make_binding(action, "ctrl-f12", cx).is_some(),
                    "no make_binding arm for action {action}; \
                     anyone who binds a key to it in Settings gets nothing"
                );
            }
        })
    }

    #[test]
    fn tmux_preset_keystrokes_all_parse_and_map_to_actions() {
        let cx = gpui::TestAppContext::single();
        cx.read(|cx| {
            for (action, key) in tmux_preset("ctrl-b") {
                assert!(
                    keystroke_is_valid(&key),
                    "tmux preset keystroke for {action} does not parse: {key:?}"
                );
                assert!(
                    make_binding(&action, &key, cx).is_some(),
                    "tmux preset action {action} has no make_binding arm"
                );
            }
        })
    }

    #[test]
    fn shift_enter_prefix_binding_still_waits_for_its_second_key() {
        let cx = gpui::TestAppContext::single();
        cx.read(|cx| {
            let mut effective = default_bindings()
                .into_iter()
                .map(|(action, key)| (action.to_string(), key.to_string()))
                .collect::<Vec<_>>();
            effective
                .iter_mut()
                .find(|(action, _)| action == "OpenSettings")
                .unwrap()
                .1 = "shift-enter x".to_string();

            let mut keymap = gpui::Keymap::default();
            keymap.add_bindings(action_bindings(&effective, cx));
            let input = [parse_keystroke("shift-enter").unwrap()];
            let context = [gpui::KeyContext::parse("Terminal").unwrap()];
            let (_, pending) = keymap.bindings_for_input(&input, &context);

            assert!(pending, "the keymap must wait for the second key");

            let input = [
                parse_keystroke("shift-enter").unwrap(),
                parse_keystroke("x").unwrap(),
            ];
            let (matched, pending) = keymap.bindings_for_input(&input, &context);
            assert!(!pending);
            assert!(
                matched
                    .first()
                    .is_some_and(|binding| binding.action().partial_eq(&OpenSettings))
            );
        })
    }

    #[test]
    fn paste_ships_both_terminal_chords_off_macos_and_retires_together() {
        let effective: Vec<(String, String)> = default_bindings()
            .into_iter()
            .map(|(a, k)| (a.to_string(), k.to_string()))
            .collect();
        if cfg!(target_os = "macos") {
            assert!(
                !extra_keystrokes(&effective)
                    .iter()
                    .any(|(a, _)| *a == "PasteText"),
                "macOS pastes with Cmd+V; no extra chord should install there"
            );
            return;
        }
        assert_eq!(
            effective
                .iter()
                .find(|(a, _)| a == "PasteText")
                .map(|(_, k)| k.as_str()),
            Some("ctrl-shift-v")
        );
        assert_eq!(
            effective
                .iter()
                .find(|(a, _)| a == "CopyText")
                .map(|(_, k)| k.as_str()),
            Some("ctrl-shift-c")
        );
        assert!(extra_keystrokes(&effective).contains(&("PasteText", "shift-insert")));
        for key in ["ctrl-shift-v", "shift-insert"] {
            assert!(
                dispatched(&effective, key, "Terminal").contains(&PasteText::name_for_type()),
                "{key} must paste in a terminal"
            );
            assert!(
                !dispatched(&effective, key, "Workspace").contains(&PasteText::name_for_type()),
                "{key} is a terminal chord and must not paste outside one"
            );
        }
        // Plain Ctrl+V is `AlternatePaste`, and only where no full-screen
        // program is running: on the alternate screen the chord belongs to
        // that program and reaches it as SYN.
        assert!(
            dispatched(&effective, "ctrl-v", "Terminal").contains(&AlternatePaste::name_for_type()),
            "ctrl-v pastes at a prompt off macOS"
        );
        assert!(
            dispatched(&effective, "ctrl-v", "Terminal alt_screen").is_empty(),
            "a full-screen program owns ctrl-v"
        );
        assert!(
            dispatched(&effective, "ctrl-shift-v", "Terminal alt_screen")
                .contains(&PasteText::name_for_type()),
            "Ctrl+Shift+V is the paste that works on every screen"
        );
        // The two ways out, both of which the control-code validator has to
        // let through: retire the chord, or hand it the whole screen. Both are
        // one line in a `config.json`, so both are asserted against the whole
        // default table with that line applied — a bare one-entry table would
        // pass either assertion without the escape hatch working at all.
        let with = |action: &str, value: KeybindingOverride| {
            let overrides = std::collections::HashMap::from([(action.to_string(), value)]);
            flatten_chords(&resolve_chords("default", "ctrl-b", &overrides))
        };
        let retired = with("AlternatePaste", KeybindingOverride::Add(String::new()));
        assert!(
            dispatched(&retired, "ctrl-v", "Terminal").is_empty(),
            "an emptied AlternatePaste gives Ctrl+V back to the shell"
        );
        let everywhere = with("PasteText", KeybindingOverride::Add("ctrl-v".to_string()));
        for context in ["Terminal", "Terminal alt_screen"] {
            assert!(
                dispatched(&everywhere, "ctrl-v", context).contains(&PasteText::name_for_type()),
                "a user may put Paste itself on Ctrl+V and have it on every screen"
            );
        }
        let rebound = vec![("PasteText".to_string(), "ctrl-alt-v".to_string())];
        assert!(
            !extra_keystrokes(&rebound)
                .iter()
                .any(|(a, _)| *a == "PasteText"),
            "moving Paste off its default must retire Shift+Insert with it"
        );
    }

    #[test]
    fn action_context_matches_the_scope_make_binding_installs() {
        let cx = gpui::TestAppContext::single();
        cx.read(|cx| {
            let extra_actions = extra_keystrokes(
                &default_bindings()
                    .into_iter()
                    .map(|(a, k)| (a.to_string(), k.to_string()))
                    .collect::<Vec<_>>(),
            );
            let actions = default_bindings()
                .into_iter()
                .map(|(a, _)| a)
                .chain(extra_actions.into_iter().map(|(a, _)| a));
            for action in actions {
                let binding = make_binding(action, "f13", cx)
                    .unwrap_or_else(|| panic!("no arm for {action}"));
                let installed = binding.predicate().map(|p| p.to_string());
                assert_eq!(
                    installed.as_deref(),
                    action_context(action),
                    "{action} is installed in a different context than `action_context` reports"
                );
            }
        })
    }

    #[test]
    fn no_default_binding_sits_on_a_terminal_control_code() {
        // The invariant is "no default may *swallow* a terminal control code".
        // The exceptions are named and justified in
        // `control_code_binding_allowed`; anything new needs a fall-through of
        // its own to join them.
        for (action, spec) in default_bindings() {
            // Both spellings, because `push_binding` installs both. "Window
            // actions belong on ctrl-shift-*" stops being an escape over the
            // digits and the punctuation the moment the fold clears the Shift
            // again: `ctrl-shift-2` goes into the keymap as `ctrl-@` (#750).
            let folded = folded_spec(spec);
            for spelling in [Some(spec), folded.as_deref()].into_iter().flatten() {
                for chord in spelling.split_whitespace() {
                    parse_keystroke(chord).expect("default chords parse");
                    assert!(
                        !steals_a_control_code(chord)
                            || control_code_binding_allowed(action, chord),
                        "{action} is bound to {spec}, installed as {chord}, which the shell \
                         needs as a control code \
                         (Ctrl+[ is ESC, Ctrl+D is EOF, Ctrl+W deletes a word, \
                         Ctrl+2..8 are NUL/ESC/FS/GS/RS/US/DEL). \
                         Window actions belong on ctrl-shift-* off macOS — over a letter, \
                         where the platform keeps the Shift."
                    );
                }
            }
        }
    }

    /// F1..F12 now encode (#834), so a default sitting on one takes it away
    /// from the shell the same way a Ctrl binding takes a control code: gpui
    /// matches bindings before the pane's key handler runs, so a bound
    /// function key never reaches the PTY at all.
    ///
    /// The three that do are named here rather than left to be discovered,
    /// and each answers for the shell key it stands on:
    ///
    /// * F11 keeps fullscreen outright. It is the chord Windows Terminal,
    ///   GNOME Terminal and konsole all use, and no shell binds it —
    ///   PSReadLine's F keys are F3, F7 and F8.
    /// * F3 and Shift+F3 are Find Next / Previous, and they fall through:
    ///   with no find bar open the listener in `terminal::view` calls
    ///   `cx.propagate()`, so PSReadLine's CharacterSearch still gets the key.
    ///
    /// A fourth needs a fall-through of its own to join them.
    #[test]
    fn f11_is_the_only_default_on_a_bare_function_key() {
        let mut bound = Vec::new();
        for (action, spec) in default_bindings() {
            for chord in spec.split_whitespace() {
                let ks = parse_keystroke(chord).expect("default chords parse");
                if ks
                    .key
                    .strip_prefix('f')
                    .and_then(|n| n.parse::<u8>().ok())
                    .is_some_and(|n| (1..=24).contains(&n))
                {
                    bound.push((action, chord));
                }
            }
        }
        let expected: &[(&str, &str)] = if cfg!(target_os = "macos") {
            &[]
        } else {
            &[
                ("FindNext", "f3"),
                ("FindPrevious", "shift-f3"),
                ("ToggleFullscreen", "f11"),
            ]
        };
        bound.sort();
        let mut expected = expected.to_vec();
        expected.sort();
        assert_eq!(
            bound, expected,
            "a default on a function key hides it from the shell; \
             PSReadLine wants F3, F7 and F8, readline's `bind -x` any of them"
        );
    }

    #[test]
    fn every_default_chord_is_claimed_by_exactly_one_action() {
        // Per context, not globally: gpui resolves a keystroke by walking the
        // focus chain outwards, so a chord bound inside a narrow context and
        // again with no context is not a clash — the narrow one wins while
        // that element has focus and the global one applies everywhere else.
        // Two bindings sharing a chord *and* a context is still a bug,
        // because then which one fires is arbitrary.
        // `same_chord`, not string equality: `secondary-shift-]` and
        // `secondary-}` are two spellings of one keystroke and both install it.
        let mut seen: Vec<(&str, &str, Option<&'static str>)> = Vec::new();
        for (action, spec) in default_bindings() {
            if spec.is_empty() {
                continue;
            }
            let context = action_context(action);
            if let Some((other, _, _)) = seen
                .iter()
                .find(|(_, s, c)| same_chord(s, spec) && *c == context)
            {
                panic!("{action} and {other} both claim {spec} in context {context:?}");
            }
            seen.push((action, spec, context));
        }
    }

    #[test]
    fn spec_from_keystroke_round_trips_through_parse() {
        for spec in [
            "secondary-t",
            "secondary-shift-t",
            "secondary-alt-left",
            "ctrl-shift-tab",
            "secondary--",
        ] {
            let ks = parse_keystroke(spec).unwrap();
            let round = spec_from_keystroke(&ks).expect("real key produces a spec");
            let reparsed = parse_keystroke(&round).unwrap();
            assert_eq!(
                (reparsed.modifiers, reparsed.key),
                (ks.modifiers, ks.key),
                "round trip diverged for {spec}"
            );
        }
    }

    #[test]
    fn a_shifted_punctuation_chord_dispatches_however_it_is_spelled() {
        // Recording ⌘⇧] wrote `secondary-}` — correctly, since that is the
        // keystroke it was handed — while the default table and every config
        // in the wild spell the same chord `secondary-shift-]`. Only one of
        // the two used to reach the dispatcher (#750); all three do now.
        let typed = delivered_with_secondary("}");
        let recorded = spec_from_keystroke(&typed).expect("the recorder spells the chord");
        assert_eq!(recorded, "secondary-}");
        for spec in [recorded.as_str(), "secondary-shift-]", "secondary-shift-}"] {
            let effective = [("NextTab".to_string(), spec.to_string())];
            assert_eq!(
                dispatched_keystrokes(&effective, std::slice::from_ref(&typed), ""),
                vec![NextTab::name_for_type()],
                "{spec} never reaches the dispatcher"
            );
        }
    }

    #[test]
    fn the_default_pane_cycle_chord_reaches_its_action() {
        // `secondary-]` on macOS, `secondary-shift-]` everywhere else — and
        // either way the backend reports the chord as the bare glyph. The
        // second spelling installed a binding nothing could press.
        let effective: Vec<(String, String)> = default_bindings()
            .into_iter()
            .map(|(a, k)| (a.to_string(), k.to_string()))
            .collect();
        let glyph = if cfg!(target_os = "macos") { "]" } else { "}" };
        assert!(
            dispatched_keystrokes(&effective, &[delivered_with_secondary(glyph)], "")
                .contains(&FocusNextPane::name_for_type()),
            "the default pane-cycle chord dispatches nothing"
        );
    }

    #[test]
    fn folding_leaves_every_other_spelling_alone() {
        // Shift over a letter is a real modifier, and the named keys have no
        // shifted half at all — folding either would break far more than it
        // fixed.
        for spec in [
            "secondary-shift-t",
            "shift-enter",
            "shift-insert",
            "shift-tab",
            "ctrl-shift-up",
            "secondary-t",
            "secondary--",
            "ctrl-b n",
        ] {
            assert_eq!(folded_spec(spec), None, "{spec} was rewritten");
        }
        // A prefix chord folds per chord, not as a whole.
        assert_eq!(folded_spec("ctrl-b shift-5").as_deref(), Some("ctrl-b %"));
    }

    #[test]
    fn a_chord_the_platform_delivers_with_shift_intact_stays_bound() {
        // Windows leaves Shift on the numpad's `/ * + -` — they are absent
        // from gpui's `need_to_convert_to_shifted_key`, so Ctrl+Shift+numpad-/
        // arrives as `ctrl-shift-/` and the recorder writes exactly that.
        // Folding is an addition, never a substitution, so that spec has to
        // survive the treatment `secondary-shift-]` gets (#750).
        let numpad = Keystroke {
            modifiers: gpui::Modifiers {
                control: cfg!(not(target_os = "macos")),
                platform: cfg!(target_os = "macos"),
                shift: true,
                alt: false,
                function: false,
            },
            key: "/".to_string(),
            key_char: None,
        };
        assert_eq!(
            spec_from_keystroke(&numpad).as_deref(),
            Some("secondary-shift-/")
        );
        let effective = [("NextTab".to_string(), "secondary-shift-/".to_string())];
        assert_eq!(
            dispatched_keystrokes(&effective, std::slice::from_ref(&numpad), ""),
            vec![NextTab::name_for_type()],
            "the numpad chord lost the binding written for it"
        );
        // And the main row's `?`, which the same spec also stands for, still
        // reaches the action — the fold added it rather than taking over.
        assert_eq!(
            dispatched_keystrokes(&effective, &[delivered_with_secondary("?")], ""),
            vec![NextTab::name_for_type()],
        );
    }

    #[test]
    fn two_spellings_of_one_chord_are_recognised_as_one() {
        // What `assign_keybinding` leans on to displace the action already
        // holding a chord, whichever way either of them is written down.
        assert!(same_chord("secondary-shift-]", "secondary-}"));
        assert!(same_chord("secondary-}", "secondary-shift-]"));
        assert!(same_chord("secondary-shift-}", "secondary-shift-]"));
        assert!(same_chord("secondary-t", "secondary-t"));
        assert!(!same_chord("secondary-shift-]", "secondary-shift-["));
        assert!(!same_chord("secondary-}", "secondary-{"));
    }

    #[test]
    fn the_control_code_guard_follows_the_folded_chord() {
        // The half of the control-code rule that only warns. A chord carrying
        // Shift reads as safe on its own — `steals_a_control_code` recognises a
        // bare Ctrl and nothing else — and it *was* safe while nothing could
        // press it. The fold makes it live, and Ctrl+Shift+2 is delivered as
        // Ctrl+@, which is the NUL the program on the far end is waiting for.
        // So the guard is asked about the spelling that goes into the keymap
        // (#750); `action_bindings` runs exactly this pair.
        for (written, folded) in [
            ("ctrl-shift-2", per_platform("ctrl-@", "secondary-@")),
            ("ctrl-shift-6", per_platform("ctrl-^", "secondary-^")),
            ("ctrl-shift--", per_platform("ctrl-_", "secondary-_")),
            ("ctrl-shift-/", per_platform("ctrl-?", "secondary-?")),
        ] {
            assert!(
                !steals_a_control_code(written),
                "{written} reads as safe as written, which is why the fold has to be checked"
            );
            assert_eq!(folded_spec(written).as_deref(), Some(folded));
            assert!(
                steals_a_control_code(folded),
                "{written} is installed as {folded} and takes a control code"
            );
        }
    }

    #[test]
    fn a_key_the_us_table_does_not_know_is_left_alone() {
        // `SHIFTED_GLYPHS` is a US keyboard, and every other layout prints
        // something else on the shifted half: `?` is Shift+ß on a German
        // keyboard and Shift+, on a French one. The fold cannot know that, so
        // it declines rather than guesses — a key it has never seen keeps its
        // Shift, and the binding written for it stays exactly as written. That
        // is also what keeps the fold additive: it can widen a spec's reach but
        // never move it onto a chord the user did not ask for.
        for spec in [
            "secondary-shift-ß",
            "secondary-shift-é",
            "secondary-shift-ä",
            "secondary-shift-ç",
            "secondary-shift-ñ",
        ] {
            assert_eq!(folded_spec(spec), None, "{spec} was guessed at");
        }
        // And the recorded spelling, which is what a non-US layout actually
        // produces, needs no fold to reach the dispatcher in the first place.
        let effective = [("NextTab".to_string(), "secondary-?".to_string())];
        assert_eq!(
            dispatched_keystrokes(&effective, &[delivered_with_secondary("?")], ""),
            vec![NextTab::name_for_type()],
        );
    }

    #[test]
    fn spec_from_keystroke_ignores_a_lone_modifier() {
        let ks = parse_keystroke("secondary").unwrap();
        assert_eq!(spec_from_keystroke(&ks), None);
    }

    #[test]
    fn new_agent_tab_ships_a_chord_only_where_one_is_free() {
        let cx = gpui::TestAppContext::single();
        cx.read(|cx| {
            let mut effective: Vec<(String, String)> = default_bindings()
                .into_iter()
                .map(|(a, k)| (a.to_string(), k.to_string()))
                .collect();
            let default = effective
                .iter()
                .find(|(action, _)| action == "NewAgentTab")
                .map(|(_, key)| key.clone())
                .expect("NewAgentTab has to be listed or it cannot be bound");
            if cfg!(target_os = "macos") {
                assert_eq!(default, "secondary-shift-a");
                assert_eq!(
                    dispatched(&effective, &default, "Terminal"),
                    vec![NewAgentTab::name_for_type()],
                    "{default} must reach NewAgentTab, and nothing else may answer it"
                );
            } else {
                assert_eq!(default, "", "Ctrl+Shift+A is select-all off macOS");
            }

            // Each agent's own launch is a slot of its own, and a key bound to it
            // dispatches that agent.
            effective
                .iter_mut()
                .find(|(action, _)| action == "LaunchAgent:codex")
                .expect("every agent has a keymap slot")
                .1 = "ctrl-alt-shift-c".to_string();
            let mut keymap = gpui::Keymap::default();
            keymap.add_bindings(action_bindings(&effective, cx));
            let typed = [parse_keystroke("ctrl-alt-shift-c").unwrap()];
            let context = [gpui::KeyContext::parse("Terminal").unwrap()];
            let hits = keymap.bindings_for_input(&typed, &context).0;
            assert_eq!(hits.len(), 1);
            assert!(hits[0].action().partial_eq(&LaunchAgent {
                agent: tty7_core::core::cli_agent::CLIAgent::Codex,
            }));
            assert_eq!(action_entry("LaunchAgent:codex").0, CommandGroup::Agents);
        })
    }
}

#[cfg(test)]
mod gpui_tests {
    use super::*;
    use crate::core::config::Config;
    use gpui::TestAppContext;

    #[gpui::test]
    fn init_then_rebind_installs_the_merged_table(cx: &mut TestAppContext) {
        cx.update(|cx| {
            gpui::component::init(cx);
            cx.set_global(Config::default());
            init(cx);

            {
                let cfg = cx.global_mut::<Config>();
                cfg.keybinding_preset = "tmux".to_string();
                cfg.keybindings.insert(
                    "NewTab".to_string(),
                    KeybindingOverride::Add("secondary-shift-n".to_string()),
                );
            }
            rebind(cx);

            let eff = effective_bindings(cx);
            let keys_of = |action: &str| {
                eff.iter()
                    .filter(|(a, _)| a == action)
                    .map(|(_, k)| k.clone())
                    .collect::<Vec<_>>()
            };
            assert_eq!(keys_of("NewTab"), ["ctrl-b c", "secondary-shift-n"]);
            assert_eq!(keys_of("SplitRight"), ["ctrl-b %"]);

            cx.global_mut::<Config>().keybinding_preset = "default".to_string();
            rebind(cx);
            let eff = effective_bindings(cx);
            assert_eq!(
                eff.iter()
                    .find(|(a, _)| a == "SplitRight")
                    .map(|(_, k)| k.as_str()),
                Some(per_platform("secondary-d", "secondary-shift-d"))
            );
        });
    }

    #[gpui::test]
    fn the_keybinding_triple_only_moves_when_a_binding_does(cx: &mut TestAppContext) {
        cx.update(|cx| {
            gpui::component::init(cx);
            cx.set_global(Config::default());
            init(cx);

            let before = keybinding_config(cx);
            assert_eq!(
                keybinding_config(cx),
                before,
                "reading the config twice changes nothing"
            );

            // An unrelated edit leaves the triple alone — this is the gate the
            // watcher compares, and a sidebar drag must not rebind.
            cx.global_mut::<Config>().dim_inactive_panes = false;
            assert_eq!(keybinding_config(cx), before);

            // A real binding edit moves it.
            cx.global_mut::<Config>().keybindings.insert(
                "RenameTab".to_string(),
                KeybindingOverride::Add("ctrl-shift-r".to_string()),
            );
            assert_ne!(keybinding_config(cx), before);

            // So does the preset, and the prefix.
            let with_override = keybinding_config(cx);
            cx.global_mut::<Config>().keybinding_preset = "tmux".to_string();
            assert_ne!(keybinding_config(cx), with_override);
            let with_preset = keybinding_config(cx);
            cx.global_mut::<Config>().prefix = "ctrl-a".to_string();
            assert_ne!(keybinding_config(cx), with_preset);
        });
    }

    /// An app whose `config.json` reads `json`, keymap and all. Parsed from
    /// text rather than built in Rust, because the file is the surface #868 is
    /// about: what a hand-written line means is the thing under test.
    fn running_on_json(cx: &mut gpui::App, json: &str) {
        gpui::component::init(cx);
        cx.set_global(Config(
            serde_json::from_str(json).expect("the config parses"),
        ));
        init(cx);
    }

    /// What the live keymap dispatches for `keys` typed in a terminal, best
    /// match first — the first entry is the action a real keypress runs.
    fn fired(cx: &gpui::App, keys: &str) -> Vec<&'static str> {
        let input: Vec<Keystroke> = keys
            .split(' ')
            .map(|k| parse_keystroke(k).expect("the typed keystroke parses"))
            .collect();
        let context = [gpui::KeyContext::parse("Terminal").expect("the context parses")];
        cx.key_bindings()
            .borrow()
            .bindings_for_input(&input, &context)
            .0
            .iter()
            .map(|b| b.action().name())
            .collect()
    }

    #[gpui::test]
    fn a_chord_added_in_config_keeps_the_default_one(cx: &mut TestAppContext) {
        use gpui::Action as _;
        cx.update(|cx| {
            // The report, word for word: a second way to reach the tab switcher
            // took the first one away (#868).
            running_on_json(
                cx,
                r#"{"keybindings": {"NextTab": "ctrl-alt-]", "PrevTab": "ctrl-alt-["}}"#,
            );
            assert_eq!(
                fired(cx, "ctrl-tab").first(),
                Some(&NextTab::name_for_type()),
                "the default chord survives a chord added beside it"
            );
            assert_eq!(
                fired(cx, "ctrl-shift-tab").first(),
                Some(&PrevTab::name_for_type())
            );
            assert_eq!(
                fired(cx, "ctrl-alt-]").first(),
                Some(&NextTab::name_for_type()),
                "and the added chord works too"
            );
            assert_eq!(
                fired(cx, "ctrl-alt-[").first(),
                Some(&PrevTab::name_for_type())
            );
            assert_eq!(effective_key("NextTab", cx).as_deref(), Some("ctrl-tab"));
        });
    }

    #[gpui::test]
    fn an_empty_chord_still_unbinds_the_action(cx: &mut TestAppContext) {
        cx.update(|cx| {
            // What config files already say to retire a default — the docs spell
            // `"AlternatePaste": ""` — and what Settings used to write for an
            // action that lost its chord. It has to keep meaning "no key".
            running_on_json(cx, r#"{"keybindings": {"NextTab": ""}}"#);
            assert!(fired(cx, "ctrl-tab").is_empty());
            assert_eq!(effective_key("NextTab", cx), None);
        });
    }

    #[gpui::test]
    fn a_list_is_the_whole_set_of_chords_for_an_action(cx: &mut TestAppContext) {
        use gpui::Action as _;
        cx.update(|cx| {
            running_on_json(
                cx,
                r#"{"keybindings": {"NextTab": ["ctrl-alt-]", "ctrl-alt-n"]}}"#,
            );
            assert!(
                fired(cx, "ctrl-tab").is_empty(),
                "a list replaces the default rather than joining it"
            );
            for chord in ["ctrl-alt-]", "ctrl-alt-n"] {
                assert_eq!(
                    fired(cx, chord).first(),
                    Some(&NextTab::name_for_type()),
                    "{chord} is in the list"
                );
            }
        });
    }

    #[gpui::test]
    fn an_empty_list_unbinds_the_action(cx: &mut TestAppContext) {
        cx.update(|cx| {
            running_on_json(cx, r#"{"keybindings": {"NextTab": []}}"#);
            assert!(fired(cx, "ctrl-tab").is_empty());
            assert_eq!(effective_key("NextTab", cx), None);
        });
    }

    /// #901: Alt+1…9 is how vim switches tabs, and tty7's default was eating
    /// the whole row of them. Both halves of what the reporter wanted have to
    /// hold, and hold across a restart — the complaint was that the default
    /// "keeps coming back".
    #[gpui::test]
    fn alt_digits_can_be_moved_off_the_tab_actions_for_good(cx: &mut TestAppContext) {
        use gpui::Action as _;
        cx.update(|cx| {
            let default_chord = per_platform("secondary-1", "alt-1");
            running_on_json(
                cx,
                r#"{"keybindings": {"ActivateTab1": ["alt-shift-1"],
                                    "ActivateTab2": []}}"#,
            );
            assert!(
                fired(cx, default_chord).is_empty(),
                "a list replaces the shipped chord, so the digit reaches the shell"
            );
            assert!(
                fired(cx, per_platform("secondary-2", "alt-2")).is_empty(),
                "and an empty list leaves the action with no chord at all"
            );
            assert_eq!(
                fired(cx, "alt-shift-1").first(),
                Some(&ActivateTab1::name_for_type()),
                "the chord asked for in its place is live"
            );
            assert_eq!(effective_key("ActivateTab2", cx), None);

            // The half that reads as "老是自动恢复": whatever the app saves has
            // to load back as the same thing. A `Config` serialized and read
            // again is exactly what a restart does with `config.json`.
            let saved =
                serde_json::to_string(&**cx.global::<Config>()).expect("the config serializes");
            cx.set_global(Config(
                serde_json::from_str(&saved).expect("the saved config parses"),
            ));
            rebind(cx);
            assert!(
                fired(cx, default_chord).is_empty(),
                "the shipped chord must not come back across a save and reload"
            );
            assert_eq!(
                fired(cx, "alt-shift-1").first(),
                Some(&ActivateTab1::name_for_type()),
            );
            assert_eq!(effective_key("ActivateTab2", cx), None);
        });
    }

    /// The hold the tab-number badges answer to is the one the tab chords are
    /// on, not the platform's secondary: off macOS the nine ship on Alt, where
    /// a badge raised by Ctrl hints at nothing that switches a tab.
    #[gpui::test]
    fn the_badge_hold_follows_the_tab_chord(cx: &mut TestAppContext) {
        cx.update(|cx| {
            running_on_json(cx, "{}");
            let shipped = parse_keystroke(per_platform("secondary-1", "alt-1"))
                .expect("the shipped chord parses");
            assert_eq!(
                tab_badge_hold(cx),
                Some(shipped.modifiers),
                "the shipped hold is the shipped chord's modifier"
            );

            for (json, expected) in [
                (
                    r#"{"keybindings": {"ActivateTab1": ["alt-1"]}}"#,
                    Some(Modifiers::alt()),
                ),
                // Nothing to advertise: no chord at all, a chord with no
                // modifier to hold, a key that is not the digit the badge
                // draws, and the tmux preset's two-chord `prefix 1`.
                (r#"{"keybindings": {"ActivateTab1": []}}"#, None),
                (r#"{"keybindings": {"ActivateTab1": ["1"]}}"#, None),
                (r#"{"keybindings": {"ActivateTab1": ["alt-q"]}}"#, None),
                (r#"{"keybinding_preset": "tmux"}"#, None),
            ] {
                cx.set_global(Config(
                    serde_json::from_str(json).expect("the config parses"),
                ));
                assert_eq!(tab_badge_hold(cx), expected, "{json}");
            }
        });
    }

    #[gpui::test]
    fn a_chord_added_under_the_tmux_preset_joins_the_preset_chord(cx: &mut TestAppContext) {
        use gpui::Action as _;
        cx.update(|cx| {
            running_on_json(
                cx,
                r#"{"keybinding_preset": "tmux",
                    "keybindings": {"SelectNextTab": "ctrl-alt-]", "SplitRight": ["ctrl-alt-d"]}}"#,
            );
            // The preset is a scheme, and it still replaces the default chord.
            assert!(fired(cx, per_platform("secondary-shift-]", "ctrl-pagedown")).is_empty());
            // A chord added on top of it is added to the preset's chord.
            assert_eq!(
                fired(cx, "ctrl-b n").first(),
                Some(&SelectNextTab::name_for_type())
            );
            assert_eq!(
                fired(cx, "ctrl-alt-]").first(),
                Some(&SelectNextTab::name_for_type())
            );
            // The MRU switcher the preset no longer takes over keeps Ctrl+Tab.
            assert_eq!(
                fired(cx, "ctrl-tab").first(),
                Some(&NextTab::name_for_type())
            );
            // A list replaces the preset's chord the way it replaces a default.
            assert!(fired(cx, "ctrl-b %").is_empty());
            assert_eq!(
                fired(cx, "ctrl-alt-d").first(),
                Some(&SplitRight::name_for_type())
            );
        });
    }

    #[gpui::test]
    fn a_chord_the_user_adds_wins_over_a_default_already_on_it(cx: &mut TestAppContext) {
        use gpui::Action as _;
        cx.update(|cx| {
            // `ctrl-tab` is `NextTab`'s default, and `NewTab` sits above it in the
            // table. The keymap resolves a tie to the binding added last, so a
            // user chord laid down in table order lost to the default whenever
            // its action happened to come first — the line in config.json was
            // read and did nothing.
            running_on_json(cx, r#"{"keybindings": {"NewTab": "ctrl-tab"}}"#);
            assert_eq!(
                fired(cx, "ctrl-tab").first(),
                Some(&NewTab::name_for_type()),
                "the chord the user asked for is the one that runs"
            );
            assert_eq!(
                fired(cx, per_platform("secondary-t", "secondary-shift-t")).first(),
                Some(&NewTab::name_for_type()),
                "and NewTab keeps its own default"
            );
        });
    }

    #[cfg(target_os = "macos")]
    #[gpui::test]
    fn cmd_shift_w_closes_the_window_with_its_shells_still_running(cx: &mut TestAppContext) {
        use gpui::Action as _;
        cx.update(|cx| {
            running_on_json(cx, "{}");
            assert_eq!(
                fired(cx, "cmd-shift-w").first(),
                Some(&CloseWindow::name_for_type()),
                "Cmd+Q stops the server, so a Mac needs a chord that only puts tty7 away"
            );
        });
    }

    #[gpui::test]
    fn repeated_rebinds_do_not_grow_the_keymap(cx: &mut TestAppContext) {
        cx.update(|cx| {
            gpui::component::init(cx);
            cx.set_global(Config::default());
            init(cx);
            let size = cx.key_bindings().borrow().bindings().len();
            // A rebind that clears first stays the same size however many
            // times it runs; the append-only one this replaced grew by a full
            // table each call, and every keystroke walks the whole map (#548).
            for _ in 0..3 {
                rebind(cx);
            }
            assert_eq!(
                cx.key_bindings().borrow().bindings().len(),
                size,
                "rebind clears before it binds, so the map does not grow"
            );
        });
    }

    #[gpui::test]
    fn a_rebuild_keeps_the_bindings_tty7_did_not_install(cx: &mut TestAppContext) {
        cx.update(|cx| {
            // gpui-component binds the whole `Input` editing table — backspace
            // among it — from its own `init`, which runs once, before ours. A
            // rebuild replaces the map, so it has to carry them; without this
            // the first rebind left every text field in the app unable to
            // delete a character (#548).
            gpui::component::init(cx);
            cx.set_global(Config::default());

            let backspace = |cx: &App| {
                let typed = [parse_keystroke("backspace").unwrap()];
                cx.key_bindings()
                    .borrow()
                    .all_bindings_for_input(&typed)
                    .iter()
                    .map(|b| b.action().name().to_string())
                    .collect::<Vec<_>>()
            };

            let inherited = backspace(cx);
            assert!(
                !inherited.is_empty(),
                "gpui-component binds backspace before tty7 touches the keymap"
            );

            init(cx);
            assert_eq!(backspace(cx), inherited, "init must not drop them");
            rebind(cx);
            assert_eq!(backspace(cx), inherited, "nor may a rebind");
        });
    }
}

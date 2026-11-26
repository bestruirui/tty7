pub mod agent_launch;
pub mod app;
pub mod assets;
pub mod dialog;
pub mod forwards;
pub mod hints;
pub mod home;
pub mod host_ops;
pub mod host_registry;
pub mod i18n;
pub mod keymap;
pub mod local_link;
pub mod machine_mirror;
pub mod notice;
pub mod pane;
pub mod pane_drag;
pub mod path_display;
pub mod pending_pane;
pub mod perf;
pub mod prefill;
pub mod presets;
pub mod proc_usage;
pub mod prompt;
pub mod remote_connect;
pub mod remote_workspace;
pub mod reorder;
pub mod right_panel;
pub mod rounding;
pub mod scrollbar;
pub mod settings;
pub mod settings_window;
pub mod ssh_connect;
pub mod ssh_prompt;
pub mod switcher;
pub mod tab_sidebar;
pub mod theme;
pub mod title_bar;
pub mod tray;
pub mod tree_sync;
pub mod windows;

pub(crate) fn fuzzy_score(query: &str, text: &str) -> Option<i32> {
    let needle: Vec<char> = query
        .chars()
        .flat_map(char::to_lowercase)
        .filter(|c| !c.is_whitespace())
        .collect();
    if needle.is_empty() {
        return Some(0);
    }
    let hay: Vec<char> = text.chars().flat_map(char::to_lowercase).collect();
    if needle.len() > hay.len() {
        return None;
    }

    // Every place the first letter occurs is a place the match could begin,
    // and the best of them is the score. Matching greedily from the left
    // alone let an early stray letter decide the alignment: `worktree`
    // against `New Worktree Tab…` spent its `w` on "New" and scattered the
    // rest, scoring the title as a poor match of the very word it contains.
    let best = (0..hay.len())
        .filter(|&i| hay[i] == needle[0])
        .filter_map(|start| align(&needle, &hay, start))
        .max()?;

    let mut score = best;
    if hay == needle {
        score += 120;
    } else if hay.starts_with(&needle) {
        score += 50;
    }
    score -= (hay.len() as i32) / 6;
    Some(score)
}

/// `needle` as a subsequence of `hay` beginning at `start`, taken greedily
/// from there; `None` when the rest of it does not fit.
fn align(needle: &[char], hay: &[char], start: usize) -> Option<i32> {
    let mut qi = 0usize;
    let mut score = 0i32;
    let mut run = 0i32;
    let mut prev_hit = false;
    for (i, ch) in hay.iter().enumerate().skip(start) {
        if qi >= needle.len() {
            break;
        }
        if *ch != needle[qi] {
            prev_hit = false;
            run = 0;
            continue;
        }
        score += 1;
        let word_start = i == 0 || !hay[i - 1].is_alphanumeric();
        if word_start {
            score += 12;
        }
        if i == 0 {
            score += 10;
        }
        if prev_hit {
            run += 1;
            score += 6 + run.min(8);
        } else {
            run = 0;
        }
        prev_hit = true;
        qi += 1;
    }
    (qi == needle.len()).then_some(score)
}

/// The two answers a confirmation dialog gets, arranged the way macOS arranges
/// them: the action on the right, where Return lands and where every other app
/// on the machine puts it, and the safe answer on the left, marked as the
/// cancel so it also answers Escape, Space and the initial keyboard focus.
///
/// gpui renders answer 0 rightmost, so the action goes first — `Ok(0)` is
/// "they meant it" and everything else, including a dropped channel, is "leave
/// it alone".
pub(crate) fn confirm_answers(action: &str, keep: &str) -> [gpui::PromptButton; 2] {
    [
        gpui::PromptButton::ok(action),
        gpui::PromptButton::cancel(keep),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_action_answers_return_and_the_safe_one_answers_escape() {
        // gpui hands answer 0 to the platform first, and both NSAlert and
        // TaskDialog draw that one on the right and give it Return. Reversing
        // these two puts Delete under the mouse where Cancel belongs.
        let [action, keep] = super::confirm_answers("Delete", "Cancel");
        assert_eq!(action.label(), "Delete");
        assert!(!action.is_cancel(), "answer 0 has to keep Return");
        assert_eq!(keep.label(), "Cancel");
        assert!(keep.is_cancel(), "only a cancel answer is given Escape");
    }
}

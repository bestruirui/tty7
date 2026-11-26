use gpui::component::button::{Button, ButtonVariants as _};
use gpui::component::input::{Input, InputEvent};
use gpui::component::kbd::Kbd;
use gpui::component::menu::{
    ContextMenu, ContextMenuExt as _, DropdownMenu as _, PopupMenu, PopupMenuItem,
};
use gpui::component::tooltip::Tooltip;
use gpui::component::{ActiveTheme as _, Icon, IconName, Side, Sizable as _, h_flex, v_flex};
use gpui::{
    Animation, AnimationExt as _, AnyElement, App, Axis, Bounds, Context, Div, ExternalPaths,
    FontWeight, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, SharedString,
    Stateful, Window, canvas, deferred, div, ease_out_quint, linear_color_stop, linear_gradient,
    prelude::*, px,
};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use unicode_segmentation::UnicodeSegmentation as _;

use std::path::{Path, PathBuf};

use crate::core::actions::{
    CloseActiveTab, CloseOtherTabs, CloseTabsBelow, CopyAgentSessionId, CopyWorkingDirectory,
    ForkAgentSession, HibernateTab, MarkTabUnread, RenameTab, SelectWorkspace1, SelectWorkspace2,
    SelectWorkspace3, SelectWorkspace4, SelectWorkspace5, SelectWorkspace6, SelectWorkspace7,
    SelectWorkspace8, SelectWorkspace9, SplitDown, SplitRight,
};
use crate::core::cli_agent::CLIAgent;
use crate::core::config::Config;
use crate::core::group_key::{
    AutoKey, GroupId, GroupKey, PinnedGroup, WorkspaceGroups, auto_key, auto_names, pinned_names,
    place,
};
use crate::core::shells::DetectedShell;
use crate::daemon::protocol::ShellSpec;
use crate::terminal::git_status::GitStatusCache;
use crate::ui::app::{SpawnWhere, Tab, TitleBarRow as _, Tty7App};
use crate::ui::hints::tab_badge_label;
use crate::ui::i18n::{L10nKey, t, t_fmt};
use crate::ui::reorder::{self, Reorder, Surface};
use crate::ui::right_panel::RESIZE_HANDLE_WIDTH;
use crate::ui::title_bar::{RAIL_TILE_RADIUS, chrome_tile, chrome_tile_variant, hit_target};

pub(crate) const MIN_SIDEBAR_WIDTH: f32 = 180.;

const GRAB_HANDLE_W: f32 = 24.;

const ROW_GAP: f32 = 1.;

/// A single-line tab row: one line of `text_sm` and a little air.
const ROW_HEIGHT: f32 = 30.;

/// The air between two groups — what makes a group a group without a box.
const GROUP_GAP: f32 = 16.;

/// A group heading's line.
const HEADER_HEIGHT: f32 = 22.;

const META_REM: f32 = 11.5 / 16.;

/// The rail's side inset, for its header tiles, controls and rows alike.
const RAIL_INSET: f32 = 12.;

/// The workspace chip and the search field under the rail's header.
const HEAD_CONTROL_HEIGHT: f32 = 28.;
const HEAD_CONTROL_RADIUS: f32 = 7.;

fn tabular() -> gpui::FontFeatures {
    gpui::FontFeatures(std::sync::Arc::new(vec![("tnum".to_string(), 1)]))
}

/// The row chrome the text budget has to be measured around. These are the
/// numbers the layout below is built from, not a second guess at it — a row
/// that elides against a budget wider than it really has falls back to CSS
/// truncation, which drops the tail this whole module exists to keep.
mod row_metrics {
    /// `border_r_1` on the sidebar itself.
    pub(super) const BORDER: f32 = 1.;
    /// `px_3` on the scrolling list that holds the rows.
    pub(super) const LIST_PAD: f32 = 12.;
    /// `pl_2` + `pr_2` on the row.
    pub(super) const ROW_PAD: f32 = 8.;
    /// The avatar handed to `tab_avatar_plain`.
    pub(super) const AVATAR: f32 = 16.;
    /// `gap_2p5` between the row's children.
    pub(super) const GAP: f32 = 10.;
    /// The trailing status mark, when the row has one.
    pub(super) const STATUS: f32 = super::ROW_STATUS_SLOT;
    /// The ⌘N badge, when one is shown.
    pub(super) const BADGE: f32 = 20.;
    /// The zoom mark, when the tab has a pane zoomed over the others.
    pub(super) const ZOOM: f32 = 16.;
    /// `gap_1p5`, between a branch and the counts after it.
    pub(super) const META_GAP: f32 = 6.;
    /// `px_2` on a group header.
    pub(super) const HEADER_PAD: f32 = 8. + 8.;
    /// The chevron a header opens with, and the pin that marks a folder
    /// group: both `xsmall` icons, which resolve to 12px.
    pub(super) const HEADER_ICON: f32 = 12.;

    /// What a row can spend on text, before the badge is taken out.
    pub(super) const fn text_budget(width: f32) -> f32 {
        width - BORDER - 2. * LIST_PAD - 2. * ROW_PAD - AVATAR - GAP
    }

    /// What a group header can spend on its name and the branch beside it.
    /// The chevron, the pin and the folded row count come off at the call
    /// site, which knows whether they are drawn — an open group draws no
    /// chevron, and reserving one anyway elided its branch with 18px to spare.
    pub(super) const fn header_budget(width: f32) -> f32 {
        width - BORDER - 2. * LIST_PAD - HEADER_PAD
    }
}

/// The narrowest a group's heading is allowed to get: three or four capitals
/// and an ellipsis, which is still a name and not a stub.
const HEADER_NAME_FLOOR: f32 = 40.;

/// The narrowest a row's title is allowed to get before the working directory
/// beside it stops taking room, and the narrowest that path may be drawn at:
/// below this it is an ellipsis and a slash, which names no directory.
const ROW_TITLE_FLOOR: f32 = 48.;
const ROW_CWD_FLOOR: f32 = 24.;

/// What a sidebar row rendered, next to what it had to leave out, so the
/// hover card can be built by comparison instead of deriving the same strings
/// a second time — the two derivations have to agree, and the shortest way to
/// guarantee that is to only ever have one.
struct SidebarRowShown {
    /// The elided title, and the full string it came from. `None` when the
    /// row is showing a placeholder (`Shell 3`) rather than a real title,
    /// which nothing can expand.
    title: Option<(SharedString, SharedString)>,
    cwd: Option<(SharedString, SharedString)>,
}

#[derive(Clone)]
pub(crate) struct DragGroup;

impl Render for DragGroup {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

/// Every detail a sidebar row could not fit, collected so the hover card can
/// be rendered from cloneable data (an `AnyElement` cannot be cloned, but the
/// tooltip closure has to rebuild its content on every hover).
#[derive(Clone)]
struct SidebarInfo {
    /// Full path, when the row's title was elided.
    title: Option<SharedString>,
    /// Full working directory, when the row's second line was elided.
    cwd: Option<SharedString>,
    /// Remote host, when the avatar only shows a dot for it.
    host: Option<SharedString>,
}

impl Tty7App {
    pub(crate) fn sidebar_open(&self) -> bool {
        !self.tabs.is_empty() && !self.sidebar_collapsed
    }

    /// What the right panel has reserved, from the sidebar's point of view.
    pub(crate) fn right_panel_floor(&self) -> f32 {
        if self.right_panel_open() {
            crate::ui::right_panel::MIN_WIDTH
        } else {
            0.
        }
    }

    pub(crate) fn sidebar_max_px(&self, window: &Window) -> f32 {
        crate::ui::app::side_panel_max(
            window.viewport_size().width.as_f32(),
            MIN_SIDEBAR_WIDTH,
            self.right_panel_floor(),
        )
    }

    /// How wide the sidebar is drawn, given the live cell and the cap the rest
    /// of the window leaves it. Read here rather than clamped at each caller so
    /// the document column's budget and the sidebar itself can never disagree
    /// about how much width is already spoken for.
    pub(crate) fn sidebar_px(&self, window: &Window) -> f32 {
        self.sidebar_width
            .get()
            .clamp(MIN_SIDEBAR_WIDTH, self.sidebar_max_px(window))
    }

    pub(crate) fn tab_sidebar(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + use<> {
        let active = self.active;
        let sf = cx.global::<crate::ui::presets::Surfaces>().rail;
        let show_badges = self.mod_hint_badges;
        let width = self.sidebar_px(window);
        let query = self.sidebar_search.read(cx).value().trim().to_lowercase();
        // Blanked here, written again from paint: a row filtered out by the
        // search — or hidden with its collapsed group — must leave no rectangle
        // behind for a pane to be dropped between.
        *self.sidebar_slots.borrow_mut() = vec![Bounds::default(); self.tabs.len()];
        // Read before it is blanked: the rectangles a tab held over the
        // sidebar is measured against are the ones drawn last frame, the
        // same way a pane dropped here is. Blanked and written again below
        // so a group that folds or filters away stops accepting drops.
        let over_group = self.sidebar_regroup_target(window);
        // Taken rather than read, for the same reason: last frame's divider is
        // what a drag is measured against, and the one drawn below replaces it.
        let divider_last = self.sidebar_divider.take();
        let lifting_row = crate::ui::reorder::dragged_sidebar_tab(&self.reorder).is_some();
        // Offered every frame the pointer is over a group, and cleared with
        // the rest of the drag's pending state on the frames it is not — so
        // letting go anywhere else drops on nothing.
        if let Some(target) = over_group {
            crate::ui::reorder::set_regroup(&self.reorder, target);
        }
        self.sidebar_group_slots.borrow_mut().clear();
        // Every group drops itself when its rows filter out, so a query that
        // matches nothing left the sidebar showing only its own search box.
        let mut any_rows = false;

        let mut list = v_flex()
            .id("tab-sidebar-list")
            // A folder dragged in from Finder is pinned as a group of its own.
            .drag_over::<ExternalPaths>(|s, _, _, cx| s.bg(cx.theme().drag_border.opacity(0.06)))
            .on_drop(cx.listener(|this, paths: &ExternalPaths, _window, cx| {
                this.pin_dropped_folders(paths.paths(), cx)
            }))
            .track_scroll(&self.sidebar_scroll)
            .flex_1()
            .min_h_0()
            .w_full()
            .overflow_y_scroll()
            .px_3()
            .pb(px(20.))
            .gap(px(GROUP_GAP));

        let keys: Rc<Vec<Option<GroupKey>>> = Rc::new(self.sidebar_group_keys(cx));
        let groups = &self.sidebar_groups;
        let sections = sidebar_sections(&keys, groups);
        // A search outranks a fold. Typing something that matches a row inside
        // a folded group has to show that row — a box that says nothing
        // matches while the match sits behind a chevron is just lying.
        let folds_apply = query.is_empty();

        // ⌘N runs ActivateTabN, which goes through `activate_visual` — the
        // Nth row as the sidebar lays it out, not the Nth tab in `self.tabs`.
        // The badge has to be read off the same order or it names a chord that
        // opens a different tab, so take it from `visual_tab_order` rather than
        // flattening `sections` a second time here.
        let badge_pos: Vec<usize> = {
            let mut pos = vec![0usize; self.tabs.len()];
            for (n, i) in self.visual_tab_order(cx).into_iter().enumerate() {
                pos[i] = n;
            }
            pos
        };

        let visible_by_section: Vec<Vec<usize>> = sections
            .iter()
            .map(|s| {
                s.tabs
                    .iter()
                    .copied()
                    .filter(|&i| {
                        query.is_empty()
                            || self
                                .tab_label(&self.tabs[i], i, Some(window), cx)
                                .to_lowercase()
                                .contains(&query)
                            || self.tabs[i]
                                .leaf_title(Some(window), cx)
                                .to_lowercase()
                                .contains(&query)
                    })
                    .collect()
            })
            .collect();

        let pointer = window.mouse_position();
        // The row text is measured against real glyphs before it is elided:
        // `text_sm` is 0.875rem and `text_xs` 0.75rem, resolved here so the
        // measurement and the render use the same sizes and family.
        let font = gpui::Font {
            family: cx.theme().font_family.clone(),
            features: Default::default(),
            fallbacks: None,
            weight: Default::default(),
            style: Default::default(),
        };
        // The active row renders its title at `FontWeight::MEDIUM`, which is
        // wider than the regular weight in any proportional face. Measuring
        // it as regular would let the one row the user is looking at overflow
        // into the truncation this is here to avoid.
        let title_font_active = gpui::Font {
            weight: FontWeight::MEDIUM,
            ..font.clone()
        };
        let rem = window.rem_size().as_f32();
        // Measure with the same interface scale used to paint the header.
        let header_size = rem * META_REM;
        let meta_font = gpui::Font {
            features: tabular(),
            ..font.clone()
        };
        let header_font = gpui::Font {
            weight: FontWeight::MEDIUM,
            ..meta_font.clone()
        };
        let rail_fill: gpui::Hsla = gpui::rgb(sf.base).into();
        // A pinned group is drawn with no tabs in it — it is kept, and the
        // row it grows says how to fill it — except while a search is
        // narrowing the list, where a group with nothing matching is noise.
        let rendered = |ix: &usize| {
            !visible_by_section[*ix].is_empty()
                || (query.is_empty() && sections[*ix].pinned().is_some())
        };
        // Pinned headers reorder among themselves, by hand; that order is the
        // workspace's and is written back whole when a drag lands.
        let pinned_slots: Vec<usize> = (0..sections.len())
            .filter(|&ix| sections[ix].pinned().is_some())
            .filter(rendered)
            .collect();
        let pinned_rects: Rc<RefCell<Vec<Bounds<Pixels>>>> =
            Rc::new(RefCell::new(vec![Bounds::default(); pinned_slots.len()]));
        let pinned_preview = reorder::preview(
            &self.reorder,
            &Surface::PinnedGroups,
            pinned_slots.len(),
            pointer,
        );
        let pinned_display: Vec<usize> = match &pinned_preview {
            Some(p) => {
                let all: Vec<GroupId> = groups.pinned.iter().map(|g| g.id).collect();
                let shown: Vec<GroupId> = pinned_slots
                    .iter()
                    .filter_map(|&ix| sections[ix].pinned())
                    .collect();
                if let Some(order) = reordered_pinned(&all, &shown, p.from, p.target) {
                    reorder::set_pending(&self.reorder, &Surface::PinnedGroups, order);
                }
                p.order.clone()
            }
            None => (0..pinned_slots.len()).collect(),
        };
        // Every auto group draws a header, and a header is what there is to
        // grab, so these are the slots the auto-group surface runs over.
        // Ungrouped is excluded: it is where the keyless tabs fall, and it
        // always sits last.
        let keyed_slots: Vec<usize> = (0..sections.len())
            .filter(|&ix| {
                sections[ix]
                    .key
                    .as_ref()
                    .is_some_and(|k| k.auto().is_some())
            })
            .filter(rendered)
            .collect();
        let keyed_groups = keyed_slots.len();
        let group_slots: Rc<RefCell<Vec<Bounds<Pixels>>>> =
            Rc::new(RefCell::new(vec![Bounds::default(); keyed_groups]));
        let group_preview = reorder::preview(
            &self.reorder,
            &Surface::SidebarGroups,
            keyed_groups,
            pointer,
        );
        let keyed_keys: Vec<AutoKey> = keyed_slots
            .iter()
            .filter_map(|&ix| sections[ix].key.as_ref()?.auto().cloned())
            .collect();
        let slot_display: Vec<usize> = match &group_preview {
            Some(p) => {
                if let (Some(from), Some(to)) = (keyed_keys.get(p.from), keyed_keys.get(p.target))
                    && let Some(order) = regrouped_order(&keys, from, to)
                {
                    reorder::set_pending(&self.reorder, &Surface::SidebarGroups, order);
                }
                p.order.clone()
            }
            None => (0..keyed_groups).collect(),
        };
        let mut blocks: Vec<(Option<HeaderSlot>, usize)> = pinned_display
            .into_iter()
            .map(|slot| (Some(HeaderSlot::Pinned(slot)), pinned_slots[slot]))
            .collect();
        let first_unpinned = blocks.len();
        blocks.extend(
            slot_display
                .into_iter()
                .map(|slot| (Some(HeaderSlot::Auto(slot)), keyed_slots[slot])),
        );
        blocks.extend(
            (0..sections.len())
                .filter(|&ix| sections[ix].key.is_none())
                .filter(rendered)
                .map(|ix| (None, ix)),
        );
        // The divider: kept groups above it, derived ones below. Drawn
        // whenever something is kept, and while an auto header is in the air
        // even when nothing is yet — carrying it above the divider is how it
        // gets pinned, so there has to be a divider to carry it above.
        let lifted_auto = reorder::dragged_auto_group(&self.reorder);
        let show_divider = !groups.pinned.is_empty() || lifted_auto.is_some();
        let pin_hover = lifted_auto.is_some()
            && divider_last.is_some_and(|d| pointer.y < d.origin.y + d.size.height);
        if pin_hover {
            reorder::set_pin(&self.reorder);
        }
        let divider_lit = pin_hover || over_group == Some(reorder::Regroup::ToAuto);
        let divider_zone = groups.pinned.is_empty();
        let mut divider_drawn = false;
        // Whether the tab in the air is kept in a pinned group — then
        // everything below the divider is somewhere it can go.
        let lifted_is_kept = reorder::dragged_sidebar_tab(&self.reorder).is_some_and(|id| {
            self.tabs
                .iter()
                .find(|t| t.tree_id.get() == id)
                .and_then(|t| t.group.get())
                .is_some_and(|g| groups.contains(g))
        });
        let workspace_is_local = !crate::core::session::WorkspaceStore::all(cx)
            .get(self.workspace)
            .is_some_and(|w| w.is_remote());

        // A reveal whose row is not drawn this frame (folded away, or the tab
        // gone) is dropped, so it cannot fire when the row turns up later.
        let reveal = self.sidebar_reveal.get();
        let mut reveal_drawn = false;
        for (n, (group_slot, group_ix)) in blocks.into_iter().enumerate() {
            if n == first_unpinned && show_divider {
                list = list.child(self.sidebar_divider(divider_lit, divider_zone, cx));
                divider_drawn = true;
            }
            let section = &sections[group_ix];
            let group_key = section.key.clone();
            // Only a group that draws a header can be folded — there is
            // nothing to click otherwise, and the one headerless section (the
            // list below the pinned groups, when nothing else is there) must
            // never answer to Ungrouped's fold.
            let folded =
                section.name.is_some() && folds_apply && groups.is_folded(group_key.as_ref());
            let mut rows: Vec<ContextMenu<Stateful<Div>>> = Vec::new();
            // The header keeps counting every row the group has; folding only
            // stops them being drawn. Nothing downstream then registers a
            // rectangle for them, which is what keeps a pane from being
            // dropped into a group that is shut.
            //
            // No exception for the active tab. A fold that leaves one row
            // hanging under a shut chevron, with the header counting rows
            // that are not there, reads as a list that failed to load. The
            // cost is that ⌘T inside a folded group — `spawn_group` seeds
            // the new tab with the group it came from — puts the new tab
            // behind the chevron: the pane area shows the fresh shell and the
            // header count goes up, but the row waits for the group to open.
            let row_count = visible_by_section[group_ix].len();
            let visible: Vec<usize> = match folded {
                true => Vec::new(),
                false => visible_by_section[group_ix].clone(),
            };
            let visible_tabs: Vec<usize> = visible.clone();
            let row_slots: Rc<RefCell<Vec<Bounds<Pixels>>>> =
                Rc::new(RefCell::new(vec![Bounds::default(); visible.len()]));
            let row_preview = reorder::preview(
                &self.reorder,
                &Surface::SidebarRows(group_key.clone()),
                visible.len(),
                pointer,
            );
            for (slot, i) in visible.into_iter().enumerate() {
                let badge_pos = badge_pos[i];
                let tab = &self.tabs[i];
                reveal_drawn |= reveal == Some(tab.tree_id.get());
                let is_active = i == active;
                let ssh_dot = self.tab_ssh_dot(tab, cx);
                let asleep = tab.is_asleep();
                let agent = tab.agent(cx).or_else(|| tab.asleep_agent());
                let agent_status = tab.agent_status(cx);
                let agent_unread = tab.agent_unread_count(cx);
                let badge_extra = if show_badges && badge_pos < 9 {
                    row_metrics::BADGE + row_metrics::GAP
                } else {
                    0.
                };
                let zoomed = self.tab_is_zoomed(i);
                let zoom_extra = if zoomed {
                    row_metrics::ZOOM + row_metrics::GAP
                } else {
                    0.
                };
                // The sleep mark takes the zoom mark's size; a sleeping tab
                // has no panes on screen to zoom, so the two never share a row.
                let zoom_extra = if asleep {
                    zoom_extra + row_metrics::ZOOM + row_metrics::GAP
                } else {
                    zoom_extra
                };
                // Status rides at the row's trailing end, where the badge
                // would be; the badge wins while the modifier is held.
                let status_dot = match show_badges && badge_pos < 9 {
                    true => None,
                    false => self.row_status_dot(agent_status, agent_unread, rail_fill),
                };
                let status_extra = match status_dot.is_some() {
                    true => row_metrics::STATUS + row_metrics::GAP,
                    false => 0.,
                };
                // Elision is measured against this budget so the label and
                // branch never wrap or overflow into CSS truncation.
                let label_avail =
                    (row_metrics::text_budget(width) - badge_extra - zoom_extra - status_extra)
                        .max(48.);
                let title_size = 0.875 * rem;
                let meta_size = META_REM * rem;
                let title_font = if is_active { &title_font_active } else { &font };
                // Title: the *full* label, elided further down once the
                // working directory beside it has said how much of the line
                // it wants. A wide sidebar shows the whole thing and a narrow
                // one keeps whichever end identifies it — the tail for a
                // path, both edges for anything else. A fixed segment cap
                // (`short_title`) would elide even when the row has room, so
                // only the width may decide here.
                //
                // `full_title` is the unelided string the card can expand
                // back to; `None` means the row is showing a placeholder that
                // no card can improve on.
                //
                // The ladder the title bar and the switcher climb, read here for
                // the name and not for the shortening: this column measures in
                // pixels and lets a card expand the row back to the whole
                // string, so it wants what `label_of` would have cut down
                // rather than the cut. A renamed tab is elided like anything
                // else — and so the card has to be able to spell it back out.
                let (title_text, full_title) = match self.full_tab_label(tab, Some(window), cx) {
                    Some(raw) => {
                        let full = SharedString::from(raw);
                        (full.clone(), Some(full))
                    }
                    // Nothing to expand: the row is naming an unnamed shell,
                    // not hiding a title behind an ellipsis.
                    None => {
                        let placeholder = SharedString::from(t_fmt(
                            L10nKey::TabUnnamedShell,
                            &[("n", &((i + 1).to_string()))],
                        ));
                        (placeholder, None)
                    }
                };
                let mut cwd_shown: Option<(SharedString, SharedString)> = None;
                let cwd_full: Option<SharedString> = tab
                    .pane
                    .focused_or_first(window, cx)
                    .and_then(|leaf| {
                        let leaf = leaf.read(cx);
                        Some((leaf.effective_cwd()?, leaf.display_home(cx)))
                    })
                    .map(|(cwd, home)| {
                        let text = cwd.display().to_string();
                        SharedString::from(abbreviate_home(&text, home.as_deref()).into_owned())
                    })
                    .filter(|full| full.as_ref() != title_text.as_ref());
                // The path takes what it needs up to half the line and the
                // title keeps the rest — the same split a group header makes
                // with the branch beside its heading. Flex would hand the
                // overflow to the two of them in proportion to what each
                // asked for, which cuts the longer string hardest.
                let cwd_want = cwd_full.as_ref().map(|full| {
                    row_metrics::META_GAP
                        + measure_text(&window.text_system(), &font, meta_size, full)
                });
                let title_avail = match cwd_want {
                    Some(want) => (label_avail - want.min(label_avail * 0.5)).max(ROW_TITLE_FLOOR),
                    None => label_avail,
                };
                let shown_title = elide_label(
                    &window.text_system(),
                    title_font,
                    title_size,
                    &title_text,
                    title_avail,
                );
                if let Some(full) = cwd_full {
                    // Measured against what the title actually took, not what
                    // it was allowed to: a short title hands the slack back
                    // instead of leaving the path elided around a gap.
                    let avail = (label_avail
                        - measure_text(
                            &window.text_system(),
                            title_font,
                            title_size,
                            &shown_title,
                        )
                        - row_metrics::META_GAP)
                        .max(0.);
                    if avail >= ROW_CWD_FLOOR {
                        let shown = elide_path_keep_tail(
                            &window.text_system(),
                            &font,
                            meta_size,
                            &full,
                            avail,
                        );
                        cwd_shown = Some((shown, full));
                    }
                }
                let rename_input = self
                    .renaming
                    .as_ref()
                    .filter(|r| r.tab == tab.tree_id.get())
                    .map(|r| r.input.clone());

                let shown = SidebarRowShown {
                    title: full_title.map(|full| (shown_title.clone(), full)),
                    cwd: cwd_shown.clone(),
                };
                let info = self.sidebar_info(tab, window, cx, &shown);
                // Colors are captured by value so the tooltip builder (which
                // borrows no app state) can style the card on its own.
                let muted = cx.theme().muted_foreground;
                let label_region = match rename_input {
                    Some(input) => div()
                        .id(("sidebar-rename", i))
                        .flex_1()
                        .min_w_0()
                        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                        // The row switches tabs on the *release* now, so
                        // holding the press back is no longer enough: a click
                        // landing in the field would reach the row behind it
                        // and switch away from the name being typed, taking
                        // the focus with it.
                        .on_click(|_, _, cx| cx.stop_propagation())
                        // No inset of its own: the label it replaces starts
                        // flush, and the field's 12px padding jumped the name
                        // sideways the moment rename began.
                        .child(Input::new(&input).appearance(false).px_0())
                        .into_any_element(),
                    None => v_flex()
                        .id(("sidebar-label", i))
                        .flex_1()
                        .min_w_0()
                        .justify_center()
                        .when_some(info, |col, info| {
                            col.tooltip(move |window, cx| {
                                // `Tooltip::element` rebuilds its content on
                                // every hover, so the captured info is cloned
                                // per call instead of being moved out.
                                let info = info.clone();
                                gpui::component::tooltip::Tooltip::element(move |_window, _cx| {
                                    let card = v_flex()
                                        .gap_1()
                                        // The card is the one place that
                                        // promised the whole string, so a long
                                        // path wraps here rather than being
                                        // truncated a second time.
                                        .when_some(info.title.clone(), |c, title| {
                                            c.child(
                                                div()
                                                    .max_w(px(420.))
                                                    .text_sm()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .child(title),
                                            )
                                        })
                                        .when_some(info.cwd.clone(), |c, cwd| {
                                            c.child(
                                                div()
                                                    .max_w(px(420.))
                                                    .text_xs()
                                                    .text_color(muted)
                                                    .child(cwd),
                                            )
                                        })
                                        .when_some(info.host.clone(), |c, host| {
                                            c.child(
                                                h_flex()
                                                    .items_center()
                                                    .gap_1p5()
                                                    .text_xs()
                                                    .text_color(muted)
                                                    .child(
                                                        gpui::svg()
                                                            .path("icons/machine-remote.svg")
                                                            .flex_shrink_0()
                                                            .size(px(11.))
                                                            .text_color(muted),
                                                    )
                                                    .child(div().truncate().child(host)),
                                            )
                                        });
                                    card
                                })
                                .build(window, cx)
                            })
                        })
                        .child(
                            h_flex()
                                .w_full()
                                .items_center()
                                .gap_1p5()
                                .child(
                                    div()
                                        .flex_1()
                                        .min_w_0()
                                        .truncate()
                                        .text_sm()
                                        .when(is_active, |d| d.font_weight(FontWeight::MEDIUM))
                                        .child(shown_title),
                                )
                                // The path is elided to the room the title
                                // left, so it may not shrink again here — a
                                // second cut would come out of its tail, the
                                // half that says which directory this is.
                                .when_some(cwd_shown.map(|(cwd, _)| cwd), |line, cwd| {
                                    line.child(
                                        div()
                                            .flex_shrink_0()
                                            .text_xs()
                                            .text_color(cx.theme().muted_foreground)
                                            .child(cwd),
                                    )
                                }),
                        )
                        .into_any_element(),
                };

                let row = h_flex()
                    .id(("tab-row", i))
                    // What a screen reader announces and presses: the full
                    // title, not the elided one drawn.
                    .role(gpui::Role::Tab)
                    .aria_label(SharedString::from(title_text.to_string()))
                    .aria_selected(is_active)
                    .group(SharedString::from(format!("tab-row-{i}")))
                    .cursor_pointer()
                    .on_drag(DragTab, {
                        let state = self.reorder.clone();
                        let slots = row_slots.clone();
                        let group_key = group_key.clone();
                        let id = tab.tree_id.get();
                        move |_drag, grab, _window, cx| {
                            cx.stop_propagation();
                            *state.borrow_mut() = Some(
                                Reorder::new(
                                    Surface::SidebarRows(group_key.clone()),
                                    slot,
                                    slots.borrow().clone(),
                                    px(ROW_GAP),
                                    grab,
                                )
                                .of_tab(id),
                            );
                            cx.new(|_| DragTab)
                        }
                    })
                    .w_full()
                    .h(px(ROW_HEIGHT))
                    .flex_shrink_0()
                    .items_center()
                    .justify_between()
                    .gap(px(row_metrics::GAP))
                    .px_2()
                    .rounded(crate::ui::rounding::ROW_RADIUS)
                    .when(is_active, |s| {
                        s.bg(gpui::rgb(sf.selected))
                            .text_color(gpui::rgb(sf.text_selected))
                    })
                    .when(!is_active, |s| {
                        s.text_color(gpui::rgb(sf.text_resting))
                            .hover(|s| s.bg(gpui::rgb(sf.hover)))
                    })
                    // Faded as well as marked: the
                    // tabs holding nothing should be the quiet ones in the column.
                    .when(asleep, |s| s.opacity(0.6))
                    .when(row_preview.as_ref().is_some_and(|p| p.from == slot), |s| {
                        s.opacity(0.75)
                    })
                    .child(
                        canvas(
                            {
                                let slots = row_slots.clone();
                                // The row by tab as well as by slot: reordering
                                // reads the slots of one group, a pane dropped
                                // on the sidebar reads every row there is.
                                let by_tab = self.sidebar_slots.clone();
                                let reveal = self.sidebar_reveal.clone();
                                let scroll = self.sidebar_scroll.clone();
                                let id = tab.tree_id.get();
                                // The header sits directly above a group's
                                // first row, one gap away.
                                let lead = match slot == 0 && section.name.is_some() {
                                    true => px(HEADER_HEIGHT + ROW_GAP),
                                    false => px(0.),
                                };
                                move |bounds, window, _cx| {
                                    if let Some(s) = slots.borrow_mut().get_mut(slot) {
                                        *s = bounds;
                                    }
                                    if let Some(s) = by_tab.borrow_mut().get_mut(i) {
                                        *s = bounds;
                                    }
                                    if reveal.get() == Some(id) {
                                        reveal.set(None);
                                        let view = scroll.bounds();
                                        let shift = reveal_shift(
                                            (bounds.top(), bounds.bottom()),
                                            lead,
                                            (view.top(), view.bottom()),
                                        );
                                        if shift != px(0.) {
                                            let mut offset = scroll.offset();
                                            offset.y += shift;
                                            scroll.set_offset(offset);
                                            window.refresh();
                                        }
                                    }
                                }
                            },
                            |_, _, _, _| {},
                        )
                        .absolute()
                        .inset_0(),
                    )
                    // Switched on the release, not the press: a press that turns
                    // into a drag is the tab being picked up, and a tab on its
                    // way into another tab's layout must not put itself on
                    // screen on the way there.
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .on_click(cx.listener(move |this, _, window, cx| {
                        cx.stop_propagation();
                        this.activate(i, window, cx);
                    }))
                    .child(self.tab_avatar_plain(
                        ("sidebar-avatar", i),
                        agent,
                        agent_status,
                        ssh_dot,
                        row_metrics::AVATAR,
                        cx,
                    ))
                    // Leading: the trailing end of a row is
                    // the badge's, and the close button fades in over it.
                    .when(zoomed, |row| {
                        row.child(self.zoom_mark(("sidebar-zoom", i), cx))
                    })
                    .when(asleep, |row| {
                        row.child(self.sleep_mark(("sidebar-asleep", i), cx))
                    })
                    .child(label_region)
                    .children(status_dot)
                    .when(show_badges && badge_pos < 9, |row| {
                        row.child(
                            div()
                                .flex_shrink_0()
                                .flex()
                                .items_center()
                                .justify_center()
                                .size(px(20.))
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(if is_active {
                                    gpui::rgb(sf.text_selected).into()
                                } else {
                                    cx.theme().muted_foreground
                                })
                                .child(tab_badge_label(badge_pos)),
                        )
                    })
                    .when(!(show_badges && badge_pos < 9), |row| {
                        let backing: gpui::Hsla = if is_active {
                            gpui::rgb(sf.selected).into()
                        } else {
                            gpui::rgb(sf.hover).into()
                        };
                        let mut fade_from = backing;
                        fade_from.a = 0.;
                        row.child(
                            h_flex()
                                .absolute()
                                .top_0()
                                .bottom_0()
                                .right(px(6.))
                                .opacity(0.)
                                .group_hover(SharedString::from(format!("tab-row-{i}")), |s| {
                                    s.opacity(1.)
                                })
                                .child(div().w(px(10.)).h_full().bg(linear_gradient(
                                    90.,
                                    linear_color_stop(fade_from, 0.),
                                    linear_color_stop(backing, 1.),
                                )))
                                .child(
                                    div().h_full().flex().items_center().bg(backing).child(
                                        hit_target(
                                            Button::new(("sidebar-close", i))
                                                .icon(IconName::Close)
                                                .ghost()
                                                .xsmall(),
                                        )
                                        .tooltip(t(L10nKey::TabContextCloseTab))
                                        // Held here, because the row behind it
                                        // switches tabs on the release too:
                                        // without this the same click closes
                                        // tab `i` and then activates whichever
                                        // tab slid into its place.
                                        .on_click(
                                            cx.listener(move |this, _, window, cx| {
                                                cx.stop_propagation();
                                                this.close_tab(i, window, cx);
                                            }),
                                        ),
                                    ),
                                ),
                        )
                    });

                let menu_app = cx.entity().downgrade();
                rows.push(row.context_menu(move |menu, window, cx| {
                    Tty7App::tab_context_menu(menu, i, &menu_app, window, cx)
                }));
            }

            // An empty pinned group is still drawn: it is kept until deleted.
            if row_count == 0 && section.pinned().is_none() {
                continue;
            }

            let row_display: Vec<usize> = match &row_preview {
                Some(p) => {
                    if let Some(order) =
                        reordered_rows(&keys, groups, &group_key, &visible_tabs, p.from, p.target)
                    {
                        reorder::set_pending(
                            &self.reorder,
                            &Surface::SidebarRows(group_key.clone()),
                            order,
                        );
                    }
                    p.order.clone()
                }
                None => (0..rows.len()).collect(),
            };
            let mut rows: Vec<Option<ContextMenu<Stateful<Div>>>> =
                rows.into_iter().map(Some).collect();
            let rows: Vec<AnyElement> = row_display
                .into_iter()
                .map(|slot| match &row_preview {
                    Some(p) if p.from == slot => deferred(
                        rows[slot]
                            .take()
                            .expect("each slot emitted once")
                            .relative()
                            .top(p.held),
                    )
                    .into_any_element(),
                    Some(p) => {
                        let offset = p.offsets[slot].as_f32();
                        rows[slot]
                            .take()
                            .expect("each slot emitted once")
                            .with_animation(
                                (
                                    SharedString::from(format!("row-slide-{}", p.generation)),
                                    slot,
                                ),
                                Animation::new(std::time::Duration::from_millis(REORDER_SLIDE_MS))
                                    .with_easing(ease_out_quint()),
                                move |el, delta| el.top(px(offset * (1. - delta))),
                            )
                            .into_any_element()
                    }
                    None => rows[slot]
                        .take()
                        .expect("each slot emitted once")
                        .into_any_element(),
                })
                .collect();
            // A pinned group carries a mark. It is the only thing separating
            // it on sight from a derived one — they behave differently (a
            // `cd` moves a tab out of a repo group and never out of this
            // one), and a group pinned on a real repo would otherwise print a
            // header identical to that repo's.
            let pinned_id = section.pinned();
            let pinned = pinned_id.is_some();
            let pinned_folder: Option<SharedString> = pinned_id
                .and_then(|id| groups.get(id))
                .and_then(|g| g.folder.clone())
                .map(SharedString::from);
            let auto_key_here = group_key.as_ref().and_then(GroupKey::auto).cloned();
            let renaming_group = self
                .group_rename
                .as_ref()
                .filter(|r| Some(r.group) == pinned_id)
                .map(|r| r.input.clone());
            let header = section.name.clone().map(|name| {
                let shared_git = {
                    let mut known = visible_by_section[group_ix].iter().filter_map(|&i| {
                        let leaf = self.tabs[i].pane.focused_or_first(window, cx)?;
                        let view = leaf.read(cx);
                        cx.try_global::<GitStatusCache>()?
                            .status_for(view.host_id(), &view.effective_host_cwd()?)
                    });
                    known
                        .next()
                        .filter(|status| known.all(|other| other == *status))
                };
                let ts = window.text_system();
                let mut avail = row_metrics::header_budget(width);
                if folded {
                    avail -= row_metrics::HEADER_ICON + row_metrics::META_GAP;
                }
                if pinned {
                    avail -= 11. + row_metrics::META_GAP;
                }
                let count_label = row_count.to_string();
                if folded {
                    avail -= measure_text(&ts, &meta_font, header_size, &count_label)
                        + row_metrics::META_GAP;
                }
                let avail = avail.max(HEADER_NAME_FLOOR);
                let counts: Vec<String> = shared_git
                    .as_ref()
                    .map(|status| {
                        [
                            (status.added > 0).then(|| format!("+{}", status.added)),
                            (status.removed > 0).then(|| format!("−{}", status.removed)),
                        ]
                        .into_iter()
                        .flatten()
                        .collect()
                    })
                    .unwrap_or_default();
                let counts_w = if counts.is_empty() {
                    0.
                } else {
                    counts
                        .iter()
                        .map(|text| measure_text(&ts, &meta_font, header_size, text))
                        .sum::<f32>()
                        + measure_text(&ts, &meta_font, header_size, "·")
                        + (counts.len() + 1) as f32 * row_metrics::META_GAP
                };
                let name_avail = shared_git.as_ref().map_or(avail, |status| {
                    let want = 2. * row_metrics::META_GAP
                        + measure_text(&ts, &meta_font, header_size, &status.branch)
                        + counts_w;
                    (avail - want.min(avail * 0.5)).max(HEADER_NAME_FLOOR)
                });
                let label = elide_label(&ts, &header_font, header_size, &name, name_avail);
                let name_w = measure_text(&ts, &header_font, header_size, &label);
                let hover_group = SharedString::from(format!("sidebar-group-{group_ix}"));
                let bar = h_flex()
                    .id(("sidebar-group", group_ix))
                    .group(hover_group.clone())
                    .relative()
                    .w_full()
                    .items_center()
                    .gap_1p5()
                    .px_2()
                    // The air above a heading is the list's `GROUP_GAP`;
                    // the heading itself is one short caption line.
                    .h(px(HEADER_HEIGHT))
                    .flex_shrink_0()
                    .text_size(px(header_size))
                    .text_color(cx.theme().muted_foreground)
                    .font_features(tabular())
                    .on_click(cx.listener({
                        let key = group_key.clone();
                        move |this, _, _window, cx| this.toggle_sidebar_group(key.as_ref(), cx)
                    }))
                    .when_some(group_slot, |header, slot| {
                        let (surface, slot, slots) = match slot {
                            HeaderSlot::Pinned(s) => {
                                (Surface::PinnedGroups, s, pinned_rects.clone())
                            }
                            HeaderSlot::Auto(s) => (Surface::SidebarGroups, s, group_slots.clone()),
                        };
                        let auto = auto_key_here.clone();
                        crate::ui::reorder::cursor_grab(header).on_drag(DragGroup, {
                            let state = self.reorder.clone();
                            move |_drag, grab, _window, cx| {
                                cx.stop_propagation();
                                let reorder = Reorder::new(
                                    surface.clone(),
                                    slot,
                                    slots.borrow().clone(),
                                    px(GROUP_GAP),
                                    grab,
                                );
                                // An auto header says which group it is, so
                                // letting it go above the divider can pin it.
                                *state.borrow_mut() = Some(match auto.clone() {
                                    Some(key) => reorder.of_auto(key),
                                    None => reorder,
                                });
                                cx.new(|_| DragGroup)
                            }
                        })
                    })
                    // The heading names the group; the chevron only says
                    // something when there is something behind it. An open
                    // group shows its rows, which is its own answer.
                    .when(folded, |header| {
                        header.child(
                            div()
                                .flex_shrink_0()
                                .child(Icon::new(IconName::ChevronRight).xsmall()),
                        )
                    })
                    // Every kept group carries the mark; it is what sets them
                    // apart from the derived groups below. On a folder group it
                    // is also the way to stop keeping it — a click unpins, and
                    // the folder's tabs fall back to the groups their cwds
                    // resolve to. A label group's mark is only a mark: without a
                    // folder there is nothing to fall back on, so letting go of
                    // it is Delete Group, in its menu, not a stray click.
                    .when_some(pinned_id, |header, id| {
                        let mark = div()
                            .flex_shrink_0()
                            .child(Icon::empty().path("icons/pin.svg").size(px(11.)));
                        header.child(match pinned_folder.is_some() {
                            false => mark.into_any_element(),
                            true => mark
                                .id(("sidebar-group-unpin", group_ix))
                                .cursor_pointer()
                                .hover(|s| s.text_color(cx.theme().foreground))
                                .tooltip(|window, cx| {
                                    gpui::component::tooltip::Tooltip::new(t(
                                        L10nKey::SidebarUnpinGroup,
                                    ))
                                    .build(window, cx)
                                })
                                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                                .on_click(cx.listener(move |this, _, _window, cx| {
                                    cx.stop_propagation();
                                    this.delete_group(id, cx);
                                }))
                                .into_any_element(),
                        })
                    })
                    .child(match renaming_group {
                        Some(input) => div()
                            .id(("sidebar-group-rename", group_ix))
                            .flex_1()
                            .min_w_0()
                            // The header folds the group on click, so a click
                            // landing in the field would shut the very group
                            // whose name is being typed — and take the focus
                            // with it.
                            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                            .on_click(|_, _, cx| cx.stop_propagation())
                            // No inset of its own: the label it replaces starts
                            // flush, and the field's 12px padding jumped the name
                            // sideways the moment rename began.
                            .child(Input::new(&input).appearance(false).px_0())
                            .into_any_element(),
                        // Elided above, so the truncation here is only the
                        // backstop for a face that measures wider than it
                        // paints; the name no longer gives room to the branch.
                        None => div()
                            .id(("sidebar-group-name", group_ix))
                            .group_hover(hover_group.clone(), |s| {
                                s.text_color(cx.theme().foreground)
                            })
                            .flex_shrink_0()
                            .min_w_0()
                            .truncate()
                            // A caption, not a title: the rows under it are
                            // what the column is for, so the heading keeps
                            // the metadata's grey and only a medium weight
                            // to stand apart from the branch beside it.
                            .font_weight(FontWeight::MEDIUM)
                            .child(label)
                            // A folder group may have been renamed to anything;
                            // the folder it keeps is what the name stands for.
                            .when_some(pinned_folder.clone(), |name, folder| {
                                name.tooltip(move |window, cx| {
                                    gpui::component::tooltip::Tooltip::new(folder.clone())
                                        .build(window, cx)
                                })
                            })
                            .into_any_element(),
                    })
                    .when_some(shared_git, |bar, status| {
                        let branch_avail =
                            (avail - name_w - 2. * row_metrics::META_GAP - counts_w).max(0.);
                        let line = h_flex()
                            .id(("sidebar-group-git", group_ix))
                            .flex_shrink(1.)
                            .min_w_0()
                            .items_center()
                            .gap_1p5()
                            .child(
                                div()
                                    .max_w(px(branch_avail))
                                    .min_w_0()
                                    .truncate()
                                    .child(status.branch),
                            )
                            .when(!counts.is_empty(), |line| {
                                line.child(div().flex_shrink_0().child("·")).children(
                                    counts.into_iter().map(|text| {
                                        div()
                                            .flex_shrink_0()
                                            .whitespace_nowrap()
                                            .text_color(cx.theme().muted_foreground)
                                            .child(text)
                                    }),
                                )
                            });
                        bar.child(div().flex_1()).child(line)
                    })
                    // The count is redundant while the rows are on screen; it
                    // is what a shut group has instead of them.
                    .when(folded, |bar| {
                        bar.child(div().flex_shrink_0().child(count_label))
                    })
                    .children(self.header_actions(
                        group_ix,
                        group_key.clone(),
                        hover_group,
                        rail_fill,
                        cx,
                    ));
                // Renaming is offered on a menu rather than a double click:
                // the first click of a double would fold the group, so the
                // name would be edited on a box that just shut. An auto group
                // offers no rename — it is named after its repo or host, and a
                // rename there could only lie about where its tabs are; pin it
                // first, and it is the user's to name.
                //
                // Attached last and erased to `AnyElement`, because the menu
                // wrapper changes the element's type and the arms have to
                // agree.
                let app = cx.entity().downgrade();
                match (pinned_id, auto_key_here.clone()) {
                    (Some(id), _) => {
                        let has_folder = pinned_folder.is_some();
                        bar.context_menu(move |menu, window, cx| {
                            pinned_header_menu(
                                menu,
                                id,
                                has_folder,
                                workspace_is_local,
                                &app,
                                window,
                                cx,
                            )
                        })
                        .into_any_element()
                    }
                    (None, Some(key)) => bar
                        .context_menu(move |menu, _window, _cx| {
                            auto_header_menu(menu, key.clone(), &app)
                        })
                        .into_any_element(),
                    (None, None) => bar.into_any_element(),
                }
            });

            // A tab in the air makes the difference between the two kinds of
            // group visible: the ones that can take it stay lit, the ones
            // that cannot fade back. Until a drag is under way they look
            // alike, and this is where a user finds out which is which
            // without being told.
            let takes_drops = pinned || (lifted_is_kept && n >= first_unpinned);
            let (preview, slot) = match group_slot {
                Some(HeaderSlot::Pinned(s)) => (pinned_preview.as_ref(), Some(s)),
                Some(HeaderSlot::Auto(s)) => (group_preview.as_ref(), Some(s)),
                None => (None, None),
            };
            // An empty pinned group says how to fill it, in the place its
            // rows would be: a tab can be dragged onto it, or opened here.
            let empty_row = (row_count == 0 && !folded)
                .then_some(pinned_id)
                .flatten()
                .map(|id| {
                    h_flex()
                        .id(("sidebar-group-empty", group_ix))
                        .w_full()
                        .h(px(ROW_HEIGHT))
                        .items_center()
                        .gap_2()
                        .pl_2()
                        .rounded(crate::ui::rounding::CARD_RADIUS)
                        .cursor_pointer()
                        .text_sm()
                        .text_color(cx.theme().muted_foreground)
                        .hover(|s| {
                            s.bg(gpui::rgb(sf.hover))
                                .text_color(cx.theme().sidebar_foreground)
                        })
                        .child(
                            div()
                                .flex()
                                .flex_shrink_0()
                                .items_center()
                                .justify_center()
                                .size(px(row_metrics::AVATAR))
                                .child(Icon::empty().path("icons/plus.svg").xsmall()),
                        )
                        .child(t(L10nKey::SidebarGroupNewTab))
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.new_tab_in_group(GroupKey::Pinned(id), window, cx)
                        }))
                });
            let block = v_flex()
                .w_full()
                .gap(px(ROW_GAP))
                .when(preview.is_some_and(|p| Some(p.from) == slot), |b| {
                    b.opacity(0.75)
                })
                .when(lifting_row && !takes_drops, |b| b.opacity(0.4))
                .when(
                    pinned_id.is_some_and(|id| over_group == Some(reorder::Regroup::Into(id))),
                    |b| b.rounded_md().bg(cx.theme().drag_border.opacity(0.15)),
                )
                .children(header)
                .children(rows)
                .children(empty_row)
                .child(
                    canvas(
                        {
                            let slots = match group_slot {
                                Some(HeaderSlot::Pinned(_)) => pinned_rects.clone(),
                                _ => group_slots.clone(),
                            };
                            let landing = self.sidebar_group_slots.clone();
                            // Only a pinned group is recorded, so a drag
                            // looking for somewhere to land finds nothing over
                            // an auto group or over Ungrouped.
                            move |bounds, _window, _cx| {
                                if let Some(slot) = slot
                                    && let Some(s) = slots.borrow_mut().get_mut(slot)
                                {
                                    *s = bounds;
                                }
                                if let Some(id) = pinned_id {
                                    landing.borrow_mut().push((id, bounds));
                                }
                            }
                        },
                        |_, _, _, _| {},
                    )
                    .absolute()
                    .inset_0(),
                );

            any_rows = true;
            list = list.child(match (preview, slot) {
                (Some(p), Some(slot)) if p.from == slot => {
                    deferred(block.relative().top(p.held)).into_any_element()
                }
                (Some(p), Some(slot)) => {
                    let offset = p.offsets[slot].as_f32();
                    block
                        .with_animation(
                            (
                                SharedString::from(format!("group-slide-{}", p.generation)),
                                slot,
                            ),
                            Animation::new(std::time::Duration::from_millis(REORDER_SLIDE_MS))
                                .with_easing(ease_out_quint()),
                            move |el, delta| el.top(px(offset * (1. - delta))),
                        )
                        .into_any_element()
                }
                _ => block.into_any_element(),
            });
        }
        if !reveal_drawn {
            self.sidebar_reveal.set(None);
        }
        if show_divider && !divider_drawn {
            list = list.child(self.sidebar_divider(divider_lit, divider_zone, cx));
        }

        // The room under the last row. Right-clicking a row opens that tab's
        // menu, so the list's own — new tab, new group — lives here rather
        // than on the list, where it would open over every row's as well.
        // It keeps a row's height when the list scrolls, so there is always
        // somewhere to click.
        list = list.child(
            div()
                .id("tab-sidebar-empty")
                .flex_1()
                .min_h(px(ROW_HEIGHT))
                .context_menu({
                    let app = cx.entity().downgrade();
                    move |menu, _window, _cx| {
                        let new_tab = app.clone();
                        let new_group = app.clone();
                        menu.min_w(px(200.))
                            .item(PopupMenuItem::new(t(L10nKey::AppMenuNewTab)).on_click(
                                move |_, window, cx| {
                                    let _ = new_tab.update(cx, |this, cx| this.new_tab(window, cx));
                                },
                            ))
                            .item(PopupMenuItem::new(t(L10nKey::SidebarNewGroup)).on_click(
                                move |_, window, cx| {
                                    let _ = new_group
                                        .update(cx, |this, cx| this.new_empty_group(window, cx));
                                },
                            ))
                    }
                }),
        );

        if !any_rows && !query.is_empty() {
            list = list.child(
                div()
                    .px_2()
                    .py_3()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child(crate::ui::i18n::t_fmt(
                        crate::ui::i18n::L10nKey::SettingsNothingMatches,
                        &[("query", &query)],
                    )),
            );
        }

        // On screen at rest unless the Appearance switch says otherwise; then
        // these controls wait for the pointer to reach the rail. The mark
        // stays either way — it names the window rather than doing anything.
        let auto_hide_chrome = cx.global::<Config>().auto_hide_titlebar_buttons;
        let chrome_shown = crate::ui::app::titlebar_chrome_shown(
            auto_hide_chrome,
            self.sidebar_chrome_hover.get(),
        );
        let controls = h_flex()
            .flex_shrink_0()
            .title_bar_row()
            .justify_end()
            .gap(px(2.))
            .pr(px(crate::ui::app::tile_trailing_inset()))
            .when_some(crate::ui::app::window_mark(), |row, mark| {
                row.child(
                    div()
                        .flex_shrink_0()
                        .pl(px(crate::ui::app::tile_trailing_inset()))
                        .child(
                            div()
                                .size(px(crate::ui::app::TILE_SIZE))
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(mark),
                        ),
                )
                .child(div().flex_1().min_w(px(GRAB_HANDLE_W)))
            })
            .child(
                crate::ui::app::resting_chrome(div().occlude().flex_shrink_0(), chrome_shown)
                    // These are the same controls as the collapsed title bar's
                    // pair. Keep their hit targets and glyphs unchanged when the
                    // sidebar opens, rather than switching to row-sized tiles.
                    .child(Self::new_tab_button("sidebar-add", cx)),
            )
            .child(
                crate::ui::app::resting_chrome(div().occlude().flex_shrink_0(), chrome_shown)
                    .child(
                        chrome_tile(
                            Button::new("sidebar-settings")
                                .icon(Icon::empty().path("icons/setting.svg")),
                            cx,
                        )
                        .rounded_lg()
                        .accessibility_label(t(L10nKey::SettingsWindowTitle))
                        .tooltip_with_action(
                            t(L10nKey::SettingsWindowTitle),
                            &crate::core::actions::OpenSettings,
                            None,
                        )
                        .on_click(|_, window, cx| {
                            window
                                .dispatch_action(Box::new(crate::core::actions::OpenSettings), cx);
                        }),
                    ),
            )
            .child(
                crate::ui::app::resting_chrome(div().occlude().flex_shrink_0(), chrome_shown)
                    .child(
                        chrome_tile(
                            Button::new("sidebar-collapse")
                                .icon(Icon::empty().path("icons/panel-left.svg")),
                            cx,
                        )
                        .rounded_lg()
                        .accessibility_label(t(L10nKey::TabTooltipHideSidebar))
                        .tooltip_with_action(
                            t(L10nKey::TabTooltipHideSidebar),
                            &crate::core::actions::ToggleLeftPanel,
                            None,
                        )
                        .on_click(cx.listener(|this, _, _window, cx| this.toggle_left_panel(cx))),
                    ),
            );
        // The tile inside asks for `w_full`, and a percentage is only a width
        // while some box above it has a real one. This row used to have none of
        // its own and borrowed the column's by cross-axis stretch, which did not
        // always hold; `w_full` here swapped that for a second percentage, and a
        // row whose width is `Percent` is no longer `auto`, so it lost stretch
        // as well — on the passes that size the column from its content there
        // was still nothing to resolve against and the tile fell back to hugging
        // the workspace name. Hand the row real pixels: the rail is
        // `w(px(width))` and layout is border-box, so its content is one pixel
        // narrower than that because of the right border.
        let search = h_flex()
            .w_full()
            .flex_shrink_0()
            .items_center()
            .gap(px(7.))
            .h(px(HEAD_CONTROL_HEIGHT))
            .pl_2()
            .pr_1()
            .rounded(px(HEAD_CONTROL_RADIUS))
            .bg(cx.theme().muted)
            .child(
                Icon::new(IconName::Search)
                    .size(px(12.))
                    .flex_shrink_0()
                    .text_color(cx.theme().muted_foreground),
            )
            .child(
                div().flex_1().min_w_0().child(
                    Input::new(&self.sidebar_search)
                        .appearance(false)
                        .cleanable(true)
                        .pl_0(),
                ),
            );
        let workspace_head = v_flex()
            .w(px(width - 1.))
            .flex_shrink_0()
            .gap(px(10.))
            .px(px(RAIL_INSET))
            .pt(px(4.))
            .pb(px(16.))
            .child(self.workspace_head(cx))
            .child(search);

        let container: Rc<Cell<Option<Bounds<Pixels>>>> = Rc::new(Cell::new(None));
        // Read while there is still a `cx` to read it from: the drag handler
        // below only ever sees a `Window`, and the cap it clamps against has to
        // be the same one the layout applies or the sidebar springs back from
        // wherever it was dropped.
        let others_floor = self.right_panel_floor();
        let backing = canvas(
            {
                let container = container.clone();
                move |bounds, _window, _cx| container.set(Some(bounds))
            },
            {
                let container = container.clone();
                let width_cell = self.sidebar_width.clone();
                let dragging = self.sidebar_dragging.clone();
                move |_bounds, _state, window, _cx| {
                    window.on_mouse_event({
                        let container = container.clone();
                        let width_cell = width_cell.clone();
                        let dragging = dragging.clone();
                        move |ev: &MouseMoveEvent, _phase, window, _cx| {
                            if !dragging.get() {
                                return;
                            }
                            let Some(b) = container.get() else {
                                return;
                            };
                            let raw = (ev.position.x - b.origin.x).as_f32();
                            let max = crate::ui::app::side_panel_max(
                                window.viewport_size().width.as_f32(),
                                MIN_SIDEBAR_WIDTH,
                                others_floor,
                            );
                            width_cell.set(raw.clamp(MIN_SIDEBAR_WIDTH, max));
                            window.refresh();
                        }
                    });
                    window.on_mouse_event({
                        let width_cell = width_cell.clone();
                        let dragging = dragging.clone();
                        move |_ev: &MouseUpEvent, _phase, window, cx| {
                            if !dragging.get() {
                                return;
                            }
                            dragging.set(false);
                            let w = width_cell.get();
                            let cfg = cx.global_mut::<Config>();
                            if cfg.sidebar_width != w {
                                cfg.sidebar_width = w;
                                cfg.save();
                            }
                            window.refresh();
                        }
                    });
                }
            },
        )
        .absolute()
        .size_full();

        let handle_active = self.sidebar_dragging.get();
        let handle = div()
            .group("sidebar-resize")
            .occlude()
            .absolute()
            .top_0()
            .right(px(-(RESIZE_HANDLE_WIDTH / 2.)))
            .w(px(RESIZE_HANDLE_WIDTH))
            .h_full()
            .flex()
            .items_center()
            .justify_center()
            .cursor_col_resize()
            .child(
                div()
                    .w(px(1.))
                    .h_full()
                    .when(handle_active, |d| d.bg(cx.theme().drag_border))
                    .group_hover("sidebar-resize", |s| s.bg(cx.theme().drag_border)),
            )
            .on_mouse_down(MouseButton::Left, {
                let dragging = self.sidebar_dragging.clone();
                move |_ev, window, _cx| {
                    dragging.set(true);
                    window.refresh();
                }
            });

        div()
            .relative()
            .flex_shrink_0()
            .w(px(width))
            .h_full()
            .bg(crate::ui::theme::rail_surface_color(cx))
            .border_r(crate::ui::theme::hairline(window))
            .border_color(cx.theme().sidebar_border)
            .child(backing)
            .child(
                // Real pixels, not `size_full`: the rail's own width is a
                // definite `px`, but a percentage off it is still a percentage,
                // and on the passes that size this column from its content it
                // resolves against nothing. Everything below asks for `w_full`
                // — the tab rows, their group blocks, the scroll area — so one
                // unresolved link here collapsed the whole chain and every row
                // fell back to hugging the longest tab name. Border-box takes
                // the rail's 1px right border off the content width.
                v_flex()
                    .w(px(width - 1.))
                    .h_full()
                    .child(crate::ui::app::title_bar_drag(
                        controls.id("sidebar-titlebar-drag"),
                        "sidebar-titlebar-drag",
                        window,
                        cx,
                    ))
                    .child(workspace_head)
                    .child(crate::ui::scrollbar::with_vertical_scrollbar(
                        "tab-sidebar-scrollbar",
                        list,
                        &self.sidebar_scroll,
                    )),
            )
            .child(handle)
            .when(auto_hide_chrome, |rail| {
                rail.child(crate::ui::app::hover_sheet(
                    "sidebar-chrome-hover",
                    &self.sidebar_chrome_hover,
                ))
            })
    }

    /// The boundary between the kept groups and the derived ones, recording
    /// where it lies so a drag next frame can tell which side it is on.
    ///
    /// Not drawn at rest — the mark on each kept header already says which
    /// side a group is on, and a hairline across the list was one more rule
    /// for the eye to cross. It keeps its place in the layout all the same, so
    /// nothing jumps when a drag starts, and shows as a line only while it is
    /// the target: when letting go would pin a header or hand a tab back.
    ///
    /// With nothing pinned yet it only appears while an auto header is being
    /// carried, and then as a drop zone rather than a hairline: a one-pixel
    /// line at the very top of the list is not something anyone could aim
    /// above. `lit` while letting go would pin the header, or hand a kept tab
    /// back to auto grouping.
    fn sidebar_divider(&self, lit: bool, zone: bool, cx: &Context<Self>) -> AnyElement {
        let bounds = self.sidebar_divider.clone();
        let ink = match (lit, zone) {
            (true, _) => cx.theme().drag_border,
            (false, true) => cx.theme().sidebar_border,
            (false, false) => gpui::transparent_black(),
        };
        let body = match zone {
            true => h_flex()
                .h(px(ROW_HEIGHT))
                .w_full()
                .items_center()
                .justify_center()
                .rounded(crate::ui::rounding::CARD_RADIUS)
                .border_1()
                .border_color(ink)
                .when(lit, |d| d.bg(cx.theme().drag_border.opacity(0.15)))
                .text_xs()
                .text_color(cx.theme().muted_foreground)
                .child(t(L10nKey::SidebarDropToPin))
                .into_any_element(),
            false => div()
                .w_full()
                .h(px(if lit { 2. } else { 1. }))
                .bg(ink)
                .into_any_element(),
        };
        v_flex()
            .id("sidebar-divider")
            .relative()
            .w_full()
            .px_2()
            .map(|d| match zone {
                true => d.pt(px(4.)).pb(px(2.)),
                // The rule takes up the gap between two groups instead of
                // adding one of its own: as a child of the list it would
                // otherwise sit between two `GROUP_GAP`s, and kept groups
                // stood more than twice as far from the rest as groups stand
                // from each other.
                false => d.h(px(GROUP_GAP)).my(px(-GROUP_GAP)).justify_center(),
            })
            .child(
                canvas(move |b, _window, _cx| bounds.set(Some(b)), |_, _, _, _| {})
                    .absolute()
                    .inset_0(),
            )
            .child(body)
            .into_any_element()
    }

    /// The buttons a header shows on hover, laid over its right end: pin for
    /// an auto group, and "+" to open a tab in the group. `None` for
    /// Ungrouped, which has neither.
    fn header_actions(
        &self,
        group_ix: usize,
        key: Option<GroupKey>,
        hover: SharedString,
        // The rail's own fill, so the buttons cover what they sit over
        // without a patch of another grey around them.
        backing: gpui::Hsla,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let key = key?;
        let mut fade = backing;
        fade.a = 0.;
        let muted = cx.theme().muted_foreground;
        let ink = cx.theme().foreground;
        let button = |id: &'static str, glyph: AnyElement, tip: L10nKey| {
            div()
                .id((id, group_ix))
                .flex_shrink_0()
                .p(px(2.))
                .rounded_sm()
                .cursor_pointer()
                .text_color(muted)
                .hover(move |s| s.text_color(ink))
                .child(glyph)
                .tooltip(move |window, cx| {
                    gpui::component::tooltip::Tooltip::new(t(tip)).build(window, cx)
                })
        };
        let mut buttons = h_flex().gap_0p5().bg(backing);
        if let Some(auto) = key.auto().cloned() {
            buttons = buttons.child(
                button(
                    "sidebar-group-pin",
                    // The mark the group will carry once kept, the size of
                    // the icon beside it so the two buttons line up.
                    Icon::empty()
                        .path("icons/pin.svg")
                        .xsmall()
                        .into_any_element(),
                    L10nKey::SidebarPinGroup,
                )
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _: &MouseDownEvent, _window, cx| {
                        cx.stop_propagation();
                        this.pin_auto_group(auto.clone(), cx);
                    }),
                ),
            );
        }
        if !matches!(key, GroupKey::Auto(AutoKey::SshHost(_))) {
            buttons = buttons.child(
                button(
                    "sidebar-group-add",
                    Icon::empty()
                        .path("icons/plus.svg")
                        .xsmall()
                        .into_any_element(),
                    L10nKey::SidebarGroupNewTab,
                )
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _: &MouseDownEvent, window, cx| {
                        cx.stop_propagation();
                        this.new_tab_in_group(key.clone(), window, cx);
                    }),
                ),
            );
        }
        Some(
            h_flex()
                .absolute()
                .right(px(4.))
                // Centred on the heading's line: 16px of buttons in `HEADER_HEIGHT`.
                .top(px((HEADER_HEIGHT - 16.) / 2.))
                .opacity(0.)
                .group_hover(hover, |s| s.opacity(1.))
                .child(div().w(px(10.)).h(px(16.)).bg(linear_gradient(
                    90.,
                    linear_color_stop(fade, 0.),
                    linear_color_stop(backing, 1.),
                )))
                .child(buttons)
                .into_any_element(),
        )
    }

    /// What the sidebar row hid: the full title, the working directory, and
    /// the remote host the avatar only
    /// dots. `None` when the row showed everything — a card would add noise,
    /// not information. The host is included even for an untruncated row,
    /// because the title strips the `user@host:` prefix the avatar cannot
    /// spell out.
    ///
    /// Every line is decided by comparing what the row rendered against the
    /// string it was elided from. Both come from the row itself: deriving
    /// them here a second time is how a renamed tab ended up with a name the
    /// row shortened and the card refused to expand.
    fn sidebar_info(
        &self,
        tab: &crate::ui::app::Tab,
        window: &mut Window,
        cx: &gpui::App,
        shown: &SidebarRowShown,
    ) -> Option<SidebarInfo> {
        let elided = |pair: &Option<(SharedString, SharedString)>| {
            pair.as_ref()
                .filter(|(shown, full)| shown != full)
                .map(|(_, full)| full.clone())
        };
        let mut info = SidebarInfo {
            title: elided(&shown.title),
            // The cwd only earns a card line when it was rendered *and*
            // elided: a repo row already shows the full path as its title, so
            // repeating the cwd under it would be noise, not information.
            cwd: elided(&shown.cwd),
            host: None,
        };
        // The host is read off the same leaf the title and cwd came from; a
        // split tab whose panes sit on different machines would otherwise
        // name whichever one happens to be first.
        if let Some(target) = tab.pane.focused_or_first(window, cx).and_then(|leaf| {
            leaf.read(cx)
                .remote_context()
                .map(|r| SharedString::from(r.target.clone()))
        }) {
            info.host = Some(target);
        }
        (info.title.is_some() || info.cwd.is_some() || info.host.is_some()).then_some(info)
    }

    /// Where a tab being dragged in the sidebar would land if let go now,
    /// when that is somewhere other than a new place in its own group.
    ///
    /// Over a pinned group it did not come from, into that group; anywhere
    /// below the divider, back to auto grouping — but only for a tab that is
    /// in a pinned group, since one that is not already is auto grouped and
    /// a drag that would change nothing offers nothing, falling back to plain
    /// reordering.
    ///
    /// No auto group is a target. Its membership is decided by its tabs'
    /// cwds, so "put this tab in tty7" is a request the sidebar has no honest
    /// way to honour; the only thing a drop below the divider can mean is
    /// "stop keeping this tab by hand".
    fn sidebar_regroup_target(&self, window: &Window) -> Option<reorder::Regroup> {
        let dragged = crate::ui::reorder::dragged_sidebar_tab(&self.reorder)?;
        let here = self
            .tabs
            .iter()
            .find(|t| t.tree_id.get() == dragged)
            .and_then(|t| t.group.get())
            .filter(|g| self.sidebar_groups.contains(*g));
        let pointer = window.mouse_position();
        if let Some((id, _)) = self
            .sidebar_group_slots
            .borrow()
            .iter()
            .find(|(_, bounds)| bounds.contains(&pointer))
        {
            return (Some(*id) != here).then_some(reorder::Regroup::Into(*id));
        }
        let below = self
            .sidebar_divider
            .get()
            .is_some_and(|d| pointer.y >= d.origin.y);
        (below && here.is_some()).then_some(reorder::Regroup::ToAuto)
    }

    /// The pinned groups there are, in sidebar order, with the name each
    /// header reads — for menus that offer them.
    pub(crate) fn pinned_group_names(&self) -> Vec<(GroupId, String)> {
        let names = pinned_names(&self.sidebar_groups.pinned);
        self.sidebar_groups
            .pinned
            .iter()
            .map(|g| g.id)
            .zip(names)
            .collect()
    }

    /// Change this workspace's groups, and send the change to the machine so
    /// every other window onto the workspace draws it too.
    pub(crate) fn edit_groups(
        &mut self,
        cx: &mut Context<Self>,
        edit: impl FnOnce(&mut WorkspaceGroups),
    ) {
        edit(&mut self.sidebar_groups);
        // Up as one `WorkspaceSetGroups`, queued ahead of any `TabSetGroup`
        // the same edit made — which the save below sends.
        crate::ui::tree_sync::push_groups(cx, self.workspace, self.sidebar_groups.clone());
        self.save_session(cx);
        cx.notify();
    }

    /// Take the workspace's groups as another window, or the machine, has them.
    ///
    /// Every tab's [`EntryWatch`](crate::core::group_key::EntryWatch) starts
    /// over from where it is. A folder that arrives from elsewhere is not one
    /// this window's tabs walked into — and in particular, a window that drew
    /// a frame before its copy of the groups landed saw no folders at all, so
    /// without this every tab sitting in a pinned folder at launch would read
    /// the groups landing as an entry, and a tab dragged out of its folder
    /// group would be pulled back in on every start. The window that pinned
    /// the folder gathers the tabs itself and says so tab by tab.
    pub(crate) fn adopt_sidebar_groups(&mut self, groups: WorkspaceGroups, cx: &mut Context<Self>) {
        if self.sidebar_groups == groups {
            return;
        }
        self.sidebar_groups = groups;
        for tab in &self.tabs {
            tab.folder_watch
                .set(crate::core::group_key::EntryWatch::baseline());
        }
        cx.notify();
    }

    /// Put the dragged tab where it was dropped.
    ///
    /// By id rather than index: a drag is several frames long, and a tab
    /// closing anywhere else in that time would shift every index after it.
    pub(crate) fn regroup_tab(
        &mut self,
        tab: tty7_core::core::machine::TabId,
        target: reorder::Regroup,
        cx: &mut Context<Self>,
    ) {
        let Some(index) = self.tabs.iter().position(|t| t.tree_id.get() == tab) else {
            return;
        };
        let group = match target {
            reorder::Regroup::Into(id) => Some(id),
            reorder::Regroup::ToAuto => None,
        };
        self.set_tab_group(index, group, cx);
    }

    /// Put tab `index` in pinned group `group`, or hand it back to auto
    /// grouping when `group` is `None`.
    pub(crate) fn set_tab_group(
        &mut self,
        index: usize,
        group: Option<GroupId>,
        cx: &mut Context<Self>,
    ) {
        let Some(tab) = self.tabs.get(index) else {
            return;
        };
        tab.group.set(group);
        // Carries the move to the daemon as a `TabSetGroup`, so another
        // window on the same workspace sees it too.
        self.save_session(cx);
        cx.notify();
    }

    /// Put tab `index` in group `key`. An auto group's members are decided by
    /// their cwds, so a tab goes into one by pinning the group, as its
    /// header's pin does — the tab joins in the same edit.
    pub(crate) fn move_tab_to(&mut self, index: usize, key: GroupKey, cx: &mut Context<Self>) {
        match key {
            GroupKey::Pinned(id) => self.set_tab_group(index, Some(id), cx),
            GroupKey::Auto(auto) => self.pin_auto_group_with(auto, Some(index), cx),
        }
    }

    /// A name no pinned group is using yet, for a group about to be made.
    ///
    /// The placeholder only has to be unique — the rename box opens on it
    /// selected, so the first keystroke replaces it. Unique still matters:
    /// two headers reading the same name are two groups nobody can tell
    /// apart.
    fn fresh_group_name(&self) -> String {
        let taken: Vec<String> = self
            .pinned_group_names()
            .into_iter()
            .map(|(_, n)| n)
            .collect();
        let base = t(L10nKey::SidebarNewGroupName).to_string();
        (1..)
            .map(|n| match n {
                1 => base.clone(),
                n => format!("{base} {n}"),
            })
            .find(|candidate| !taken.contains(candidate))
            .expect("an unbounded range always reaches an untaken name")
    }

    /// Make a new label group, put tab `index` in it, and open its header for
    /// renaming.
    ///
    /// No dialog: the tab is in the group before a character is typed, so
    /// what the name is being given to is on screen while it is chosen.
    pub(crate) fn new_tab_group(
        &mut self,
        index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let group = PinnedGroup::label(self.fresh_group_name());
        let id = group.id;
        if let Some(tab) = self.tabs.get(index) {
            tab.group.set(Some(id));
        }
        self.edit_groups(cx, |groups| groups.pinned.push(group));
        self.start_group_rename(id, window, cx);
    }

    pub(crate) fn new_empty_group(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let group = PinnedGroup::label(self.fresh_group_name());
        let id = group.id;
        self.edit_groups(cx, |groups| groups.pinned.push(group));
        self.start_group_rename(id, window, cx);
    }

    /// Keep the auto group `key`: pin it, with the tabs in it now.
    ///
    /// A repo group becomes a folder group on the repo's home, so tabs that
    /// walk into the repo later join it too, and it reads the repo's name
    /// until renamed. A host group has no folder on this workspace's machine
    /// to keep, so it becomes a label group named after the host.
    ///
    /// The fold comes along: a group that was shut stays shut, rather than
    /// springing open in its new place.
    pub(crate) fn pin_auto_group(&mut self, key: AutoKey, cx: &mut Context<Self>) {
        self.pin_auto_group_with(key, None, cx);
    }

    /// [`pin_auto_group`](Self::pin_auto_group), taking tab `also` into the
    /// new group in the same edit.
    fn pin_auto_group_with(&mut self, key: AutoKey, also: Option<usize>, cx: &mut Context<Self>) {
        let mut group = match &key {
            AutoKey::Repo(root) => PinnedGroup::folder(root),
            AutoKey::SshHost(host) => PinnedGroup::label(host.clone()),
        };
        group.collapsed = self.sidebar_groups.auto_collapsed.contains(&key);
        // A repo whose folder is already kept — a tab dragged out of that
        // group while still in the repo is drawn under the repo again — joins
        // the kept group rather than making a second: two groups keeping one
        // folder split its tabs between them by nothing but list order.
        let kept = group.folder.as_deref().and_then(|folder| {
            self.sidebar_groups
                .pinned
                .iter()
                .find(|g| g.folder.as_deref() == Some(folder))
                .map(|g| g.id)
        });
        let id = kept.unwrap_or(group.id);
        let wanted = Some(GroupKey::Auto(key.clone()));
        for (i, (tab, place)) in self
            .tabs
            .iter()
            .zip(self.sidebar_group_keys(cx))
            .enumerate()
        {
            if place == wanted || also == Some(i) {
                tab.group.set(Some(id));
            }
        }
        self.edit_groups(cx, |groups| {
            groups.auto_collapsed.retain(|k| *k != key);
            if kept.is_none() {
                groups.pinned.push(group);
            }
        });
    }

    /// Pin `folder` as a group of its own — a folder dropped from Finder.
    /// Pinning one that is already pinned does nothing: two groups keeping
    /// one folder would split its tabs between them by nothing but list order.
    ///
    /// Tabs already sitting in the folder are gathered in on the next frame,
    /// the way a tab walking in would be: the folder is new, so each of them
    /// has just "entered" it as far as [`EntryWatch`](crate::core::group_key::EntryWatch) can tell.
    pub(crate) fn pin_folder(&mut self, folder: PathBuf, cx: &mut Context<Self>) -> GroupId {
        let spelled = folder.to_string_lossy();
        if let Some(kept) = self
            .sidebar_groups
            .pinned
            .iter()
            .find(|g| g.folder.as_deref() == Some(&*spelled))
        {
            return kept.id;
        }
        let group = PinnedGroup::folder(&folder);
        let id = group.id;
        self.edit_groups(cx, |groups| groups.pinned.push(group));
        id
    }

    /// Folders dropped on the sidebar from Finder or Explorer, pinned as
    /// groups. Anything that is not a directory is let fall: a file names no
    /// group, and guessing its parent would pin something nobody pointed at.
    ///
    /// Only on a workspace on this computer. A dropped path is this
    /// machine's, and a folder group keeps a directory on the workspace's
    /// host — on a remote one the same spelling names something else, or
    /// nothing at all.
    pub(crate) fn pin_dropped_folders(&mut self, paths: &[PathBuf], cx: &mut Context<Self>) {
        if crate::core::session::WorkspaceStore::all(cx)
            .get(self.workspace)
            .is_some_and(|w| w.is_remote())
        {
            return;
        }
        for path in paths.iter().filter(|p| p.is_dir()) {
            self.pin_folder(path.clone(), cx);
        }
    }

    /// Delete pinned group `id`. Its tabs are not closed — they go back to
    /// auto grouping, which is where a tab nobody filed by hand belongs.
    ///
    /// Unpinning a folder group is the same act: the group stops being kept,
    /// and its tabs fall back to the auto groups their cwds resolve to.
    pub(crate) fn delete_group(&mut self, id: GroupId, cx: &mut Context<Self>) {
        for tab in &self.tabs {
            if tab.group.get() == Some(id) {
                tab.group.set(None);
            }
        }
        if self.group_rename.as_ref().is_some_and(|r| r.group == id) {
            self.group_rename = None;
        }
        self.edit_groups(cx, |groups| groups.pinned.retain(|g| g.id != id));
    }

    /// Point pinned group `id` at `folder`, or make it a label group when
    /// `folder` is `None`.
    ///
    /// A folder group nobody renamed reads its folder's name, and would read
    /// nothing once the folder is gone — so clearing the folder keeps the
    /// name it was showing.
    pub(crate) fn set_group_folder(
        &mut self,
        id: GroupId,
        folder: Option<PathBuf>,
        cx: &mut Context<Self>,
    ) {
        let shown = self
            .pinned_group_names()
            .into_iter()
            .find(|(g, _)| *g == id)
            .map(|(_, n)| n);
        self.edit_groups(cx, |groups| {
            let Some(group) = groups.get_mut(id) else {
                return;
            };
            if folder.is_none() && group.given_name().is_none() {
                group.name = shown;
            }
            group.folder = folder.map(|f| f.to_string_lossy().into_owned());
        });
    }

    /// Ask for a folder for pinned group `id` with the system picker. Only
    /// for a workspace on this machine: the picker browses this computer, and
    /// a path picked here means nothing to a remote one.
    pub(crate) fn pick_group_folder(&mut self, id: GroupId, cx: &mut Context<Self>) {
        let rx = cx.prompt_for_paths(gpui::PathPromptOptions {
            files: false,
            directories: true,
            multiple: false,
            prompt: None,
        });
        cx.spawn(async move |this, cx| {
            if let Ok(Ok(Some(paths))) = rx.await
                && let Some(path) = paths.into_iter().next()
            {
                let _ = this.update(cx, |this, cx| this.set_group_folder(id, Some(path), cx));
            }
        })
        .detach();
    }

    /// The directory the active tab is working in, on the workspace's host —
    /// what "Use Current Tab's Folder" pins a group to.
    pub(crate) fn active_tab_folder(&self, window: &Window, cx: &gpui::App) -> Option<PathBuf> {
        let leaf = self
            .tabs
            .get(self.active)?
            .pane
            .focused_or_first(window, cx)?;
        leaf.read(cx).effective_host_cwd()
    }

    /// Open a tab in the group drawn under `key` — a header's "+" and its
    /// menu's "New Tab".
    ///
    /// A folder group opens in its folder, a repo group in its repo; a label
    /// group has no directory of its own and opens where ⌘T would. The tab
    /// joins a pinned group outright rather than waiting for its cwd to walk
    /// in: a label group has nothing to walk into, and "new tab here" should
    /// not depend on a probe.
    pub(crate) fn new_tab_in_group(
        &mut self,
        key: GroupKey,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let before = self.tabs.len();
        let folder = match &key {
            GroupKey::Pinned(id) => self
                .sidebar_groups
                .get(*id)
                .and_then(|g| g.folder_path().map(Path::to_path_buf)),
            GroupKey::Auto(AutoKey::Repo(root)) => Some(root.clone()),
            GroupKey::Auto(AutoKey::SshHost(_)) => None,
        };
        match folder {
            Some(folder) => self.new_tab_at(folder, window, cx),
            None => self.new_tab(window, cx),
        }
        if self.tabs.len() == before {
            return;
        }
        if let GroupKey::Pinned(id) = key
            && let Some(tab) = self.tabs.get(self.active)
        {
            tab.group.set(Some(id));
            self.save_session(cx);
        }
    }

    /// Put the pinned groups in `order` — indices into the list as it stands.
    pub(crate) fn apply_pinned_order(&mut self, order: &[usize], cx: &mut Context<Self>) {
        let pinned = &self.sidebar_groups.pinned;
        if order.len() != pinned.len() || order.iter().enumerate().all(|(i, &o)| i == o) {
            return;
        }
        let reordered: Vec<PinnedGroup> = order
            .iter()
            .filter_map(|&i| pinned.get(i).cloned())
            .collect();
        if reordered.len() != pinned.len() {
            return;
        }
        self.edit_groups(cx, |groups| groups.pinned = reordered);
    }

    /// Open the header of pinned group `id` for renaming.
    pub(crate) fn start_group_rename(
        &mut self,
        id: GroupId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(current) = self
            .pinned_group_names()
            .into_iter()
            .find(|(g, _)| *g == id)
            .map(|(_, n)| n)
        else {
            return;
        };
        let input = Self::rename_box(current, window, cx);
        let subs = vec![cx.subscribe_in(
            &input,
            window,
            |this, _input, ev: &InputEvent, window, cx| match ev {
                InputEvent::PressEnter { .. } | InputEvent::Blur => {
                    this.commit_group_rename(window, cx)
                }
                _ => {}
            },
        )];
        self.group_rename = Some(crate::ui::app::GroupRename {
            group: id,
            input,
            _subs: subs,
        });
        cx.notify();
    }

    /// Write the typed name onto the group being renamed.
    pub(crate) fn commit_group_rename(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(rename) = self.group_rename.take() else {
            return;
        };
        let value = rename.input.read(cx).value().trim().to_string();
        self.edit_groups(cx, |groups| {
            let Some(group) = groups.get_mut(rename.group) else {
                return;
            };
            match (value.is_empty(), group.folder.is_some()) {
                // A folder group handed a blank name goes back to reading
                // its folder's — which is how a rename is undone.
                (true, true) => group.name = None,
                // A label group has nothing to fall back on: a blank name
                // would draw an unlabelled header, so it keeps the one it had.
                (true, false) => {}
                (false, _) => group.name = Some(value),
            }
        });
        self.focus_active(window, cx);
        cx.notify();
    }

    /// Fold the group drawn under `key` (`None`: Ungrouped), or unfold it if
    /// it is already shut. Stored with the workspace: a group folded away is a
    /// statement about work you are done with for now, and every window onto
    /// the workspace — and the next launch — should still show it shut.
    pub(crate) fn toggle_sidebar_group(&mut self, key: Option<&GroupKey>, cx: &mut Context<Self>) {
        self.edit_groups(cx, |groups| groups.toggle_folded(key));
    }

    /// Rule 2: a tab whose cwd walks into a pinned folder joins that group.
    ///
    /// Run once per frame, before anything asks where a tab is drawn. Every
    /// tab's [`EntryWatch`](crate::core::group_key::EntryWatch) is fed whether or not the tab is in a pinned group
    /// — that is what lets a tab dragged out of a folder group while it is
    /// still inside the folder stay out: the watch already knows it is
    /// inside, so staying there is no entry. Only a tab not already kept in a
    /// pinned group acts on an entry; a kept tab never leaves on its own.
    ///
    /// A tab is only looked at once its repo probe has answered. Before that
    /// the question "is it inside" has half an answer — the cwd without the
    /// repo home a worktree is matched by — and recording that half as where
    /// the tab *was* would read the other half landing a frame later as an
    /// entry, pulling every restored worktree tab into its group at launch.
    ///
    /// SSH panes are never looked at: their cwd is on another machine, and a
    /// folder pinned in this workspace names a directory on this one.
    ///
    /// Also where a tab's auto-group hint is refreshed from the live answer
    /// and, when that answer moved, sent up with the tab — so the next launch
    /// draws the tab where it last was instead of in Ungrouped.
    pub(crate) fn settle_sidebar_groups(&mut self, cx: &mut Context<Self>) {
        let mut changed = false;
        for tab in &self.tabs {
            let before = tab.auto_group.borrow().clone();
            changed |= self.tab_auto_group(tab, cx) != before;
        }
        let Some(cache) = cx.try_global::<GitStatusCache>() else {
            if changed {
                self.save_session(cx);
            }
            return;
        };
        let mut joined = changed;
        for tab in &self.tabs {
            let Some(leaf) = tab.pane.first_leaf() else {
                continue;
            };
            let Some(view) = leaf.terminal() else {
                continue;
            };
            let view = view.read(cx);
            if view.remote_context().is_some() {
                continue;
            }
            let Some(cwd) = view.git_status_cwd() else {
                continue;
            };
            let Some(home) = cache.known_repo_for(view.host_id(), cwd) else {
                continue;
            };
            let inside = self.sidebar_groups.folder_for(Some(cwd), home.as_deref());
            let mut watch = tab.folder_watch.get();
            let entered = watch.observe(inside);
            tab.folder_watch.set(watch);
            let kept = tab
                .group
                .get()
                .is_some_and(|g| self.sidebar_groups.contains(g));
            if let Some(id) = entered
                && !kept
            {
                tab.group.set(Some(id));
                joined = true;
            }
        }
        if joined {
            self.save_session(cx);
        }
    }

    /// The auto group tab `tab` resolves to (rule 3): the host for an SSH
    /// pane, otherwise the repo its cwd is in, otherwise none (Ungrouped).
    ///
    /// Remembered on the tab, and the memory is what answers while there is
    /// nothing to go on — a probe in flight, or a pane (a WSL one, say) whose
    /// paths no probe reaches — so a tab does not bounce through Ungrouped
    /// between one answer and the next.
    fn tab_auto_group(&self, tab: &Tab, cx: &gpui::App) -> Option<AutoKey> {
        let resolved = tab.pane.first_leaf().and_then(|leaf| {
            let view = leaf.terminal()?.read(cx);
            let host = ssh_host(view.remote_context().as_ref());
            let known = view.git_status_cwd().and_then(|cwd| {
                cx.try_global::<GitStatusCache>()?
                    .known_repo_for(view.host_id(), cwd)
            });
            auto_key(host.as_deref(), known)
        });
        if let Some(key) = resolved {
            *tab.auto_group.borrow_mut() = key;
        }
        tab.auto_group.borrow().clone()
    }

    /// Where each tab is drawn, in `self.tabs` order: its pinned group, its
    /// auto group, or `None` for Ungrouped (or for the flat list below the
    /// pinned groups, with auto grouping off).
    pub(crate) fn sidebar_group_keys(&self, cx: &gpui::App) -> Vec<Option<GroupKey>> {
        let auto_grouping = cx.global::<Config>().sidebar_auto_grouping;
        self.tabs
            .iter()
            .map(|tab| {
                // Resolved even with auto grouping off, so the remembered
                // answer is warm the moment it is turned back on.
                let auto = self.tab_auto_group(tab, cx);
                place(tab.group.get(), &self.sidebar_groups, auto_grouping, auto)
            })
            .collect()
    }

    pub(crate) fn visual_tab_order(&self, cx: &gpui::App) -> Vec<usize> {
        let keys = self.sidebar_group_keys(cx);
        sidebar_sections(&keys, &self.sidebar_groups)
            .into_iter()
            .flat_map(|s| s.tabs)
            .collect()
    }

    /// The tabs in `index`'s sidebar section, in their display order.
    pub(crate) fn tab_run(&self, index: usize, cx: &gpui::App) -> Vec<usize> {
        let keys = self.sidebar_group_keys(cx);
        sidebar_sections(&keys, &self.sidebar_groups)
            .into_iter()
            .map(|s| s.tabs)
            .find(|tabs| tabs.contains(&index))
            .unwrap_or_default()
    }

    pub(crate) fn activate_visual(
        &mut self,
        n: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(&i) = self.visual_tab_order(cx).get(n) {
            self.activate(i, window, cx);
        }
    }

    /// Where a tab about to be spawned in `cwd` goes.
    ///
    /// A tab spawned from one kept in a pinned group joins it, before the cwd
    /// is consulted at all — the cwd says nothing about a group the user
    /// stated by hand. Every caller here spawns from the active tab (the ones
    /// that start from a named tab activate it first), so that is the one to
    /// inherit from. Without this, ⌘T inside a folded group would draw
    /// nothing but the header's count going up by one.
    ///
    /// Otherwise a cwd inside a pinned folder joins that folder's group, and
    /// the auto group is seeded from the repo cache when its probe for that
    /// directory has already landed. A tab's auto group otherwise starts
    /// empty and only fills in once its shell has reported a cwd, which parks
    /// every new tab in Ungrouped at the bottom of the sidebar until then; a
    /// tab spawned from one already in a repo inherits a warm cache, so this
    /// lands it in its group on the first frame.
    pub(crate) fn spawn_group(&self, cwd: Option<&Path>, cx: &gpui::App) -> SpawnPlace {
        let kept = self
            .tabs
            .get(self.active)
            .and_then(|t| t.group.get())
            .filter(|g| self.sidebar_groups.contains(*g));
        let known = cwd.and_then(|cwd| {
            let host = self
                .window_workspace(cx)
                .as_ref()
                .map_or(crate::ui::host_ops::HostId::LOCAL, |ws| ws.target.host_id());
            cx.try_global::<GitStatusCache>()?.known_repo_for(host, cwd)
        });
        let entered = cwd.and_then(|cwd| {
            let home = known.clone().flatten();
            self.sidebar_groups.folder_for(Some(cwd), home.as_deref())
        });
        SpawnPlace {
            group: kept.or(entered),
            auto: auto_key(None, known),
        }
    }
}

/// A header that can be picked up, and which of the two orders it belongs to.
#[derive(Clone, Copy)]
enum HeaderSlot {
    /// Slot `n` among the pinned headers, above the divider.
    Pinned(usize),
    /// Slot `n` among the auto headers, below it.
    Auto(usize),
}

/// One item of a group header's menu, acting on the app when it is chosen.
fn menu_item(
    label: L10nKey,
    app: &gpui::WeakEntity<Tty7App>,
    act: impl Fn(&mut Tty7App, &mut Window, &mut Context<Tty7App>) + 'static,
) -> PopupMenuItem {
    let app = app.clone();
    PopupMenuItem::new(t(label)).on_click(move |_, window, cx| {
        let _ = app.update(cx, |this, cx| act(this, window, cx));
    })
}

/// A pinned group's header menu. Everything about a kept group is the
/// user's to change: its name, the folder it keeps, and whether it is kept at
/// all. Deleting closes nothing — its tabs go back to auto grouping.
///
/// "Set Folder…" asks the system picker, which browses this computer, so it
/// is only offered for a workspace on it; "Use Current Tab's Folder" works
/// anywhere, since the tab's cwd is on the workspace's own host.
fn pinned_header_menu(
    menu: gpui::component::menu::PopupMenu,
    id: GroupId,
    has_folder: bool,
    local: bool,
    app: &gpui::WeakEntity<Tty7App>,
    window: &Window,
    cx: &gpui::App,
) -> gpui::component::menu::PopupMenu {
    let current = app
        .upgrade()
        .and_then(|a| a.read(cx).active_tab_folder(window, cx));
    let mut menu = menu.min_w(px(200.)).item(menu_item(
        L10nKey::SidebarRenameGroup,
        app,
        move |this, w, cx| this.start_group_rename(id, w, cx),
    ));
    if local {
        menu = menu.item(menu_item(
            L10nKey::SidebarSetGroupFolder,
            app,
            move |this, _, cx| this.pick_group_folder(id, cx),
        ));
    }
    menu = menu.item(
        menu_item(
            L10nKey::SidebarUseCurrentTabFolder,
            app,
            move |this, w, cx| {
                if let Some(folder) = this.active_tab_folder(w, cx) {
                    this.set_group_folder(id, Some(folder), cx);
                }
            },
        )
        .disabled(current.is_none()),
    );
    if has_folder {
        menu = menu.item(menu_item(
            L10nKey::SidebarClearGroupFolder,
            app,
            move |this, _, cx| this.set_group_folder(id, None, cx),
        ));
    }
    menu = menu
        .separator()
        .item(menu_item(
            L10nKey::SidebarGroupNewTab,
            app,
            move |this, w, cx| this.new_tab_in_group(GroupKey::Pinned(id), w, cx),
        ))
        .separator();
    if has_folder {
        menu = menu.item(menu_item(
            L10nKey::SidebarUnpinGroup,
            app,
            move |this, _, cx| this.delete_group(id, cx),
        ));
    }
    menu.item(menu_item(
        L10nKey::SidebarDeleteGroup,
        app,
        move |this, _, cx| this.delete_group(id, cx),
    ))
}

/// An auto group's header menu: pin it, or open a tab in it. Nothing else —
/// an auto group is derived, so there is nothing about it to edit until it
/// is pinned, and pinning is always an explicit act.
fn auto_header_menu(
    menu: gpui::component::menu::PopupMenu,
    key: AutoKey,
    app: &gpui::WeakEntity<Tty7App>,
) -> gpui::component::menu::PopupMenu {
    let pin = key.clone();
    let mut menu = menu.min_w(px(200.)).item(menu_item(
        L10nKey::SidebarPinGroup,
        app,
        move |this, _, cx| this.pin_auto_group(pin.clone(), cx),
    ));
    // A host group has no directory to open a tab in, and no way to open a
    // shell on the host that would not guess at how the others got there.
    if let AutoKey::Repo(_) = &key {
        menu = menu.item(menu_item(
            L10nKey::SidebarGroupNewTab,
            app,
            move |this, w, cx| this.new_tab_in_group(GroupKey::Auto(key.clone()), w, cx),
        ));
    }
    menu
}

/// Where a tab about to be spawned goes, worked out before it exists — see
/// [`Tty7App::spawn_group`].
pub(crate) struct SpawnPlace {
    group: Option<GroupId>,
    auto: Option<Option<AutoKey>>,
}

impl SpawnPlace {
    /// Seed the auto group with SSH host `host`, for a tab dialling a machine
    /// whose pane has not reported its remote context yet.
    pub(crate) fn on_host(mut self, host: Option<String>) -> Self {
        if let Some(auto) = auto_key(host.as_deref(), None) {
            self.auto = Some(auto);
        }
        self
    }

    /// Put `tab` where this says.
    pub(crate) fn seat(&self, tab: &Tab) {
        if let Some(id) = self.group {
            tab.group.set(Some(id));
        }
        if let Some(auto) = &self.auto {
            *tab.auto_group.borrow_mut() = auto.clone();
        }
    }
}

/// The host an SSH pane is on, when it is one.
///
/// A native SSH pane and a shell that has ssh'd onward from a local prompt
/// are grouped alike, by the target they name: in neither case does a `Host`
/// of ours reach the far side, so neither has a repo to probe — the first
/// reports a remote cwd that names nothing here, the second only the local
/// directory the `ssh` was typed in, which says nothing about where its shell
/// now is. The host is the one thing about either that is certain. A WSL pane
/// is not an SSH one: its distro is on this machine, and it keeps the group it
/// had.
fn ssh_host(remote: Option<&crate::daemon::protocol::RemoteContext>) -> Option<String> {
    use crate::daemon::protocol::RemoteKind;
    remote
        .filter(|r| matches!(r.kind, RemoteKind::Ssh | RemoteKind::NativeSsh))
        .map(|r| r.target.clone())
}

#[derive(Debug, PartialEq)]
struct Section {
    key: Option<GroupKey>,
    name: Option<String>,
    tabs: Vec<usize>,
}

impl Section {
    fn pinned(&self) -> Option<GroupId> {
        self.key.as_ref().and_then(GroupKey::pinned)
    }
}

/// How far to move the sidebar's scroll offset so a newly active row shows.
/// A row with any part in view stays put — clicking a row must not move the
/// list under the pointer; one out of view comes in at the nearest edge.
/// `lead` is what sits on top of the row and belongs with it — its group's
/// header, for the first row — so a row coming in from above brings the
/// name of the group it is in, not just itself.
fn reveal_shift(row: (Pixels, Pixels), lead: Pixels, view: (Pixels, Pixels)) -> Pixels {
    let ((top, bottom), (view_top, view_bottom)) = (row, view);
    if bottom <= view_top {
        view_top - (top - lead)
    } else if top >= view_bottom {
        view_bottom - bottom
    } else {
        px(0.)
    }
}

/// The sidebar's sections, top to bottom: every pinned group in the user's
/// order — an empty one too, since a kept group stays until it is deleted —
/// then the auto groups in the order their first tab appears, then
/// Ungrouped.
///
/// Ungrouped only draws a header beside other groups below the divider. With
/// no auto group to set it apart from — grouping off, or nothing resolved
/// yet — it is just the list, and a header over the whole of it would be a
/// label on nothing. The same goes for a sidebar with no groups at all, which
/// comes out as one headerless section holding every tab.
fn sidebar_sections(keys: &[Option<GroupKey>], groups: &WorkspaceGroups) -> Vec<Section> {
    let members = |key: &GroupKey| -> Vec<usize> {
        (0..keys.len())
            .filter(|&i| keys[i].as_ref() == Some(key))
            .collect()
    };
    let mut auto_order: Vec<&AutoKey> = Vec::new();
    for k in keys.iter().flatten().filter_map(GroupKey::auto) {
        if !auto_order.contains(&k) {
            auto_order.push(k);
        }
    }
    let rest: Vec<usize> = (0..keys.len()).filter(|&i| keys[i].is_none()).collect();
    if groups.pinned.is_empty() && auto_order.is_empty() {
        return vec![Section {
            key: None,
            name: None,
            tabs: rest,
        }];
    }
    let pinned = pinned_names(&groups.pinned);
    let auto = auto_names(&auto_order);
    let mut sections: Vec<Section> = groups
        .pinned
        .iter()
        .zip(pinned)
        .map(|(g, name)| {
            let key = GroupKey::Pinned(g.id);
            Section {
                tabs: members(&key),
                key: Some(key),
                name: Some(name),
            }
        })
        .collect();
    sections.extend(auto_order.iter().zip(auto).map(|(k, name)| {
        let key = GroupKey::Auto((*k).clone());
        Section {
            tabs: members(&key),
            key: Some(key),
            name: Some(name),
        }
    }));
    if !rest.is_empty() {
        sections.push(Section {
            key: None,
            name: (!auto_order.is_empty()).then(|| t(L10nKey::SidebarUngroupedGroup).to_string()),
            tabs: rest,
        });
    }
    sections
}

/// A row of the tab menu's "Move to Group".
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct MoveTarget {
    pub key: GroupKey,
    pub name: String,
    pub checked: bool,
}

/// Every group the sidebar draws, in its order — pinned and auto alike —
/// with the one tab `index` is in checked.
pub(crate) fn move_targets(
    keys: &[Option<GroupKey>],
    groups: &WorkspaceGroups,
    index: usize,
) -> Vec<MoveTarget> {
    let here = keys.get(index).cloned().flatten();
    sidebar_sections(keys, groups)
        .into_iter()
        .filter_map(|s| {
            let key = s.key?;
            Some(MoveTarget {
                checked: here.as_ref() == Some(&key),
                name: s.name.expect("a section with a key has a header"),
                key,
            })
        })
        .collect()
}

fn reordered_rows(
    keys: &[Option<GroupKey>],
    groups: &WorkspaceGroups,
    group: &Option<GroupKey>,
    visible: &[usize],
    from: usize,
    to: usize,
) -> Option<Vec<usize>> {
    let (&moved, &anchor) = (visible.get(from)?, visible.get(to)?);
    if moved == anchor {
        return None;
    }
    let mut members: Vec<usize> = (0..keys.len()).filter(|&i| keys[i] == *group).collect();
    members.retain(|&i| i != moved);
    let at = members.iter().position(|&i| i == anchor)? + usize::from(to > from);
    members.insert(at, moved);

    let mut out: Vec<usize> = Vec::with_capacity(keys.len());
    for g in sidebar_sections(keys, groups).iter().map(|s| &s.key) {
        if g == group {
            out.extend_from_slice(&members);
        } else {
            out.extend((0..keys.len()).filter(|&i| keys[i] == *g));
        }
    }
    Some(out)
}

/// The tab order that moves auto group `from` to where auto group `to` is.
///
/// Auto groups are ordered by where their first tab sits, so moving one is
/// moving its tabs. Pinned groups keep an order of their own and take no part:
/// their tabs follow the auto groups' here, where they draw no differently.
fn regrouped_order(keys: &[Option<GroupKey>], from: &AutoKey, to: &AutoKey) -> Option<Vec<usize>> {
    if from == to {
        return None;
    }
    let mut order: Vec<&AutoKey> = Vec::new();
    for k in keys.iter().flatten().filter_map(GroupKey::auto) {
        if !order.contains(&k) {
            order.push(k);
        }
    }
    let fi = order.iter().position(|g| *g == from)?;
    let ti = order.iter().position(|g| *g == to)?;
    let moved = order.remove(fi);
    order.insert(ti, moved);

    let auto_of = |i: usize| keys[i].as_ref().and_then(GroupKey::auto);
    let mut out: Vec<usize> = Vec::with_capacity(keys.len());
    for g in &order {
        out.extend((0..keys.len()).filter(|&i| auto_of(i) == Some(*g)));
    }
    out.extend((0..keys.len()).filter(|&i| auto_of(i).is_none()));
    Some(out)
}

/// The order of all pinned groups after the header in visible slot `from` is
/// dropped on slot `to`, as indices into `all`. `shown` is the pinned groups
/// on screen, in order — a search can hide some, and those keep their places.
fn reordered_pinned(
    all: &[GroupId],
    shown: &[GroupId],
    from: usize,
    to: usize,
) -> Option<Vec<usize>> {
    let (&moved, &anchor) = (shown.get(from)?, shown.get(to)?);
    if moved == anchor {
        return None;
    }
    let mut order: Vec<GroupId> = all.to_vec();
    order.retain(|g| *g != moved);
    let at = order.iter().position(|g| *g == anchor)? + usize::from(to > from);
    order.insert(at, moved);
    order
        .iter()
        .map(|g| all.iter().position(|a| a == g))
        .collect()
}

/// One duration and one curve for every transition the app runs, so a fade and
/// a slide read as the same hand. Long enough to be seen as movement, short
/// enough that nobody waits on it.
pub(crate) const TRANSITION_MS: u64 = 140;
pub(crate) const REORDER_SLIDE_MS: u64 = TRANSITION_MS;

const KEEP_SEGMENTS: usize = 3;

/// Builds a launch specification without recomputing argument ownership locally.
/// The inventory may originate from a remote host, so only its transported
/// metadata can distinguish tty7 launch defaults from user-authored arguments.
fn shell_spec(shell: &DetectedShell) -> ShellSpec {
    ShellSpec {
        program: shell.program.clone(),
        args: shell.args.clone(),
        args_are_tty7_defaults: shell.args_are_tty7_defaults,
    }
}

/// Shared with the switcher's tab column and the CLI, which name tabs of
/// workspaces this process does not own and have to cut the same head off the
/// same titles.
pub(crate) use tty7_core::core::tab_view::strip_host_prefix;

/// `home` is the home directory of the machine `path` is on, from the
/// workspace's host; `None` leaves the path spelled out (#580).
pub(crate) fn abbreviate_home<'a>(
    path: &'a str,
    home: Option<&std::path::Path>,
) -> std::borrow::Cow<'a, str> {
    use std::borrow::Cow;
    if path.starts_with('~') {
        return Cow::Borrowed(path);
    }
    // The shared comparison: separators normalized, case folded — a Windows
    // pane whose cwd spells itself `C:/Users/…` shortens under a
    // `C:\Users\…` home too (#544).
    crate::ui::path_display::abbreviate_home(path, home)
}

/// The separator a path spells itself with. A path carrying a single `\` is
/// a Windows path and has to be put back together with `\`: rejoining it with
/// `/` would make one tab spell its location two ways, `C:\Users\dev\app`
/// while it fits and `C:/…/app` once it has to be elided.
fn path_separator(path: &str) -> char {
    if path.contains('\\') { '\\' } else { '/' }
}

fn join_segments(segments: &[&str], sep: char) -> String {
    segments.join(sep.encode_utf8(&mut [0u8; 4]) as &str)
}

pub(crate) fn short_title(raw: &str, home: Option<&std::path::Path>) -> String {
    let raw = raw.trim();
    if raw.is_empty() {
        return String::new();
    }
    let after_host = strip_host_prefix(raw);
    let after_host = after_host.trim();
    if after_host.is_empty() {
        return String::new();
    }
    let abbreviated = abbreviate_home(after_host, home);
    let path: &str = abbreviated.as_ref();

    enum Kind {
        Home,
        Absolute,
        Relative,
    }
    let (kind, body) = if let Some(rest) = path.strip_prefix("~/") {
        (Kind::Home, rest)
    } else if path == "~" {
        return "~".to_string();
    } else if let Some(rest) = path.strip_prefix('/') {
        (Kind::Absolute, rest)
    } else {
        (Kind::Relative, path)
    };

    // Both separators: Windows shells report `C:\Users\…` while git and the
    // terminal integration use `/`, and a path must be cut on either one.
    let segments: Vec<&str> = body.split(['/', '\\']).filter(|s| !s.is_empty()).collect();
    if segments.is_empty() {
        return match kind {
            Kind::Home => "~",
            Kind::Absolute => "/",
            Kind::Relative => "",
        }
        .to_string();
    }

    let sep = path_separator(path);
    let depth = segments.len() + usize::from(matches!(kind, Kind::Home));
    let mut label = if depth > KEEP_SEGMENTS {
        let tail = &segments[segments.len() - KEEP_SEGMENTS..];
        format!("…{sep}{}", join_segments(tail, sep))
    } else {
        match kind {
            Kind::Home => format!("~{sep}{}", join_segments(&segments, sep)),
            Kind::Absolute => format!("/{}", join_segments(&segments, sep)),
            Kind::Relative => join_segments(&segments, sep),
        }
    };
    // Clamped on cluster boundaries, or a label ending in an emoji comes back
    // holding half of one.
    let cells = clusters(&label);
    if cells.len() > 40 {
        label = format!("{}…", cells[..40].concat());
    }
    label
}

/// The one place a tab gets its displayed name, whichever surface is asking.
///
/// `label()` ranks the evidence — a given name, then the title the pane is
/// showing, then an agent, then the working directory, then the process it is
/// running — and this renders whatever came back. Both callers arrive with a
/// [`TabView`](crate::ui::machine_mirror::TabView): the switcher reads one out
/// of the machine tree for a window it does not own, and the sidebar builds one
/// from its own live panes in
/// [`Tab::label_view`](crate::ui::app::Tab::label_view).
pub(crate) fn label_of(
    view: &crate::ui::machine_mirror::TabView,
    index: usize,
    home: Option<&std::path::Path>,
) -> String {
    use crate::ui::machine_mirror::TabLabel;

    let unnamed = || {
        t_fmt(
            L10nKey::TabUnnamedShell,
            &[("n", &((index + 1).to_string()))],
        )
    };
    // A path can shorten away to nothing (a bare "user@host:"), and the process
    // name the tree carries ("zsh") is still worth more than a number.
    //
    // Through `stated_title` because a tab of *this* window has no process name
    // to offer: `Tab::label_view` fills that slot with the placeholder a pane
    // answers to before anything has spoken, and printing the app's own name
    // here would hide the numbered placeholder for an unnamed tab.
    let shortened = |raw: &str| match short_title(raw, home) {
        shortened if !shortened.trim().is_empty() => shortened,
        _ => match crate::terminal::view::stated_title(&view.title) {
            Some(title) => title.to_string(),
            None => unnamed(),
        },
    };
    match view.label() {
        TabLabel::Named(name) => name.to_string(),
        // Through `short_title` because a title is so often a path: the shell
        // integration writes `user@host:~/dir`, and a tab spelling that out in
        // full where the one beside it says "…/dir" would be the same
        // disagreement in a new place.
        TabLabel::Osc(title) => shortened(title),
        TabLabel::Agent(agent) => agent.display_name().to_string(),
        TabLabel::Cwd(cwd) => shortened(cwd),
        TabLabel::Process(title) => title.to_string(),
        TabLabel::Unknown => unnamed(),
    }
}

/// Width of `text` shaped in `font` at `size`, in pixels.
///
/// The window's text system caches shaped runs, so measuring the same labels
/// across frames is cheap. The sidebar elides against real glyph widths
/// instead of guessing at character counts — that is the only way a mixed
/// CJK/Latin label can be squeezed without tearing mid-token.
pub(crate) fn measure_text(
    text_system: &gpui::WindowTextSystem,
    font: &gpui::Font,
    size: f32,
    text: &str,
) -> f32 {
    text_system
        .shape_line(
            SharedString::from(text),
            px(size),
            &[gpui::TextRun {
                len: text.len(),
                font: font.clone(),
                color: gpui::Hsla::default(),
                background_color: None,
                underline: None,
                strikethrough: None,
            }],
            None,
        )
        .width
        .as_f32()
}

/// Elides a path from the front when it cannot fit `max_width`, keeping the
/// root marker (drive letter, `~`, or the leading slash) and every trailing
/// segment that fits.
///
/// The tail is what a user identifies a tab by — the file or directory they
/// are actually working on — so it is never torn: whole segments drop off the
/// front first (a half-eaten directory name reads as noise), and when even
/// the last segment is too wide, only that segment is elided character by
/// character, still tail-first.
pub(crate) fn elide_path_keep_tail(
    text_system: &gpui::WindowTextSystem,
    font: &gpui::Font,
    size: f32,
    path: &str,
    max_width: f32,
) -> SharedString {
    let path = path.trim();
    if path.is_empty() || measure_text(text_system, font, size, path) <= max_width {
        return SharedString::from(path);
    }
    let sep = path_separator(path);
    let segments: Vec<&str> = path.split(['/', '\\']).collect();
    // A leading slash splits into an empty first segment; `~` and drive
    // letters (`E:`) carry the same "where this tree lives" weight, and a
    // leading `…` means `short_title` already elided once — that marker is
    // replaced by the new elision instead of stacking two ellipses. Keep
    // whichever marker there is so the result never reads as a bare
    // relative path.
    let root: &str = match segments.first() {
        Some(&"") => "/",
        Some(&"~") => "~",
        Some(&"…") => "",
        Some(head) if head.ends_with(':') => head,
        _ => "",
    };
    let root_kept = segments
        .first()
        .is_some_and(|s| s.is_empty() || *s == "~" || *s == "…" || s.ends_with(':'));
    let prefix = if root.is_empty() {
        format!("…{sep}")
    } else if root == "/" {
        // The absolute-path root is already the slash itself.
        format!("/…{sep}")
    } else {
        format!("{root}{sep}…{sep}")
    };
    // Drop whole segments from the front until the remaining tail fits. The
    // width only shrinks as segments leave, so the first fit is the widest
    // one — greedy is optimal here.
    //
    // With a root marker, `head = 1` would spell the root, the ellipsis, and
    // then every remaining segment — strictly wider than the original that
    // already failed to fit — so that candidate is skipped rather than
    // measured.
    let mut head = if root_kept { 2 } else { 0 };
    while head < segments.len() {
        let candidate = if head == 0 {
            join_segments(&segments, sep)
        } else {
            format!("{prefix}{}", join_segments(&segments[head..], sep))
        };
        if measure_text(text_system, font, size, &candidate) <= max_width {
            return SharedString::from(candidate);
        }
        if head + 1 >= segments.len() {
            break;
        }
        head += 1;
    }
    // Even the last segment alone is too wide: keep its tail after the
    // ellipsis, with no slash so the reader sees the segment was torn.
    elide_tail_clusters(
        text_system,
        font,
        size,
        segments[segments.len() - 1],
        max_width,
    )
}

/// Characters a token is allowed to break on. Space is in the set because
/// this also elides labels a human typed — `Backend server logs` — not just
/// branch names, and a word boundary is the cut a reader forgives.
const TOKEN_BREAKS: [char; 5] = ['-', '_', '/', '.', ' '];

/// Splits `text` into grapheme clusters — what a reader counts as one
/// character, and the only place a label may be cut.
///
/// Slicing by `char` passes every width check and still tears the result:
/// `👨‍👩‍👧` loses the joiner holding it together, `❤️` loses the variation
/// selector that makes it an emoji (and the orphan then attaches itself to the
/// ellipsis), and `🇨🇳` leaves behind a lone regional indicator that renders
/// as a bare letter.
fn clusters(text: &str) -> Vec<&str> {
    text.graphemes(true).collect()
}

/// The head this token would rather keep: six clusters, extended to just past
/// the next break so the cut lands on a boundary (`window-…` rather than
/// `window…`). When no break is within reach the plain six is kept — running
/// on to the cap would spend the whole budget on a prefix and leave the tail,
/// which is what identifies the token, with nothing.
fn preferred_head(clusters: &[&str]) -> usize {
    let base = clusters.len().min(6);
    let cap = clusters.len().min(12);
    if base >= cap {
        return base;
    }
    match clusters[base..cap]
        .iter()
        .position(|c| c.chars().next().is_some_and(|c| TOKEN_BREAKS.contains(&c)))
    {
        Some(offset) => base + offset + 1,
        None => base,
    }
}

/// Elides the middle of a single token (a branch name, a shell name, a label
/// the user typed) so both its head and its identifying tail survive:
/// `window-transparency-backdrop` reads `window-…backdrop` in a narrow
/// sidebar instead of losing its tail to a trailing ellipsis.
///
/// A head that fits but leaves no room for a tail is worse than no head at
/// all, so the preferred head is given up for a shorter one when that is what
/// it takes to keep a few trailing clusters; only when even a three-cluster
/// head cannot buy a tail does this fall back to a tail-only elision.
pub(crate) fn elide_keep_edges(
    text_system: &gpui::WindowTextSystem,
    font: &gpui::Font,
    size: f32,
    text: &str,
    max_width: f32,
) -> SharedString {
    let text = text.trim();
    if text.is_empty() || measure_text(text_system, font, size, text) <= max_width {
        return SharedString::from(text);
    }
    let cells = clusters(text);
    let shaped = |head_n: usize, tail_n: usize| -> f32 {
        let mut s = cells[..head_n].concat();
        s.push('…');
        s.push_str(&cells[cells.len() - tail_n..].concat());
        measure_text(text_system, font, size, &s)
    };
    // Width is monotone in the tail length, so a binary search finds the
    // longest tail that still fits behind a given head.
    let longest_tail = |head_n: usize| -> usize {
        let (mut lo, mut hi) = (0usize, cells.len() - head_n);
        while lo < hi {
            let mid = (lo + hi + 1) / 2;
            if shaped(head_n, mid) <= max_width {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }
        lo
    };
    /// Enough trailing clusters to tell two sibling branches apart.
    const MIN_TAIL: usize = 3;
    let preferred = preferred_head(&cells);
    let mut candidates = vec![preferred, 6, 3];
    candidates.retain(|&h| h > 0 && h <= cells.len());
    candidates.dedup();
    let mut best: Option<(usize, usize)> = None;
    for head_n in candidates {
        if shaped(head_n, 0) > max_width {
            continue;
        }
        let tail = longest_tail(head_n);
        if tail >= MIN_TAIL.min(cells.len() - head_n) {
            best = Some((head_n, tail));
            break;
        }
        if best.is_none_or(|(_, best_tail)| tail > best_tail) {
            best = Some((head_n, tail));
        }
    }
    let Some((head, tail)) = best.filter(|&(_, tail)| tail > 0) else {
        // No head buys a tail worth showing; a bare tail says more.
        return elide_tail_clusters(text_system, font, size, text, max_width);
    };
    let mut out = cells[..head].concat();
    out.push('…');
    out.push_str(&cells[cells.len() - tail..].concat());
    SharedString::from(out)
}

/// Elides a row label. A path keeps its tail — the file or directory being
/// worked on — while anything else keeps both edges.
///
/// A shell title is not always a path: `npm run dev`, `man git-log`, or a name
/// the user typed into the rename box. Running those through the path rule
/// drops their head, which is the part that names them, and `… server logs`
/// says less than the CSS truncation this replaced.
pub(crate) fn elide_label(
    text_system: &gpui::WindowTextSystem,
    font: &gpui::Font,
    size: f32,
    text: &str,
    max_width: f32,
) -> SharedString {
    if text.contains('/') || text.contains('\\') {
        elide_path_keep_tail(text_system, font, size, text, max_width)
    } else {
        elide_keep_edges(text_system, font, size, text, max_width)
    }
}

/// Keeps the longest tail of `text` that fits after a bare ellipsis. Shared
/// by the path and token elisions as their last resort.
pub(crate) fn elide_tail_clusters(
    text_system: &gpui::WindowTextSystem,
    font: &gpui::Font,
    size: f32,
    text: &str,
    max_width: f32,
) -> SharedString {
    // Nothing to cut. The search below is bounded by the budget *after* the
    // ellipsis, so without this a branch that fit came back as "…" plus the
    // whole of itself — the rail's group header printed `…feat/v4-redesign`
    // with room to spare.
    if measure_text(text_system, font, size, text) <= max_width {
        return SharedString::from(text.to_string());
    }
    let budget = max_width - measure_text(text_system, font, size, "…");
    if budget <= 0. {
        return SharedString::from("…");
    }
    let cells = clusters(text);
    let (mut lo, mut hi) = (0usize, cells.len());
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        let s = cells[cells.len() - mid..].concat();
        if measure_text(text_system, font, size, &s) <= budget {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    if lo == 0 {
        return SharedString::from("…");
    }
    let mut out = String::with_capacity(1 + text.len());
    out.push('…');
    out.push_str(&cells[cells.len() - lo..].concat());
    SharedString::from(out)
}

#[derive(Clone)]
pub(crate) struct DragTab;

impl Render for DragTab {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

pub(crate) fn chord_tooltip(
    what: impl Into<SharedString>,
    action: &str,
    cx: &gpui::App,
) -> impl Fn(&mut Window, &mut App) -> gpui::Entity<Tooltip> + 'static {
    let what: SharedString = what.into();
    // Resolved now, while there is a `cx`: the builder below runs on hover, and
    // is handed only the window it is drawing into.
    let kbd = crate::ui::home::key_stroke(action, cx).map(Kbd::new);
    move |_window, cx| cx.new(|_| Tooltip::new(what.clone()).key_binding(kbd.clone()))
}

/// A sidebar row's trailing status dot, and the box it is centred in (wide
/// enough for the unread pill that replaces it).
pub(crate) const ROW_STATUS_DOT: f32 = 5.;
pub(crate) const ROW_STATUS_SLOT: f32 = 14.;
const MENU_HOSTS: usize = 6;

/// How wide the New Tab menu is allowed to get.
const MENU_W: Pixels = px(360.);

const MENU_H: Pixels = px(720.);

/// How this platform spells the key that turns a New Tab row into a split.
fn split_modifier() -> &'static str {
    if cfg!(target_os = "macos") {
        "⌥"
    } else {
        "Alt"
    }
}

struct NewTabMenu {
    app: gpui::WeakEntity<Tty7App>,
    shells: Vec<SharedString>,
    default_shell: SharedString,
    hosts: Vec<(uuid::Uuid, SharedString, SharedString)>,
    agents: Vec<CLIAgent>,
}

impl NewTabMenu {
    fn build(&self, menu: PopupMenu, window: &Window) -> PopupMenu {
        // Whatever [`MENU_H`] asks for, a menu still has to fit the window it
        // hangs off — on a short one the list gives way, not the window.
        //
        // Measured off the viewport, not `window_bounds()`: that one answers
        // "how should this window be reopened after it is closed", so on macOS
        // a fullscreen window reports the small bounds it would restore to,
        // not the screen it currently fills. A terminal spends much of its
        // life fullscreen, and reading that would cap the menu at 80% of a
        // window nobody is looking at — putting back the scrollbar and the
        // cut-off `Local` this whole change is here to remove.
        let ceiling = MENU_H.min(window.viewport_size().height * 0.8);
        let mut menu = menu
            .min_w(px(240.))
            // A menu is a list of names, not a place to read a full address.
            // Left to itself the panel widens to its longest row — one saved
            // host with a descriptive name and a long `user@host` drags every
            // other row out with it and the menu stops looking like chrome.
            .max_w(MENU_W)
            .scrollable(true)
            // Our own ceiling rather than `PopupMenu`'s built-in 450px, so the
            // one that applies is the one [`MENU_H`] reasons about.
            .max_h(ceiling)
            .item(PopupMenuItem::label(t(L10nKey::TabMenuLocalShells)));
        for label in &self.shells {
            let app = self.app.clone();
            let open = label.clone();
            let row = if *label == self.default_shell {
                let label = label.clone();
                PopupMenuItem::element(move |_window, cx| {
                    menu_row(label.clone(), t(L10nKey::ShellDefault).into(), cx)
                })
            } else {
                PopupMenuItem::new(label.clone())
            };
            menu = menu.item(row.on_click(move |_, window, cx| {
                let at = SpawnWhere::from_modifiers(window.modifiers());
                if let Some(app) = app.upgrade() {
                    app.update(cx, |this, cx| this.open_listed_shell(&open, at, window, cx));
                }
            }));
        }
        // No inventory yet — the machine has not answered, or this is a host
        // that reports none. The default shell is still openable.
        if self.shells.is_empty() {
            let app = self.app.clone();
            menu = menu.item(PopupMenuItem::new(t(L10nKey::AppMenuNewTab)).on_click(
                move |_, window, cx| {
                    let at = SpawnWhere::from_modifiers(window.modifiers());
                    if let Some(app) = app.upgrade() {
                        app.update(cx, |this, cx| this.open_shell(None, at, window, cx));
                    }
                },
            ));
        }
        menu = menu
            .item(PopupMenuItem::separator())
            .item(PopupMenuItem::label(t(L10nKey::CmdGroupSsh)));
        for (id, name, endpoint) in &self.hosts {
            let (id, name, endpoint) = (*id, name.clone(), endpoint.clone());
            let app = self.app.clone();
            // Every host row is a custom element, note or not: a plain item
            // renders its label as bare text with nothing to elide against,
            // and a host saved on its address alone is *named* `user@host:port`
            // — the longest string in the menu, on the row least able to cut
            // it. [`menu_row`] drops the right half when there is no note.
            let row = PopupMenuItem::element(move |_window, cx| {
                menu_row(name.clone(), endpoint.clone(), cx)
            });
            menu = menu.item(row.on_click(move |_, window, cx| {
                let at = SpawnWhere::from_modifiers(window.modifiers());
                if let Some(app) = app.upgrade() {
                    app.update(cx, |this, cx| {
                        this.connect_ssh_profile_at(id, at, window, cx)
                    });
                }
            }));
        }
        // With no hosts saved, the row that closes the section is the one that
        // gets you your first — the list of hosts is not somewhere to send
        // someone who has none.
        let app = self.app.clone();
        let empty = self.hosts.is_empty();
        let last = if empty {
            L10nKey::TabMenuAddHost
        } else {
            L10nKey::TabMenuAllHosts
        };
        menu = menu.item(PopupMenuItem::new(t(last)).on_click(move |_, window, cx| {
            if let Some(app) = app.upgrade() {
                app.update(cx, |this, cx| {
                    if empty {
                        this.open_new_ssh_host(window, cx);
                    } else {
                        this.open_settings_section(
                            crate::ui::settings::SettingsSection::Ssh,
                            window,
                            cx,
                        );
                    }
                });
            }
        }));

        if !self.agents.is_empty() {
            menu = menu
                .item(PopupMenuItem::separator())
                .item(PopupMenuItem::label(t(L10nKey::CmdGroupAgents)));
            for agent in self.agents.iter().copied() {
                let app = self.app.clone();
                menu = menu.item(PopupMenuItem::new(agent.display_name()).on_click(
                    move |_, window, cx| {
                        let at = SpawnWhere::from_modifiers(window.modifiers());
                        if let Some(app) = app.upgrade() {
                            app.update(cx, |this, cx| this.launch_agent(agent, at, window, cx));
                        }
                    },
                ));
            }
        }

        // The one place ⌥ is spelled out. Nothing else in the app teaches it,
        // and a modifier nobody is told about is a feature nobody has. No rule
        // above it: a separator divides two lists of things to pick, and this
        // is a footnote about the list it follows, not a section of its own.
        menu.item(PopupMenuItem::label(t_fmt(
            L10nKey::TabMenuSplitHint,
            &[("key", split_modifier())],
        )))
    }
}

pub(crate) fn shells_by_frecency<'a>(
    shells: &'a [DetectedShell],
    default: &str,
    usage: &std::collections::HashMap<String, crate::core::config::ProfileUsage>,
    now: u64,
) -> Vec<&'a DetectedShell> {
    let score = |s: &DetectedShell| usage.get(&s.label).map_or(0.0, |u| u.score(now));
    let mut sorted: Vec<&DetectedShell> = shells.iter().collect();
    // Stable, so shells nobody has opened keep the order the inventory chose.
    sorted.sort_by(|a, b| {
        (b.label == default)
            .cmp(&(a.label == default))
            .then_with(|| {
                score(b)
                    .partial_cmp(&score(a))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    });
    sorted
}

/// The hosts the menu names, in the order they were handed over — frecency,
/// so the ones that fit are the ones actually used.
///
/// A host saved without a name has nothing to show but where it goes, so it
/// leads with the endpoint rather than leaving the row blank — and then drops
/// the endpoint beside it, because a row that says `root@build.lan` twice
/// tells the reader nothing the first half did not.
fn menu_hosts(
    profiles: Vec<crate::core::ssh_profile::SshProfile>,
) -> Vec<(uuid::Uuid, SharedString, SharedString)> {
    profiles
        .into_iter()
        .take(MENU_HOSTS)
        .map(|p| {
            let endpoint = crate::core::ssh_profile::to_connect_string(&p);
            let name = if p.name.trim().is_empty() {
                endpoint.clone()
            } else {
                p.name.clone()
            };
            let note = if name == endpoint {
                String::new()
            } else {
                endpoint
            };
            (p.id, SharedString::from(name), SharedString::from(note))
        })
        .collect()
}

/// A menu row that names a thing on the left and says what it is on the right.
///
/// Both halves are cut rather than allowed to push: the panel stops at
/// [`MENU_W`], and a descriptive host name next to a long `user@host:port`
/// asks for more than that. Which half gives way is the whole point — the name
/// is what the reader is picking by, so it keeps its width up to
/// [`MENU_NAME_W`], and the endpoint takes whatever is left and elides first.
/// Sized the other way round (name growing from nothing, endpoint at its
/// natural width) a long address squeezes the name down to `..` and the row
/// names nothing at all.
///
/// Both caps are fixed lengths, never a share of the row. The panel is sized
/// to fit its content, so while the text is laid out no row has a width yet:
/// anything capped by `relative(..)` or by flex shrinking measures at full
/// length, is squeezed afterwards, and is clipped mid-glyph at the panel edge
/// (`git@ssh.github.con`) instead of ending in an ellipsis. A pixel cap is
/// known up front, so the text elides against it. The two caps and the row's
/// insets add up to [`MENU_W`].
///
/// An empty note drops the right half entirely rather than leaving a zero-width
/// child to hold the `gap_3` open — the name is then free to use the full row,
/// still eliding at the panel edge.
fn menu_row(label: SharedString, note: SharedString, cx: &gpui::App) -> impl IntoElement + use<> {
    let muted = cx.theme().muted_foreground;
    h_flex()
        .w_full()
        .items_center()
        .justify_between()
        .gap_3()
        .child(
            div()
                .flex_shrink_0()
                .max_w(match note.is_empty() {
                    true => MENU_W - MENU_ROW_INSET,
                    false => MENU_NAME_W,
                })
                .truncate()
                .child(label),
        )
        .when(!note.is_empty(), |this| {
            // Pushed right by the cell, not by `text_right`: a line set
            // with `truncate` ignores its alignment and hugs the name.
            this.child(
                h_flex().flex_1().justify_end().child(
                    div()
                        .max_w(MENU_NOTE_W)
                        .truncate()
                        .text_color(muted)
                        .child(note),
                ),
            )
        })
}

/// The most of a [`menu_row`] its name keeps before it elides — the larger
/// share, since the name is what the row is picked by.
const MENU_NAME_W: Pixels = px(172.);

/// The most the endpoint beside a name gets: room for `user@host:port` on a
/// typical host, and the gap and the insets take the rest of [`MENU_W`].
const MENU_NOTE_W: Pixels = px(150.);

/// What the panel spends on either side of a row's content: its own 5px
/// inset and the row's 8px padding, twice.
const MENU_ROW_INSET: Pixels = px(26.);

/// The words behind the status dot's colour.
pub(crate) fn agent_status_label(
    status: Option<crate::core::cli_agent::AgentStatus>,
) -> Option<&'static str> {
    use crate::core::cli_agent::AgentStatus;
    match status? {
        AgentStatus::Idle => None,
        AgentStatus::Working => Some(t(L10nKey::AgentStatusWorking)),
        AgentStatus::Waiting => Some(t(L10nKey::AgentStatusWaiting)),
        AgentStatus::Done => Some(t(L10nKey::AgentStatusDone)),
    }
}

pub(crate) const LIVE_DOT: u32 = 0x22C55E;
pub(crate) const UNKNOWN_DOT: u32 = 0x9AA0A6;

pub(crate) fn select_workspace_action(index: usize) -> Option<Box<dyn gpui::Action>> {
    Some(match index {
        0 => Box::new(SelectWorkspace1) as Box<dyn gpui::Action>,
        1 => Box::new(SelectWorkspace2),
        2 => Box::new(SelectWorkspace3),
        3 => Box::new(SelectWorkspace4),
        4 => Box::new(SelectWorkspace5),
        5 => Box::new(SelectWorkspace6),
        6 => Box::new(SelectWorkspace7),
        7 => Box::new(SelectWorkspace8),
        8 => Box::new(SelectWorkspace9),
        _ => return None,
    })
}

impl Tty7App {
    pub(crate) const AVATAR_PX: f32 = 18.0;

    pub(crate) fn workspace_head(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        if let Some(rename) = self.workspace_rename.as_ref() {
            return h_flex()
                .id("workspace-rename")
                .flex_shrink_0()
                .items_center()
                .h(px(28.))
                .w_full()
                .px(px(9.))
                .rounded_md()
                .bg(cx.theme().sidebar_accent)
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .child(Input::new(&rename.input).appearance(false).xsmall())
                .into_any_element();
        }

        crate::terminal::pane_liveness::sweep(cx);
        let current = crate::ui::machine_mirror::display_name_for(cx, self.workspace)
            .unwrap_or_else(|| "tty7".to_string());
        let monogram: String = current
            .chars()
            .next()
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_else(|| "~".to_string());

        div()
            .occlude()
            .w_full()
            .capture_any_mouse_down(|ev: &gpui::MouseDownEvent, _window, cx| {
                if ev.button == MouseButton::Right {
                    cx.stop_propagation();
                }
            })
            .child(
                Button::new("rail-workspace-head")
                    .custom(chrome_tile_variant(cx))
                    .child(
                        h_flex()
                            .id("rail-workspace-head-ink")
                            .w_full()
                            .h_full()
                            .items_center()
                            .gap(px(6.))
                            // The tile's own hover is a fill the palette keeps
                            // a hair off the surface, which on the rail is
                            // barely a change at all — and the name and the
                            // chevron pinned their own ink, so the pointer
                            // landing on the one control at the top of the
                            // column said nothing. Answer the way a group
                            // header does: the ink steps up to full strength.
                            .text_color(cx.theme().muted_foreground)
                            .hover(|s| s.text_color(cx.theme().foreground))
                            .child(
                                div()
                                    .flex()
                                    .flex_shrink_0()
                                    .items_center()
                                    .justify_center()
                                    .size(px(Self::AVATAR_PX))
                                    .rounded_full()
                                    .bg(cx.theme().secondary)
                                    .text_size(px(10.))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child(monogram),
                            )
                            .child(
                                // Chrome, not a row: the tile inherits the
                                // rail's title ink, which now belongs to the
                                // tabs. The workspace name reads at the group
                                // headers' weight so the one dark line in
                                // the column stays the tab in front.
                                div()
                                    .flex_shrink(1.)
                                    .min_w_0()
                                    .truncate()
                                    .text_size(px(13.))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(cx.theme().foreground)
                                    .child(SharedString::from(current.clone())),
                            )
                            .child(
                                // Not a chevron-down: this opens a centred
                                // panel, not a menu hanging off the button.
                                Icon::empty()
                                    .path("icons/chevrons-up-down.svg")
                                    .size(px(11.))
                                    .flex_shrink_0(),
                            ),
                    )
                    .xsmall()
                    .w_full()
                    .h(px(28.))
                    .rounded(px(7.))
                    // Button pins the arrow for every non-link variant; the
                    // switcher's own rows point, so the tile that opens them
                    // does too.
                    .cursor_pointer()
                    .accessibility_label(t(L10nKey::HomeSwitchWorkspace))
                    .tooltip_with_action(
                        t(L10nKey::HomeSwitchWorkspace),
                        &crate::core::actions::ToggleSwitcher,
                        None,
                    )
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.toggle_switcher(window, cx);
                    })),
            )
            .into_any_element()
    }

    /// Working and Done differ only in hue (blue vs green), and Waiting vs Done
    /// — the pair that actually decides whether you go and look — is amber vs
    /// green, the pair red-green colour vision separates worst. Give Waiting a
    /// hole so it is a different *shape*, not just a different colour.
    fn status_dot(
        rgb: u32,
        unread: usize,
        size: f32,
        ring: gpui::Hsla,
        hollow: bool,
        faded: bool,
    ) -> gpui::AnyElement {
        let d = (size * 0.42).max(7.);
        // The halo was the surface itself, which is only a ring while the
        // surface is light — on a dark theme it went near-black and read as a
        // notch bitten out of the avatar rather than a badge sitting on it.
        // Light themes already ring the dot in white; give the dark ones the
        // same white edge, and the hollow Waiting dot the same white hole.
        let bg = match crate::ui::presets::surface_is_dark(ring) {
            true => gpui::white(),
            false => ring,
        };
        // The dim beat of a blink is a paler fill, still opaque: fading the
        // whole badge let the avatar show through the dot and its ring.
        let fill: gpui::Hsla = match faded {
            true => bg.blend(gpui::Hsla::from(gpui::rgb(rgb)).opacity(0.4)),
            false => gpui::rgb(rgb).into(),
        };
        if unread > 0 {
            let nd = (size * 0.72).max(13.0);
            let label = unread.min(9).to_string();
            div()
                .absolute()
                .right(px(-(nd - d) / 2.0 - d * 0.22))
                .bottom(px(-(nd - d) / 2.0 - d * 0.22))
                .size(px(nd))
                .rounded_full()
                .border_1()
                .border_color(bg)
                .bg(fill)
                .flex()
                .items_center()
                .justify_center()
                .text_size(px((nd * 0.62).round()))
                .font_weight(FontWeight::BOLD)
                .text_color(gpui::white())
                .child(label)
                .into_any_element()
        } else {
            div()
                .absolute()
                .right(px(-(d * 0.22)))
                .bottom(px(-(d * 0.22)))
                .size(px(d))
                .rounded_full()
                .border_2()
                .border_color(bg)
                .bg(fill)
                .when(hollow, |dot| {
                    dot.flex()
                        .items_center()
                        .justify_center()
                        .child(div().size(px((d * 0.36).max(2.5))).rounded_full().bg(bg))
                })
                .into_any_element()
        }
    }

    pub(crate) fn tab_avatar(
        &self,
        id: impl Into<gpui::ElementId>,
        agent: Option<crate::core::cli_agent::CLIAgent>,
        status: Option<crate::core::cli_agent::AgentStatus>,
        unread: usize,
        ssh: Option<u32>,
        size: f32,
        cx: &App,
    ) -> gpui::AnyElement {
        self.tab_avatar_badged(id, agent, status, unread, ssh, size, true, cx)
    }

    /// The avatar without the agent's status badge, for a row that says the
    /// status at its trailing end instead (see [`row_status_dot`]). The
    /// tooltip still names the state; the SSH dot on a shell stays, since it
    /// describes the connection rather than an agent's run.
    ///
    /// [`row_status_dot`]: Self::row_status_dot
    pub(crate) fn tab_avatar_plain(
        &self,
        id: impl Into<gpui::ElementId>,
        agent: Option<crate::core::cli_agent::CLIAgent>,
        status: Option<crate::core::cli_agent::AgentStatus>,
        ssh: Option<u32>,
        size: f32,
        cx: &App,
    ) -> gpui::AnyElement {
        self.tab_avatar_badged(id, agent, status, 0, ssh, size, false, cx)
    }

    /// A row's trailing status mark: a 5px dot in the state's colour, hollow
    /// while the agent waits on the user, blinking while it works — or the
    /// unread count in a small pill once there is something to read. `None`
    /// for a row with nothing to report.
    pub(crate) fn row_status_dot(
        &self,
        status: Option<crate::core::cli_agent::AgentStatus>,
        unread: usize,
        surface: gpui::Hsla,
    ) -> Option<gpui::AnyElement> {
        let rgb = status.and_then(|s| s.dot_rgb())?;
        let hollow = status == Some(crate::core::cli_agent::AgentStatus::Waiting);
        let faded =
            status == Some(crate::core::cli_agent::AgentStatus::Working) && !self.working_dot_on;
        let ink: gpui::Hsla = gpui::rgb(rgb).into();
        let fill = match faded {
            true => surface.blend(ink.opacity(0.4)),
            false => ink,
        };
        let slot = div()
            .flex_shrink_0()
            .flex()
            .items_center()
            .justify_center()
            .min_w(px(ROW_STATUS_SLOT));
        Some(
            match unread > 0 {
                true => slot.child(
                    div()
                        .h(px(14.))
                        .min_w(px(14.))
                        .px(px(3.))
                        .rounded_full()
                        .bg(fill)
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(9.))
                        .font_weight(FontWeight::BOLD)
                        .text_color(gpui::white())
                        .child(unread.min(9).to_string()),
                ),
                false => slot.child(
                    div()
                        .size(px(ROW_STATUS_DOT))
                        .rounded_full()
                        .when(!hollow, |d| d.bg(fill))
                        .when(hollow, |d| d.border_1().border_color(fill)),
                ),
            }
            .into_any_element(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn tab_avatar_badged(
        &self,
        id: impl Into<gpui::ElementId>,
        agent: Option<crate::core::cli_agent::CLIAgent>,
        status: Option<crate::core::cli_agent::AgentStatus>,
        unread: usize,
        ssh: Option<u32>,
        size: f32,
        badge: bool,
        cx: &App,
    ) -> gpui::AnyElement {
        avatar_disc(
            id,
            Avatar {
                agent,
                status,
                unread,
                ssh,
            },
            size,
            self.working_dot_on,
            badge,
            cx,
        )
    }

    /// The mark a tab wears while one of its panes is zoomed over the others
    /// (#752). Without it a zoomed tab is pixel-for-pixel a tab that only ever
    /// had one pane, and the only way to tell was to toggle the zoom off.
    ///
    /// Drawn in the sidebar entry so it says something about the tabs you are *not*
    /// looking at — the zoom outlives a switch away from them.
    /// The mark a sleeping tab carries (#762), in the slot the zoom mark uses:
    /// leading, beside the other state marks, where the pointer that comes to
    /// read it does not cover it.
    pub(crate) fn sleep_mark(&self, id: impl Into<gpui::ElementId>, cx: &App) -> gpui::AnyElement {
        let tip = SharedString::from(t(L10nKey::TabTooltipAsleep));
        div()
            .id(id)
            .flex_shrink_0()
            .flex()
            .items_center()
            .justify_center()
            .size(px(16.))
            .text_color(cx.theme().muted_foreground)
            .child(Icon::new(IconName::Moon).size(px(11.)))
            .tooltip(move |window, cx| Tooltip::new(tip.clone()).build(window, cx))
            .into_any_element()
    }

    pub(crate) fn zoom_mark(&self, id: impl Into<gpui::ElementId>, cx: &App) -> gpui::AnyElement {
        let tip = chord_tooltip(t(L10nKey::TabTooltipZoomed), "ToggleMaximizePane", cx);
        div()
            .id(id)
            .flex_shrink_0()
            .flex()
            .items_center()
            .justify_center()
            .size(px(16.))
            .text_color(cx.theme().muted_foreground)
            .child(Icon::new(IconName::Maximize).size(px(11.)))
            .tooltip(move |window, cx| tip(window, cx).into())
            .into_any_element()
    }

    /// What this window puts on a tab of its own — the same ladder, through the
    /// same renderer, as the switcher uses for a tab of somebody else's window.
    /// See [`label_of`].
    pub(crate) fn tab_label(
        &self,
        tab: &Tab,
        index: usize,
        window: Option<&Window>,
        cx: &App,
    ) -> String {
        let (view, home) = tab.label_view(window, cx);
        label_of(&view, index, home.as_deref())
    }

    /// The same ladder as [`tab_label`](Self::tab_label), left whole: a path
    /// is only abbreviated under the home and stripped of its `user@host:`,
    /// never cut down to its last segments. For the surfaces with room to
    /// spare — the sidebar and the title bar — where only the available width
    /// decides what gets elided. `None` when there is nothing to say and the caller should
    /// fall back to its own placeholder.
    pub(crate) fn full_tab_label(
        &self,
        tab: &Tab,
        window: Option<&Window>,
        cx: &App,
    ) -> Option<String> {
        use crate::ui::machine_mirror::TabLabel;

        if let Some(name) = tab.name.as_ref().filter(|n| !n.trim().is_empty()) {
            return Some(name.trim().to_string());
        }
        let (view, home) = tab.label_view(window, cx);
        let raw = match view.label() {
            TabLabel::Osc(title) | TabLabel::Cwd(title) => {
                abbreviate_home(strip_host_prefix(title.trim()), home.as_deref()).into_owned()
            }
            TabLabel::Agent(agent) => agent.display_name().to_string(),
            TabLabel::Named(name) => name.to_string(),
            TabLabel::Process(title) => title.to_string(),
            TabLabel::Unknown => String::new(),
        };
        (!raw.trim().is_empty()).then_some(raw)
    }

    /// The New Tab control: one `+` that drops the list of everything it could
    /// open — the installed shells, and the saved SSH hosts.
    ///
    /// It was briefly split in two, a `+` that opened a tab outright next to a
    /// caret for the list, the way Windows Terminal and VS Code split theirs.
    /// That buys back the click the menu costs on the most common action, and
    /// it costs a second mark in a row of single ones. This row is four icons
    /// wide and reads as four icons; a fifth that was half of the fourth had
    /// to earn its place by looking like a pair, and a pair is exactly what a
    /// chrome row of plain icons has no vocabulary for.
    ///
    /// So: one tile, drawn and hovered like every other tile beside it, and
    /// the click it costs is answered by ⌘T rather than by a second button.
    pub(crate) fn new_tab_button(id: &'static str, cx: &Context<Self>) -> impl IntoElement + use<> {
        let app = cx.entity().downgrade();
        chrome_tile(Button::new(id).icon(Icon::new(IconName::Plus)), cx)
            .rounded(px(RAIL_TILE_RADIUS))
            // Every other tile in this row names itself on hover — Switch
            // Workspace, More, Hide Sidebar. The New Tab buttons that
            // come through here were the ones left silent. The chord is worth
            // more here than anywhere else in the row: it is the way back to
            // opening a tab without reading a menu first.
            .accessibility_label(t(L10nKey::AppMenuNewTab))
            .tooltip_with_action(
                t(L10nKey::AppMenuNewTab),
                &crate::core::actions::NewTab,
                None,
            )
            // Built when the menu opens, not when the button draws: this
            // closure runs once per press, and again after each dismissal.
            .dropdown_menu(move |menu, window, cx| {
                let Some(this) = app.upgrade() else {
                    return menu;
                };
                this.read(cx)
                    .new_tab_menu_rows(app.clone(), cx)
                    .build(menu, window)
            })
    }

    /// What the menu offers, read off the app as the menu opens — the builder
    /// runs on the popup's own entity, so the rows carry a weak handle back.
    fn new_tab_menu_rows(&self, app: gpui::WeakEntity<Self>, cx: &App) -> NewTabMenu {
        let default_shell = self.default_shell_label(cx);
        let usage = &cx.global::<Config>().shell_frecency;
        let now = crate::core::config::unix_now();
        NewTabMenu {
            app,
            shells: shells_by_frecency(&self.shells.shells, &default_shell, usage, now)
                .into_iter()
                .map(|s| SharedString::from(s.label.clone()))
                .collect(),
            default_shell: SharedString::from(default_shell),
            hosts: menu_hosts(crate::ui::ssh_connect::ssh_profiles_by_frecency(cx)),
            agents: self.offered_agents(cx),
        }
    }

    pub(crate) fn open_listed_shell(
        &mut self,
        label: &str,
        at: SpawnWhere,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(spec) = self
            .shells
            .shells
            .iter()
            .find(|s| s.label == label)
            .map(shell_spec)
        else {
            return;
        };
        self.update_config(cx, |cfg| {
            let entry = cfg.shell_frecency.entry(label.to_string()).or_default();
            entry.count = entry.count.saturating_add(1);
            entry.last_used = crate::core::config::unix_now();
        });
        self.open_shell(Some(spec), at, window, cx);
    }

    pub(crate) fn tab_context_menu(
        menu: PopupMenu,
        index: usize,
        app: &gpui::WeakEntity<Self>,
        window: &mut Window,
        cx: &mut Context<PopupMenu>,
    ) -> PopupMenu {
        let Some(entity) = app.upgrade() else {
            return menu;
        };
        let this = entity.read(cx);
        let targets = crate::ui::tab_sidebar::move_targets(
            &this.sidebar_group_keys(cx),
            &this.sidebar_groups,
            index,
        );
        let in_kept_group = this
            .tabs
            .get(index)
            .and_then(|t| t.group.get())
            .is_some_and(|g| this.sidebar_groups.contains(g));
        let has_tabs_below = this
            .visual_tab_order(cx)
            .last()
            .is_some_and(|&i| i != index);
        let tab_count = this.tabs.len();
        let cwd = this.tab_cwd_text(index, window, cx);
        let has_cwd = cwd.is_some();
        let mut menu = menu.min_w(px(200.));

        // Every item here acts on *this* tab, so the work is done by the click
        // handler and the action is carried only so `PopupMenu` can look its
        // chord up and print it. The handler wins when both are set. Without
        // this the tab menu was the one context menu in the app that taught no
        // shortcuts — right-clicking a pane offered "Split Right ⌘D" while
        // right-clicking its tab offered a bare "Split Right".
        menu = menu.item(
            PopupMenuItem::new(t(L10nKey::AppMenuRenameTab))
                .action(Box::new(RenameTab))
                .on_click({
                    let app = app.clone();
                    move |_, window, cx| {
                        let _ = app.update(cx, |this, cx| this.start_rename(index, window, cx));
                    }
                }),
        );

        let tab = this.tabs.get(index);
        if tab.is_some_and(|t| t.agent(cx).is_some()) {
            let done = tab.and_then(|t| t.agent_status(cx))
                == Some(crate::core::cli_agent::AgentStatus::Done);
            menu = menu.item(
                PopupMenuItem::new(t(L10nKey::TabContextMarkUnread))
                    .action(Box::new(MarkTabUnread))
                    .disabled(!done)
                    .on_click({
                        let app = app.clone();
                        move |_, _window, cx| {
                            let _ = app.update(cx, |this, cx| this.mark_tab_unread(index, cx));
                        }
                    }),
            );
        }

        // Sleep and wake (#762). Waking is also what selecting the tab does;
        // the item is here for waking one without leaving the tab on screen.
        // Hibernate is offered only where the machine can do it, and greyed
        // out for a tab it cannot be done to right now — the last one awake,
        // or one still connecting — rather than vanishing from under the
        // pointer that came looking for it.
        if tab.is_some_and(|t| t.is_asleep()) {
            menu = menu.item(PopupMenuItem::new(t(L10nKey::TabContextWake)).on_click({
                let app = app.clone();
                move |_, window, cx| {
                    let _ = app.update(cx, |this, cx| this.wake_tab(index, window, cx));
                }
            }));
        } else if crate::ui::tree_sync::can_hibernate_on(cx, this.workspace) {
            menu = menu.item(
                PopupMenuItem::new(t(L10nKey::TabContextHibernate))
                    .action(Box::new(HibernateTab))
                    .disabled(!this.can_hibernate_tab(index, cx))
                    .on_click({
                        let app = app.clone();
                        move |_, window, cx| {
                            let _ =
                                app.update(cx, |this, cx| this.hibernate_tab(index, window, cx));
                        }
                    }),
            );
        }

        // Where this tab sits, and where it could be put instead: every group
        // the sidebar draws, auto ones included, behind a submenu — a sidebar
        // grouping by repo can hold dozens, far too many to lay out flat.
        //
        // An auto group's membership is its tabs' cwds, so a tab can only be
        // put in one by keeping the group: picking it pins the group (as its
        // header's pin does) and the tab with it. Remove from Group, offered
        // only for a tab kept in a group, hands it back to auto grouping.
        //
        {
            let app = app.clone();
            menu = menu.separator().submenu(
                t(L10nKey::SidebarMoveToGroup),
                window,
                cx,
                move |sub, _window, _cx| {
                    // A left check makes every row of the menu reserve a
                    // check column, so the whole submenu sat one icon's
                    // width right of the tab menu beside it. On the right,
                    // its labels line up with the parent's.
                    let mut sub = sub.check_side(Side::Right);
                    for (i, target) in targets.iter().enumerate() {
                        // Pinned groups, then the auto ones, as the divider
                        // splits them in the sidebar.
                        if i > 0 && targets[i - 1].key.is_pinned() && !target.key.is_pinned() {
                            sub = sub.separator();
                        }
                        let mut item =
                            PopupMenuItem::new(target.name.clone()).checked(target.checked);
                        if !target.checked {
                            let app = app.clone();
                            let key = target.key.clone();
                            item = item.on_click(move |_, _window, cx| {
                                let _ = app.update(cx, |this, cx| {
                                    this.move_tab_to(index, key.clone(), cx)
                                });
                            });
                        }
                        sub = sub.item(item);
                    }
                    if !targets.is_empty() {
                        sub = sub.separator();
                    }
                    let remove = app.clone();
                    let new = app.clone();
                    sub.item(
                        PopupMenuItem::new(t(L10nKey::SidebarRemoveFromGroup))
                            .disabled(!in_kept_group)
                            .on_click(move |_, _window, cx| {
                                let _ = remove
                                    .update(cx, |this, cx| this.set_tab_group(index, None, cx));
                            }),
                    )
                    .item(
                        PopupMenuItem::new(t(L10nKey::SidebarNewGroup)).on_click(
                            move |_, window, cx| {
                                let _ = new
                                    .update(cx, |this, cx| this.new_tab_group(index, window, cx));
                            },
                        ),
                    )
                },
            );
        }
        // Read again: the submenu above needed `cx` mutably.
        let this = entity.read(cx);

        let agent_session = this.tab_agent_session(index, window, cx);
        if let Some((source, session)) = &agent_session
            && let Some(label) = session.fork_label
        {
            menu = menu.separator();
            let forkable = session.forkable();
            menu = menu.item(
                PopupMenuItem::new(label)
                    .action(Box::new(ForkAgentSession))
                    .disabled(!forkable)
                    .on_click({
                        let app = app.clone();
                        let source = source.clone();
                        move |_, window, cx| {
                            let source = source.clone();
                            let _ = app.update(cx, |this, cx| {
                                this.fork_agent_session(
                                    index,
                                    source,
                                    crate::ui::app::ForkPlacement::NewTab,
                                    window,
                                    cx,
                                )
                            });
                        }
                    }),
            );
        }

        if let Some(id) = this.tabs.get(index).and_then(|tab| {
            let leaf = tab.pane.focused_or_first(window, cx)?;
            let spec = leaf.read(cx).ssh_spec()?;
            let id = uuid::Uuid::parse_str(spec.profile_id.as_deref()?).ok()?;
            cx.global::<Config>()
                .ssh_profiles
                .iter()
                .any(|profile| profile.id == id)
                .then_some(id)
        }) {
            menu =
                menu.separator()
                    .item(PopupMenuItem::new(t(L10nKey::SwitcherEditHost)).on_click({
                        let app = app.clone();
                        move |_, window, cx| {
                            let _ = app.update(cx, |this, cx| {
                                this.open_ssh_profile_in_settings(id, window, cx)
                            });
                        }
                    }));
        }

        menu = menu
            .separator()
            .item(
                PopupMenuItem::new(t(L10nKey::AppMenuSplitRight))
                    .action(Box::new(SplitRight))
                    .on_click({
                        let app = app.clone();
                        move |_, window, cx| {
                            let _ = app.update(cx, |this, cx| {
                                this.activate(index, window, cx);
                                this.split(Axis::Horizontal, window, cx);
                            });
                        }
                    }),
            )
            .item(
                PopupMenuItem::new(t(L10nKey::AppMenuSplitDown))
                    .action(Box::new(SplitDown))
                    .on_click({
                        let app = app.clone();
                        move |_, window, cx| {
                            let _ = app.update(cx, |this, cx| {
                                this.activate(index, window, cx);
                                this.split(Axis::Vertical, window, cx);
                            });
                        }
                    }),
            );

        menu = menu.separator().item(
            PopupMenuItem::new(t(L10nKey::AppMenuCopyWorkingDirectory))
                .action(Box::new(CopyWorkingDirectory))
                .disabled(!has_cwd)
                .on_click(move |_, _window, cx| {
                    if let Some(text) = cwd.as_ref() {
                        cx.write_to_clipboard(gpui::ClipboardItem::new_string(text.clone()));
                    }
                }),
        );

        if let Some(session_id) = agent_session.map(|(_, s)| s.session_id) {
            menu = menu.item(
                PopupMenuItem::new(t(L10nKey::AppMenuCopySessionId))
                    .action(Box::new(CopyAgentSessionId))
                    .disabled(session_id.is_none())
                    .on_click(move |_, _window, cx| {
                        if let Some(id) = session_id.as_ref() {
                            cx.write_to_clipboard(gpui::ClipboardItem::new_string(id.clone()));
                        }
                    }),
            );
        }

        menu.separator()
            .item(
                PopupMenuItem::new(t(L10nKey::TabContextCloseTab))
                    .action(Box::new(CloseActiveTab))
                    .on_click({
                        let app = app.clone();
                        move |_, window, cx| {
                            let _ = app.update(cx, |this, cx| this.close_tab(index, window, cx));
                        }
                    }),
            )
            .item(
                PopupMenuItem::new(t(L10nKey::AppMenuCloseOtherTabs))
                    .action(Box::new(CloseOtherTabs))
                    .disabled(tab_count <= 1)
                    .on_click({
                        let app = app.clone();
                        move |_, window, cx| {
                            let _ =
                                app.update(cx, |this, cx| this.close_other_tabs(index, window, cx));
                        }
                    }),
            )
            .item(
                PopupMenuItem::new(t(L10nKey::AppMenuCloseTabsBelow))
                    .action(Box::new(CloseTabsBelow))
                    .disabled(!has_tabs_below)
                    .on_click({
                        let app = app.clone();
                        move |_, window, cx| {
                            let _ =
                                app.update(cx, |this, cx| this.close_tabs_below(index, window, cx));
                        }
                    }),
            )
    }
}

#[derive(Clone, Copy)]
struct Avatar {
    agent: Option<CLIAgent>,
    status: Option<crate::core::cli_agent::AgentStatus>,
    unread: usize,
    ssh: Option<u32>,
}

fn avatar_disc(
    id: impl Into<gpui::ElementId>,
    Avatar {
        agent,
        status,
        unread,
        ssh,
    }: Avatar,
    size: f32,
    blink_on: bool,
    badge: bool,
    cx: &App,
) -> gpui::AnyElement {
    // The wrapper positions; the disc below carries the radius.
    // `status_dot` hangs itself off the edge with negative offsets — that
    // overhang is what makes it a badge on the avatar rather than a notch
    // in it — and as a child of the rounded element the overhang was
    // clipped along the arc, leaving a crescent.
    let base = div().id(id).flex_shrink_0().relative().size(px(size));
    // Fill, hairline and mark all live here, so the radius only ever clips
    // the disc's own paint.
    let disc = || {
        div()
            .size(px(size))
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
    };
    match agent {
        Some(agent) => {
            let hollow = status == Some(crate::core::cli_agent::AgentStatus::Waiting);
            let dot = status
                .filter(|_| badge)
                .and_then(|s| s.dot_rgb())
                .map(|rgb| {
                    // A working agent's dot blinks, so a column of tabs
                    // says at a glance which ones are still going.
                    let faded =
                        status == Some(crate::core::cli_agent::AgentStatus::Working) && !blink_on;
                    Tty7App::status_dot(rgb, unread, size, cx.theme().background, hollow, faded)
                });
            // Which agent this is, and what it wants, were carried entirely
            // by a brand hue and a nine-pixel dot. Say it in words too.
            let tip = match agent_status_label(status) {
                Some(state) => format!("{} — {state}", agent.display_name()),
                None => agent.display_name().to_string(),
            };
            // The disc is a solid fill of the agent's brand on every
            // row, lit or not: a tint reads as a disabled tab, and the
            // colour is how the eye tells one agent from another down a
            // column of twenty.
            let accent = agent.accent_rgb();
            let surface = cx.theme().background;
            base.child(
                disc()
                    .bg(gpui::rgb(accent))
                    // Codex and Grok are both pure black, which is the
                    // window fill on a dark theme — the disc dissolves and
                    // leaves the glyph floating. A hairline keeps it a disc
                    // in any theme.
                    .when(crate::ui::presets::needs_edge(accent, surface), |d| {
                        d.border_1().border_color(cx.theme().border)
                    })
                    .child(
                        gpui::svg()
                            .path(agent.icon_path())
                            .size(px(size * 0.54))
                            // SVG assets render as a single-colour mask, so
                            // the mark's colour comes from the agent rather
                            // than from the file. The tray icon reads the
                            // same answer.
                            .text_color(gpui::rgb(agent.icon_rgb())),
                    ),
            )
            .when_some(dot, |b, dot| b.child(dot))
            .tooltip(move |window, cx| {
                gpui::component::tooltip::Tooltip::new(tip.clone()).build(window, cx)
            })
            .into_any_element()
        }
        None => base
            .child(
                disc().bg(cx.theme().muted).child(
                    gpui::svg()
                        .path("icons/terminal.svg")
                        .size(px(size * 0.56))
                        .text_color(cx.theme().foreground.opacity(0.65)),
                ),
            )
            .when_some(ssh, |b, rgb| {
                b.child(Tty7App::status_dot(
                    rgb,
                    0,
                    size,
                    cx.theme().background,
                    false,
                    false,
                ))
            })
            .into_any_element(),
    }
}

#[cfg(test)]
mod fold_tests {
    use super::*;
    use crate::ui::app::test_window::harness_with_tabs;
    use gpui::TestAppContext;

    /// Bounds a row registered for itself while it was on screen. A folded
    /// row leaves the default rectangle behind, and that is what stops a pane
    /// being dropped into a group that is shut.
    fn drawn(app: &Tty7App, i: usize) -> bool {
        app.sidebar_slots.borrow()[i].size.height > px(0.)
    }

    /// Put tab `i` in a directory and tell the cache that directory is in the
    /// repo `root`, whose home is `home` — the same as `root` unless the
    /// checkout is a linked worktree. Both halves are needed: the sidebar
    /// reads the tab's cwd and looks it up in the cache, and either one
    /// missing makes it return "no decision", which would leave every group
    /// below untouched and every assertion about moving vacuous.
    fn plant(app: &Tty7App, i: usize, cwd: &str, root: &str, home: &str, cx: &mut gpui::App) {
        use crate::terminal::git_status::{GitStatusCache, RepoSnapshot};
        use crate::ui::host_ops::HostId;

        let cwd = PathBuf::from(cwd);
        let leaf = app.tabs[i].pane.first_leaf().expect("test tab has a pane");
        leaf.terminal()
            .expect("test pane is a terminal")
            .update(cx, |view, _| {
                view.set_git_status_cwd_for_test(Some(cwd.clone()))
            });
        cx.update_global::<GitStatusCache, _>(|cache, _| {
            cache.finish_probe(
                HostId::LOCAL,
                &cwd,
                Some(RepoSnapshot {
                    root: PathBuf::from(root),
                    home: PathBuf::from(home),
                    branch: "main".into(),
                    counts: Some((0, 0)),
                }),
            );
        });
    }

    fn plant_repo(app: &Tty7App, i: usize, cwd: &str, root: &str, cx: &mut gpui::App) {
        plant(app, i, cwd, root, root, cx);
    }

    /// Put tab `i` somewhere the cache knows is in no repo at all.
    fn plant_plain(app: &Tty7App, i: usize, cwd: &str, cx: &mut gpui::App) {
        use crate::terminal::git_status::GitStatusCache;
        use crate::ui::host_ops::HostId;

        let cwd = PathBuf::from(cwd);
        let leaf = app.tabs[i].pane.first_leaf().expect("test tab has a pane");
        leaf.terminal()
            .expect("test pane is a terminal")
            .update(cx, |view, _| {
                view.set_git_status_cwd_for_test(Some(cwd.clone()))
            });
        cx.update_global::<GitStatusCache, _>(|cache, _| {
            cache.finish_probe(HostId::LOCAL, &cwd, None);
        });
    }

    fn repo(s: &str) -> Option<GroupKey> {
        Some(GroupKey::Auto(AutoKey::Repo(PathBuf::from(s))))
    }

    /// Pin a label group called `name`, answering its id.
    fn label(app: &mut Tty7App, name: &str, cx: &mut Context<Tty7App>) -> GroupId {
        let group = PinnedGroup::label(name);
        let id = group.id;
        app.edit_groups(cx, |groups| groups.pinned.push(group));
        id
    }

    /// Pin `folder`, answering the group's id.
    fn folder(app: &mut Tty7App, folder: &str, cx: &mut Context<Tty7App>) -> GroupId {
        app.pin_folder(PathBuf::from(folder), cx);
        app.sidebar_groups
            .pinned
            .iter()
            .find(|g| g.folder.as_deref() == Some(folder))
            .expect("just pinned")
            .id
    }

    /// Rule 1. A tab kept in a pinned group is not the probe's to move:
    /// without this the cwd would drag a hand-placed tab back into its repo
    /// on the very next frame, and no amount of clicking would keep it where
    /// it was put.
    ///
    /// Tab 1 is the control. It is kept nowhere, so the same probe that must
    /// leave tab 0 alone has to file tab 1 — otherwise this test would pass
    /// just as well with the probe switched off entirely.
    #[gpui::test]
    fn a_probe_files_an_auto_tab_and_never_a_pinned_one(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 2);

        let work = app.update(&mut vcx, |app, cx| {
            let work = label(app, "work", cx);
            app.set_tab_group(0, Some(work), cx);
            for i in 0..2 {
                plant_repo(app, i, "/w/probed/sub", "/w/probed", cx);
            }
            cx.notify();
            work
        });
        vcx.run_until_parked();

        app.update(&mut vcx, |app, cx| {
            let keys = app.sidebar_group_keys(cx);
            assert_eq!(
                keys[0],
                Some(GroupKey::Pinned(work)),
                "the pinned group survived a probe that had a real answer"
            );
            assert_eq!(
                keys[1],
                repo("/w/probed"),
                "and that same probe did file the tab that was kept nowhere"
            );
        });
    }

    /// Dropping a tab below the divider is the way out of a pinned group: the
    /// cwd decides again.
    #[gpui::test]
    fn a_tab_handed_back_to_auto_follows_its_cwd_again(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 1);

        let (work, dragged) = app.update(&mut vcx, |app, cx| {
            plant_repo(app, 0, "/w/probed/sub", "/w/probed", cx);
            let work = label(app, "work", cx);
            app.set_tab_group(0, Some(work), cx);
            (work, app.tabs[0].tree_id.get())
        });
        vcx.run_until_parked();
        app.update(&mut vcx, |app, cx| {
            assert_eq!(app.sidebar_group_keys(cx)[0], Some(GroupKey::Pinned(work)));
        });

        app.update(&mut vcx, |app, cx| {
            app.regroup_tab(dragged, reorder::Regroup::ToAuto, cx)
        });
        vcx.run_until_parked();

        app.update(&mut vcx, |app, cx| {
            assert_eq!(
                app.sidebar_group_keys(cx)[0],
                repo("/w/probed"),
                "dropped below the divider, so the probe takes the tab back"
            );
        });
    }

    /// A drag is several frames long, so the tab it is carrying is named by
    /// id. Were it an index, any tab closing before the drop — in another
    /// window on the same workspace, or by a shell exiting — would shift it,
    /// and the drop would land on whichever tab slid into that slot.
    #[gpui::test]
    fn a_drop_finds_its_tab_after_the_indexes_shift(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 3);

        let dragged = app.update(&mut vcx, |app, _| app.tabs[2].tree_id.get());
        let work = app.update_in(&mut vcx, |app, window, cx| {
            let work = label(app, "work", cx);
            app.close_tab(0, window, cx);
            app.regroup_tab(dragged, reorder::Regroup::Into(work), cx);
            work
        });
        vcx.run_until_parked();

        app.update(&mut vcx, |app, _| {
            let moved = app
                .tabs
                .iter()
                .find(|t| t.tree_id.get() == dragged)
                .expect("the dragged tab is still open");
            assert_eq!(
                moved.group.get(),
                Some(work),
                "the tab that was picked up is the tab that moved"
            );
            assert!(
                app.tabs
                    .iter()
                    .filter(|t| t.tree_id.get() != dragged)
                    .all(|t| t.group.get().is_none()),
                "and no bystander was regrouped in its place"
            );
        });
    }

    /// ⌘T inside a pinned group has to land in it. Otherwise the new tab
    /// goes wherever its cwd says, and if the group it was opened from is
    /// folded, the only thing that happens on screen is the header's count
    /// going up by one — the symptom #804 fixed for repo groups.
    #[gpui::test]
    fn a_tab_spawned_inside_a_pinned_group_joins_it(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 1);

        let work = app.update(&mut vcx, |app, cx| {
            // A probe that would send the tab somewhere else if it were
            // consulted, so this cannot pass by there being no answer.
            plant_repo(app, 0, "/w/probed/sub", "/w/probed", cx);
            let work = label(app, "work", cx);
            app.set_tab_group(0, Some(work), cx);
            app.active = 0;
            work
        });
        vcx.run_until_parked();

        app.update(&mut vcx, |app, cx| {
            let place = app.spawn_group(Some(&PathBuf::from("/w/probed/sub")), cx);
            assert_eq!(
                place.group,
                Some(work),
                "the pinned group is inherited ahead of anything the cwd says"
            );
        });

        app.update(&mut vcx, |app, cx| app.set_tab_group(0, None, cx));
        vcx.run_until_parked();

        app.update(&mut vcx, |app, cx| {
            let place = app.spawn_group(Some(&PathBuf::from("/w/probed/sub")), cx);
            assert_eq!(place.group, None, "with nothing kept, nothing inherited");
            assert_eq!(
                place.auto,
                Some(Some(AutoKey::Repo(PathBuf::from("/w/probed")))),
                "and the cwd seeds the auto group from the warm cache"
            );
        });
    }

    /// ⌘T from an SSH tab dials the same host, and the new tab is filed
    /// under that host before its pane has reported where it is — not in
    /// Ungrouped, which is where the local shell it used to open landed.
    #[gpui::test]
    fn a_tab_dialling_a_host_is_filed_under_it(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 1);

        app.update(&mut vcx, |app, cx| {
            let place = app.spawn_group(None, cx).on_host(Some("hermes_ali".into()));
            assert_eq!(
                place.auto,
                Some(Some(AutoKey::SshHost("hermes_ali".into())))
            );
            let place = app.spawn_group(None, cx).on_host(None);
            assert_eq!(place.auto, None, "no host, no opinion");
        });
    }

    /// A tab opened in a pinned folder is filed there before its shell has
    /// said a word — from the cwd it was opened in.
    #[gpui::test]
    fn a_tab_spawned_in_a_pinned_folder_joins_it(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 1);

        app.update(&mut vcx, |app, cx| {
            let tty7 = folder(app, "/w/tty7", cx);
            let place = app.spawn_group(Some(&PathBuf::from("/w/tty7/src")), cx);
            assert_eq!(place.group, Some(tty7));
            let place = app.spawn_group(Some(&PathBuf::from("/w/else")), cx);
            assert_eq!(place.group, None);
        });
    }

    /// A rename lands on the group itself: its tabs point at it by id, so
    /// nothing about them has to change.
    #[gpui::test]
    fn renaming_a_group_keeps_its_tabs_in_it(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 2);

        let work = app.update_in(&mut vcx, |app, window, cx| {
            let work = label(app, "work", cx);
            app.set_tab_group(0, Some(work), cx);
            app.set_tab_group(1, Some(work), cx);
            app.start_group_rename(work, window, cx);
            app.group_rename
                .as_ref()
                .expect("the box is up")
                .input
                .update(cx, |state, cx| state.set_value("urgent", window, cx));
            app.commit_group_rename(window, cx);
            work
        });
        vcx.run_until_parked();

        app.update(&mut vcx, |app, _| {
            assert_eq!(app.pinned_group_names(), vec![(work, "urgent".to_string())]);
            assert!(app.tabs.iter().all(|t| t.group.get() == Some(work)));
        });
    }

    /// Clearing the box and dismissing it must not leave a label group with
    /// no name to draw; a folder group, which has its folder's name to fall
    /// back on, goes back to reading it.
    #[gpui::test]
    fn a_blank_rename_keeps_a_label_and_resets_a_folder(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 1);

        app.update_in(&mut vcx, |app, window, cx| {
            let work = label(app, "work", cx);
            let tty7 = folder(app, "/w/tty7", cx);
            app.edit_groups(cx, |groups| {
                groups.get_mut(tty7).expect("pinned").name = Some("mine".into())
            });
            for id in [work, tty7] {
                app.start_group_rename(id, window, cx);
                app.group_rename
                    .as_ref()
                    .expect("the box is up")
                    .input
                    .update(cx, |state, cx| state.set_value("   ", window, cx));
                app.commit_group_rename(window, cx);
            }
            assert_eq!(
                app.pinned_group_names(),
                vec![(work, "work".to_string()), (tty7, "tty7".to_string())],
                "the label kept its name; the folder went back to its own"
            );
        });
    }

    /// The placeholder only has to be unique — the box opens selected, so
    /// the first keystroke replaces it. But two headers with one name are two
    /// groups nobody can tell apart.
    #[gpui::test]
    fn a_second_new_group_does_not_land_on_the_first(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 2);

        app.update_in(&mut vcx, |app, window, cx| {
            app.new_tab_group(0, window, cx);
            app.commit_group_rename(window, cx);
            app.new_tab_group(1, window, cx);
            app.commit_group_rename(window, cx);
        });
        vcx.run_until_parked();

        app.update(&mut vcx, |app, _| {
            let names = app.pinned_group_names();
            assert_eq!(names.len(), 2, "two groups, not one shared by both tabs");
            assert_ne!(names[0].1, names[1].1);
            assert_ne!(app.tabs[0].group.get(), app.tabs[1].group.get());
        });
    }

    /// "Auto grouping off" hides the groups the sidebar works out, and
    /// Ungrouped's header with them — but a pinned group is the user's, and
    /// still stands.
    #[gpui::test]
    fn auto_grouping_off_hides_auto_groups_and_keeps_pinned_ones(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 3);

        let work = app.update(&mut vcx, |app, cx| {
            let work = label(app, "work", cx);
            app.set_tab_group(0, Some(work), cx);
            plant_repo(app, 1, "/w/alpha", "/w/alpha", cx);
            plant_plain(app, 2, "/tmp", cx);
            let mut cfg = cx.global::<Config>().clone();
            cfg.sidebar_auto_grouping = false;
            cx.set_global(cfg);
            cx.notify();
            work
        });
        vcx.run_until_parked();

        app.update(&mut vcx, |app, cx| {
            let keys = app.sidebar_group_keys(cx);
            assert_eq!(keys, vec![Some(GroupKey::Pinned(work)), None, None]);
            let sections = sidebar_sections(&keys, &app.sidebar_groups);
            assert_eq!(
                sections.iter().map(|s| s.name.clone()).collect::<Vec<_>>(),
                vec![Some("work".into()), None],
                "the pinned header, then a flat list with no header over it"
            );
        });

        app.update(&mut vcx, |_, cx| {
            let mut cfg = cx.global::<Config>().clone();
            cfg.sidebar_auto_grouping = true;
            cx.set_global(cfg);
            cx.notify();
        });
        vcx.run_until_parked();

        app.update(&mut vcx, |app, cx| {
            let keys = app.sidebar_group_keys(cx);
            assert_eq!(
                keys,
                vec![Some(GroupKey::Pinned(work)), repo("/w/alpha"), None]
            );
            let sections = sidebar_sections(&keys, &app.sidebar_groups);
            assert_eq!(
                sections.last().and_then(|s| s.name.clone()),
                Some("Ungrouped".into()),
                "beside an auto group, the rest is Ungrouped again"
            );
        });
    }

    /// Rule 2: a tab walking into a pinned folder joins it.
    #[gpui::test]
    fn a_tab_walking_into_a_pinned_folder_joins_it(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 2);

        let probed = app.update(&mut vcx, |app, cx| {
            plant_plain(app, 0, "/w/elsewhere", cx);
            plant_plain(app, 1, "/w/elsewhere", cx);
            let probed = folder(app, "/w/probed", cx);
            cx.notify();
            probed
        });
        vcx.run_until_parked();
        app.update(&mut vcx, |app, cx| {
            plant_repo(app, 0, "/w/probed/sub", "/w/probed", cx);
            cx.notify();
        });
        vcx.run_until_parked();

        app.update(&mut vcx, |app, cx| {
            assert_eq!(app.tabs[0].group.get(), Some(probed), "walked in: joined");
            assert_eq!(app.tabs[1].group.get(), None, "stayed out: did not");
            assert_eq!(
                app.sidebar_group_keys(cx)[0],
                Some(GroupKey::Pinned(probed)),
                "and the pinned folder beats the repo root it sits in"
            );
        });
    }

    /// Nested folders: the deepest one a tab is inside wins, so pinning a
    /// monorepo and a package in it files the package's tabs under the
    /// package.
    #[gpui::test]
    fn nested_pinned_folders_file_a_tab_under_the_deepest(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 2);

        let (mono, pkg) = app.update(&mut vcx, |app, cx| {
            let mono = folder(app, "/w/mono", cx);
            let pkg = folder(app, "/w/mono/pkg", cx);
            plant_repo(app, 0, "/w/mono/pkg/src", "/w/mono", cx);
            plant_repo(app, 1, "/w/mono/docs", "/w/mono", cx);
            cx.notify();
            (mono, pkg)
        });
        vcx.run_until_parked();

        app.update(&mut vcx, |app, _| {
            assert_eq!(app.tabs[0].group.get(), Some(pkg));
            assert_eq!(app.tabs[1].group.get(), Some(mono));
        });
    }

    /// A linked worktree lives outside its main checkout, but its repo home
    /// is that checkout — so pinning the repo keeps its worktrees too.
    #[gpui::test]
    fn a_worktree_joins_the_folder_its_repo_home_is_pinned_as(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 1);

        let tty7 = app.update(&mut vcx, |app, cx| {
            let tty7 = folder(app, "/w/tty7", cx);
            plant(
                app,
                0,
                "/tmp/wt/feature/src",
                "/tmp/wt/feature",
                "/w/tty7",
                cx,
            );
            cx.notify();
            tty7
        });
        vcx.run_until_parked();

        app.update(&mut vcx, |app, _| {
            assert_eq!(app.tabs[0].group.get(), Some(tty7));
        });
    }

    /// The edge the whole rule turns on. A tab dragged out of a folder group
    /// while it is still in the folder has been told where to go; it must not
    /// be pulled straight back, and only walking out and in again rejoins it.
    #[gpui::test]
    fn a_tab_dragged_out_is_not_pulled_back_until_it_re_enters(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 1);

        let probed = app.update(&mut vcx, |app, cx| {
            let probed = folder(app, "/w/probed", cx);
            plant_repo(app, 0, "/w/probed/sub", "/w/probed", cx);
            cx.notify();
            probed
        });
        vcx.run_until_parked();
        app.update(&mut vcx, |app, cx| {
            assert_eq!(
                app.tabs[0].group.get(),
                Some(probed),
                "joined on the way in"
            );
            app.set_tab_group(0, None, cx);
        });
        for _ in 0..3 {
            app.update(&mut vcx, |_, cx| cx.notify());
            vcx.run_until_parked();
        }
        app.update(&mut vcx, |app, cx| {
            assert_eq!(app.tabs[0].group.get(), None, "still inside: stays out");
            plant_plain(app, 0, "/w/elsewhere", cx);
            cx.notify();
        });
        vcx.run_until_parked();
        app.update(&mut vcx, |app, cx| {
            assert_eq!(app.tabs[0].group.get(), None, "left: nothing to join");
            plant_repo(app, 0, "/w/probed/sub", "/w/probed", cx);
            cx.notify();
        });
        vcx.run_until_parked();
        app.update(&mut vcx, |app, _| {
            assert_eq!(app.tabs[0].group.get(), Some(probed), "re-entered: joined");
        });
    }

    /// A tab restored from the machine tree was filed by whoever had it
    /// last. Sitting in a folder at launch is not walking into it.
    #[gpui::test]
    fn a_restored_tab_is_not_pulled_in_by_where_it_already_is(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 1);

        app.update(&mut vcx, |app, cx| {
            app.tabs[0]
                .folder_watch
                .set(crate::core::group_key::EntryWatch::baseline());
            folder(app, "/w/probed", cx);
            plant_repo(app, 0, "/w/probed/sub", "/w/probed", cx);
            cx.notify();
        });
        vcx.run_until_parked();

        app.update(&mut vcx, |app, _| {
            assert_eq!(app.tabs[0].group.get(), None);
        });
    }

    /// Deleting a group closes nothing: its tabs go back to auto grouping.
    #[gpui::test]
    fn deleting_a_group_returns_its_tabs_to_auto_grouping(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 2);

        let work = app.update(&mut vcx, |app, cx| {
            let work = label(app, "work", cx);
            plant_repo(app, 0, "/w/alpha", "/w/alpha", cx);
            app.set_tab_group(0, Some(work), cx);
            app.set_tab_group(1, Some(work), cx);
            work
        });
        vcx.run_until_parked();

        app.update(&mut vcx, |app, cx| app.delete_group(work, cx));
        vcx.run_until_parked();

        app.update(&mut vcx, |app, cx| {
            assert_eq!(app.tabs.len(), 2, "no tab was closed");
            assert!(app.sidebar_groups.pinned.is_empty());
            assert!(app.tabs.iter().all(|t| t.group.get().is_none()));
            assert_eq!(app.sidebar_group_keys(cx)[0], repo("/w/alpha"));
        });
    }

    /// Pinning an auto group keeps the tabs in it now, keeps its fold, and —
    /// for a repo — keeps its folder, so tabs walking in later join too.
    #[gpui::test]
    fn pinning_an_auto_group_keeps_its_tabs_and_its_folder(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 3);

        app.update(&mut vcx, |app, cx| {
            plant_repo(app, 0, "/w/alpha", "/w/alpha", cx);
            plant_repo(app, 1, "/w/alpha/src", "/w/alpha", cx);
            plant_repo(app, 2, "/w/beta", "/w/beta", cx);
            app.toggle_sidebar_group(repo("/w/alpha").as_ref(), cx);
            cx.notify();
        });
        vcx.run_until_parked();
        app.update(&mut vcx, |app, cx| {
            app.pin_auto_group(AutoKey::Repo(PathBuf::from("/w/alpha")), cx)
        });
        vcx.run_until_parked();

        app.update(&mut vcx, |app, _| {
            let group = app.sidebar_groups.pinned.first().expect("pinned");
            assert_eq!(group.folder.as_deref(), Some("/w/alpha"));
            assert!(group.collapsed, "the fold came along");
            assert!(app.sidebar_groups.auto_collapsed.is_empty());
            assert_eq!(app.tabs[0].group.get(), Some(group.id));
            assert_eq!(app.tabs[1].group.get(), Some(group.id));
            assert_eq!(app.tabs[2].group.get(), None, "beta stays auto");
        });
    }

    /// Moving a tab into an auto group pins that group, in one edit: the
    /// group's own tabs and the moved one end up in a single folder group.
    #[gpui::test]
    fn moving_a_tab_into_an_auto_group_pins_it_with_the_tab(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 3);
        let r = AutoKey::Repo(PathBuf::from("/w/r"));
        app.update(&mut vcx, |app, cx| {
            for i in 0..2 {
                *app.tabs[i].auto_group.borrow_mut() = Some(r.clone());
            }
            *app.tabs[2].auto_group.borrow_mut() = Some(AutoKey::Repo(PathBuf::from("/w/s")));
            cx.notify();
        });
        vcx.run_until_parked();

        app.update(&mut vcx, |app, cx| {
            app.move_tab_to(2, GroupKey::Auto(r.clone()), cx)
        });
        vcx.run_until_parked();

        app.update(&mut vcx, |app, cx| {
            let [group] = app.sidebar_groups.pinned.as_slice() else {
                panic!("exactly one pinned group: {:?}", app.sidebar_groups.pinned);
            };
            assert_eq!(group.folder.as_deref(), Some("/w/r"));
            for i in 0..3 {
                assert_eq!(app.tabs[i].group.get(), Some(group.id), "tab {i}");
            }
            let keys = app.sidebar_group_keys(cx);
            assert!(
                !keys.contains(&Some(GroupKey::Auto(r.clone()))),
                "r is no longer an auto group"
            );
        });
    }

    /// A tab dragged out of a folder group while still in the repo is drawn
    /// under the repo's auto group, beside the folder group keeping the same
    /// directory. Moving a tab into that auto group joins the kept group
    /// rather than pinning the folder a second time.
    #[gpui::test]
    fn moving_into_an_auto_group_whose_folder_is_kept_joins_the_kept_group(
        cx: &mut TestAppContext,
    ) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 3);
        let r = AutoKey::Repo(PathBuf::from("/w/r"));
        let kept = PinnedGroup::folder(Path::new("/w/r"));
        let id = kept.id;
        app.update(&mut vcx, |app, cx| {
            app.sidebar_groups.pinned.push(kept);
            app.tabs[0].group.set(Some(id));
            for i in 0..2 {
                *app.tabs[i].auto_group.borrow_mut() = Some(r.clone());
            }
            *app.tabs[2].auto_group.borrow_mut() = Some(AutoKey::Repo(PathBuf::from("/w/s")));
            cx.notify();
        });
        vcx.run_until_parked();
        app.update(&mut vcx, |app, cx| {
            assert!(
                app.sidebar_group_keys(cx)
                    .contains(&Some(GroupKey::Auto(r.clone()))),
                "tab 1 is drawn under the repo's auto group"
            );
            app.move_tab_to(2, GroupKey::Auto(r.clone()), cx)
        });
        vcx.run_until_parked();

        app.update(&mut vcx, |app, _| {
            let [group] = app.sidebar_groups.pinned.as_slice() else {
                panic!("still one pinned group: {:?}", app.sidebar_groups.pinned);
            };
            assert_eq!(group.id, id);
            for i in 0..3 {
                assert_eq!(app.tabs[i].group.get(), Some(id), "tab {i}");
            }
        });
    }

    /// A pinned group with no tabs is still drawn — it is kept until it is
    /// deleted — while an auto group with none simply is not there.
    #[gpui::test]
    fn an_empty_pinned_group_still_has_a_section(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 1);

        app.update(&mut vcx, |app, cx| {
            let work = label(app, "work", cx);
            let keys = app.sidebar_group_keys(cx);
            let sections = sidebar_sections(&keys, &app.sidebar_groups);
            assert_eq!(sections[0].key, Some(GroupKey::Pinned(work)));
            assert!(sections[0].tabs.is_empty());
        });
    }

    #[gpui::test]
    fn folding_a_group_takes_its_rows_off_the_sidebar(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 3);
        let alpha = AutoKey::Repo(PathBuf::from("/w/alpha"));
        let beta = AutoKey::Repo(PathBuf::from("/w/beta"));

        app.update(&mut vcx, |app, cx| {
            for (i, root) in [(0, &alpha), (1, &alpha), (2, &beta)] {
                *app.tabs[i].auto_group.borrow_mut() = Some(root.clone());
            }
            app.active = 2;
            cx.notify();
        });
        vcx.run_until_parked();

        app.update(&mut vcx, |app, _| {
            assert!(
                (0..3).all(|i| drawn(app, i)),
                "every row is on screen before anything is folded"
            );
        });

        let alpha_key = GroupKey::Auto(alpha.clone());
        app.update(&mut vcx, |app, cx| {
            app.toggle_sidebar_group(Some(&alpha_key), cx)
        });
        vcx.run_until_parked();

        app.update(&mut vcx, |app, _| {
            assert!(
                !drawn(app, 0) && !drawn(app, 1),
                "the folded group's rows left no rectangle behind"
            );
            assert!(drawn(app, 2), "the group next to it is untouched");
            assert_eq!(
                app.sidebar_groups.auto_collapsed,
                vec![alpha.clone()],
                "the fold is kept with the workspace, where the next launch reads it"
            );
        });

        app.update(&mut vcx, |app, cx| {
            app.toggle_sidebar_group(Some(&alpha_key), cx)
        });
        vcx.run_until_parked();

        app.update(&mut vcx, |app, _| {
            assert!((0..3).all(|i| drawn(app, i)), "unfolding brings them back");
            assert!(
                app.sidebar_groups.auto_collapsed.is_empty(),
                "and takes the entry back out rather than piling up"
            );
        });
    }

    #[gpui::test]
    fn a_search_outranks_a_fold(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 2);
        let alpha = AutoKey::Repo(PathBuf::from("/w/alpha"));

        app.update(&mut vcx, |app, cx| {
            for i in 0..2 {
                *app.tabs[i].auto_group.borrow_mut() = Some(alpha.clone());
            }
            app.toggle_sidebar_group(Some(&GroupKey::Auto(alpha.clone())), cx);
        });
        vcx.run_until_parked();
        app.update(&mut vcx, |app, _| {
            assert!(!drawn(app, 1), "folded, so the row is not drawn");
        });

        // Whatever the row is actually showing — the label is derived from the
        // test process's cwd, and this has to be a query that matches it.
        app.update_in(&mut vcx, |app, window, cx| {
            let label = app.tab_label(&app.tabs[1], 1, Some(window), cx).to_string();
            app.sidebar_search.update(cx, |state, cx| {
                state.set_value(&label, window, cx);
            });
        });
        vcx.run_until_parked();

        app.update(&mut vcx, |app, _| {
            assert!(
                drawn(app, 1),
                "a row a query matches has to show, fold or no fold"
            );
        });
    }

    /// A fold hides every row the group has, the active one included. The
    /// alternative — leaving the active row on screen under a shut chevron,
    /// with the header counting rows that are not drawn — looks like a list
    /// that failed to load, which is what folding a group you are working in
    /// used to produce.
    #[gpui::test]
    fn a_fold_hides_the_active_row_too(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 2);
        let alpha = GroupKey::Auto(AutoKey::Repo(PathBuf::from("/w/alpha")));

        app.update(&mut vcx, |app, cx| {
            for i in 0..2 {
                *app.tabs[i].auto_group.borrow_mut() = alpha.auto().cloned();
            }
            app.active = 0;
            app.toggle_sidebar_group(Some(&alpha), cx);
        });
        vcx.run_until_parked();

        app.update(&mut vcx, |app, _| {
            assert!(!drawn(app, 0), "the active row folds away with the rest");
            assert!(!drawn(app, 1), "and so does everything else in the group");
        });

        app.update(&mut vcx, |app, cx| {
            app.toggle_sidebar_group(Some(&alpha), cx)
        });
        vcx.run_until_parked();

        app.update(&mut vcx, |app, _| {
            assert!((0..2).all(|i| drawn(app, i)), "unfolding brings both back");
        });
    }
    /// The divider stands between kept and derived groups, so it is there
    /// exactly when something is kept — and it is recorded, because a tab
    /// dropped below it leaves its pinned group.
    #[gpui::test]
    fn the_divider_is_drawn_once_something_is_pinned(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 2);
        app.update(&mut vcx, |_, cx| cx.notify());
        vcx.run_until_parked();
        app.update(&mut vcx, |app, _| {
            assert!(app.sidebar_divider.get().is_none(), "nothing kept, no line");
        });

        app.update(&mut vcx, |app, cx| {
            label(app, "work", cx);
        });
        vcx.run_until_parked();
        app.update(&mut vcx, |app, _| {
            assert!(app.sidebar_divider.get().is_some(), "kept, so a line");
        });
    }

    /// An empty pinned group stays on screen, and takes drops like any other:
    /// its block is recorded where a dragged tab looks for somewhere to land.
    #[gpui::test]
    fn an_empty_pinned_group_is_drawn_and_takes_drops(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 1);

        let work = app.update(&mut vcx, |app, cx| label(app, "work", cx));
        vcx.run_until_parked();

        app.update(&mut vcx, |app, _| {
            let landing = app.sidebar_group_slots.borrow();
            let block = landing
                .iter()
                .find(|(id, _)| *id == work)
                .expect("the empty group was drawn");
            assert!(block.1.size.height > px(0.));
        });
    }

    /// Pinned headers are ordered by hand; a drag lands as a new order for
    /// the list, not for the tabs.
    #[gpui::test]
    fn a_pinned_drag_reorders_the_groups_not_the_tabs(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 2);

        app.update(&mut vcx, |app, cx| {
            let (a, b, c) = (
                label(app, "a", cx),
                label(app, "b", cx),
                label(app, "c", cx),
            );
            app.set_tab_group(0, Some(a), cx);
            app.set_tab_group(1, Some(c), cx);
            let tabs: Vec<_> = app.tabs.iter().map(|t| t.tree_id.get()).collect();
            app.apply_pinned_order(&[2, 0, 1], cx);
            assert_eq!(
                app.pinned_group_names()
                    .into_iter()
                    .map(|(id, _)| id)
                    .collect::<Vec<_>>(),
                vec![c, a, b]
            );
            assert_eq!(
                app.tabs.iter().map(|t| t.tree_id.get()).collect::<Vec<_>>(),
                tabs,
                "the tabs stayed where they were"
            );
            // An order that is not a permutation of the list changes nothing.
            app.apply_pinned_order(&[0, 1], cx);
            assert_eq!(app.sidebar_groups.pinned.len(), 3);
        });
    }

    /// "New Tab" on a pinned group opens a tab in it — joining outright,
    /// without waiting for a cwd to walk in, since a label group has nothing
    /// to walk into.
    #[gpui::test]
    fn a_label_group_can_be_filled_from_its_header(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 1);

        let before = app.update(&mut vcx, |app, _| app.tabs.len());
        let work = app.update_in(&mut vcx, |app, window, cx| {
            let work = label(app, "work", cx);
            app.new_tab_in_group(GroupKey::Pinned(work), window, cx);
            work
        });
        vcx.run_until_parked();

        app.update(&mut vcx, |app, _| {
            // A harness without a daemon may refuse to spawn; when it does
            // open one, it has to be in the group.
            if app.tabs.len() > before {
                assert_eq!(app.tabs[app.active].group.get(), Some(work));
            }
        });
    }

    /// Clearing a folder keeps the name the header was showing: a folder
    /// group nobody renamed reads its folder's name, and would read nothing
    /// once the folder is gone.
    #[gpui::test]
    fn clearing_a_folder_keeps_the_name_it_was_showing(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 1);

        app.update(&mut vcx, |app, cx| {
            let tty7 = folder(app, "/w/tty7", cx);
            app.set_group_folder(tty7, None, cx);
            let group = app.sidebar_groups.get(tty7).expect("still pinned");
            assert_eq!(group.folder, None);
            assert_eq!(group.given_name(), Some("tty7"));
            app.set_group_folder(tty7, Some(PathBuf::from("/w/else")), cx);
            let group = app.sidebar_groups.get(tty7).expect("still pinned");
            assert_eq!(group.folder.as_deref(), Some("/w/else"));
            assert_eq!(group.given_name(), Some("tty7"), "a set name stays");
        });
    }
    /// A folder dropped from Finder is pinned; a file is let fall, and a
    /// folder already pinned is not pinned twice.
    #[gpui::test]
    fn a_dropped_folder_is_pinned_and_a_dropped_file_is_not(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 1);
        let dir = tempfile::tempdir().expect("tempdir");
        let file = dir.path().join("notes.txt");
        std::fs::write(&file, "x").expect("write");

        app.update(&mut vcx, |app, cx| {
            let paths = [dir.path().to_path_buf(), file.clone()];
            app.pin_dropped_folders(&paths, cx);
            app.pin_dropped_folders(&paths, cx);
            let folders: Vec<_> = app
                .sidebar_groups
                .pinned
                .iter()
                .map(|g| g.folder.clone())
                .collect();
            assert_eq!(
                folders,
                vec![Some(dir.path().to_string_lossy().into_owned())]
            );
        });
    }

    #[gpui::test]
    fn a_new_empty_group_is_ready_to_name(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 1);

        app.update_in(&mut vcx, |app, window, cx| {
            app.new_empty_group(window, cx);
            let id = app.sidebar_groups.pinned[0].id;
            assert_eq!(app.group_rename.as_ref().map(|r| r.group), Some(id));
            assert!(app.tabs.iter().all(|t| t.group.get().is_none()));
        });
    }

    /// Groups that arrive from another window, or from the machine at launch,
    /// are not folders this window's tabs walked into. A tab sitting in one —
    /// here, one that was dragged out of it — stays where it is.
    #[gpui::test]
    fn groups_arriving_from_elsewhere_pull_no_tab_in(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 1);

        app.update(&mut vcx, |app, cx| {
            plant_repo(app, 0, "/w/probed/sub", "/w/probed", cx);
            cx.notify();
        });
        vcx.run_until_parked();
        app.update(&mut vcx, |app, cx| {
            let mut groups = WorkspaceGroups::default();
            groups
                .pinned
                .push(PinnedGroup::folder(Path::new("/w/probed")));
            app.adopt_sidebar_groups(groups, cx);
        });
        vcx.run_until_parked();

        app.update(&mut vcx, |app, _| {
            assert_eq!(app.tabs[0].group.get(), None);
        });
    }

    /// A tab restored from the tree is drawn in the auto group it last had
    /// before any probe has answered — the launch no longer parks every tab
    /// in Ungrouped — and the live answer, once it lands, wins and becomes
    /// the new hint. A hint never outranks a pinned group.
    #[gpui::test]
    fn a_restored_tab_is_drawn_from_its_hint_until_a_probe_answers(cx: &mut TestAppContext) {
        let (app, mut vcx, _streams) = harness_with_tabs(cx, 2);

        app.update(&mut vcx, |app, cx| {
            for i in 0..2 {
                let pane = std::mem::replace(&mut app.tabs[i].pane, crate::ui::pane::Pane::Empty);
                let tree = tty7_core::core::machine::Tab {
                    id: app.tabs[i].tree_id.get(),
                    name: None,
                    group: None,
                    last_auto: Some(AutoKey::Repo(PathBuf::from("/w/alpha"))),
                    root: tty7_core::core::machine::PaneNode::Leaf { pane: 1 },
                    hibernated: false,
                };
                app.tabs[i] = crate::ui::app::Tab::from_tree(&tree, pane);
            }
            let work = label(app, "work", cx);
            app.tabs[1].group.set(Some(work));
            let keys = app.sidebar_group_keys(cx);
            assert_eq!(
                keys[0],
                repo("/w/alpha"),
                "drawn from the hint, no probe yet"
            );
            assert_eq!(
                keys[1],
                Some(GroupKey::Pinned(work)),
                "a pinned group outranks any hint"
            );
            plant_repo(app, 0, "/w/beta/src", "/w/beta", cx);
            cx.notify();
        });
        vcx.run_until_parked();

        app.update(&mut vcx, |app, cx| {
            assert_eq!(
                app.sidebar_group_keys(cx)[0],
                repo("/w/beta"),
                "the probe wins"
            );
            let (desired, _, _) = crate::ui::tree_sync::desired_tabs(app, cx);
            assert_eq!(
                desired[0].last_auto,
                Some(AutoKey::Repo(PathBuf::from("/w/beta"))),
                "and is what goes up as the next launch's hint"
            );
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reveal_shift_moves_only_rows_out_of_view() {
        let view = (px(100.), px(400.));
        // On screen, or cut by an edge: a click there scrolls nothing.
        assert_eq!(reveal_shift((px(200.), px(230.)), px(0.), view), px(0.));
        assert_eq!(reveal_shift((px(90.), px(120.)), px(0.), view), px(0.));
        assert_eq!(reveal_shift((px(390.), px(420.)), px(0.), view), px(0.));
        // Above: its top lands on the top edge.
        assert_eq!(reveal_shift((px(10.), px(40.)), px(0.), view), px(90.));
        // Below: its bottom lands on the bottom edge.
        assert_eq!(reveal_shift((px(500.), px(530.)), px(0.), view), px(-130.));
    }

    #[test]
    fn a_first_row_revealed_from_above_brings_its_header() {
        let view = (px(100.), px(400.));
        // The header's top, not the row's, lands on the top edge.
        assert_eq!(reveal_shift((px(10.), px(40.)), px(23.), view), px(113.));
        // A header half under the top edge, row on screen: still nothing.
        assert_eq!(reveal_shift((px(110.), px(140.)), px(23.), view), px(0.));
        // Below: the header is already above the row, only the bottom counts.
        assert_eq!(reveal_shift((px(500.), px(530.)), px(23.), view), px(-130.));
    }

    fn p(s: &str) -> PathBuf {
        PathBuf::from(s)
    }

    /// The auto group keyed on repo root `s`.
    fn g(s: &str) -> GroupKey {
        GroupKey::Auto(AutoKey::Repo(p(s)))
    }

    fn host(s: &str) -> GroupKey {
        GroupKey::Auto(AutoKey::SshHost(s.into()))
    }

    fn none() -> WorkspaceGroups {
        WorkspaceGroups::default()
    }

    /// Two machines' `/home/ubuntu` are two directories: each host gets a
    /// group of its own, named for the host.
    #[test]
    fn ssh_tabs_on_two_hosts_land_in_two_groups() {
        let keys = [
            Some(host("ubuntu@alpha")),
            Some(host("ubuntu@beta")),
            Some(host("ubuntu@alpha")),
        ];
        let sections = sidebar_sections(&keys, &none());
        let shape: Vec<(Option<String>, Vec<usize>)> =
            sections.into_iter().map(|s| (s.name, s.tabs)).collect();
        assert_eq!(
            shape,
            vec![
                (Some("ubuntu@alpha".into()), vec![0, 2]),
                (Some("ubuntu@beta".into()), vec![1]),
            ]
        );
    }

    #[test]
    fn a_shell_that_sshd_onward_groups_by_host_and_wsl_does_not() {
        use crate::daemon::protocol::{RemoteContext, RemoteKind};
        let ctx = |kind| RemoteContext {
            kind,
            argv: vec![],
            target: "u@h".into(),
        };
        assert_eq!(
            ssh_host(Some(&ctx(RemoteKind::Ssh))).as_deref(),
            Some("u@h")
        );
        assert_eq!(
            ssh_host(Some(&ctx(RemoteKind::NativeSsh))).as_deref(),
            Some("u@h")
        );
        assert_eq!(ssh_host(Some(&ctx(RemoteKind::Wsl))), None);
        assert_eq!(ssh_host(None), None);
    }

    #[test]
    fn sections_order_groups_by_first_appearance_ungrouped_last() {
        let keys = vec![
            Some(g("/w/beta")),
            None,
            Some(g("/w/alpha")),
            Some(g("/w/beta")),
        ];
        let sections = sidebar_sections(&keys, &none());
        let shape: Vec<(Option<GroupKey>, Option<String>, Vec<usize>)> = sections
            .into_iter()
            .map(|s| (s.key, s.name, s.tabs))
            .collect();
        assert_eq!(
            shape,
            vec![
                (Some(g("/w/beta")), Some("beta".into()), vec![0, 3]),
                (Some(g("/w/alpha")), Some("alpha".into()), vec![2]),
                (None, Some("Ungrouped".into()), vec![1]),
            ]
        );

        let flat = sidebar_sections(&[None, None], &none());
        assert_eq!(flat.len(), 1);
        assert_eq!(flat[0].name, None);
        assert_eq!(flat[0].tabs, vec![0, 1]);
    }

    /// Pinned groups come first, in the order the user gave them, whatever
    /// order their tabs are in — and an empty one keeps its place.
    #[test]
    fn pinned_groups_lead_in_their_own_order_empty_or_not() {
        let mut groups = none();
        let (a, b, c) = (
            PinnedGroup::label("a"),
            PinnedGroup::folder(Path::new("/w/b")),
            PinnedGroup::label("c"),
        );
        groups.pinned = vec![a.clone(), b.clone(), c.clone()];
        let keys = vec![
            Some(g("/w/r")),
            Some(GroupKey::Pinned(c.id)),
            Some(GroupKey::Pinned(a.id)),
            None,
        ];
        let sections = sidebar_sections(&keys, &groups);
        let shape: Vec<(Option<String>, Vec<usize>)> =
            sections.into_iter().map(|s| (s.name, s.tabs)).collect();
        assert_eq!(
            shape,
            vec![
                (Some("a".into()), vec![2]),
                (Some("b".into()), vec![]),
                (Some("c".into()), vec![1]),
                (Some("r".into()), vec![0]),
                (Some("Ungrouped".into()), vec![3]),
            ]
        );
    }

    /// "Move to Group" offers every group the sidebar draws — auto groups
    /// too, not just the pinned ones — in sidebar order.
    #[test]
    fn move_targets_list_every_sidebar_group_in_order() {
        let mut groups = none();
        let work = PinnedGroup::label("work");
        groups.pinned = vec![work.clone()];
        let keys = vec![
            Some(g("/w/r")),
            Some(GroupKey::Pinned(work.id)),
            Some(host("u@h")),
            None,
        ];
        let shape = |index| -> Vec<(GroupKey, String, bool)> {
            move_targets(&keys, &groups, index)
                .into_iter()
                .map(|m| (m.key, m.name, m.checked))
                .collect()
        };
        assert_eq!(
            shape(0),
            vec![
                (GroupKey::Pinned(work.id), "work".into(), false),
                (g("/w/r"), "r".into(), true),
                (host("u@h"), "u@h".into(), false),
            ]
        );
        let checked = |index| -> Vec<bool> { shape(index).into_iter().map(|m| m.2).collect() };
        assert_eq!(checked(1), vec![true, false, false]);
        assert_eq!(
            checked(3),
            vec![false, false, false],
            "an ungrouped tab is in none"
        );
        assert!(move_targets(&[None], &none(), 0).is_empty());
    }

    /// With pinned groups and nothing auto-grouped below them, the rest is
    /// just the list — a header reading "Ungrouped" over all of it would be a
    /// label on nothing.
    #[test]
    fn below_pinned_groups_the_rest_needs_no_header_on_its_own() {
        let mut groups = none();
        let work = PinnedGroup::label("work");
        groups.pinned = vec![work.clone()];
        let keys = vec![Some(GroupKey::Pinned(work.id)), None];
        let sections = sidebar_sections(&keys, &groups);
        assert_eq!(sections.len(), 2);
        assert_eq!(sections[1].name, None);
        assert_eq!(sections[1].tabs, vec![1]);
    }

    /// The badge on a row and the tab ⌘N opens are two readings of one order,
    /// taken in two places. Grouping makes them diverge from `self.tabs`
    /// order — tab 3 sits in the second row here — so if they are ever read
    /// off different things, the badge names a chord that opens another tab.
    #[test]
    fn a_row_badge_names_the_chord_that_opens_that_row() {
        let keys = vec![
            Some(g("/w/beta")),
            None,
            Some(g("/w/alpha")),
            Some(g("/w/beta")),
        ];
        // What `visual_tab_order` returns for the sidebar.
        let order: Vec<usize> = sidebar_sections(&keys, &none())
            .into_iter()
            .flat_map(|s| s.tabs)
            .collect();
        assert_eq!(order, vec![0, 3, 2, 1]);

        let mut badge_pos = vec![0usize; keys.len()];
        for (n, i) in order.iter().copied().enumerate() {
            badge_pos[i] = n;
        }
        for (row, tab) in order.iter().copied().enumerate() {
            // ActivateTabN → activate_visual(N - 1) → order[N - 1].
            let chord = tab_badge_label(badge_pos[tab]);
            let opens = order[badge_pos[tab]];
            assert_eq!(
                opens, tab,
                "row {row} badges ⌘{chord}, which opens tab {opens}"
            );
        }
    }

    #[test]
    fn reordered_rows_moves_within_the_group_only() {
        let keys = vec![
            Some(g("/w/alpha")),
            Some(g("/w/beta")),
            Some(g("/w/alpha")),
            None,
        ];
        let alpha = Some(g("/w/alpha"));
        assert_eq!(
            reordered_rows(&keys, &none(), &alpha, &[0, 2], 0, 1),
            Some(vec![2, 0, 1, 3])
        );
        assert_eq!(
            reordered_rows(&keys, &none(), &alpha, &[0, 2], 1, 0),
            Some(vec![2, 0, 1, 3])
        );
        assert_eq!(reordered_rows(&keys, &none(), &alpha, &[0, 2], 1, 1), None);
    }

    #[test]
    fn reordered_rows_leaves_filtered_out_rows_alone() {
        let keys = vec![Some(g("/w/a")), Some(g("/w/a")), Some(g("/w/a"))];
        let a = Some(g("/w/a"));
        assert_eq!(
            reordered_rows(&keys, &none(), &a, &[0, 2], 0, 1),
            Some(vec![1, 2, 0])
        );
    }

    /// A pinned group is where a tab was put, and rows move within it the
    /// same way they move within a repo group.
    #[test]
    fn rows_reorder_inside_a_pinned_group_too() {
        let mut groups = none();
        let work = PinnedGroup::label("work");
        groups.pinned = vec![work.clone()];
        let w = Some(GroupKey::Pinned(work.id));
        let keys = vec![w.clone(), Some(g("/w/beta")), w.clone()];
        assert_eq!(
            reordered_rows(&keys, &groups, &w, &[0, 2], 0, 1),
            Some(vec![2, 0, 1])
        );
    }

    #[test]
    fn regrouped_order_moves_the_group_into_the_target_slot() {
        let keys = vec![
            Some(g("/w/alpha")),
            None,
            Some(g("/w/beta")),
            Some(g("/w/alpha")),
            Some(g("/w/gamma")),
        ];
        let r = |s: &str| AutoKey::Repo(p(s));
        assert_eq!(
            regrouped_order(&keys, &r("/w/gamma"), &r("/w/alpha")),
            Some(vec![4, 0, 3, 2, 1])
        );
        assert_eq!(
            regrouped_order(&keys, &r("/w/alpha"), &r("/w/gamma")),
            Some(vec![2, 4, 0, 3, 1])
        );
    }

    #[test]
    fn regrouped_order_ignores_self_and_unknown_roots() {
        let keys = vec![Some(g("/w/alpha")), Some(g("/w/beta"))];
        let r = |s: &str| AutoKey::Repo(p(s));
        assert_eq!(regrouped_order(&keys, &r("/w/alpha"), &r("/w/alpha")), None);
        assert_eq!(regrouped_order(&keys, &r("/w/gone"), &r("/w/beta")), None);
        assert_eq!(regrouped_order(&keys, &r("/w/alpha"), &r("/w/gone")), None);
    }

    #[test]
    fn reordered_pinned_moves_a_header_among_the_ones_shown() {
        let ids: Vec<GroupId> = (0..4).map(|_| GroupId::new()).collect();
        // All four shown: dragging the first onto the third.
        assert_eq!(reordered_pinned(&ids, &ids, 0, 2), Some(vec![1, 2, 0, 3]));
        assert_eq!(reordered_pinned(&ids, &ids, 3, 0), Some(vec![3, 0, 1, 2]));
        assert_eq!(reordered_pinned(&ids, &ids, 1, 1), None);
        // A search hides the second; it keeps its place.
        let shown = [ids[0], ids[2], ids[3]];
        assert_eq!(reordered_pinned(&ids, &shown, 2, 0), Some(vec![3, 0, 1, 2]));
    }
}

#[cfg(test)]
mod tab_entry_tests {
    use super::*;
    use gpui::TestAppContext;
    use std::path::Path;
    use unicode_segmentation::UnicodeSegmentation;

    fn short_title(raw: &str) -> String {
        super::short_title(raw, None)
    }

    fn host(name: &str, user: &str, addr: &str) -> crate::core::ssh_profile::SshProfile {
        let mut p = crate::core::ssh_profile::SshProfile::new(name);
        p.user = user.to_string();
        p.host = addr.to_string();
        p
    }

    #[test]
    fn the_new_tab_menu_stops_naming_hosts_before_it_becomes_a_list() {
        // Frecency has already put the useful ones first by the time this
        // runs, so the cut can only ever drop the tail.
        let many: Vec<_> = (0..MENU_HOSTS + 4)
            .map(|i| host(&format!("box-{i}"), "dev", &format!("10.0.0.{i}")))
            .collect();
        let rows = menu_hosts(many);
        assert_eq!(rows.len(), MENU_HOSTS);
        assert_eq!(rows[0].1, "box-0", "the order handed in is the order shown");
        assert_eq!(rows[0].2, "dev@10.0.0.0");
    }

    #[test]
    fn a_host_saved_without_a_name_still_says_where_it_goes() {
        // Settings lets a host be saved on its address alone. A row that led
        // with an empty name would be a blank line you could click.
        let rows = menu_hosts(vec![host("", "root", "build.lan")]);
        assert_eq!(rows[0].1, "root@build.lan");
        // And it says it once: the endpoint has already led the row, so
        // repeating it in the note column would be a row talking to itself.
        assert_eq!(rows[0].2, "");
    }

    #[test]
    fn a_host_named_after_its_own_address_does_not_say_it_twice() {
        // Quick Connect saves a host under the target that was typed, so this
        // is the ordinary shape of a host nobody has renamed.
        let rows = menu_hosts(vec![host("deploy@10.0.0.5", "deploy", "10.0.0.5")]);
        assert_eq!(rows[0].1, "deploy@10.0.0.5");
        assert_eq!(rows[0].2, "");
    }

    #[test]
    fn only_alt_turns_a_new_tab_row_into_a_split() {
        use crate::ui::app::SpawnWhere;
        use gpui::Modifiers;
        assert_eq!(
            SpawnWhere::from_modifiers(Modifiers::none()),
            SpawnWhere::NewTab
        );
        assert_eq!(
            SpawnWhere::from_modifiers(Modifiers::alt()),
            SpawnWhere::Split
        );
        // One modifier means one thing. Every other one a hand might be
        // resting on leaves the row doing what it says it does.
        assert_eq!(
            SpawnWhere::from_modifiers(if cfg!(target_os = "macos") {
                Modifiers::command()
            } else {
                Modifiers::control()
            }),
            SpawnWhere::NewTab
        );
        assert_eq!(
            SpawnWhere::from_modifiers(Modifiers::shift()),
            SpawnWhere::NewTab
        );
    }

    #[test]
    fn every_visible_agent_state_has_words_for_it() {
        use crate::core::cli_agent::AgentStatus;
        crate::ui::i18n::set_locale("en");
        // Idle draws no dot, so it has nothing to name.
        assert_eq!(agent_status_label(None), None);
        assert_eq!(agent_status_label(Some(AgentStatus::Idle)), None);
        // Every state that does draw a dot can be read out loud.
        for status in [
            AgentStatus::Working,
            AgentStatus::Waiting,
            AgentStatus::Done,
        ] {
            assert!(status.dot_rgb().is_some());
            assert!(
                agent_status_label(Some(status)).is_some_and(|s| !s.is_empty()),
                "{status:?} paints a dot with no words behind it"
            );
        }
        // Waiting is the state worth acting on; it must not read as Done.
        assert_ne!(
            agent_status_label(Some(AgentStatus::Waiting)),
            agent_status_label(Some(AgentStatus::Done))
        );
    }

    #[test]
    fn a_brand_disc_that_matches_the_window_gets_an_edge() {
        use crate::ui::presets::needs_edge;
        let dark: gpui::Hsla = gpui::rgb(0x111111).into();
        let light: gpui::Hsla = gpui::rgb(0xffffff).into();
        let codex = crate::core::cli_agent::CLIAgent::Codex.accent_rgb();
        let claude = crate::core::cli_agent::CLIAgent::Claude.accent_rgb();

        assert_eq!(codex, 0x000000, "Codex's disc is pure black");
        assert!(
            needs_edge(codex, dark),
            "a black disc on a dark window is not a disc"
        );
        assert!(!needs_edge(codex, light));
        assert!(!needs_edge(claude, dark) && !needs_edge(claude, light));
    }

    #[test]
    fn short_title_strips_user_host_and_shows_shallow_path_in_full() {
        assert_eq!(short_title("user@host:~/projects/app"), "~/projects/app");
        // Debian's stock bash title, which spaces the path off the colon.
        assert_eq!(short_title("user@host: ~/projects/app"), "~/projects/app");
        assert_eq!(short_title("/usr/local/bin"), "/usr/local/bin");
        assert_eq!(short_title("plain"), "plain");
    }

    /// A title shortens under the home of the machine it came from, and
    /// under no other (#580).
    #[test]
    fn short_title_shortens_under_the_home_it_was_given() {
        let server = Path::new("/home/deploy");
        assert_eq!(
            super::short_title("/home/deploy/app", Some(server)),
            "~/app"
        );
        // This machine's home is not a stand-in for the server's: the same
        // path stays whole when the home naming it is somewhere else.
        assert_eq!(
            super::short_title("/home/deploy/app", Some(Path::new("/Users/thomas"))),
            "/home/deploy/app"
        );
        // And a pane nothing here can place — no link to its host, or a
        // shell that has ssh'd on — shortens against nothing.
        assert_eq!(
            super::short_title("/home/deploy/app", None),
            "/home/deploy/app"
        );
    }

    /// The name a freshly dialled SSH pane wears until the remote shell says
    /// otherwise. Cutting at the colon left the tab reading "2222" (#438).
    #[test]
    fn short_title_keeps_an_ssh_address_whole() {
        assert_eq!(short_title("deploy@10.0.0.5:2222"), "deploy@10.0.0.5:2222");
        assert_eq!(short_title("root@prod"), "root@prod");
        assert_eq!(short_title("prod-web"), "prod-web");
        // Only a port stops the cut: a drive letter is still a path, and this
        // is the title tty7's own pwsh integration writes on Windows.
        assert_eq!(short_title(r"ann@BOX:C:/Users/app"), r"C:/Users/app");
    }

    #[test]
    fn short_title_truncates_deep_paths_to_trailing_segments() {
        assert_eq!(short_title("user@host:~/repo/025/tty7"), "…/repo/025/tty7");
        assert_eq!(short_title("/usr/local/share/man"), "…/local/share/man");
        assert_eq!(short_title("a/b/c/d"), "…/b/c/d");
    }

    #[test]
    fn short_title_keeps_home_tilde_and_normalizes_trailing_slash() {
        assert_eq!(short_title("user@host:~"), "~");
        assert_eq!(short_title("~"), "~");
        assert_eq!(short_title("a/b/c/"), "a/b/c");
    }

    #[test]
    fn short_title_blank_input_is_empty_and_long_names_are_clamped() {
        assert_eq!(short_title("   "), "");
        let long = "a".repeat(50);
        let out = short_title(&long);
        assert_eq!(out.chars().count(), 41);
        assert!(out.ends_with('…'));
    }

    /// `TestAppContext` shapes through gpui's `NoopTextSystem`, where every
    /// glyph is exactly one em — weight-agnostic and, more to the point,
    /// CJK-agnostic. That keeps these tests identical on all three CI targets,
    /// but it also means they cannot speak to the proportional and mixed-script
    /// widths the elision exists for: what they pin is the contract — which
    /// parts of a label must survive, and that the result fits its budget.
    fn elide_setup(cx: &mut TestAppContext) -> (gpui::WindowTextSystem, gpui::Font, f32) {
        let size = 14.;
        (
            gpui::WindowTextSystem::new(cx.text_system().clone()),
            gpui::Font::default(),
            size,
        )
    }

    #[gpui::test]
    fn elide_path_fits_shallow_paths_untouched(cx: &mut TestAppContext) {
        let (ts, font, size) = elide_setup(cx);
        let path = "~/tty7";
        let max = measure_text(&ts, &font, size, path) + 1.;
        assert_eq!(elide_path_keep_tail(&ts, &font, size, path, max), "~/tty7");
    }

    #[gpui::test]
    fn elide_path_shows_the_whole_deep_path_when_the_budget_allows(cx: &mut TestAppContext) {
        let (ts, font, size) = elide_setup(cx);
        // A wide sidebar must not elide a deep path: only the width may
        // decide, never a fixed segment cap.
        let path = "E:/work/toolbox/crates/tty7-core/src/client";
        let max = measure_text(&ts, &font, size, path) + 1.;
        assert_eq!(elide_path_keep_tail(&ts, &font, size, path, max), path);
    }

    #[gpui::test]
    fn elide_path_keeps_drive_tail_and_budget(cx: &mut TestAppContext) {
        let (ts, font, size) = elide_setup(cx);
        let path = "E:/work/toolbox/src/ui/tab_sidebar.rs";
        let max = 200.;
        assert!(
            measure_text(&ts, &font, size, path) > max,
            "the fixture has to be wider than the budget to exercise elision"
        );
        let out = elide_path_keep_tail(&ts, &font, size, path, max);
        assert!(out.starts_with("E:/…/"), "drive letter survives: {out}");
        assert!(
            out.ends_with("tab_sidebar.rs"),
            "the file name always survives: {out}"
        );
        assert!(
            measure_text(&ts, &font, size, &out) <= max,
            "the elided label fits the budget"
        );
    }

    #[gpui::test]
    fn elide_path_keeps_tilde_and_leading_slash(cx: &mut TestAppContext) {
        let (ts, font, size) = elide_setup(cx);
        let home = "~/projects/toolbox/src/ui/tab_sidebar.rs";
        assert!(
            measure_text(&ts, &font, size, home) > 200.,
            "the fixture has to be wider than the budget to exercise elision"
        );
        let out = elide_path_keep_tail(&ts, &font, size, home, 200.);
        assert!(out.starts_with("~/…/"), "tilde root survives: {out}");
        assert!(out.ends_with("tab_sidebar.rs"));

        let abs = "/usr/local/share/man/man1/git.1";
        assert!(
            measure_text(&ts, &font, size, abs) > 120.,
            "the fixture has to be wider than the budget to exercise elision"
        );
        let out = elide_path_keep_tail(&ts, &font, size, abs, 120.);
        assert!(out.starts_with("/…/"), "absolute root survives: {out}");
        assert!(out.ends_with("git.1"));
    }

    #[gpui::test]
    fn elide_path_tears_only_the_last_segment_as_a_last_resort(cx: &mut TestAppContext) {
        let (ts, font, size) = elide_setup(cx);
        let path = "E:/supercalifragilisticexpialidocious";
        let max = 60.;
        assert!(measure_text(&ts, &font, size, path) > max);
        let out = elide_path_keep_tail(&ts, &font, size, path, max);
        assert!(out.starts_with('…'), "a torn segment reads as torn: {out}");
        assert!(
            out.chars().nth(1) != Some('/'),
            "no slash after a torn segment: {out}"
        );
        assert!(out.ends_with('s'), "the word's tail survives: {out}");
        assert!(measure_text(&ts, &font, size, &out) <= max);
    }

    #[gpui::test]
    fn elide_edges_keeps_both_ends_of_a_branch(cx: &mut TestAppContext) {
        let (ts, font, size) = elide_setup(cx);
        let branch = "window-transparency-backdrop";
        let max = 140.;
        assert!(measure_text(&ts, &font, size, branch) > max);
        let out = elide_keep_edges(&ts, &font, size, branch, max);
        assert!(out.starts_with("window-"), "head survives: {out}");
        assert!(out.ends_with("backdrop"), "tail survives: {out}");
        assert!(out.contains('…'));
        assert!(measure_text(&ts, &font, size, &out) <= max);
        assert!(out.chars().count() < branch.chars().count());
    }

    #[gpui::test]
    fn elide_edges_leaves_short_branches_alone(cx: &mut TestAppContext) {
        let (ts, font, size) = elide_setup(cx);
        let branch = "main";
        let max = measure_text(&ts, &font, size, branch) + 1.;
        assert_eq!(elide_keep_edges(&ts, &font, size, branch, max), "main");
    }

    #[gpui::test]
    fn elide_edges_falls_back_to_a_tail_sliver_when_the_head_cannot_fit(cx: &mut TestAppContext) {
        let (ts, font, size) = elide_setup(cx);
        let branch = "window-transparency-backdrop";
        let out = elide_keep_edges(&ts, &font, size, branch, 30.);
        assert!(out.starts_with('…'));
        assert!(measure_text(&ts, &font, size, &out) <= 30.);
    }

    #[gpui::test]
    fn elide_path_cuts_windows_backslash_paths_on_segments(cx: &mut TestAppContext) {
        let (ts, font, size) = elide_setup(cx);
        let path = r"C:\Users\dev\AppData\Local\Temp\verify-build";
        let max = 200.;
        assert!(measure_text(&ts, &font, size, path) > max);
        let out = elide_path_keep_tail(&ts, &font, size, path, max);
        assert!(out.starts_with(r"C:\…\"), "drive letter survives: {out}");
        assert!(
            out.ends_with("verify-build"),
            "the leaf segment survives: {out}"
        );
        assert!(measure_text(&ts, &font, size, &out) <= max);
    }

    /// One tab must not spell its location two ways depending on how wide the
    /// sidebar happens to be.
    #[gpui::test]
    fn elide_path_keeps_the_separator_the_path_arrived_with(cx: &mut TestAppContext) {
        let (ts, font, size) = elide_setup(cx);
        let windows = r"C:\Users\dev\projects\toolbox\src\ui\app.rs";
        let wide = measure_text(&ts, &font, size, windows) + 1.;
        assert_eq!(
            elide_path_keep_tail(&ts, &font, size, windows, wide),
            windows,
            "a path that fits is left exactly as it arrived"
        );
        let out = elide_path_keep_tail(&ts, &font, size, windows, 120.);
        assert!(!out.contains('/'), "no forward slash creeps in: {out}");

        let unix = "/home/dev/projects/toolbox/src/ui/app.rs";
        let out = elide_path_keep_tail(&ts, &font, size, unix, 120.);
        assert!(!out.contains('\\'), "no backslash creeps in: {out}");
    }

    /// A branch with no `-`, `_`, `/` or `.` in reach used to lose its head
    /// entirely, which is the one thing this function promises not to do.
    #[gpui::test]
    fn elide_edges_keeps_a_head_on_a_separatorless_token(cx: &mut TestAppContext) {
        let (ts, font, size) = elide_setup(cx);
        let branch = "verylongbranchnamewithoutseps";
        for max in [60., 80., 100., 120.] {
            let out = elide_keep_edges(&ts, &font, size, branch, max);
            assert!(
                out.starts_with('v'),
                "head survives at {max}px: {out}",
                max = max
            );
            assert!(out.ends_with('s'), "tail survives at {max}px: {out}");
            assert!(measure_text(&ts, &font, size, &out) <= max);
        }
    }

    /// A head that fits but leaves nothing behind the ellipsis says less than
    /// a shorter head that keeps the identifying tail.
    #[gpui::test]
    fn elide_edges_gives_up_head_room_to_keep_a_tail(cx: &mut TestAppContext) {
        let (ts, font, size) = elide_setup(cx);
        let branch = "feature/some-really-long-thing";
        let max = 80.;
        assert!(measure_text(&ts, &font, size, branch) > max);
        let out = elide_keep_edges(&ts, &font, size, branch, max);
        assert!(
            !out.ends_with('…'),
            "the tail is never traded away for a longer head: {out}"
        );
        assert!(out.ends_with('g'), "the identifying tail survives: {out}");
        assert!(measure_text(&ts, &font, size, &out) <= max);
    }

    /// The sidebar title is not always a path. `elide_label` has to notice,
    /// because the path rule drops the head — and for a command line or a name
    /// the user typed, the head is the part that names it.
    #[gpui::test]
    fn elide_label_keeps_the_head_of_a_non_path(cx: &mut TestAppContext) {
        let (ts, font, size) = elide_setup(cx);
        for text in ["Backend server logs", "npm run dev -- --watch"] {
            let max = 90.;
            assert!(measure_text(&ts, &font, size, text) > max);
            let out = elide_label(&ts, &font, size, text, max);
            let first = text.chars().next().unwrap();
            assert!(
                out.starts_with(first),
                "a non-path keeps its head: {out} (from {text})"
            );
            assert!(measure_text(&ts, &font, size, &out) <= max);
        }
    }

    /// …while a path still gets the tail-first treatment through the same
    /// entry point.
    #[gpui::test]
    fn elide_label_still_keeps_the_tail_of_a_path(cx: &mut TestAppContext) {
        let (ts, font, size) = elide_setup(cx);
        let path = "~/projects/toolbox/src/ui/tab_sidebar.rs";
        let out = elide_label(&ts, &font, size, path, 200.);
        assert!(out.starts_with("~/…/"), "root survives: {out}");
        assert!(out.ends_with("tab_sidebar.rs"), "leaf survives: {out}");
    }

    #[test]
    fn short_title_cuts_windows_paths_on_backslashes() {
        assert_eq!(
            short_title(r"C:\Users\dev\projects\app"),
            r"…\dev\projects\app"
        );
        assert_eq!(
            short_title(r"C:\Users\dev\repo\deep\path\src\ui"),
            r"…\path\src\ui"
        );
        // A shallow Windows path keeps its drive and its backslashes.
        assert_eq!(short_title(r"C:\Users\app"), r"C:\Users\app");
    }

    /// Every way of slicing `text` that lands on a grapheme-cluster boundary.
    fn cluster_prefixes(text: &str) -> Vec<String> {
        let clusters: Vec<&str> = text.graphemes(true).collect();
        (0..=clusters.len())
            .map(|n| clusters[..n].concat())
            .collect()
    }

    fn cluster_suffixes(text: &str) -> Vec<String> {
        let clusters: Vec<&str> = text.graphemes(true).collect();
        (0..=clusters.len())
            .map(|n| clusters[clusters.len() - n..].concat())
            .collect()
    }

    /// An elision may only drop whole grapheme clusters, so whatever survives
    /// on either side of the ellipsis has to be a cluster-aligned prefix and
    /// suffix of what went in. Slicing by `char` instead passes every width
    /// check and still tears `👨‍👩‍👧` into a dangling joiner, strips the
    /// variation selector off `❤️`, or leaves half of `🇨🇳` to render as a
    /// bare letter.
    #[track_caller]
    fn assert_cut_on_cluster_boundaries(input: &str, out: &str, max: f32) {
        let Some((head, tail)) = out.split_once('…') else {
            assert_eq!(out, input, "an unelided label comes back verbatim");
            return;
        };
        assert!(
            cluster_prefixes(input).iter().any(|p| p == head),
            "head {head:?} is not a cluster-aligned prefix of {input:?} (@{max}px)"
        );
        assert!(
            cluster_suffixes(input).iter().any(|s| s == tail),
            "tail {tail:?} is not a cluster-aligned suffix of {input:?} (@{max}px)"
        );
    }

    /// Fixtures whose clusters are wider than one `char`, placed so that a
    /// `char`-indexed cut lands inside one at some width.
    const CLUSTER_FIXTURES: [&str; 7] = [
        "ab\u{1F468}\u{200d}\u{1F469}\u{200d}\u{1F467}cdefghijklmnop",
        "release-notes-final-ab\u{1F468}\u{200d}\u{1F469}\u{200d}\u{1F467}",
        "abcdef\u{2764}\u{fe0f}ghijklmnopqr",
        "long-branch-name-x\u{2764}\u{fe0f}",
        "abcdef\u{1F1E8}\u{1F1F3}ghijklmnopqr",
        "abcde\u{301}fghijklmnopqrst",
        "review-\u{1F44D}\u{1F3FD}-approved-changes",
    ];

    #[gpui::test]
    fn elide_edges_cuts_only_on_cluster_boundaries(cx: &mut TestAppContext) {
        let (ts, font, size) = elide_setup(cx);
        for text in CLUSTER_FIXTURES {
            let mut max = 30.;
            while max <= 200. {
                let out = elide_keep_edges(&ts, &font, size, text, max);
                assert_cut_on_cluster_boundaries(text, &out, max);
                assert!(
                    measure_text(&ts, &font, size, &out) <= max,
                    "{out:?} still has to fit its budget (@{max}px)"
                );
                max += 2.;
            }
        }
    }

    /// The path elision tears its last segment character by character as a
    /// last resort; that resort has to respect clusters too.
    #[gpui::test]
    fn elide_path_tears_its_last_segment_on_cluster_boundaries(cx: &mut TestAppContext) {
        let (ts, font, size) = elide_setup(cx);
        for leaf in CLUSTER_FIXTURES {
            let path = format!("~/projects/toolbox/{leaf}");
            let mut max = 30.;
            while max <= 120. {
                let out = elide_path_keep_tail(&ts, &font, size, &path, max);
                // Whatever it settled on, the tail after the ellipsis has to
                // be cluster-aligned against the path it came from.
                if let Some((_, tail)) = out.split_once('…') {
                    let tail = tail.trim_start_matches('/');
                    assert!(
                        cluster_suffixes(&path).iter().any(|s| s == tail),
                        "tail {tail:?} is not a cluster-aligned suffix of {path:?} (@{max}px)"
                    );
                }
                max += 2.;
            }
        }
    }

    /// `short_title`'s 40-glyph clamp is the other `char`-indexed cut.
    #[test]
    fn short_title_clamps_on_cluster_boundaries() {
        for tail in ["\u{1F1E8}\u{1F1F3}-suffix", "\u{2764}\u{fe0f}-suffix"] {
            for pad in 37..=41 {
                let name = format!("{}{tail}", "a".repeat(pad));
                let out = short_title(&name);
                let Some(body) = out.strip_suffix('…') else {
                    continue;
                };
                assert!(
                    cluster_prefixes(&name).iter().any(|p| p == body),
                    "clamped to {body:?}, not a cluster-aligned prefix of {name:?}"
                );
            }
        }
    }

    #[test]
    fn configured_shell_arguments_remain_user_authored_in_the_menu() {
        let shell = DetectedShell {
            label: "custom".into(),
            program: "custom-shell".into(),
            args: vec!["--login".into()],
            args_are_tty7_defaults: false,
            user_authored: false,
        };
        let spec = shell_spec(&shell);

        assert_eq!(spec.program, "custom-shell");
        assert_eq!(spec.args, ["--login"]);
        assert!(!spec.args_are_tty7_defaults);
    }

    /// A tab of this window as the sidebar reads it: `tab_label` is nothing but
    /// [`label_of`] over the [`TabView`](crate::ui::machine_mirror::TabView)
    /// that [`Tab::label_view`](crate::ui::app::Tab::label_view) builds from
    /// the live leaf, so naming one here climbs the same ladder a real tab
    /// climbs. `title` is the placeholder `label_view` fills that slot with —
    /// the machine tree puts a process name there, a live pane has only the
    /// name it answers to before anything has spoken.
    fn tab_view() -> crate::ui::machine_mirror::TabView {
        crate::ui::machine_mirror::TabView {
            id: tty7_core::core::machine::TabId::new(),
            name: None,
            title: crate::terminal::view::DEFAULT_TITLE.to_string(),
            osc_title: None,
            cwd: None,
            agent: None,
            status: None,
            live: true,
            panes: 1,
        }
    }

    /// The home the paths below are measured against — named rather than read
    /// off this machine, so the assertions do not depend on who is running
    /// them (#580).
    fn home() -> &'static Path {
        Path::new("/Users/x")
    }

    #[test]
    fn a_renamed_tab_keeps_its_name_over_every_other_answer() {
        let mut tab = tab_view();
        tab.name = Some("  build  ".into());
        tab.osc_title = Some("vim — main.rs".into());
        tab.cwd = Some("/Users/x/repo/tty7".into());

        assert_eq!(label_of(&tab, 0, Some(home())), "build");
    }

    #[test]
    fn a_pane_showing_a_title_is_named_by_it_and_not_by_its_directory() {
        let mut tab = tab_view();
        tab.osc_title = Some("vim — main.rs".into());
        tab.cwd = Some("/Users/x/repo/tty7".into());

        assert_eq!(label_of(&tab, 0, Some(home())), "vim — main.rs");

        // Including the title an SSH pane answers to before the far shell has
        // said anything (#438): `label_view` hands that up here, so a window
        // full of them still reads as hosts rather than as directories.
        tab.osc_title = Some("prod-web".into());
        assert_eq!(label_of(&tab, 0, Some(home())), "prod-web");
    }

    /// #740: every shell tty7 ships integration for except PowerShell reports
    /// its directory over OSC 7 and never sets a title, which left the tab
    /// reading "tty7" — the app's own name — while the switcher listing the
    /// very same tab showed the directory.
    #[test]
    fn a_pane_that_has_only_said_where_it_is_is_named_after_that() {
        let mut tab = tab_view();
        tab.cwd = Some("/Users/x/repo/tty7".into());

        assert_eq!(label_of(&tab, 0, Some(home())), "~/repo/tty7");
        // Through the same shortener as a title, so a deep directory is cut
        // where a deep path in a title would be.
        tab.cwd = Some("/Users/x/repo/tty7/crates/tty7-core/src".into());
        assert_eq!(
            label_of(&tab, 0, Some(home())),
            super::short_title("/Users/x/repo/tty7/crates/tty7-core/src", Some(home())),
        );
    }

    /// The one test that fails if any of the wiring is put back: a real tab,
    /// built the way the window builds one, named through `tab_label` — and
    /// checked against what the switcher renders from the machine tree's view
    /// of that very same pane.
    #[gpui::test]
    fn a_titleless_tab_is_named_like_its_switcher_entry(cx: &mut TestAppContext) {
        use crate::ui::pane::{Pane, PaneSlot};

        let (app, mut vcx) = crate::ui::app::test_window::harness(cx);
        let _stream = app.update_in(&mut vcx, |app, window, cx| {
            let (view, stream) = crate::terminal::view::quiet_test_pane(1, window, cx);
            // A pane that has reported where it is over OSC 7 and has never
            // titled itself — every shell tty7 ships integration for except
            // PowerShell.
            view.read(cx)
                .terminal
                .seed_cwd(Some(std::path::PathBuf::from("/work/repo")));
            app.tabs
                .push(crate::ui::app::Tab::new(Pane::leaf(PaneSlot::Ready(view))));
            app.active = app.tabs.len() - 1;
            stream
        });
        vcx.background_executor.run_until_parked();

        app.update_in(&mut vcx, |app, window, cx| {
            let index = app.active;
            let tab = &app.tabs[index];
            let (view, home) = tab.label_view(Some(window), cx);
            assert_eq!(view.osc_title, None, "the pane never titled itself");
            assert_eq!(view.cwd.as_deref(), Some("/work/repo"));

            let label = app.tab_label(tab, index, Some(window), cx);
            assert_eq!(label, "/work/repo");
            assert_ne!(
                label,
                crate::terminal::view::DEFAULT_TITLE,
                "and is not named after the app any more"
            );

            // The machine tree's reading of the same pane, which is all the
            // switcher ever has: no title was seen, the cwd is the one above,
            // and `title` is the foreground process name.
            let from_tree = crate::ui::machine_mirror::TabView {
                id: tab.tree_id.get(),
                name: None,
                title: "zsh".into(),
                osc_title: None,
                cwd: Some("/work/repo".into()),
                agent: None,
                status: None,
                live: true,
                panes: 1,
            };
            assert_eq!(
                label,
                label_of(&from_tree, index, home.as_deref()),
                "the two columns name the same tab the same way"
            );
        });
    }

    #[test]
    fn a_pane_with_nothing_to_say_falls_back_the_way_it_always_did() {
        // No title and no directory: the placeholder, exactly as before.
        let tab = tab_view();
        assert_eq!(label_of(&tab, 0, Some(home())), "tty7");

        // And a tab holding no live pane at all is still numbered.
        let mut empty = tab_view();
        empty.title = String::new();
        assert!(label_of(&empty, 2, Some(home())).contains('3'));
    }

    /// The rung under the shortener, which the two surfaces reach holding
    /// different things. A shell that has said who and where it is but not
    /// *where* — `user@host:` with nothing after the colon — leaves nothing to
    /// show, and whatever stands in has to be something the tab does not
    /// already say: the switcher has the foreground process name, and a tab of
    /// this window has only the placeholder, which is the answer #740 removed.
    #[test]
    fn a_title_that_shortens_away_never_puts_the_app_name_back_on_the_tab() {
        let mut view = tab_view();
        view.osc_title = Some("user@host:".into());
        assert_ne!(
            label_of(&view, 0, Some(home())),
            crate::terminal::view::DEFAULT_TITLE
        );
        assert!(
            label_of(&view, 0, Some(home())).contains('1'),
            "an unnamed tab uses the numbered placeholder"
        );

        // The switcher arrives with a real process name in that slot, and it
        // is still worth more than a number.
        let from_tree = crate::ui::machine_mirror::TabView {
            title: "zsh".into(),
            osc_title: Some("user@host:".into()),
            ..tab_view()
        };
        assert_eq!(label_of(&from_tree, 0, Some(home())), "zsh");
    }

    /// A path is spelled the way the machine it is on spells it, and which
    /// machine that is has nothing to do with which one tty7 is running on: a
    /// remote pane reports POSIX to a Windows client, and a Windows pane
    /// reports backslashes to a client that has never seen one (#580).
    #[test]
    fn a_cwd_is_cut_in_its_own_spelling_whichever_client_is_reading_it() {
        let windows_home = Path::new(r"C:\Users\x");

        // A Windows pane: shortened under its own home, and a path too deep to
        // fit is rejoined with its own separator rather than with `/`.
        let mut win = tab_view();
        win.cwd = Some(r"C:\Users\x\repo".into());
        assert_eq!(label_of(&win, 0, Some(windows_home)), "~/repo");
        win.cwd = Some(r"D:\work\a\b\proj".into());
        assert_eq!(label_of(&win, 0, Some(windows_home)), r"…\a\b\proj");

        // A remote pane's cwd is POSIX even when the client reading it is the
        // Windows one: no drive to hang it off, no `~` borrowed from this
        // machine's home, and no backslash anywhere in the answer.
        let mut remote = tab_view();
        remote.cwd = Some("/srv/app".into());
        assert_eq!(label_of(&remote, 0, Some(windows_home)), "/srv/app");
        remote.cwd = Some("/home/deploy/app".into());
        assert_eq!(
            label_of(&remote, 0, Some(Path::new("/home/deploy"))),
            "~/app",
            "measured against the home of the host it is on, not of this one"
        );

        // The root of a filesystem is a directory like any other: a tab
        // sitting in it says so, and says nothing more on hover.
        let mut root = tab_view();
        root.cwd = Some("/".into());
        assert_eq!(label_of(&root, 0, Some(home())), "/");
    }
}

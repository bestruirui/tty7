use gpui::{Bounds, Pixels, Point, Styled, px};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use tty7_core::core::group_key::{AutoKey, GroupId, GroupKey};
use tty7_core::core::machine::TabId;

pub(crate) type ReorderState = Rc<RefCell<Option<Reorder>>>;

/// The cursor for a grip you pick up and drag. Win32 ships no open-hand
/// cursor, and gpui's Windows backend quietly answers `OpenHand` with the
/// plain arrow — so a grip there would look like ordinary background. The
/// pointing hand is the closest thing Windows has that still reads as
/// "this responds to the mouse".
pub(crate) fn cursor_grab<E: Styled>(el: E) -> E {
    if cfg!(target_os = "windows") {
        el.cursor_pointer()
    } else {
        el.cursor_grab()
    }
}

pub(crate) struct Preview {
    pub(crate) order: Vec<usize>,
    pub(crate) target: usize,
    pub(crate) from: usize,
    pub(crate) generation: usize,
    pub(crate) offsets: Vec<Pixels>,
    pub(crate) held: Pixels,
}

pub(crate) fn preview(
    state: &ReorderState,
    surface: &Surface,
    len: usize,
    pointer: Point<Pixels>,
) -> Option<Preview> {
    let state = state.borrow();
    let r = state
        .as_ref()
        .filter(|r| !r.suspended.get())
        .filter(|r| r.covers(surface, len))?;
    let target = r.target(pointer);
    let (generation, prev) = r.begin_frame(target);
    Some(Preview {
        order: r.order(target),
        target,
        from: r.from,
        generation,
        offsets: (0..len)
            .map(|slot| r.flip_offset(slot, prev, target))
            .collect(),
        held: r.held_offset(pointer, target),
    })
}

pub(crate) fn set_pending(state: &ReorderState, surface: &Surface, order: Vec<usize>) {
    if let Some(r) = state.borrow().as_ref().filter(|r| r.surface == *surface) {
        *r.pending.borrow_mut() = Some(order);
    }
}

pub(crate) fn clear_pending(state: &ReorderState) {
    if let Some(r) = state.borrow().as_ref() {
        r.pending.borrow_mut().take();
        r.regroup.borrow_mut().take();
        r.pin.set(false);
    }
}

/// Where a tab being dragged in the sidebar would go if it were let go now,
/// when that is somewhere other than a new place in its own group.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Regroup {
    /// Into this pinned group.
    Into(GroupId),
    /// Below the divider: out of its pinned group, back to auto grouping.
    ToAuto,
}

/// Offer the group the pointer is currently over.
///
/// Reordering answers "where in this group", and this answers "which group" —
/// two questions one drag can ask, so they are recorded side by side and
/// resolved together when it lands. A drag held over a group it did not come
/// from stops reordering (the surface it belongs to no longer sees the
/// pointer) and starts offering this instead.
pub(crate) fn set_regroup(state: &ReorderState, target: Regroup) {
    if let Some(r) = state.borrow().as_ref().filter(|r| !r.suspended.get()) {
        *r.regroup.borrow_mut() = Some(target);
    }
}

/// Offer to pin the auto group being dragged — its header is above the
/// divider. Cleared with everything else each frame, so a header carried back
/// down before it is let go reorders instead.
pub(crate) fn set_pin(state: &ReorderState) {
    if let Some(r) = state.borrow().as_ref().filter(|r| !r.suspended.get()) {
        r.pin.set(r.auto.is_some());
    }
}

/// What a finished drag asks for.
pub(crate) struct Landed {
    /// A new order, from the surface the drag ran over — of the tabs, or for
    /// [`Surface::PinnedGroups`] of the pinned groups.
    pub(crate) order: Option<Vec<usize>>,
    /// The surface `order` is an order of.
    pub(crate) surface: Option<Surface>,
    /// A tab to put in another group, from the group it was held over.
    pub(crate) regroup: Option<(TabId, Regroup)>,
    /// An auto group to pin, from its header being let go above the divider.
    pub(crate) pin: Option<AutoKey>,
}

/// Ends the drag and answers what it was asking for when it ended.
pub(crate) fn take_landed(state: &ReorderState) -> Landed {
    let Some(r) = state.borrow_mut().take() else {
        return Landed {
            order: None,
            surface: None,
            regroup: None,
            pin: None,
        };
    };
    let regroup = r.regroup.into_inner();
    let pin = r.pin.get().then_some(r.auto).flatten();
    Landed {
        order: r.pending.into_inner(),
        surface: Some(r.surface),
        regroup: r.tab.zip(regroup),
        pin,
    }
}

/// The tab a drag in flight picked up, when it picked one up.
///
/// A tab is dragged to reorder it, and — once the pointer leaves the sidebar for
/// the layout — to merge it into the tab on screen. Both are the same drag, so
/// the reorder is where the answer to "which tab is in the air" lives.
pub(crate) fn dragged_tab(state: &ReorderState) -> Option<TabId> {
    state.borrow().as_ref()?.tab
}

/// The auto group a *sidebar header* drag picked up, if it is one.
pub(crate) fn dragged_auto_group(state: &ReorderState) -> Option<AutoKey> {
    let state = state.borrow();
    state.as_ref().filter(|r| !r.suspended.get())?.auto.clone()
}

/// The tab a *sidebar row* drag picked up.
///
/// `None` for a group header or a suspended drag.
pub(crate) fn dragged_sidebar_tab(state: &ReorderState) -> Option<TabId> {
    let state = state.borrow();
    let r = state.as_ref().filter(|r| !r.suspended.get())?;
    r.tab
}

/// Holds the reorder off while the drag is asking for something else.
///
/// A suspended reorder offers no preview and records nothing pending, so the
/// rows sit still while the pointer is out over the layout and a drop there
/// cannot also shuffle the sidebar.
pub(crate) fn suspend(state: &ReorderState, yes: bool) {
    if let Some(r) = state.borrow().as_ref() {
        r.suspended.set(yes);
        if yes {
            r.pending.borrow_mut().take();
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) enum Surface {
    SidebarRows(Option<GroupKey>),
    /// The auto groups' headers, below the divider.
    SidebarGroups,
    /// The pinned groups' headers, above it. A surface of its own so a
    /// pinned header can only be dropped among the pinned ones: the two
    /// halves of the sidebar are ordered by different things — the user's
    /// hand above, first appearance below.
    PinnedGroups,
}

pub(crate) struct Reorder {
    pub(crate) surface: Surface,
    pub(crate) from: usize,
    rects: Vec<Bounds<Pixels>>,
    gap: Pixels,
    grab: Point<Pixels>,
    prev: Cell<usize>,
    generation: Cell<usize>,
    pending: RefCell<Option<Vec<usize>>>,
    /// The group this drag is being held over, when the pointer has left the
    /// group the tab came from.
    regroup: RefCell<Option<Regroup>>,
    /// The auto group whose header this drag picked up, for the one gesture
    /// that pins: carrying it above the divider.
    auto: Option<AutoKey>,
    /// Set on the frames the carried auto header is above the divider.
    pin: Cell<bool>,
    /// The tab this drag picked up, for the surfaces that drag tabs. `None` on
    /// a surface that drags something else — a sidebar group, say, which is
    /// several tabs and cannot be merged into one.
    tab: Option<TabId>,
    suspended: Cell<bool>,
}

impl Reorder {
    pub(crate) fn new(
        surface: Surface,
        from: usize,
        rects: Vec<Bounds<Pixels>>,
        gap: Pixels,
        grab: Point<Pixels>,
    ) -> Self {
        Self {
            surface,
            from,
            rects,
            gap,
            grab,
            prev: Cell::new(from),
            generation: Cell::new(0),
            pending: RefCell::new(None),
            regroup: RefCell::new(None),
            auto: None,
            pin: Cell::new(false),
            tab: None,
            suspended: Cell::new(false),
        }
    }

    /// Names the tab this drag is carrying.
    pub(crate) fn of_tab(mut self, tab: TabId) -> Self {
        self.tab = Some(tab);
        self
    }

    /// Names the auto group whose header this drag is carrying.
    pub(crate) fn of_auto(mut self, key: AutoKey) -> Self {
        self.auto = Some(key);
        self
    }

    pub(crate) fn covers(&self, surface: &Surface, len: usize) -> bool {
        self.surface == *surface && self.rects.len() == len && self.from < len
    }

    fn shift(&self) -> Pixels {
        self.rects[self.from].size.height + self.gap
    }

    fn measured_slots(&self) -> impl Iterator<Item = &Bounds<Pixels>> {
        self.rects.iter().filter(|r| r.size.height > px(0.))
    }

    pub(crate) fn target(&self, pointer: Point<Pixels>) -> usize {
        let leading = self.free_origin(pointer);
        let trailing = leading + self.rects[self.from].size.height;
        self.rects
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != self.from)
            .filter(|(i, r)| {
                // Keep unmeasured rows in place until their bounds arrive.
                if r.size.height <= px(0.) {
                    return *i < self.from;
                }
                let centre = r.origin.y + r.size.height / 2.;
                if *i < self.from {
                    leading >= centre
                } else {
                    trailing > centre
                }
            })
            .count()
    }

    fn free_origin(&self, pointer: Point<Pixels>) -> Pixels {
        pointer.y - self.grab.y
    }

    fn held_origin(&self, pointer: Point<Pixels>) -> Pixels {
        let free = self.free_origin(pointer);
        // Nothing drawn yet: there is no span to hold the row inside, so let
        // it follow the pointer rather than clamping against a made-up one.
        let (Some(head), Some(tail)) = (self.measured_slots().next(), self.measured_slots().last())
        else {
            return free;
        };
        let first = head.origin.y;
        let end = tail.origin.y + tail.size.height;
        let last_start = (end - self.rects[self.from].size.height).max(first);
        free.clamp(first, last_start)
    }

    pub(crate) fn held_offset(&self, pointer: Point<Pixels>, target: usize) -> Pixels {
        let home = self.rects[self.from].origin.y;
        self.held_origin(pointer) - (home + self.displacement(self.from, target))
    }

    pub(crate) fn order(&self, target: usize) -> Vec<usize> {
        let mut order: Vec<usize> = (0..self.rects.len()).collect();
        let dragged = order.remove(self.from);
        order.insert(target.min(order.len()), dragged);
        order
    }

    pub(crate) fn begin_frame(&self, target: usize) -> (usize, usize) {
        let prev = self.prev.get();
        if prev != target {
            self.generation.set(self.generation.get() + 1);
            self.prev.set(target);
        }
        (self.generation.get(), prev)
    }

    fn displacement(&self, slot: usize, target: usize) -> Pixels {
        if slot == self.from {
            let crossed = if target > self.from {
                self.from + 1..=target
            } else {
                target..=self.from.saturating_sub(1)
            };
            let span: Pixels = crossed
                .filter(|&i| i != self.from && i < self.rects.len())
                .filter(|&i| self.rects[i].size.height > px(0.))
                .map(|i| self.rects[i].size.height + self.gap)
                .fold(px(0.), |a, b| a + b);
            if target > self.from { span } else { -span }
        } else if self.from < slot && slot <= target {
            -self.shift()
        } else if target <= slot && slot < self.from {
            self.shift()
        } else {
            px(0.)
        }
    }

    pub(crate) fn flip_offset(&self, slot: usize, prev: usize, target: usize) -> Pixels {
        self.displacement(slot, prev) - self.displacement(slot, target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{point, size};

    fn column(n: usize, h: f32, gap: f32, from: usize) -> Reorder {
        let rects = (0..n)
            .map(|i| Bounds {
                origin: point(px(0.), px(i as f32 * (h + gap))),
                size: size(px(200.), px(h)),
            })
            .collect();
        Reorder::new(
            Surface::SidebarRows(None),
            from,
            rects,
            px(gap),
            point(px(100.), px(h / 2.)),
        )
    }

    #[test]
    fn target_follows_the_pointer_across_neighbours() {
        let r = column(4, 30., 2., 0);
        assert_eq!(r.target(point(px(100.), px(32.))), 0);
        assert_eq!(r.target(point(px(100.), px(34.))), 1);
        assert_eq!(r.target(point(px(100.), px(66.))), 2);
        assert_eq!(r.target(point(px(100.), px(200.))), 3);
    }

    #[test]
    fn order_lifts_the_dragged_slot_into_the_target() {
        let r = column(4, 30., 2., 3);
        assert_eq!(r.target(point(px(100.), px(2.))), 0);
        assert_eq!(r.order(0), vec![3, 0, 1, 2]);
        assert_eq!(r.order(1), vec![0, 3, 1, 2]);
        assert_eq!(r.order(3), vec![0, 1, 2, 3]);
    }

    #[test]
    fn flip_offset_animates_only_the_slot_just_crossed() {
        let r = column(4, 30., 2., 0);
        assert_eq!(r.flip_offset(1, 0, 1), px(32.));
        assert_eq!(r.flip_offset(2, 0, 1), px(0.));
        assert_eq!(r.flip_offset(3, 0, 1), px(0.));
        assert_eq!(r.flip_offset(0, 0, 3), px(-96.));
        assert_eq!(r.flip_offset(0, 3, 0), px(96.));
        assert_eq!(r.flip_offset(1, 1, 0), px(-32.));
    }

    #[test]
    fn unequal_sizes_swap_in_both_directions() {
        let rects = vec![
            Bounds {
                origin: point(px(0.), px(0.)),
                size: size(px(200.), px(60.)),
            },
            Bounds {
                origin: point(px(0.), px(62.)),
                size: size(px(200.), px(140.)),
            },
        ];
        let grab = point(px(100.), px(10.));
        let tall = Reorder::new(Surface::SidebarGroups, 1, rects.clone(), px(2.), grab);
        assert_eq!(tall.target(point(px(100.), px(41.))), 1);
        assert_eq!(tall.target(point(px(100.), px(39.))), 0);
        assert_eq!(tall.target(point(px(100.), px(72.))), 1);

        let short = Reorder::new(Surface::SidebarGroups, 0, rects, px(2.), grab);
        assert_eq!(short.target(point(px(100.), px(80.))), 0);
        assert_eq!(short.target(point(px(100.), px(84.))), 1);
    }

    #[test]
    fn held_offset_tracks_the_pointer_across_a_crossing() {
        let r = column(4, 30., 2., 0);
        assert_eq!(r.held_offset(point(px(100.), px(25.)), 0), px(10.));
        assert_eq!(r.held_offset(point(px(100.), px(48.)), 1), px(1.));
        assert_eq!(r.held_offset(point(px(100.), px(900.)), 3), px(0.));
    }

    #[test]
    fn pending_only_survives_the_frame_that_recorded_it() {
        let state: ReorderState = Rc::new(RefCell::new(Some(column(3, 30., 2., 0))));
        let mine = Surface::SidebarRows(None);

        clear_pending(&state);
        set_pending(&state, &mine, vec![1, 0, 2]);
        set_pending(&state, &Surface::SidebarGroups, vec![2, 1, 0]);

        clear_pending(&state);
        assert_eq!(take_landed(&state).order, None);
        assert!(state.borrow().is_none());

        *state.borrow_mut() = Some(column(3, 30., 2., 0));
        clear_pending(&state);
        set_pending(&state, &mine, vec![1, 0, 2]);
        assert_eq!(take_landed(&state).order, Some(vec![1, 0, 2]));
    }

    /// A group offered on one frame and not the next must not still be there
    /// when the drag ends — letting go away from every group has to drop on
    /// nothing, the same way an un-recorded order does.
    #[test]
    fn a_regroup_lasts_only_as_long_as_the_pointer_is_over_the_group() {
        let tab = TabId::new();
        let work = Regroup::Into(GroupId::new());
        let fresh = || Some(column(3, 30., 2., 0).of_tab(tab));

        let state: ReorderState = Rc::new(RefCell::new(fresh()));
        clear_pending(&state);
        set_regroup(&state, work);
        clear_pending(&state);
        assert_eq!(take_landed(&state).regroup, None, "the frame moved on");

        *state.borrow_mut() = fresh();
        clear_pending(&state);
        set_regroup(&state, work);
        assert_eq!(take_landed(&state).regroup, Some((tab, work)));
    }

    /// A drag that is not carrying a tab — a sidebar group being reordered —
    /// has nothing to put in a group, so it must not answer with one.
    #[test]
    fn a_drag_with_no_tab_never_lands_in_a_group() {
        let state: ReorderState = Rc::new(RefCell::new(Some(column(3, 30., 2., 0))));
        clear_pending(&state);
        set_regroup(&state, Regroup::Into(GroupId::new()));
        assert_eq!(take_landed(&state).regroup, None);
    }

    /// Pinning is offered frame by frame, like a regroup: a header carried
    /// above the divider and back down before it is let go only reorders.
    #[test]
    fn a_pin_lasts_only_as_long_as_the_header_is_above_the_divider() {
        let key = AutoKey::Repo("/w/r".into());
        let fresh = || Some(column(3, 30., 2., 0).of_auto(key.clone()));

        let state: ReorderState = Rc::new(RefCell::new(fresh()));
        clear_pending(&state);
        set_pin(&state);
        clear_pending(&state);
        assert_eq!(take_landed(&state).pin, None, "carried back down");

        *state.borrow_mut() = fresh();
        clear_pending(&state);
        set_pin(&state);
        assert_eq!(take_landed(&state).pin, Some(key));
    }

    /// A finished drag says which surface its order is for: the pinned
    /// headers' order is the list's, not the tabs'.
    #[test]
    fn a_landing_names_the_surface_its_order_is_for() {
        let r = Reorder::new(
            Surface::PinnedGroups,
            0,
            column(3, 30., 2., 0).rects,
            px(2.),
            point(px(0.), px(0.)),
        );
        let state: ReorderState = Rc::new(RefCell::new(Some(r)));
        clear_pending(&state);
        set_pending(&state, &Surface::PinnedGroups, vec![1, 0, 2]);
        let landed = take_landed(&state);
        assert_eq!(landed.surface, Some(Surface::PinnedGroups));
        assert_eq!(landed.order, Some(vec![1, 0, 2]));
    }

    /// Only an auto header can be pinned by dragging; a drag carrying
    /// anything else offers nothing.
    #[test]
    fn a_drag_with_no_auto_group_never_pins() {
        let state: ReorderState = Rc::new(RefCell::new(Some(column(3, 30., 2., 0))));
        clear_pending(&state);
        set_pin(&state);
        assert_eq!(take_landed(&state).pin, None);
    }

    #[test]
    fn begin_frame_bumps_the_generation_only_on_change() {
        let r = column(3, 30., 2., 0);
        assert_eq!(r.begin_frame(0), (0, 0));
        assert_eq!(r.begin_frame(1), (1, 0));
        assert_eq!(r.begin_frame(1), (1, 1));
        assert_eq!(r.begin_frame(2), (2, 1));
    }

    /// Bounds can still be absent before a row's first paint.
    fn measured_rows(n: usize, visible: std::ops::Range<usize>, from: usize) -> Reorder {
        let h = 30.;
        let gap = 6.;
        let rects = (0..n)
            .map(|i| {
                if !visible.contains(&i) {
                    return Bounds::default();
                }
                let slot = i - visible.start;
                Bounds {
                    origin: point(px(0.), px(slot as f32 * (h + gap))),
                    size: size(px(120.), px(h)),
                }
            })
            .collect();
        Reorder::new(
            Surface::SidebarRows(None),
            from,
            rects,
            px(gap),
            point(px(60.), px(h / 2.)),
        )
    }

    #[test]
    fn an_unmeasured_slot_does_not_invert_the_held_clamp() {
        // Six tabs, only 2..5 on screen, dragging the middle visible one.
        let r = measured_rows(6, 2..5, 3);
        for y in [0., 60., 130., 400., 5000.] {
            // Would have panicked with "assertion failed: min <= max".
            let _ = r.held_offset(point(px(60.), px(y)), r.target(point(px(60.), px(y))));
        }
        let held = |y: f32| r.held_origin(point(px(60.), px(y)));
        assert_eq!(held(-500.), px(0.), "clamped to the first visible slot");
        assert_eq!(held(5000.), px(72.), "clamped to the last visible slot");
    }

    #[test]
    fn an_unmeasured_row_keeps_its_place_relative_to_the_dragged_row() {
        // Six rows, only 2..5 measured, dragging row 3.
        let r = measured_rows(6, 2..5, 3);
        let at = |y: f32| r.target(point(px(60.), px(y)));
        // Tabs 0 and 1 are hidden ahead of the drag and always count; tab 5 is
        // hidden behind it and never does. So the reachable targets are the
        // three measured rows, and dragging up cannot pass unmeasured rows.
        assert_eq!(at(-500.), 2, "cannot pass the hidden tabs ahead of it");
        assert_eq!(at(5000.), 4, "cannot pass the hidden tab behind it");
        // A full-length permutation still comes out, with the hidden tabs
        // left where they were.
        assert_eq!(r.order(at(-500.)), vec![0, 1, 3, 2, 4, 5]);
        assert_eq!(r.order(at(5000.)), vec![0, 1, 2, 4, 3, 5]);
    }

    #[test]
    fn unmeasured_rows_do_not_panic() {
        let r = measured_rows(4, 0..0, 0);
        let _ = r.held_offset(point(px(60.), px(15.)), r.target(point(px(60.), px(15.))));
    }
}

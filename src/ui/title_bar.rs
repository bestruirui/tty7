use gpui::base::LogicalStyledExt as _;
use gpui::component::button::{Button, ButtonCustomVariant, ButtonVariants as _};
use gpui::component::{ActiveTheme as _, Icon, IconName, Selectable as _, Sizable as _, h_flex};
use gpui::{App, Context, MouseButton, Window, div, prelude::*, px};

use crate::core::config::Config;
use crate::ui::app::{TILE_GLYPH, TILE_SIZE, TitleBarRow as _, Tty7App, tile_trailing_inset};
use crate::ui::i18n::{L10nKey, t, t_fmt};

pub(crate) fn chrome_tile_variant(cx: &gpui::App) -> ButtonCustomVariant {
    chrome_tile_variant_for(false, cx)
}

pub(crate) fn chrome_tile_variant_for(selected: bool, cx: &gpui::App) -> ButtonCustomVariant {
    ButtonCustomVariant::new(cx)
        .color(cx.theme().transparent)
        // Resting chrome is *not* body ink. `sidebar_foreground` is the rung a
        // tab title is written at, so a toolbar drawn in it made the two
        // controls at the top of the rail the darkest marks in the whole
        // sidebar — louder than the twenty rows they exist to act on, which is
        // the opposite of how a native sidebar ranks itself. One rung down puts
        // them level with the workspace chip beside them, and the hover fill
        // this variant already carries is what answers the pointer.
        .foreground(if selected {
            cx.theme().foreground
        } else {
            cx.theme().muted_foreground
        })
        // `sidebar_accent` is the surface's *selected* step, and it was handed
        // to hover as well — so a hovered tile wore the fill of a selected one
        // and, with the right panel open, two tiles read as current at once.
        // Hover takes the step the palette derives for it. (A selected button
        // never renders the hover style, so this only reaches the rest.)
        .hover(gpui::rgb(cx.global::<crate::ui::presets::Surfaces>().sidebar.hover).into())
        .active(cx.theme().sidebar_accent)
}

pub(crate) const BUTTON_ICON_SCALE: f32 = 0.75;

pub(crate) const RAIL_TILE_RADIUS: f32 = 6.;

/// WCAG 2.2 SC 2.5.8 puts the desktop floor for a pointer target at 24×24, and
/// gpui-component renders an icon-only `.xsmall()` button as a 20×20 box (18×18
/// where the chrome overrode it). Grow only the box: the glyph keeps its size,
/// so the chrome looks unchanged and simply stops being fiddly to hit.
pub(crate) const MIN_TARGET: f32 = 24.;

pub(crate) fn hit_target(button: Button) -> Button {
    button.w(px(MIN_TARGET)).h(px(MIN_TARGET))
}

pub(crate) fn chrome_tile(button: Button, cx: &gpui::App) -> Button {
    chrome_tile_sized(button, TILE_SIZE, TILE_GLYPH, false, cx)
}

pub(crate) fn chrome_tile_sized(
    button: Button,
    tile: f32,
    glyph: f32,
    selected: bool,
    cx: &gpui::App,
) -> Button {
    button
        .custom(chrome_tile_variant_for(selected, cx))
        .selected(selected)
        .with_size(px(glyph / BUTTON_ICON_SCALE))
        .w(px(tile))
        .h(px(tile))
}

#[derive(gpui::IntoElement)]
struct TitleBar {
    content: gpui::AnyElement,
}

impl gpui::RenderOnce for TitleBar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let client_decorated = matches!(
            window.window_decorations(),
            gpui::Decorations::Client { .. }
        );
        let draggable = !window.is_fullscreen() || cfg!(target_os = "macos");
        let controls = (crate::ui::app::window_controls_w(window) > 0.).then(|| {
            let supported = window.window_controls();
            let mut controls = h_flex()
                .id("window-controls")
                .flex_row_dir(cx)
                .flex_shrink_0()
                .gap(px(4.))
                .pr(px(tile_trailing_inset()));
            for (id, icon, area, shown) in [
                (
                    "minimize",
                    IconName::WindowMinimize,
                    gpui::WindowControlArea::Min,
                    supported.minimize,
                ),
                (
                    "maximize",
                    if window.is_maximized() {
                        IconName::WindowRestore
                    } else {
                        IconName::WindowMaximize
                    },
                    gpui::WindowControlArea::Max,
                    supported.maximize,
                ),
                (
                    "close",
                    IconName::WindowClose,
                    gpui::WindowControlArea::Close,
                    true,
                ),
            ] {
                if !shown {
                    continue;
                }
                controls = controls.child(
                    chrome_tile(Button::new(id), cx)
                        .rounded_lg()
                        .map(|button| {
                            if area == gpui::WindowControlArea::Close {
                                button
                                    .group("window-close")
                                    .custom(
                                        chrome_tile_variant(cx)
                                            .hover(cx.theme().danger)
                                            .active(cx.theme().danger_active),
                                    )
                                    .child(
                                        div()
                                            .id("icon")
                                            .size(px(TILE_GLYPH))
                                            .group_hover("window-close", |style| {
                                                style.text_color(cx.theme().danger_foreground)
                                            })
                                            .group_active("window-close", |style| {
                                                style.text_color(cx.theme().danger_foreground)
                                            })
                                            .child(Icon::new(icon).size(px(TILE_GLYPH))),
                                    )
                            } else {
                                button.icon(icon)
                            }
                        })
                        .when(cfg!(target_os = "windows"), |button| {
                            button.window_control_area(area)
                        })
                        .when(cfg!(target_os = "linux"), |button| {
                            button
                                .on_mouse_down(MouseButton::Left, |_, window, cx| {
                                    window.prevent_default();
                                    cx.stop_propagation();
                                })
                                .on_click(move |_, window, cx| {
                                    cx.stop_propagation();
                                    match area {
                                        gpui::WindowControlArea::Min => window.minimize_window(),
                                        gpui::WindowControlArea::Max => window.zoom_window(),
                                        gpui::WindowControlArea::Close => window.remove_window(),
                                        gpui::WindowControlArea::Drag => unreachable!(),
                                    }
                                })
                        }),
                );
            }
            controls
        });
        let radii = gpui::component::window_corner_radii(window, cx);
        let bar = h_flex()
            .id("title-bar")
            .flex_shrink_0()
            .relative()
            .flex_row_dir(cx)
            .justify_between()
            .pl(px(crate::ui::app::TITLE_BAR_LEAD))
            .title_bar_row()
            .rounded_tl(radii.top_left)
            .rounded_tr(radii.top_right)
            .bg(cx.theme().transparent);
        bar.child(
            h_flex()
                .id("bar")
                .h_full()
                .min_w_0()
                .flex_1()
                .when(draggable && !cfg!(target_family = "wasm"), |row| {
                    crate::ui::app::title_bar_drag(row, "title-bar-drag", window, cx)
                })
                .when(window.is_fullscreen() && cfg!(target_os = "macos"), |row| {
                    row.ps(gpui::rems(0.75), cx)
                })
                .when(
                    cfg!(target_os = "linux") && client_decorated && draggable,
                    |row| {
                        row.on_mouse_down(MouseButton::Right, |event, window, _| {
                            window.show_window_menu(event.position);
                        })
                    },
                )
                .child(self.content),
        )
        .when_some(controls, |bar, controls| bar.child(controls))
    }
}

impl Tty7App {
    /// Persistent window controls, including when either sidebar is collapsed.
    pub(crate) fn window_chrome(
        &self,
        trailing: f32,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + use<> {
        // Opening the panel moves this control to the panel's header on
        // macOS. Reuse the same tile and inset there: a different hit target
        // moves the centred glyph even when the icon itself keeps its size.
        let panel_open = self.right_panel_open();
        h_flex()
            .flex_shrink_0()
            .items_center()
            .pr(px(trailing))
            .child(
                div().occlude().flex_shrink_0().child(
                    chrome_tile(
                        Button::new("titlebar-right-panel")
                            // Keep the outline fixed; only the panel region's
                            // fill changes to show whether the panel is visible.
                            .icon(Icon::empty().path(match panel_open {
                                true => "icons/panel-right.svg",
                                false => "icons/panel-right-collapsed.svg",
                            })),
                        cx,
                    )
                    .rounded_lg()
                    .accessibility_label(match panel_open {
                        true => t(L10nKey::TabTooltipHideDetailPanel),
                        false => t(L10nKey::TabTooltipShowDetailPanel),
                    })
                    .tooltip_with_action(
                        match panel_open {
                            true => t(L10nKey::TabTooltipHideDetailPanel),
                            false => t(L10nKey::TabTooltipShowDetailPanel),
                        },
                        &crate::core::actions::ToggleRightPanel,
                        None,
                    )
                    // On macOS this tile is drawn inside the panel's own
                    // titlebar while the panel is open, so closing from it
                    // destroys the element holding the focus — and a keymap
                    // scoped to a focused thing goes quiet with it, leaving the
                    // ⌘J that would undo this doing nothing. Hand the terminal
                    // back what it lost.
                    .on_click(cx.listener(|this, _, window, cx| {
                        let closing = this.right_panel_open();
                        this.toggle_right_panel(cx);
                        if closing {
                            this.focus_active(window, cx);
                        }
                    })),
                ),
            )
    }
    pub(crate) fn render_title_bar(
        &self,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + use<> {
        let controls_w = crate::ui::app::window_controls_w(window);
        let trailing = if controls_w > 0. {
            4.
        } else {
            tile_trailing_inset()
        };
        let chrome_band_w = (!cfg!(target_os = "macos") && self.right_panel_open())
            .then(|| (self.right_panel_px(window) - controls_w - 1.).max(0.));
        let auto_hide_chrome = cx.global::<Config>().auto_hide_titlebar_buttons;
        let titlebar_chrome_shown = crate::ui::app::titlebar_chrome_shown(
            auto_hide_chrome,
            self.titlebar_chrome_hover.get(),
        );
        let left_group = (!self.sidebar_open()).then(|| {
            h_flex()
                .flex_shrink_0()
                .items_center()
                .gap(px(2.))
                .when_some(crate::ui::app::window_mark(), |group, mark| {
                    group.child(
                        div()
                            .flex_shrink_0()
                            .size(px(TILE_SIZE))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(mark),
                    )
                })
                // The logo above stays put: it is the window's mark, not a
                // control, and a window that loses its identity when nobody is
                // pointing at it reads as a different window.
                .child(
                    crate::ui::app::resting_chrome(
                        div().occlude().flex_shrink_0(),
                        titlebar_chrome_shown,
                    )
                    .child(Self::new_tab_button("titlebar-add-collapsed", cx)),
                )
                .child(
                    crate::ui::app::resting_chrome(
                        div().occlude().flex_shrink_0(),
                        titlebar_chrome_shown,
                    )
                    .child(
                        chrome_tile(
                            Button::new("titlebar-settings")
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
                    crate::ui::app::resting_chrome(
                        div().occlude().flex_shrink_0(),
                        titlebar_chrome_shown,
                    )
                    .child(
                        chrome_tile(
                            Button::new("titlebar-expand-sidebar")
                                .icon(Icon::empty().path("icons/panel-left-collapsed.svg")),
                            cx,
                        )
                        .rounded_lg()
                        .accessibility_label(t(L10nKey::TabTooltipShowSidebar))
                        .tooltip_with_action(
                            t(L10nKey::TabTooltipShowSidebar),
                            &crate::core::actions::ToggleLeftPanel,
                            None,
                        )
                        .on_click(cx.listener(|this, _, _window, cx| this.toggle_left_panel(cx))),
                    ),
                )
        });

        // On macOS the open right panel provides its own header controls.
        let right_chrome_shown = titlebar_chrome_shown || self.right_panel_open();
        let right_chrome = (!cfg!(target_os = "macos") || !self.right_panel_open()).then(|| {
            crate::ui::app::resting_chrome(div().flex_shrink_0(), right_chrome_shown)
                .child(self.window_chrome(trailing, cx))
        });

        let content = h_flex()
            .id("titlebar-content")
            .relative()
            .items_center()
            .gap_1p5()
            .w_full()
            .pl_0()
            .min_w_0()
            .when_some(left_group, |this, g| this.child(g))
            .child(div().flex_1().min_w_0().when(
                cx.global::<Config>().show_content_title,
                |title| {
                    title
                        .text_sm()
                        .line_height(px(TILE_SIZE))
                        .text_color(cx.theme().muted_foreground)
                        .truncate()
                        .when_some(self.tabs.get(self.active), |title, tab| {
                            title.child(self.full_tab_label(tab, Some(window), cx).unwrap_or_else(
                                || {
                                    t_fmt(
                                        L10nKey::TabUnnamedShell,
                                        &[("n", &((self.active + 1).to_string()))],
                                    )
                                },
                            ))
                        })
                },
            ))
            .when_some(right_chrome, |this, chrome| match chrome_band_w {
                Some(w) => this.child(
                    h_flex()
                        .flex_none()
                        .w(px(w))
                        .items_center()
                        .pl(px(tile_trailing_inset()))
                        .child(chrome),
                ),
                None => this.child(chrome),
            })
            .when(auto_hide_chrome, |this| {
                this.child(crate::ui::app::hover_sheet(
                    "titlebar-chrome-hover",
                    &self.titlebar_chrome_hover,
                ))
            });
        TitleBar {
            content: content.into_any_element(),
        }
    }
}

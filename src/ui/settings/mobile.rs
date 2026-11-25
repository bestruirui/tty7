//! Settings → Mobile: switching phone access on, pairing a phone, and the
//! phones already paired.
//!
//! Everything this page shows or changes about the gateway goes through the
//! selected server's control interface, for both local and remote machines.

use std::io::Cursor;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tty7_core::daemon::control::{ControlRequest, ReplyOk};
use tty7_core::daemon::mobile::Status;

use super::kit::{self, BtnKind, Tk, fs};
use super::shell::MenuEntry;
use super::*;

/// How long a pairing code on screen stays good.
const PAIR_TTL: Duration = Duration::from_secs(600);
/// How often an open pairing looks for the phone that used it.
const PAIR_POLL: Duration = Duration::from_secs(1);
const QR_SIZE: f32 = 208.;
/// A pairing code on screen.
pub(crate) struct Pairing {
    code: String,
    /// The offer's secret, to ask the server whether it is still open.
    secret: String,
    qr: Option<Arc<gpui::Image>>,
    /// The phones paired before this code was made, to tell the new one by.
    before: Vec<String>,
    until: Instant,
    /// The code no longer pairs. It stays on screen, marked so, until it is
    /// replaced or dismissed: a code that vanishes by itself leaves whoever
    /// was about to scan it wondering where it went.
    spent: Option<Spent>,
}

#[derive(Clone, Copy)]
enum Spent {
    Expired,
    /// A phone tried it and it did not pair — mistyped, or the connection
    /// dropped. Any attempt closes an offer.
    Tried,
    /// Something else opened a newer offer, which replaces this one.
    Replaced,
}

/// Minutes and seconds, for the time a code has left.
fn countdown(left: Duration) -> String {
    let secs = left.as_secs();
    format!("{}:{:02}", secs / 60, secs % 60)
}

/// What the switch shows: whether phones can actually reach this machine,
/// not merely whether they were asked to be allowed.
enum Access {
    Off,
    Starting,
    On,
    /// Asked for, and the daemon could not start it: shown off, with why.
    Failed(String),
}

fn access(status: Option<&(bool, Status)>, starting: bool) -> Access {
    if starting {
        return Access::Starting;
    }
    match status {
        Some((_, Status::Running { .. })) => Access::On,
        Some((_, Status::Failed { error })) => Access::Failed(error.clone()),
        Some((false, _)) => Access::Off,
        // The server has not started it yet, or its status reply is still pending.
        _ => Access::Starting,
    }
}

/// The code as a QR image: dark modules on white with a quiet zone, whatever
/// the theme, since that is what a phone's camera reads.
fn qr_image(code: &str) -> Option<Arc<gpui::Image>> {
    let qr =
        qrcode::QrCode::with_error_correction_level(code.as_bytes(), qrcode::EcLevel::L).ok()?;
    let width = qr.width();
    let (quiet, scale) = (2, 8);
    let side = ((width + 2 * quiet) * scale) as u32;
    let mut img = image::GrayImage::from_pixel(side, side, image::Luma([255]));
    for (i, color) in qr.to_colors().iter().enumerate() {
        if *color != qrcode::Color::Dark {
            continue;
        }
        let (x0, y0) = ((i % width + quiet) * scale, (i / width + quiet) * scale);
        for y in y0..y0 + scale {
            for x in x0..x0 + scale {
                img.put_pixel(x as u32, y as u32, image::Luma([0]));
            }
        }
    }
    let mut png = Vec::new();
    image::DynamicImage::ImageLuma8(img)
        .write_to(&mut Cursor::new(&mut png), image::ImageFormat::Png)
        .ok()?;
    Some(Arc::new(gpui::Image::from_bytes(
        gpui::ImageFormat::Png,
        png,
    )))
}

impl Tty7App {
    pub(crate) fn ensure_mobile_loaded(&mut self, cx: &mut Context<Self>) {
        let Some(settings) = self.active_settings_mut()
            .filter(|s| s.mobile_generation.is_nil()) else { return };
        settings.mobile_generation = uuid::Uuid::new_v4();
        let generation = settings.mobile_generation;
        let host = settings.mobile_host;
        cx.spawn(async move |this, cx| {
            loop {
                let target = this.update(cx, |this, cx| {
                    let settings = this.active_settings()?;
                    if settings.mobile_generation != generation {
                        return None;
                    }
                    Some((
                        !settings.mobile_starting
                            && (settings.mobile_status.is_none()
                                || settings.section == SettingsSection::Mobile
                                || settings.search_active
                                || settings.mobile_pairing.is_some()),
                        crate::ui::tree_sync::control_for(cx, host),
                        settings.mobile_pairing.as_ref()
                            .filter(|p| p.until > Instant::now())
                            .map(|p| p.secret.clone()),
                    ))
                });
                let Ok(Some((visible, client, secret))) = target else { return };
                if !visible {
                    smol::Timer::after(PAIR_POLL).await;
                    continue;
                }
                let result = match client {
                    Some(client) => cx.background_executor().spawn(async move {
                        let ReplyOk::MobileStatus { enabled, status, devices } = client.call(ControlRequest::MobileStatus)? else {
                            return Err(std::io::Error::other("unexpected mobile status response"));
                        };
                        let pairing = if let Some(secret) = secret {
                            let ReplyOk::MobilePairing { open, has_open } = client.call(
                                ControlRequest::MobilePairing { secret: secret.clone() },
                            )? else {
                                return Err(std::io::Error::other("unexpected pairing response"));
                            };
                            Some((secret, open, has_open))
                        } else {
                            None
                        };
                        Ok((enabled, status, devices, pairing))
                    }).await,
                    None => Err(std::io::Error::other(t(L10nKey::SettingsMobileNoAnswer).to_string())),
                };
                let alive = this.update(cx, |this, cx| {
                    let Some(settings) = this.active_settings_mut()
                        .filter(|s| s.mobile_generation == generation) else { return false };
                    match result {
                        Ok((enabled, status, devices, pairing)) => {
                            settings.mobile_status = Some((enabled, status));
                            settings.mobile_devices = devices;
                            if let Some((secret, open, has_open)) = pairing
                                && let Some(pairing) = settings.mobile_pairing.as_mut()
                                    .filter(|p| p.secret == secret)
                            {
                                let paired = settings.mobile_devices.iter()
                                    .find(|d| !pairing.before.contains(&d.id))
                                    .map(|d| d.name.clone());
                                if let Some(name) = paired {
                                    settings.mobile_paired = Some(name);
                                    settings.mobile_pairing = None;
                                } else if pairing.spent.is_none() {
                                    pairing.spent = if Instant::now() >= pairing.until {
                                        Some(Spent::Expired)
                                    } else if open {
                                        None
                                    } else if has_open {
                                        Some(Spent::Replaced)
                                    } else {
                                        Some(Spent::Tried)
                                    };
                                }
                            }
                        }
                        Err(error) => {
                            let enabled = settings.mobile_status.as_ref().is_some_and(|(enabled, _)| *enabled);
                            settings.mobile_status = Some((enabled, Status::Failed { error: error.to_string() }));
                        }
                    }
                    if let Some(pairing) = settings.mobile_pairing.as_mut()
                        && pairing.spent.is_none()
                        && Instant::now() >= pairing.until
                    {
                        pairing.spent = Some(Spent::Expired);
                    }
                    cx.notify();
                    true
                });
                if !matches!(alive, Ok(true)) { return; }
                smol::Timer::after(PAIR_POLL).await;
            }
        }).detach();
    }

    fn select_mobile_host(&mut self, host: HostId, cx: &mut Context<Self>) {
        if self.active_settings().is_some_and(|s| s.mobile_host == host) {
            return;
        }
        self.close_mobile_pairing(cx);
        if let Some(settings) = self.active_settings_mut() {
            settings.mobile_host = host;
            settings.mobile_generation = uuid::Uuid::nil();
            settings.mobile_status = None;
            settings.mobile_devices.clear();
            settings.mobile_error = None;
            settings.mobile_paired = None;
            settings.mobile_starting = false;
            settings.mobile_pair_pending = false;
        }
        self.ensure_mobile_loaded(cx);
        cx.notify();
    }

    /// Switching phone access is a request to the selected server, so the
    /// switch holds at "starting" until it answers with the actual service state.
    pub(crate) fn set_mobile_access(&mut self, on: bool, window: &mut Window, cx: &mut Context<Self>) {
        let Some(host) = self.active_settings().map(|s| s.mobile_host) else { return };
        let Some(client) = crate::ui::tree_sync::control_for(cx, host) else {
            if let Some(settings) = self.active_settings_mut() {
                settings.mobile_error = Some(t(L10nKey::SettingsMobileNoAnswer).to_string());
            }
            cx.notify();
            return;
        };
        if let Some(settings) = self.active_settings_mut() {
            settings.mobile_starting = true;
            settings.mobile_pair_pending = false;
            settings.mobile_error = None;
            // Ignore replies from before this switch, including polls still in flight.
            settings.mobile_generation = uuid::Uuid::nil();
        }
        self.ensure_mobile_loaded(cx);
        let generation = self.active_settings().unwrap().mobile_generation;
        cx.notify();
        cx.spawn_in(window, async move |this, cx| {
            let result = cx.background_executor().spawn(async move {
                client.call(ControlRequest::MobileSetEnabled { enabled: on })
            }).await;
            let _ = this.update_in(cx, |this, window, cx| {
                if this.active_settings().is_none_or(|s| s.mobile_generation != generation) { return; }
                if !on { this.close_mobile_pairing(cx); }
                if let Some(settings) = this.active_settings_mut() {
                    settings.mobile_starting = false;
                    match result {
                        Ok(ReplyOk::MobileStatus { enabled, status, devices }) => {
                            settings.mobile_status = Some((enabled, status));
                            settings.mobile_devices = devices;
                        }
                        Err(error) => {
                            settings.mobile_error = Some(error.to_string());
                            window.push_notification(t_fmt(L10nKey::SettingsMobileStartFailed, &[("error", &error.to_string())]), cx);
                        }
                        _ => settings.mobile_error = Some("unexpected mobile status response".into()),
                    }
                }
                cx.notify();
            });
        }).detach();
    }

    /// Shows a fresh code, replacing any on screen — which the new offer
    /// closes, since the gateway keeps only one open at a time.
    fn start_mobile_pairing(&mut self, cx: &mut Context<Self>) {
        let Some(settings) = self.active_settings() else { return };
        if settings.mobile_pair_pending { return; }
        let host = settings.mobile_host;
        let generation = settings.mobile_generation;
        let Some(client) = crate::ui::tree_sync::control_for(cx, host) else {
            if let Some(settings) = self.active_settings_mut() {
                settings.mobile_error = Some(t(L10nKey::SettingsMobileNoAnswer).to_string());
            }
            cx.notify();
            return;
        };
        if let Some(settings) = self.active_settings_mut() {
            settings.mobile_pair_pending = true;
            settings.mobile_error = None;
        }
        cx.notify();
        cx.spawn(async move |this, cx| {
            let pairing_client = client.clone();
            let result = cx.background_executor().spawn(async move { client.call(ControlRequest::MobilePair) }).await;
            let issued_secret = match &result {
                Ok(ReplyOk::MobilePair { secret, .. }) => Some(secret.clone()),
                _ => None,
            };
            let shown = this.update(cx, |this, cx| {
                let Some(settings) = this.active_settings_mut()
                    .filter(|s| s.mobile_generation == generation)
                else { return false };
                settings.mobile_pair_pending = false;
                let (code, secret, devices) = match result {
                    Ok(ReplyOk::MobilePair { code, secret, devices }) => (code, secret, devices),
                    Err(error) => { settings.mobile_error = Some(error.to_string()); cx.notify(); return false; }
                    _ => { settings.mobile_error = Some("unexpected pairing response".into()); cx.notify(); return false; }
                };
                settings.mobile_pairing = Some(Pairing {
                    qr: qr_image(&code), code, secret,
                    before: devices.into_iter().map(|d| d.id).collect(),
                    until: Instant::now() + PAIR_TTL, spent: None,
                });
                settings.mobile_paired = None;
                settings.mobile_copied = false;
                cx.notify();
                true
            });
            if !matches!(shown, Ok(true)) {
                if let Some(secret) = issued_secret {
                    cx.background_executor().spawn(async move {
                        let _ = pairing_client.call(ControlRequest::MobileCancelPair { secret });
                    }).await;
                }
            }
        }).detach();
    }

    /// Takes the code off screen and withdraws its offer, so a code dismissed
    /// here cannot still be used from a photo of it.
    fn close_mobile_pairing(&mut self, cx: &mut Context<Self>) {
        let Some(settings) = self.active_settings_mut() else { return };
        let host = settings.mobile_host;
        let generation = settings.mobile_generation;
        let Some(pairing) = settings.mobile_pairing.take() else { return };
        if let Some(client) = crate::ui::tree_sync::control_for(cx, host) {
            cx.spawn(async move |this, cx| {
                let result = cx.background_executor().spawn(async move {
                    client.call(ControlRequest::MobileCancelPair { secret: pairing.secret })
                }).await;
                let _ = this.update(cx, |this, cx| {
                    if let Some(settings) = this.active_settings_mut().filter(|s| s.mobile_generation == generation)
                        && let Err(error) = result {
                        settings.mobile_error = Some(error.to_string());
                        cx.notify();
                    }
                });
            }).detach();
        } else if let Some(settings) = self.active_settings_mut() {
            settings.mobile_error = Some(t(L10nKey::SettingsMobileNoAnswer).to_string());
        }
        cx.notify();
    }

    fn unpair_mobile_device(&mut self, id: String, cx: &mut Context<Self>) {
        let Some((host, generation)) = self.active_settings().map(|s| (s.mobile_host, s.mobile_generation)) else { return };
        let Some(client) = crate::ui::tree_sync::control_for(cx, host) else {
            if let Some(settings) = self.active_settings_mut() {
                settings.mobile_error = Some(t(L10nKey::SettingsMobileNoAnswer).to_string());
            }
            cx.notify();
            return;
        };
        cx.spawn(async move |this, cx| {
            let result = cx.background_executor().spawn(async move { client.call(ControlRequest::MobileRevoke { id }) }).await;
            let _ = this.update(cx, |this, cx| {
                if let Some(settings) = this.active_settings_mut().filter(|s| s.mobile_generation == generation) {
                    match result {
                        Ok(ReplyOk::MobileDevices(devices)) => settings.mobile_devices = devices,
                        Err(error) => settings.mobile_error = Some(error.to_string()),
                        _ => settings.mobile_error = Some("unexpected mobile device response".into()),
                    }
                    cx.notify();
                }
            });
        }).detach();
    }

    pub(crate) fn render_settings_mobile(&self, cx: &mut Context<Self>) -> AnyElement {
        let tk = Tk::of(cx);
        let host = self.active_settings().map_or(HostId::LOCAL, |s| s.mobile_host);
        let starting = self.active_settings().is_some_and(|s| s.mobile_starting);
        let access = access(self.active_settings().and_then(|s| s.mobile_status.as_ref()), starting);
        let serving = matches!(access, Access::On);
        let pending = self.active_settings().is_some_and(|s| s.mobile_pair_pending);
        let devices = self.active_settings().map(|s| s.mobile_devices.clone()).unwrap_or_default();
        let machines = Self::settings_machines(cx);
        let current = machines.iter().find(|m| m.host == host).map(|m| m.label.clone()).unwrap_or_default();
        let entries = machines.into_iter().map(|machine| {
            let selected = machine.host;
            MenuEntry::item(machine.label, selected == host, move |this, _, cx| {
                this.select_mobile_host(selected, cx);
            })
        }).collect();
        let selector = self.settings_dropdown("mobile-machine", current.clone(), None, entries, cx);
        let machine_desc = if host.is_local() { t(L10nKey::SettingsMachineLocalDesc).to_string() }
            else { t_fmt(L10nKey::SettingsMachineRemoteDesc, &[("name", &current)]) };
        let machine_group = self.settings_group(None, None, [self.settings_row(
            t(L10nKey::SettingsMachine), machine_desc, selector, cx,
        ).into_any_element()], cx);

        let switch = kit::switch("mobile-access")
            .checked(matches!(access, Access::On | Access::Starting))
            .disabled(matches!(access, Access::Starting))
            .on_click(
                cx.listener(|this, on: &bool, window, cx| this.set_mobile_access(*on, window, cx)),
            )
            .into_any_element();
        let access_desc = match &access {
            Access::Off | Access::On => t(L10nKey::SettingsMobileAccessDesc).to_string(),
            Access::Starting => t(L10nKey::SettingsMobileStatusStarting).to_string(),
            Access::Failed(error) => {
                t_fmt(L10nKey::SettingsMobileStatusFailed, &[("error", error)])
            }
        };

        let pairing = self
            .active_settings()
            .and_then(|s| s.mobile_pairing.as_ref());
        // Only while that phone is still on the list: unpairing it left
        // "Paired with probe." above an empty "No phones paired yet."
        let paired = self
            .active_settings()
            .and_then(|s| s.mobile_paired.clone())
            .filter(|name| devices.iter().any(|d| &d.name == name));
        // A spent code carries its own "New code" beside the reason it is
        // spent; a second one up here would be the same button twice.
        let pair_button = if pairing.is_some_and(|p| p.spent.is_some()) {
            div().into_any_element()
        } else {
            self.settings_button(
                "mobile-pair",
                if pairing.is_some() {
                    t(L10nKey::SettingsMobileNewCode)
                } else {
                    t(L10nKey::SettingsMobileShowCode)
                },
                cx,
                |this, _, cx| this.start_mobile_pairing(cx),
            )
            .disabled(!serving || pending)
            .into_any_element()
        };
        let pair_desc = match (serving, &paired) {
            (_, Some(name)) => t_fmt(L10nKey::SettingsMobilePaired, &[("name", name)]),
            (true, None) => t(L10nKey::SettingsMobilePairDesc).to_string(),
            (false, None) => t(L10nKey::SettingsMobilePairNeedsAccess).to_string(),
        };

        let access_group = self.settings_group(
            None,
            None,
            [self
                .settings_row(t(L10nKey::SettingsMobileAccess), access_desc, switch, cx)
                .into_any_element()],
            cx,
        );

        let mut pair_rows = vec![
            self.settings_row(t(L10nKey::SettingsMobilePair), pair_desc, pair_button, cx)
                .into_any_element(),
        ];
        if let Some(pairing) = pairing {
            pair_rows.push(self.render_mobile_pairing(pairing, &tk, cx));
        }
        let pair_group = self.settings_group(None, None, pair_rows, cx);

        let phone_rows: Vec<AnyElement> = if devices.is_empty() {
            vec![
                div()
                    .py(px(10.))
                    .text_size(fs(13.))
                    .text_color(tk.k5)
                    .child(t(L10nKey::SettingsMobileNoPhones))
                    .into_any_element(),
            ]
        } else {
            devices
                .into_iter()
                .map(|device| {
                    let id = device.id.clone();
                    let unpair =
                        kit::button(
                            SharedString::from(format!("mobile-unpair-{}", device.id)),
                            t(L10nKey::SettingsMobileUnpair),
                            BtnKind::Danger,
                        )
                        .on_click(cx.listener(move |this, _, _w, cx| {
                            this.unpair_mobile_device(id.clone(), cx)
                        }))
                        .into_any_element();
                    self.settings_row(
                        device.name,
                        device.id[..device.id.len().min(12)].to_string(),
                        unpair,
                        cx,
                    )
                    .into_any_element()
                })
                .collect()
        };
        let phones_group =
            self.settings_group(Some(t(L10nKey::SettingsMobilePhones)), None, phone_rows, cx);

        let mut groups = vec![machine_group, access_group, pair_group, phones_group];
        if let Some(error) = self.active_settings().and_then(|s| s.mobile_error.as_ref()) {
            groups.push(div().text_size(fs(13.)).text_color(tk.k5).child(error.clone()).into_any_element());
        }
        Self::settings_page(groups)
    }

    fn render_mobile_pairing(
        &self,
        pairing: &Pairing,
        tk: &Tk,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let copied = self.active_settings().is_some_and(|s| s.mobile_copied);
        let code = pairing.code.clone();
        let close = kit::button(
            "mobile-pair-cancel",
            t(if pairing.spent.is_some() {
                L10nKey::Close
            } else {
                L10nKey::Cancel
            }),
            BtnKind::Link,
        )
        .on_click(cx.listener(|this, _, _w, cx| this.close_mobile_pairing(cx)))
        .into_any_element();

        let qr = match &pairing.qr {
            Some(image) => gpui::img(image.clone())
                .size(px(QR_SIZE))
                .into_any_element(),
            None => div().size(px(QR_SIZE)).into_any_element(),
        };
        let side = v_flex().flex_1().min_w_0().gap(px(12.));
        let side = match pairing.spent {
            // What happened, and the one way on from it.
            Some(spent) => {
                let renew = kit::button(
                    "mobile-pair-renew",
                    t(L10nKey::SettingsMobileNewCode),
                    BtnKind::Primary,
                )
                .disabled(self.active_settings().is_some_and(|s| s.mobile_pair_pending))
                .on_click(cx.listener(|this, _, _w, cx| this.start_mobile_pairing(cx)))
                .into_any_element();
                side.child(
                    div()
                        .text_size(fs(13.))
                        .text_color(tk.fg)
                        .child(t(match spent {
                            Spent::Expired => L10nKey::SettingsMobilePairExpired,
                            Spent::Tried => L10nKey::SettingsMobilePairTried,
                            Spent::Replaced => L10nKey::SettingsMobilePairReplaced,
                        })),
                )
                .child(
                    h_flex()
                        .gap(px(12.))
                        .items_center()
                        .child(renew)
                        .child(close),
                )
            }
            None => {
                let copy = kit::button(
                    "mobile-copy-code",
                    if copied {
                        t(L10nKey::SettingsCopied)
                    } else {
                        t(L10nKey::SettingsMobileCopyCode)
                    },
                    BtnKind::Secondary,
                )
                .on_click(cx.listener(move |this, _, _w, cx| {
                    cx.write_to_clipboard(gpui::ClipboardItem::new_string(code.clone()));
                    if let Some(s) = this.active_settings_mut() {
                        s.mobile_copied = true;
                    }
                    cx.notify();
                    cx.spawn(async move |this, cx| {
                        smol::Timer::after(Duration::from_millis(1500)).await;
                        let _ = this.update(cx, |this, cx| {
                            if let Some(s) = this.active_settings_mut() {
                                s.mobile_copied = false;
                                cx.notify();
                            }
                        });
                    })
                    .detach();
                }))
                .into_any_element();
                let left = pairing.until.saturating_duration_since(Instant::now());
                side.child(
                    div()
                        .text_size(fs(13.))
                        .text_color(tk.fg)
                        .child(t(L10nKey::SettingsMobilePairScan)),
                )
                .child(
                    div()
                        .p(px(8.))
                        .rounded(px(6.))
                        .bg(tk.k04)
                        .text_size(fs(11.))
                        .font_family(Tk::mono(cx))
                        .text_color(tk.k6)
                        .line_clamp(3)
                        .text_ellipsis()
                        .child(pairing.code.clone()),
                )
                .child(
                    h_flex()
                        .gap(px(12.))
                        .items_center()
                        .child(copy)
                        .child(close),
                )
                .child(div().text_size(fs(12.)).text_color(tk.k5).child(t_fmt(
                    L10nKey::SettingsMobilePairValid,
                    &[("time", &countdown(left))],
                )))
            }
        };

        h_flex()
            .id("mobile-pairing")
            .mt(px(12.))
            .gap(px(24.))
            .items_start()
            // White whatever the theme: a camera reads dark on light. Faded
            // once spent, so nobody scans a code that will be refused.
            .child(
                div()
                    .flex_none()
                    .p(px(8.))
                    .rounded(px(12.))
                    .bg(gpui::white())
                    .border_1()
                    .border_color(tk.k08)
                    .when(pairing.spent.is_some(), |d| d.opacity(0.15))
                    .child(qr),
            )
            .child(side)
            .into_any_element()
    }
}

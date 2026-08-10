#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app;
mod default_file_icon;
mod settings;
mod system;

use app::{AppView, Escape, MoveDown, MoveUp};
use gpui::*;

fn main() {
    gpui_platform::application()
        .with_assets(gpui_component_assets::Assets)
        .with_quit_mode(QuitMode::LastWindowClosed)
        .run(|cx| {
            gpui_component::init(cx);
            cx.bind_keys([
                KeyBinding::new("up", MoveUp, Some("ProcessKiller")),
                KeyBinding::new("down", MoveDown, Some("ProcessKiller")),
                KeyBinding::new("escape", Escape, Some("ProcessKiller")),
            ]);

            let options = WindowOptions {
                titlebar: None,
                window_bounds: Some(WindowBounds::centered(size(px(680.), px(560.)), cx)),
                kind: WindowKind::PopUp,
                is_resizable: false,
                is_minimizable: false,
                app_owns_titlebar_drag: true,
                window_background: WindowBackgroundAppearance::Transparent,
                window_decorations: Some(WindowDecorations::Client),
                ..Default::default()
            };
            cx.spawn(async move |cx| {
                cx.open_window(options, |window, cx| {
                    window.set_window_title("Process Killer");
                    let view = cx.new(|cx| AppView::new(window, cx));
                    cx.new(|cx| {
                        gpui_component::Root::new(view, window, cx)
                            .bordered(false)
                            .bg(transparent_black())
                    })
                })
                .expect("failed to open Process Killer window");
            })
            .detach();
        });
}

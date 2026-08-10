#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod settings;
mod system;

use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::{Arc, mpsc},
    time::{Duration, Instant},
};

use gpui::{prelude::FluentBuilder as _, *};
use gpui_component::{
    ActiveTheme, Icon, IconName, Selectable, Sizable, StyledExt, Theme, ThemeMode, WindowExt,
    button::{Button, ButtonVariant, ButtonVariants},
    dialog::DialogButtonProps,
    h_flex,
    input::{Input, InputEvent, InputState},
    scroll::ScrollableElement,
    tooltip::Tooltip,
    v_flex,
};
use settings::{Locale, Settings, ThemePreference};
use smol::Timer;
use system::{
    ProcessInfo, ProcessSource, extract_icon, format_memory, kill_process, matches_filter,
};

actions!(process_killer, [MoveUp, MoveDown, KillSelected, GoBack]);

#[derive(Clone, Copy, Eq, PartialEq)]
enum Page {
    Processes,
    Settings,
}

struct AppView {
    page: Page,
    settings: Settings,
    search: Entity<InputState>,
    processes: Vec<ProcessInfo>,
    source: ProcessSource,
    active: usize,
    scroll: UniformListScrollHandle,
    icons: HashMap<PathBuf, Arc<Image>>,
    requested_icons: HashSet<PathBuf>,
    icon_tx: mpsc::Sender<PathBuf>,
    icon_rx: mpsc::Receiver<(PathBuf, Option<Arc<Image>>)>,
    last_refresh: Instant,
    _subscriptions: Vec<Subscription>,
}

impl AppView {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let settings = Settings::load();
        apply_theme(settings.theme, window, cx);

        let search = cx.new(|cx| {
            InputState::new(window, cx).placeholder(match settings.locale {
                Locale::Zh => "输入进程名/PID，或输入 :端口号",
                Locale::En => "Search name/PID, or type :port",
            })
        });
        window.focus(&search.focus_handle(cx), cx);

        let (icon_tx, worker_rx) = mpsc::channel::<PathBuf>();
        let (worker_tx, icon_rx) = mpsc::channel();
        std::thread::spawn(move || {
            for path in worker_rx {
                let icon = extract_icon(&path);
                if worker_tx.send((path, icon)).is_err() {
                    break;
                }
            }
        });

        let mut view = Self {
            page: Page::Processes,
            settings,
            search: search.clone(),
            processes: Vec::new(),
            source: ProcessSource::new(),
            active: 0,
            scroll: UniformListScrollHandle::new(),
            icons: HashMap::new(),
            requested_icons: HashSet::new(),
            icon_tx,
            icon_rx,
            last_refresh: Instant::now(),
            _subscriptions: Vec::new(),
        };
        view.refresh_processes();

        view._subscriptions.push(cx.subscribe_in(&search, window, {
            let search = search.clone();
            move |this, _, event: &InputEvent, window, cx| match event {
                InputEvent::Change => {
                    this.active = 0;
                    this.scroll.scroll_to_item(0, ScrollStrategy::Top);
                    cx.notify();
                }
                InputEvent::PressEnter { .. } => this.confirm_selected(window, cx),
                _ => {
                    let _ = &search;
                }
            }
        }));
        view._subscriptions
            .push(cx.observe_window_appearance(window, |this, window, cx| {
                if this.settings.theme == ThemePreference::System {
                    Theme::sync_system_appearance(Some(window), cx);
                }
            }));

        cx.spawn(async move |this, cx| {
            loop {
                Timer::after(Duration::from_millis(200)).await;
                if this
                    .update(cx, |this, cx| {
                        let mut changed = false;
                        while let Ok((path, icon)) = this.icon_rx.try_recv() {
                            if let Some(icon) = icon {
                                this.icons.insert(path, icon);
                            }
                            changed = true;
                        }
                        if this.last_refresh.elapsed()
                            >= Duration::from_millis(this.settings.refresh_ms)
                        {
                            this.refresh_processes();
                            changed = true;
                        }
                        if changed {
                            cx.notify();
                        }
                    })
                    .is_err()
                {
                    break;
                }
            }
        })
        .detach();

        view
    }

    fn t(&self, zh: &'static str, en: &'static str) -> &'static str {
        match self.settings.locale {
            Locale::Zh => zh,
            Locale::En => en,
        }
    }

    fn refresh_processes(&mut self) {
        self.processes = self.source.collect();
        self.last_refresh = Instant::now();
        for process in &self.processes {
            if !process.exe.as_os_str().is_empty()
                && self.requested_icons.insert(process.exe.clone())
            {
                let _ = self.icon_tx.send(process.exe.clone());
            }
        }
    }

    fn filtered(&self, cx: &App) -> Vec<ProcessInfo> {
        let query = self.search.read(cx).value();
        self.processes
            .iter()
            .filter(|process| matches_filter(process, query.as_ref()))
            .cloned()
            .collect()
    }

    fn move_active(&mut self, offset: isize, cx: &mut Context<Self>) {
        let len = self.filtered(cx).len();
        if len == 0 {
            self.active = 0;
            return;
        }
        self.active = (self.active as isize + offset).rem_euclid(len as isize) as usize;
        self.scroll
            .scroll_to_item(self.active, ScrollStrategy::Nearest);
        cx.notify();
    }

    fn confirm_selected(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(process) = self.filtered(cx).get(self.active).cloned() {
            self.confirm_kill(process, window, cx);
        }
    }

    fn confirm_kill(&mut self, process: ProcessInfo, window: &mut Window, cx: &mut Context<Self>) {
        let view = cx.entity();
        let title = self.t("结束进程", "Kill Process");
        let description = match self.settings.locale {
            Locale::Zh => format!(
                "确定要结束进程 {} (PID {}) 吗？\n{}",
                process.name,
                process.pid,
                process.exe.display()
            ),
            Locale::En => format!(
                "Kill {} (PID {})?\n{}",
                process.name,
                process.pid,
                process.exe.display()
            ),
        };
        let ok = self.t("结束", "Kill");
        let cancel = self.t("取消", "Cancel");
        window.open_alert_dialog(cx, move |alert, _, _| {
            alert
                .title(title)
                .description(description.clone())
                .button_props(
                    DialogButtonProps::default()
                        .ok_variant(ButtonVariant::Danger)
                        .ok_text(ok)
                        .cancel_text(cancel)
                        .show_cancel(true),
                )
                .on_ok({
                    let view = view.clone();
                    move |_, window, cx| {
                        if kill_process(process.pid) {
                            view.update(cx, |this, cx| {
                                this.refresh_processes();
                                cx.notify();
                            });
                        } else {
                            window.push_notification("Unable to terminate process", cx);
                        }
                        true
                    }
                })
        });
    }

    fn set_page(&mut self, page: Page, window: &mut Window, cx: &mut Context<Self>) {
        self.page = page;
        if page == Page::Processes {
            window.focus(&self.search.focus_handle(cx), cx);
        }
        cx.notify();
    }

    fn update_settings(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.settings.save();
        let placeholder = self.t(
            "输入进程名/PID，或输入 :端口号",
            "Search name/PID, or type :port",
        );
        self.search.update(cx, |input, cx| {
            input.set_placeholder(placeholder, window, cx)
        });
        apply_theme(self.settings.theme, window, cx);
        cx.notify();
    }

    fn render_processes(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let processes = self.filtered(cx);
        let count = processes.len();
        let active = self.active.min(count.saturating_sub(1));
        self.active = active;
        let icons = self.icons.clone();
        let view = cx.entity();

        v_flex()
            .size_full()
            .p_3()
            .gap_2()
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        div()
                            .flex_1()
                            .rounded_xl()
                            .bg(cx.theme().input_background())
                            .px_1()
                            .child(
                                Input::new(&self.search)
                                    .prefix(Icon::new(IconName::Search))
                                    .cleanable(true)
                                    .appearance(false)
                                    .focus_bordered(false),
                            ),
                    )
                    .child(
                        Button::new("settings")
                            .ghost()
                            .icon(IconName::Settings)
                            .tooltip(self.t("设置", "Settings"))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.set_page(Page::Settings, window, cx)
                            })),
                    ),
            )
            .child(
                h_flex()
                    .w_full()
                    .h(px(28.))
                    .px_3()
                    .gap_3()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child(
                        div()
                            .min_w_0()
                            .flex_1()
                            .child(format!("{} ({count})", self.t("进程", "Process"))),
                    )
                    .child(
                        div()
                            .w(px(116.))
                            .text_right()
                            .child(self.t("端口", "Ports")),
                    )
                    .child(div().w(px(64.)).text_right().child("CPU"))
                    .child(
                        div()
                            .w(px(78.))
                            .text_right()
                            .child(self.t("内存", "Memory")),
                    ),
            )
            .child(
                div()
                    .relative()
                    .flex_1()
                    .overflow_hidden()
                    .child(
                        uniform_list("process-list", count, move |range, _, cx| {
                            range
                                .map(|index| {
                                    let process = processes[index].clone();
                                    let selected = index == active;
                                    let icon = icons.get(&process.exe).cloned();
                                    let ports = process.ports.clone();
                                    let extra_ports = ports
                                        .iter()
                                        .skip(2)
                                        .map(|port| format!(":{port}"))
                                        .collect::<Vec<_>>()
                                        .join("  ");
                                    let hover_view = view.clone();
                                    let click_view = view.clone();
                                    let clicked_process = process.clone();
                                    h_flex()
                                        .id(("process", process.pid as usize))
                                        .w_full()
                                        .h(px(50.))
                                        .px_3()
                                        .gap_3()
                                        .rounded_lg()
                                        .cursor_pointer()
                                        .when(selected, |row| row.bg(cx.theme().secondary))
                                        .hover(|row| row.bg(cx.theme().secondary_hover))
                                        .on_hover(move |hovered, _, cx| {
                                            if *hovered {
                                                hover_view.update(cx, |this, cx| {
                                                    this.active = index;
                                                    cx.notify();
                                                });
                                            }
                                        })
                                        .on_click(move |_, window, cx| {
                                            let process = clicked_process.clone();
                                            click_view.update(cx, |this, cx| {
                                                this.confirm_kill(process, window, cx)
                                            });
                                        })
                                        .child(match icon {
                                            Some(icon) => img(icon).size_7().into_any_element(),
                                            None => div()
                                                .size_7()
                                                .flex_shrink_0()
                                                .rounded_md()
                                                .bg(cx.theme().muted)
                                                .items_center()
                                                .justify_center()
                                                .text_xs()
                                                .font_semibold()
                                                .child(
                                                    process
                                                        .name
                                                        .chars()
                                                        .next()
                                                        .unwrap_or('?')
                                                        .to_uppercase()
                                                        .to_string(),
                                                )
                                                .into_any_element(),
                                        })
                                        .child(
                                            h_flex()
                                                .min_w_0()
                                                .flex_1()
                                                .gap_2()
                                                .child(div().truncate().child(process.name.clone()))
                                                .child(
                                                    div()
                                                        .flex_shrink_0()
                                                        .px_1()
                                                        .rounded_sm()
                                                        .bg(cx.theme().muted)
                                                        .text_xs()
                                                        .text_color(cx.theme().muted_foreground)
                                                        .child(process.pid.to_string()),
                                                ),
                                        )
                                        .child(
                                            h_flex()
                                                .w(px(116.))
                                                .justify_end()
                                                .gap_1()
                                                .children(ports.iter().take(2).map(|port| {
                                                    div()
                                                        .text_xs()
                                                        .text_color(cx.theme().muted_foreground)
                                                        .child(format!(":{port}"))
                                                }))
                                                .when(ports.len() > 2, |ports_row| {
                                                    ports_row.child(
                                                        div()
                                                            .id(("ports", process.pid as usize))
                                                            .px_1()
                                                            .rounded_sm()
                                                            .bg(cx.theme().muted)
                                                            .text_xs()
                                                            .child(format!("+{}", ports.len() - 2))
                                                            .tooltip(move |window, cx| {
                                                                Tooltip::new(extra_ports.clone())
                                                                    .build(window, cx)
                                                            }),
                                                    )
                                                }),
                                        )
                                        .child(
                                            div()
                                                .w(px(64.))
                                                .text_right()
                                                .text_sm()
                                                .text_color(if process.cpu_usage > 20.0 {
                                                    cx.theme().danger
                                                } else {
                                                    cx.theme().muted_foreground
                                                })
                                                .child(format!("{:.1}%", process.cpu_usage)),
                                        )
                                        .child(
                                            div()
                                                .w(px(78.))
                                                .text_right()
                                                .text_sm()
                                                .text_color(cx.theme().muted_foreground)
                                                .child(format_memory(process.memory_bytes)),
                                        )
                                })
                                .collect()
                        })
                        .size_full()
                        .track_scroll(&self.scroll),
                    )
                    .vertical_scrollbar(&self.scroll),
            )
            .into_any_element()
    }

    fn render_header(&self, title: &'static str, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .child(
                Button::new("back")
                    .ghost()
                    .icon(IconName::ArrowLeft)
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.set_page(Page::Processes, window, cx)
                    })),
            )
            .child(div().text_lg().font_semibold().child(title))
    }

    fn render_settings(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let locale = self.settings.locale;
        let theme = self.settings.theme;
        let refresh = self.settings.refresh_ms;
        v_flex()
            .size_full()
            .p_4()
            .gap_6()
            .child(self.render_header(self.t("设置", "Settings"), cx))
            .child(
                v_flex()
                    .mx_auto()
                    .w_full()
                    .max_w(px(520.))
                    .gap_5()
                    .child(
                        self.setting_row(
                            self.t("语言", "Language"),
                            h_flex()
                                .gap_1()
                                .child(self.option_button(
                                    "lang-zh",
                                    "中文",
                                    locale == Locale::Zh,
                                    cx,
                                    |this| {
                                        this.settings.locale = Locale::Zh;
                                    },
                                ))
                                .child(self.option_button(
                                    "lang-en",
                                    "English",
                                    locale == Locale::En,
                                    cx,
                                    |this| {
                                        this.settings.locale = Locale::En;
                                    },
                                )),
                        ),
                    )
                    .child(
                        self.setting_row(
                            self.t("主题", "Theme"),
                            h_flex()
                                .gap_1()
                                .child(self.option_button(
                                    "theme-system",
                                    self.t("跟随系统", "System"),
                                    theme == ThemePreference::System,
                                    cx,
                                    |this| this.settings.theme = ThemePreference::System,
                                ))
                                .child(self.option_button(
                                    "theme-light",
                                    self.t("浅色", "Light"),
                                    theme == ThemePreference::Light,
                                    cx,
                                    |this| this.settings.theme = ThemePreference::Light,
                                ))
                                .child(self.option_button(
                                    "theme-dark",
                                    self.t("深色", "Dark"),
                                    theme == ThemePreference::Dark,
                                    cx,
                                    |this| this.settings.theme = ThemePreference::Dark,
                                )),
                        ),
                    )
                    .child(
                        self.setting_row(
                            self.t("自动刷新间隔", "Refresh interval"),
                            h_flex()
                                .gap_1()
                                .children([1_000, 5_000, 10_000, 20_000].map(|ms| {
                                    self.option_button(
                                        format!("refresh-{ms}"),
                                        format!("{}s", ms / 1_000),
                                        refresh == ms,
                                        cx,
                                        move |this| this.settings.refresh_ms = ms,
                                    )
                                })),
                        ),
                    )
                    .child(
                        self.setting_row(
                            self.t("版本", "Version"),
                            div()
                                .text_color(cx.theme().muted_foreground)
                                .child(env!("CARGO_PKG_VERSION")),
                        ),
                    ),
            )
            .into_any_element()
    }

    fn setting_row(&self, label: &'static str, control: impl IntoElement) -> impl IntoElement {
        h_flex()
            .min_h(px(44.))
            .justify_between()
            .gap_4()
            .child(div().font_medium().child(label))
            .child(control)
    }

    fn option_button(
        &self,
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        selected: bool,
        cx: &mut Context<Self>,
        update: impl Fn(&mut Self) + 'static,
    ) -> Button {
        Button::new(id)
            .label(label)
            .small()
            .selected(selected)
            .on_click(cx.listener(move |this, _, window, cx| {
                update(this);
                this.update_settings(window, cx);
            }))
    }
}

impl Render for AppView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dialog_layer = gpui_component::Root::render_dialog_layer(window, cx);
        let page = match self.page {
            Page::Processes => self.render_processes(cx),
            Page::Settings => self.render_settings(cx),
        };
        let title = self.t("进程杀手", "Process Killer");

        div()
            .id("process-killer")
            .key_context("ProcessKiller")
            .track_focus(&self.search.focus_handle(cx))
            .size_full()
            .p_2()
            .on_action(cx.listener(|this, _: &MoveUp, _, cx| this.move_active(-1, cx)))
            .on_action(cx.listener(|this, _: &MoveDown, _, cx| this.move_active(1, cx)))
            .on_action(
                cx.listener(|this, _: &KillSelected, window, cx| this.confirm_selected(window, cx)),
            )
            .on_action(cx.listener(|this, _: &GoBack, window, cx| {
                this.set_page(Page::Processes, window, cx)
            }))
            .child(
                v_flex()
                    .size_full()
                    .rounded_xl()
                    .overflow_hidden()
                    .bg(cx.theme().background)
                    .text_color(cx.theme().foreground)
                    .child(
                        h_flex()
                            .w_full()
                            .h(px(34.))
                            .px_2()
                            .child(
                                h_flex()
                                    .id("window-drag-handle")
                                    .h_full()
                                    .flex_1()
                                    .px_2()
                                    .cursor_move()
                                    .text_xs()
                                    .font_medium()
                                    .text_color(cx.theme().muted_foreground)
                                    .child(title)
                                    .on_mouse_down(MouseButton::Left, |_, window, _| {
                                        window.start_window_move()
                                    }),
                            )
                            .child(
                                Button::new("close")
                                    .ghost()
                                    .xsmall()
                                    .icon(IconName::Close)
                                    .tooltip(self.t("关闭", "Close"))
                                    .on_click(|_, window, _| window.remove_window()),
                            ),
                    )
                    .child(div().min_h_0().flex_1().child(page)),
            )
            .children(dialog_layer)
    }
}

fn apply_theme(preference: ThemePreference, window: &mut Window, cx: &mut App) {
    match preference {
        ThemePreference::System => Theme::sync_system_appearance(Some(window), cx),
        ThemePreference::Light => Theme::change(ThemeMode::Light, Some(window), cx),
        ThemePreference::Dark => Theme::change(ThemeMode::Dark, Some(window), cx),
    }
}

fn main() {
    gpui_platform::application()
        .with_assets(gpui_component_assets::Assets)
        .run(|cx| {
            gpui_component::init(cx);
            cx.bind_keys([
                KeyBinding::new("up", MoveUp, Some("ProcessKiller")),
                KeyBinding::new("down", MoveDown, Some("ProcessKiller")),
                KeyBinding::new("escape", GoBack, Some("ProcessKiller")),
            ]);
            let options = WindowOptions {
                titlebar: None,
                window_bounds: Some(WindowBounds::centered(size(px(680.), px(560.)), cx)),
                kind: WindowKind::PopUp,
                is_resizable: false,
                is_minimizable: false,
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

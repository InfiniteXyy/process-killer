mod process_page;
mod settings_page;
mod title_bar;

use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::{Arc, mpsc},
    time::{Duration, Instant},
};

use gpui::*;
use gpui_component::{
    ActiveTheme, Theme, ThemeMode, WindowExt,
    button::ButtonVariant,
    dialog::DialogButtonProps,
    input::{InputEvent, InputState},
    v_flex,
};
use smol::Timer;

use crate::{
    settings::{Locale, Settings, ThemePreference},
    system::{
        ProcessInfo, ProcessSource, SortColumn, SortDirection, extract_icon, kill_process,
        matches_filter, sort_processes,
    },
};

actions!(process_killer, [MoveUp, MoveDown, KillSelected, Escape]);

#[derive(Clone, Copy, Eq, PartialEq)]
enum Page {
    Processes,
    Settings,
}

pub struct AppView {
    page: Page,
    settings: Settings,
    search: Entity<InputState>,
    processes: Vec<ProcessInfo>,
    source: ProcessSource,
    active: usize,
    sort_column: SortColumn,
    sort_direction: SortDirection,
    scroll: UniformListScrollHandle,
    icons: HashMap<PathBuf, Arc<Image>>,
    requested_icons: HashSet<PathBuf>,
    icon_tx: mpsc::Sender<PathBuf>,
    icon_rx: mpsc::Receiver<(PathBuf, Option<Arc<Image>>)>,
    last_refresh: Instant,
    _subscriptions: Vec<Subscription>,
}

impl AppView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
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
            sort_column: SortColumn::Process,
            sort_direction: SortDirection::Ascending,
            scroll: UniformListScrollHandle::new(),
            icons: HashMap::new(),
            requested_icons: HashSet::new(),
            icon_tx,
            icon_rx,
            last_refresh: Instant::now(),
            _subscriptions: Vec::new(),
        };
        view.refresh_processes();

        view._subscriptions.push(cx.subscribe_in(
            &search,
            window,
            |this, _, event: &InputEvent, window, cx| match event {
                InputEvent::Change => {
                    this.active = 0;
                    this.scroll.scroll_to_item(0, ScrollStrategy::Top);
                    cx.notify();
                }
                InputEvent::PressEnter { .. } => this.confirm_selected(window, cx),
                _ => {}
            },
        ));
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
        let mut processes: Vec<_> = self
            .processes
            .iter()
            .filter(|process| matches_filter(process, query.as_ref()))
            .cloned()
            .collect();
        sort_processes(&mut processes, self.sort_column, self.sort_direction);
        processes
    }

    fn set_sort(&mut self, column: SortColumn, cx: &mut Context<Self>) {
        if self.sort_column == column {
            self.sort_direction = self.sort_direction.reversed();
        } else {
            self.sort_column = column;
            self.sort_direction = SortDirection::Ascending;
        }
        self.active = 0;
        self.scroll.scroll_to_item(0, ScrollStrategy::Top);
        cx.notify();
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

    fn handle_escape(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        match self.page {
            Page::Processes => window.remove_window(),
            Page::Settings => self.set_page(Page::Processes, window, cx),
        }
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
}

impl Render for AppView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dialog_layer = gpui_component::Root::render_dialog_layer(window, cx);
        let page = match self.page {
            Page::Processes => self.render_processes(cx),
            Page::Settings => self.render_settings(cx),
        };

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
            .on_action(cx.listener(|this, _: &Escape, window, cx| this.handle_escape(window, cx)))
            .child(
                v_flex()
                    .size_full()
                    .rounded_xl()
                    .overflow_hidden()
                    .bg(cx.theme().background)
                    .text_color(cx.theme().foreground)
                    .child(title_bar::render(
                        self.t("进程杀手", "Process Killer"),
                        self.t("关闭", "Close"),
                        cx,
                    ))
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

use gpui::{prelude::FluentBuilder as _, *};
use gpui_component::{
    ActiveTheme, Icon, IconName,
    button::{Button, ButtonVariants},
    h_flex,
    input::Input,
    scroll::ScrollableElement,
    tooltip::Tooltip,
    v_flex,
};

use super::{AppView, Page};
use crate::system::{SortColumn, SortDirection, format_memory};

impl AppView {
    pub(super) fn render_processes(
        &mut self,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let processes = self.filtered(cx);
        let count = processes.len();
        let active = self.active.min(count.saturating_sub(1));
        let search_focused = self.search.focus_handle(cx).is_focused(window);
        self.active = active;
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
                    .h(px(30.))
                    .gap_1()
                    .text_xs()
                    .rounded_lg()
                    .bg(cx.theme().table_head)
                    .text_color(cx.theme().table_head_foreground)
                    .child(self.sort_header(
                        "sort-process",
                        format!("{} ({count})", self.t("进程", "Process")),
                        SortColumn::Process,
                        None,
                        cx,
                    ))
                    .child(self.sort_header(
                        "sort-ports",
                        self.t("端口", "Ports"),
                        SortColumn::Ports,
                        Some(px(116.)),
                        cx,
                    ))
                    .child(self.sort_header("sort-cpu", "CPU", SortColumn::Cpu, Some(px(64.)), cx))
                    .child(self.sort_header(
                        "sort-memory",
                        self.t("内存", "Memory"),
                        SortColumn::Memory,
                        Some(px(78.)),
                        cx,
                    )),
            )
            .child(
                div()
                    .relative()
                    .flex_1()
                    .overflow_hidden()
                    .child(
                        uniform_list(
                            "process-list",
                            count,
                            cx.processor(move |this, range: std::ops::Range<usize>, _, cx| {
                                range
                                    .map(|index| {
                                        let process = processes[index].clone();
                                        this.request_icon(&process);
                                        let selected = index == active;
                                        let icon = this
                                            .icons
                                            .get(&process.exe)
                                            .unwrap_or(&this.default_icon)
                                            .clone();
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
                                            .px_2()
                                            .gap_2()
                                            .rounded_lg()
                                            .cursor_pointer()
                                            .when(selected, |row| row.bg(cx.theme().table_active))
                                            .hover(|row| row.bg(cx.theme().table_hover))
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
                                            .child(img(icon).size_6().flex_shrink_0())
                                            .child(
                                                h_flex()
                                                    .min_w_0()
                                                    .flex_1()
                                                    .gap_2()
                                                    .child(
                                                        div()
                                                            .truncate()
                                                            .child(process.name.clone()),
                                                    )
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
                                                                .child(format!(
                                                                    "+{}",
                                                                    ports.len() - 2
                                                                ))
                                                                .tooltip(move |window, cx| {
                                                                    Tooltip::new(
                                                                        extra_ports.clone(),
                                                                    )
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
                            }),
                        )
                        .size_full()
                        .track_scroll(&self.scroll),
                    )
                    .vertical_scrollbar(&self.scroll),
            )
            .into_any_element()
    }

    fn sort_header(
        &self,
        id: &'static str,
        label: impl Into<SharedString>,
        column: SortColumn,
        width: Option<Pixels>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let selected = self.sort_column == column;
        let arrow = match self.sort_direction {
            SortDirection::Ascending => "↑",
            SortDirection::Descending => "↓",
        };
        let header = h_flex()
            .id(id)
            .h_full()
            .px_2()
            .gap_1()
            .rounded_md()
            .cursor_pointer()
            .hover(|header| header.bg(cx.theme().table_hover))
            .when(selected, |header| {
                header
                    .bg(cx.theme().table_active)
                    .text_color(cx.theme().foreground)
            })
            .child(label.into())
            .when(selected, |header| header.child(arrow))
            .on_click(cx.listener(move |this, _, _, cx| this.set_sort(column, cx)));

        match width {
            Some(width) => header.w(width).justify_end().into_any_element(),
            None => header.min_w_0().flex_1().into_any_element(),
        }
    }
}

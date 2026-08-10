use gpui::*;
use gpui_component::{
    ActiveTheme, IconName, Selectable, Sizable, StyledExt,
    button::{Button, ButtonVariants},
    h_flex, v_flex,
};

use super::{AppView, Page};
use crate::settings::{Locale, ThemePreference};

impl AppView {
    pub(super) fn render_settings(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let locale = self.settings.locale;
        let theme = self.settings.theme;
        let refresh = self.settings.refresh_ms;

        v_flex()
            .size_full()
            .p_4()
            .gap_4()
            .child(self.render_settings_header(cx))
            .child(
                v_flex()
                    .mx_auto()
                    .w_full()
                    .max_w(px(540.))
                    .gap_4()
                    .child(
                        self.settings_card(
                            self.t("外观与语言", "Appearance & language"),
                            self.t(
                                "选择应用显示语言和颜色主题。",
                                "Choose the display language and color theme.",
                            ),
                            v_flex()
                                .gap_3()
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
                                                |this| this.settings.locale = Locale::Zh,
                                            ))
                                            .child(self.option_button(
                                                "lang-en",
                                                "English",
                                                locale == Locale::En,
                                                cx,
                                                |this| this.settings.locale = Locale::En,
                                            )),
                                    ),
                                )
                                .child(div().h(px(1.)).bg(cx.theme().border))
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
                                                |this| {
                                                    this.settings.theme = ThemePreference::System
                                                },
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
                                ),
                            cx,
                        ),
                    )
                    .child(
                        self.settings_card(
                            self.t("进程列表", "Process list"),
                            self.t(
                                "控制系统信息的自动更新频率。",
                                "Control how often system information is refreshed.",
                            ),
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
                            cx,
                        ),
                    )
                    .child(
                        h_flex()
                            .justify_between()
                            .px_1()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child(self.t("Process Killer", "Process Killer"))
                            .child(format!("v{}", env!("CARGO_PKG_VERSION"))),
                    ),
            )
            .into_any_element()
    }

    fn render_settings_header(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_3()
            .child(
                Button::new("back")
                    .ghost()
                    .icon(IconName::ArrowLeft)
                    .tooltip(self.t("返回", "Back"))
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.set_page(Page::Processes, window, cx)
                    })),
            )
            .child(
                v_flex()
                    .gap_1()
                    .child(
                        div()
                            .text_xl()
                            .font_semibold()
                            .child(self.t("设置", "Settings")),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .child(self.t(
                                "按你的习惯调整 Process Killer",
                                "Make Process Killer work your way",
                            )),
                    ),
            )
    }

    fn settings_card(
        &self,
        title: &'static str,
        description: &'static str,
        content: impl IntoElement,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        v_flex()
            .gap_4()
            .p_4()
            .rounded_xl()
            .border_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().secondary.opacity(0.35))
            .child(
                v_flex()
                    .gap_1()
                    .child(div().font_semibold().child(title))
                    .child(
                        div()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child(description),
                    ),
            )
            .child(content)
            .into_any_element()
    }

    fn setting_row(&self, label: &'static str, control: impl IntoElement) -> AnyElement {
        h_flex()
            .min_h(px(36.))
            .justify_between()
            .gap_4()
            .child(div().font_medium().child(label))
            .child(control)
            .into_any_element()
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

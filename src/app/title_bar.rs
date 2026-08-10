use gpui::*;
use gpui_component::{
    ActiveTheme, IconName, Sizable, StyledExt,
    button::{Button, ButtonVariants},
    h_flex,
};

pub(super) fn render(
    title: &'static str,
    close_tooltip: &'static str,
    cx: &App,
) -> impl IntoElement {
    h_flex()
        .w_full()
        .h(px(34.))
        .px_2()
        .child(
            h_flex()
                .id("window-drag-handle")
                .window_control_area(WindowControlArea::Drag)
                .h_full()
                .flex_1()
                .px_2()
                .cursor_move()
                .text_xs()
                .font_medium()
                .text_color(cx.theme().muted_foreground)
                .child(title)
                .on_mouse_down(MouseButton::Left, |_, window, _| window.start_window_move()),
        )
        .child(
            Button::new("close")
                .ghost()
                .xsmall()
                .icon(IconName::Close)
                .tooltip(close_tooltip)
                .on_click(|_, window, _| window.remove_window()),
        )
}

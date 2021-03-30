use crate::custom_widget::CustomWidget;
use crate::delegate::Delegate;
use druid::widget::{Flex, Label, Painter};
use druid::{
    commands, AppLauncher, Color, Command, Env, EventCtx, FileDialogOptions, FileSpec,
    LocalizedString, RenderContext, Target, Widget, WidgetExt, WindowDesc,
};
use druid_image_viewer::*;

fn label_button(
    text: &'static str,
    f: impl Fn(&mut EventCtx, &mut AppState, &Env) + 'static,
) -> impl Widget<AppState> {
    let painter = Painter::new(|ctx, _, _env| {
        let bounds = ctx.size().to_rect();
        ctx.fill(bounds, &Color::rgb8(155, 183, 214));
        if ctx.is_active() {
            ctx.fill(bounds, &Color::rgb8(0x71, 0x71, 0x71));
        }
    });

    Label::new(LocalizedString::new(text))
        .padding(2.)
        .background(painter)
        .on_click(f)
}

fn ui_builder() -> impl Widget<AppState> {
    let open_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![FileSpec::JPG, FileSpec::PNG])
        .name_label("Target")
        .title("Choose a target for open")
        .button_text("Open");

    Flex::column()
        .with_child(
            Flex::row()
                .with_child(label_button("open-button", move |ctx, _, _| {
                    ctx.submit_command(Command::new(
                        commands::SHOW_OPEN_PANEL,
                        open_dialog_options.clone(),
                        Target::Auto,
                    ));
                }))
                .with_flex_spacer(1.0)
                .with_child(label_button("grey-button", |ctx, _, _| {
                    ctx.submit_command(Command::new(SET_IMAGE_GRAY, (), Target::Auto))
                }))
                .fix_height(25.),
        )
        .with_child(CustomWidget {})
}

fn main() {
    let window = WindowDesc::new(ui_builder())
        .window_size(WINDOW_SIZE.to_size())
        .set_position(screen_center())
        .title("Image Viewer");

    AppLauncher::with_window(window)
        .delegate(Delegate)
        .launch(AppState::default())
        .expect("Failed to launch application")
}

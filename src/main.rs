use crate::custom_widget::CustomWidget;
use crate::delegate::Delegate;
use druid::widget::{Button, Flex};
use druid::{
    commands, AppLauncher, Command, FileDialogOptions, FileSpec, Target, Widget, WidgetExt,
    WindowDesc,
};
use druid_image_viewer::*;

fn ui_builder() -> impl Widget<AppState> {
    let open_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![FileSpec::JPG, FileSpec::PNG])
        .name_label("Target")
        .title("Choose a target for open")
        .button_text("Open");

    Flex::column()
        .with_child(
            Flex::row()
                .with_child(Button::new("Open").padding(5.).on_click(move |ctx, _, _| {
                    ctx.submit_command(Command::new(
                        commands::SHOW_OPEN_PANEL,
                        open_dialog_options.clone(),
                        Target::Auto,
                    ));
                }))
                .with_flex_spacer(1.0)
                .with_child(Button::new("Gray").padding(5.).on_click(move |ctx, _, _| {
                    ctx.submit_command(Command::new(SET_IMAGE_GRAY, (), Target::Auto))
                }))
                .fix_height(35.),
        )
        .with_child(CustomWidget {})
}

fn main() {
    let window = WindowDesc::new(ui_builder())
        .window_size(WINDOW_WIDTH.to_size())
        .set_position(screen_center())
        .title("Image Viewer");

    AppLauncher::with_window(window)
        .log_to_console()
        .delegate(Delegate)
        .launch(AppState::default())
        .expect("Failed to launch application")
}

use std::borrow::BorrowMut;

use crate::{AppState, SET_IMAGE_GRAY};
use druid::piet::Image;
use druid::{commands, AppDelegate, Color, Command, DelegateCtx, Env, Handled, ImageBuf, Target};

pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            data.gray = false;
            data.buffer = ImageBuf::from_file(file_info.path()).ok();
            return Handled::Yes;
        }

        if let Some(_) = cmd.get(SET_IMAGE_GRAY) {
            data.gray = true;
            if let Some(buffer) = &data.buffer {
                let pixels: Vec<Color> = buffer.pixel_colors().flatten().collect();
                // 将RGB三通道改成一通道灰度图
                let pixels: Vec<u8> = pixels.into_iter().map(|c| c.as_rgba8().1).collect();

                data.buffer = Some(ImageBuf::from_raw(
                    pixels,
                    druid::piet::ImageFormat::Grayscale,
                    buffer.width(),
                    buffer.height(),
                ));
            }
            return Handled::Yes;
        }
        Handled::No
    }

    fn event(
        &mut self,
        _ctx: &mut DelegateCtx,
        _window_id: druid::WindowId,
        event: druid::Event,
        _data: &mut AppState,
        _env: &Env,
    ) -> Option<druid::Event> {
        Some(event)
    }

    fn window_added(
        &mut self,
        _id: druid::WindowId,
        _data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
    }

    fn window_removed(
        &mut self,
        _id: druid::WindowId,
        _data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
    }
}

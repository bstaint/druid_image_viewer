use crate::{AppState, SET_IMAGE_GRAY};
use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, ImageBuf, Target, commands};

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
            data.paint_data = ImageBuf::from_file(file_info.path()).ok();
            return Handled::Yes;
        }

        if let Some(_) = cmd.get(SET_IMAGE_GRAY) {
            data.gray = true;
            if let Some(_) = &data.paint_data {
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

    fn window_added(&mut self, _id: druid::WindowId, _data: &mut AppState, _env: &Env, _ctx: &mut DelegateCtx) {}

    fn window_removed(&mut self, _id: druid::WindowId, _data: &mut AppState, _env: &Env, _ctx: &mut DelegateCtx) {}
}

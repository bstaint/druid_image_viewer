use crate::{AppState, SET_IMAGE_GRAY};
use druid::{commands, AppDelegate, Command, DelegateCtx, Env, Handled, ImageBuf, Target};

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
            data.image = ImageBuf::from_file(file_info.path()).ok();
            return Handled::Yes;
        }

        if let Some(_) = cmd.get(SET_IMAGE_GRAY) {
            data.gray = true;
            return Handled::Yes;
        }
        Handled::No
    }
}

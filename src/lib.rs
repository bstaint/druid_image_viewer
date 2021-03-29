use druid::widget::prelude::*;
use druid::{ImageBuf, Lens, Point, Screen, Selector, Vec2};

pub mod custom_widget;
pub mod delegate;

pub const SET_IMAGE_GRAY: Selector<()> = Selector::new("custom-set-image-gray");
pub const WINDOW_WIDTH: Vec2 = Vec2 { x: 600.0, y: 400.0 };

#[derive(Clone, Default, Data, Lens)]
pub struct AppState {
    pub buffer: Option<ImageBuf>,
}

#[inline]
pub fn screen_center() -> Point {
    let screen_vec = Screen::get_display_rect().center().to_vec2();
    (screen_vec - (WINDOW_WIDTH * 0.5)).to_point()
}

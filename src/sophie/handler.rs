use super::{KeydownData, Sophie};

pub enum SophieEventResult<E> {
    Success,
    Error(E),
    Exit,
}

pub trait SophieHandler<E> {
    fn update(&mut self, dt: std::time::Duration, sophie: &mut Sophie) -> SophieEventResult<E>;
    fn resize(&mut self, _sophie: &mut Sophie, _x: i32, _y: i32) -> SophieEventResult<E> {
        SophieEventResult::Success
    }
    fn mouse_enter(&mut self, _sophie: &mut Sophie) -> SophieEventResult<E> {
        SophieEventResult::Success
    }
    fn on_keyup(&mut self, _sophie: &mut Sophie, _key: KeydownData) -> SophieEventResult<E> {
        SophieEventResult::Success
    }
    fn on_keydown(&mut self, _sophie: &mut Sophie, _key: KeydownData) -> SophieEventResult<E> {
        SophieEventResult::Success
    }
    fn on_keypress(&mut self, _sophie: &mut Sophie) -> SophieEventResult<E> {
        SophieEventResult::Success
    }
    fn before_exit(&mut self, _sophie: &mut Sophie, _default_request: bool, _cancel: &mut bool) {}
    fn fallback_error(&mut self, err: E, sophie: &mut Sophie);
}

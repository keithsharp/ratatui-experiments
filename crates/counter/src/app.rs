#[derive(Debug, Default)]
pub struct AppState {
    counter: i64,
    should_quit: bool,
}

impl AppState {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub fn counter(&self) -> i64 {
        self.counter
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}

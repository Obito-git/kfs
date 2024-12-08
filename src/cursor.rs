#[derive(Clone, Copy)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

impl Cursor {
    pub(crate) fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub(crate) fn move_right(&mut self, max_width: &usize) {
        if self.x < max_width - 1 {
            self.x += 1;
        }
    }

    pub(crate) fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    pub(crate) fn move_down(&mut self, max_height: &usize) {
        if self.y < max_height - 1 {
            self.y += 1;
        }
    }
}
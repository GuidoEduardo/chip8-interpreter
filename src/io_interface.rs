pub const KEY_COUNT: usize = 16;
pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

#[derive(Debug)]
pub struct IOInterface {
    pub display_buffer: [u32; (WIDTH * HEIGHT)],
    pub keypad: [u8; KEY_COUNT],
}

impl IOInterface {
    pub fn new() -> Self {
        Self {
            display_buffer: [0; (WIDTH * HEIGHT)],
            keypad: [0; KEY_COUNT],
        }
    }

    pub fn clear(&mut self) {
        self.display_buffer = [0; (WIDTH * HEIGHT)];
    }
}
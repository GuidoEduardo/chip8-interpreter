const WIDTH: usize = 64;
const HEIGHT: usize = 32;

struct Platform {
    display: [[u8; WIDTH]; HEIGHT],
}

impl Platform {
    fn new(&mut self) -> Self {
        Self {
            display: [[0; WIDTH]; HEIGHT]
        }
    }

    pub fn clear(&mut self) {
        self.display = [[0; WIDTH]; HEIGHT];
    }
}
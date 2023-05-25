/// TODO make this thread-based like [`rand::thread_rng()`]
pub struct Rng {
    buf: u16,
    buf_remaining: u8,
}
impl Rng {
    pub fn new() -> Self {
        Self {
            buf: 0,
            buf_remaining: 0,
        }
    }
    pub fn throw_dice(&mut self) -> u8 {
        const VALS_PER_GEN: u8 = 6;

        if self.buf_remaining > 0 {
            self.buf /= 6;
            self.buf_remaining -= 1;
            return (self.buf % 6 + 1) as u8;
        }

        let r = fastrand::u16(..);
        let ret = r % 6 + 1;
        self.buf = r;
        self.buf_remaining = VALS_PER_GEN - 1;

        ret as u8
    }
}

pub struct CPU {
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            // ┌─┬─┬──┬─┬─┬─┬─┐
            // │N│V│ss│D│I│Z│C│
            // └─┴─┴──┴─┴─┴─┴─┘
            status: 0,
            program_counter: 0,
        }
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    println!("Hello, world!");
}

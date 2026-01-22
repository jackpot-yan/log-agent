use common::event::Event;
use common::error::Result;

pub trait Output {
    fn emit(&mut self, event: Event) -> Result<()>;
}

pub struct Console;

impl Console {
    pub fn new() -> Self {
        Console
    }
}

impl Output for Console {
    fn emit(&mut self, event: Event) -> Result<()> {
        let payload = String::from_utf8_lossy(&event.payload);
        println!("{}", payload.trim());
        Ok(())
    }
}

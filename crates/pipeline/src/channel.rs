use crossbeam_channel::bounded;
use crossbeam_channel::Sender;
use crossbeam_channel::Receiver;
use common::event::Event;

#[derive(Debug, Clone)]
pub struct Channel {
    pub s: Sender<Event>,
    pub r: Receiver<Event>
}

impl Channel {
    pub fn new() -> Self {
        let (s, r) = bounded(5);
        Channel {  
            s,
            r,
        }
    }

    pub fn send(&self, event:Event) {
        self.s.send(event).unwrap();
    }
}

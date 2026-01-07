use std::time::{SystemTime, UNIX_EPOCH};

pub struct Event {
    pub timestamp: u64,
    pub source: String,
    pub payload: Vec<u8>, // 事件的原始负载
}

impl Event {
    pub fn new(source: String, payload: Vec<u8>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos() as u64;
        Event {
            timestamp,
            source,
            payload,
        }
    }
}

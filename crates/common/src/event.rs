use crate::error::Result;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Event {
    pub timestamp: u64,
    pub source: String,
    pub payload: Vec<u8>, // 事件的原始负载
}

impl Event {
    pub fn new(source: String, payload: Vec<u8>) -> Result<Self> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_nanos() as u64;
        Ok(Event {
            timestamp,
            source,
            payload,
        })
    }
}

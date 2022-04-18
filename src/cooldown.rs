use std::collections::HashMap;
use std::ops::Sub;
use std::time::{Duration, Instant};

pub struct CoolDown {
    pub available: HashMap<u8, Duration>,
    pub current: u8,
    pub ready_in: u128,
    pub completion_ratio: f32,
    pub last_used: Instant,
}

impl Default for CoolDown {
    fn default() -> Self {
        Self {
            available: HashMap::new(),
            current: 0,
            ready_in: 0,
            completion_ratio: 0.0,
            last_used: Instant::now().sub(Duration::from_secs(10000)),
        }
    }
}
use resol_vbus::chrono::prelude::*;


pub struct TickSource {
    interval: i64,
    last_interval: i64,
}


impl TickSource {
    pub fn new(interval: i64, now: DateTime<UTC>) -> TickSource {
        TickSource {
            interval,
            last_interval: now.timestamp() / interval,
        }
    }

    pub fn process(&mut self, now: DateTime<UTC>) -> bool {
        let current_interval = now.timestamp() / self.interval;

        let diff = current_interval - self.last_interval;

        let ticked = (diff > 0) || (diff < -1);
        if ticked {
            self.last_interval = current_interval;
        }

        ticked
    }
}

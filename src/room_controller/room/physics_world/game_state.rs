//This is the timer file.
//This is where the timer for each player's hit is calculated.
//This is used to determine hit "quality".


use std::time::{Duration, Instant};

pub struct Timer {
    start: Instant,
    duration: Duration,
}

impl Timer {
    pub fn new(duration_secs: u64) -> Self  {
         Timer {
            start: Instant::now(),
            duration: Duration::from_secs(duration_secs),
        }
        
    }

    pub fn start_timer(duration_secs: u64) -> Self {
        Timer {
            start: Instant::now(),
            duration: Duration::from_secs(duration_secs),
        }
    }

    pub fn remaining(&self) -> Duration {
        let elapsed = self.start.elapsed();
        if elapsed >= self.duration {
            Duration::from_secs(0)
        } else {
            self.duration - elapsed
        }
    }

    pub fn is_done(&self) -> bool {
        self.start.elapsed() >= self.duration
    }

    pub fn timer_value(&self) -> Duration {
        self.start.elapsed()
    }
}



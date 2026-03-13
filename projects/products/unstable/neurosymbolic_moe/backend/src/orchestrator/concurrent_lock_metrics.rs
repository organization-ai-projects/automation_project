#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ConcurrentLockMetrics {
    pub read_lock_acquisitions: u64,
    pub write_lock_acquisitions: u64,
    pub read_lock_contention: u64,
    pub write_lock_contention: u64,
    pub read_lock_timeouts: u64,
    pub write_lock_timeouts: u64,
    pub read_lock_spin_attempts_total: u64,
    pub write_lock_spin_attempts_total: u64,
}

impl ConcurrentLockMetrics {
    pub fn total_lock_acquisitions(self) -> u64 {
        self.read_lock_acquisitions + self.write_lock_acquisitions
    }

    pub fn total_contention_events(self) -> u64 {
        self.read_lock_contention + self.write_lock_contention
    }

    pub fn total_timeout_events(self) -> u64 {
        self.read_lock_timeouts + self.write_lock_timeouts
    }

    pub fn contention_rate(self) -> f64 {
        let acquisitions = self.total_lock_acquisitions();
        if acquisitions == 0 {
            return 0.0;
        }
        self.total_contention_events() as f64 / acquisitions as f64
    }

    pub fn timeout_rate(self) -> f64 {
        let acquisitions = self.total_lock_acquisitions();
        if acquisitions == 0 {
            return 0.0;
        }
        self.total_timeout_events() as f64 / acquisitions as f64
    }

    pub fn avg_read_spin_attempts(self) -> f64 {
        if self.read_lock_acquisitions == 0 {
            return 0.0;
        }
        self.read_lock_spin_attempts_total as f64 / self.read_lock_acquisitions as f64
    }

    pub fn avg_write_spin_attempts(self) -> f64 {
        if self.write_lock_acquisitions == 0 {
            return 0.0;
        }
        self.write_lock_spin_attempts_total as f64 / self.write_lock_acquisitions as f64
    }
}

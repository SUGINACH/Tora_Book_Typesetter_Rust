use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Default)]
pub struct JobGate {
    minimum_generation: AtomicU64,
    last_accepted_generation: AtomicU64,
}

impl JobGate {
    pub fn accept(&self, generation: u64) -> bool {
        if generation < self.minimum_generation.load(Ordering::SeqCst) {
            return false;
        }
        let mut last = self.last_accepted_generation.load(Ordering::SeqCst);
        loop {
            if generation <= last {
                return false;
            }
            match self.last_accepted_generation.compare_exchange(
                last,
                generation,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => return generation >= self.minimum_generation.load(Ordering::SeqCst),
                Err(observed) => last = observed,
            }
        }
    }

    pub fn cancel(&self, generation: u64) -> bool {
        if self.last_accepted_generation.load(Ordering::SeqCst) != generation {
            return false;
        }
        let Some(next_generation) = generation.checked_add(1) else {
            return false;
        };
        let mut minimum = self.minimum_generation.load(Ordering::SeqCst);
        loop {
            if minimum >= next_generation {
                return false;
            }
            match self.minimum_generation.compare_exchange(
                minimum,
                next_generation,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => return true,
                Err(observed) => minimum = observed,
            }
        }
    }

    pub fn current_generation(&self) -> u64 {
        self.last_accepted_generation.load(Ordering::SeqCst)
    }

    pub fn is_current(&self, generation: u64) -> bool {
        self.current_generation() == generation
            && generation >= self.minimum_generation.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::JobGate;

    #[test]
    fn rejects_stale_jobs_after_newer_generation_is_accepted() {
        let gate = JobGate::default();
        assert!(gate.accept(1));
        assert!(gate.accept(2));
        assert!(!gate.accept(1));
        assert!(!gate.accept(2));
        assert_eq!(gate.current_generation(), 2);
    }

    #[test]
    fn cancelling_an_old_generation_does_not_cancel_a_newer_job() {
        let gate = JobGate::default();
        assert!(gate.accept(7));
        assert!(gate.accept(8));
        assert!(!gate.cancel(7));
        assert!(gate.is_current(8));
    }

    #[test]
    fn cancellation_invalidates_the_cancelled_generation() {
        let gate = JobGate::default();
        assert!(gate.accept(7));
        assert!(gate.cancel(7));
        assert!(!gate.cancel(7));
        assert!(!gate.accept(7));
        assert!(gate.accept(8));
        assert!(!gate.accept(8));
        assert!(gate.is_current(8));
        assert!(!gate.is_current(7));
        assert_eq!(gate.current_generation(), 8);
    }
}

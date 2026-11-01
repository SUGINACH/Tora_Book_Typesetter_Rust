use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Default)]
pub struct JobGate {
    current_generation: AtomicU64,
}

impl JobGate {
    pub fn accept(&self, generation: u64) -> bool {
        let mut current = self.current_generation.load(Ordering::SeqCst);
        loop {
            if generation <= current {
                return false;
            }
            match self.current_generation.compare_exchange(
                current,
                generation,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => return true,
                Err(observed) => current = observed,
            }
        }
    }

    pub fn cancel(&self, generation: u64) -> bool {
        let Some(next_generation) = generation.checked_add(1) else {
            return false;
        };
        self.current_generation
            .compare_exchange(
                generation,
                next_generation,
                Ordering::SeqCst,
                Ordering::SeqCst,
            )
            .is_ok()
    }

    pub fn current_generation(&self) -> u64 {
        self.current_generation.load(Ordering::SeqCst)
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
    fn cancellation_invalidates_the_cancelled_generation() {
        let gate = JobGate::default();
        assert!(gate.accept(7));
        assert!(gate.cancel(7));
        assert!(!gate.cancel(7));
        assert!(!gate.accept(7));
        assert!(gate.accept(8));
        assert_eq!(gate.current_generation(), 8);
    }
}

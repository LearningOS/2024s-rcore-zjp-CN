use alloc::{vec, vec::Vec};
use core::ops::{Deref, DerefMut};

/// detect deadlock
#[derive(Debug)]
pub struct DeadlockDetect {
    available: Resources,
    allocation: ThreadsResources,
    need: ThreadsResources,
}

impl DeadlockDetect {
    /// empty
    pub fn new() -> Self {
        DeadlockDetect {
            available: Resources(Vec::new()),
            allocation: ThreadsResources(Vec::new()),
            need: ThreadsResources(Vec::new()),
        }
    }

    fn resources_len(&self) -> usize {
        self.available.len()
    }

    fn threads_len(&self) -> usize {
        self.allocation.len()
    }

    /// called when a new thread starts
    pub fn push_thread_id(&mut self, tid: usize) {
        assert_eq!(self.threads_len(), tid, "skip tid is not supported");
        let resources_len = self.resources_len();
        self.allocation.0.push(Resources(vec![0; resources_len]));
        self.need.0.push(Resources(vec![0; resources_len]));
    }

    /// called when a new kind of resource inits
    pub fn push_resource_id(&mut self, rid: usize, amount: u32) {
        assert_eq!(
            self.resources_len(),
            rid,
            "skip resource id is not supported"
        );
        self.available.0.push(amount);
        self.allocation.iter_mut().for_each(|r| r.0.push(0));
        self.need.iter_mut().for_each(|r| r.0.push(0));
    }

    /// ask for a resource from a thread:
    /// the check/allocation hasn't started, becasue a blocking thread can just request a need
    pub fn request_one(&mut self, tid: usize, rid: usize) {
        self.need[tid][rid] += 1;
    }

    /// detect: true for dead lock
    pub fn is_deadlock(&mut self, rid: usize) -> bool {
        let resources_len = self.resources_len();
        assert!(
            resources_len > rid,
            "rid{rid} should be less than resources_len {resources_len}"
        );
        let mut available = self.available[rid];
        for tid in 0..self.threads_len() {
            let need = self.need[tid][rid];
            if need > available {
                error!("[deadlock] tid{tid} rid{rid}: need {need} but available {available}");
                // dead lock
                return true;
            }
            available -= need;
            // allocate
            // let allocation = self.allocation[tid][rid];
            // var_name += need;
        }
        false
    }

    /// try allocating:
    /// * true for successful allocation due to no dead lock
    /// * false for no allocation due to dead lock
    pub fn try_allocate(&mut self, tid: usize, rid: usize) -> bool {
        if self.is_deadlock(rid) {
            return false;
        }
        if self.need[tid][rid] > 0 {
            self.need[tid][rid] -= 1;
            self.available[rid] -= 1;
            self.allocation[tid][rid] += 1;
        }
        true
    }

    /// a resource is deallocated and back to available
    pub fn deallocate_one(&mut self, tid: usize, rid: usize) {
        // assume: a lock is matched with an unlock
        if self.allocation[tid][rid] > 0 {
            self.allocation[tid][rid] -= 1;
            self.available[rid] += 1;
            return;
        }
        error!("[detect] allocation for tid{tid} rid{rid} is zero, but still try to deallocate");
    }
}

#[derive(Debug)]
struct Resources(Vec<u32>);

impl Deref for Resources {
    type Target = [u32];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Resources {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// x[i][j] where i is for thread, j is for resource
#[derive(Debug)]
struct ThreadsResources(Vec<Resources>);

impl Deref for ThreadsResources {
    type Target = [Resources];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ThreadsResources {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

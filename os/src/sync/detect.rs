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
        let threads_len = self.threads_len();
        assert_eq!(threads_len, tid, "tid{tid} skips threads_len {threads_len}");
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

    /// detect: false for dead lock
    pub fn is_safe(&mut self, rid: usize) -> bool {
        let resources_len = self.resources_len();
        assert!(
            resources_len > rid,
            "rid{rid} should be less than resources_len {resources_len}"
        );
        let threads_len = self.threads_len();
        let mut finish = vec![false; threads_len];

        fn detect_safe(
            work: &mut u32,
            finish: &mut [bool],
            need: &mut [u32],
            allocation: &mut [u32],
        ) -> bool {
            for tid in 0..finish.len() {
                if !finish[tid] && need[tid] <= *work {
                    finish[tid] = true;
                    let mut new_allocation = allocation.to_vec();
                    let mut new_need = need.to_vec();
                    let mut new_work = *work + new_allocation[tid];
                    // if need[tid] > 0 {
                    //     new_allocation[tid] += 1;
                    //     new_need[tid] -= 1;
                    //     new_work -= 1;
                    // }
                    let mut new_finish = finish.to_vec();
                    if detect_safe(
                        &mut new_work,
                        &mut new_finish,
                        &mut new_need,
                        &mut new_allocation,
                    ) {
                        return true;
                    }
                }
            }
            finish.iter().all(|&f| f)
        }
        let mut work = self.available[rid];
        let mut need: Vec<_> = self.need.iter().map(|thread| thread[rid]).collect();
        let mut allocation: Vec<_> = self.allocation.iter().map(|thread| thread[rid]).collect();
        detect_safe(&mut work, &mut finish, &mut need, &mut allocation)
    }

    /// try allocating:
    /// * true for successful allocation due to no dead lock
    /// * false for no allocation due to dead lock
    pub fn try_allocate(&mut self, tid: usize, rid: usize) -> bool {
        println!(
            "\u{1B}[93m[detect] tid={} rid={} available={} allocation={} need={} \
             all-allocations={:?}\
             \u{1B}[0m",
            tid,
            rid,
            self.available[rid],
            self.allocation[tid][rid],
            self.need[tid][rid],
            self.allocation
                .iter()
                .map(|thread| thread[rid])
                .collect::<Vec<_>>()
        );
        if !self.is_safe(rid) {
            return false;
        }
        if self.need[tid][rid] > 0 {
            if self.available[rid] > 0 {
                self.need[tid][rid] -= 1;
                self.available[rid] -= 1;
                self.allocation[tid][rid] += 1;
            } else {
                error!("[try_allocate] ??? should be detected as deadlock before this line");
                return false;
            }
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

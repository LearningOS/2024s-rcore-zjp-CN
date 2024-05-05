use crate::config::{MAX_APP_NUM, MAX_SYSCALL_NUM};

type SyscallCounts = [u32; MAX_SYSCALL_NUM];

/// Counting for each syscall.
struct Counts {
    calls: SyscallCounts,
}

struct AppSyscallCounts {
    apps: [Counts; MAX_APP_NUM],
}

impl AppSyscallCounts {
    const fn new() -> Self {
        const ZERO: Counts = Counts {
            calls: [0; MAX_SYSCALL_NUM],
        };
        AppSyscallCounts {
            apps: [ZERO; MAX_APP_NUM],
        }
    }

    fn get(&self, app: usize, syscall: &mut SyscallCounts) {
        syscall.copy_from_slice(&self.apps[app].calls);
    }

    fn update(&mut self, app: usize, syscall_id: usize) {
        self.apps[app].calls[syscall_id] += 1;
    }
}

static mut APPS_SYSCALL_COUNTS: AppSyscallCounts = AppSyscallCounts::new();

/// Get syscall counts for an app.
pub fn get(app: usize, syscall: &mut SyscallCounts) {
    unsafe { APPS_SYSCALL_COUNTS.get(app, syscall) };
}

/// Increment a syscall count for an app by 1.
pub fn update(app: usize, syscall_id: usize) {
    // info!("[kernel] app_id: {app}, syscall_id: {syscall_id}: count+1");
    unsafe { APPS_SYSCALL_COUNTS.update(app, syscall_id) };
}

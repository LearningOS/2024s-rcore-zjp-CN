//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        current_task_start_time, exit_current_and_run_next, suspend_current_and_run_next,
        TaskStatus,
    },
    timer::get_time_us,
};

/// Time duration since qemu starts.
#[repr(C)]
#[derive(Debug, Default)]
pub struct TimeVal {
    /// seconds
    pub sec: usize,
    /// microseconds
    pub usec: usize,
}

impl TimeVal {
    /// Current time
    pub fn now() -> Self {
        let us = get_time_us();
        TimeVal::from_us(us)
    }

    fn from_us(us: usize) -> Self {
        TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        }
    }
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    unsafe {
        *ts = TimeVal::now();
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    unsafe {
        let info = ti.as_mut().unwrap();
        // get_time in user_lib is in ms
        let (task, time) = current_task_start_time();
        info.time = time
            .map(|start| (get_time_us() - start) / 1000)
            .unwrap_or(0);
        super::counts::get(task, &mut info.syscall_times);
        info.status = TaskStatus::Running;
    }
    0
}

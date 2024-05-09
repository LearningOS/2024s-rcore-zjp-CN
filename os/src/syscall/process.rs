//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    mm::translated_byte_type_mut,
    task::{
        change_program_brk, current_task_start_time, current_task_syscall_count,
        current_user_token, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus,
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
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    translated_byte_type_mut(current_user_token(), ts, |ts| *ts = TimeVal::now());
    -1
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    translated_byte_type_mut(current_user_token(), ti, |info| {
        // get_time in user_lib is in ms
        let time = current_task_start_time();
        info.time = time
            .map(|start| (get_time_us() - start) / 1000)
            .unwrap_or(0);

        current_task_syscall_count(&mut info.syscall_times);
        info.status = TaskStatus::Running;
    });
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    -1
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    -1
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}

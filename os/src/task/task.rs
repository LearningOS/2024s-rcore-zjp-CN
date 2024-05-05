//! Types related to task management

use super::TaskContext;
use crate::timer::get_time_us;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The first time (us) of being scheduled
    pub time: Option<usize>,
    /// The task context
    pub task_cx: TaskContext,
}

impl TaskControlBlock {
    /// Set the first time the task is running.
    pub fn set_time_start(&mut self) {
        if self.time.is_none() {
            self.time = Some(get_time_us());
        }
    }
}

/// The status of a task
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

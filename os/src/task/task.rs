//! Types related to task management

use super::TaskContext;
use crate::config::{MAX_APP_NUM, MAX_SYSCALL_NUM};
/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// 任务状态
    pub task_status: TaskStatus,
    /// 任务上下文
    pub task_cx: TaskContext,
    /// 系统调用次数
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// 启动时间
    pub start_time: usize
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
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

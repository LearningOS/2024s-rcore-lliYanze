//! Process management syscalls
// use core::borrow::BorrowMut;
// use core::{borrow::BorrowMut, fmt::Debug};
use core::fmt::Debug;

use crate::{
    config::MAX_SYSCALL_NUM,
    task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, TASK_MANAGER},
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
/// Time value
///
/// 示例代码中的 TimeVal 中结构体定义了一个秒数和一个微秒数，用于表示时间。
pub struct TimeVal {
    /// 秒
    pub sec: usize,
    /// 微秒
    pub usec: usize,
}

/// Task information
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// First run time of task
    pub first_run_time: usize,
    /// Total running time of task
    pub time: usize,
}

impl TaskInfo {
    /// 用于创建一个最开始的TaskInfo
    pub fn new_uninit() -> Self {
        TaskInfo {
            status: TaskStatus::UnInit,
            syscall_times: [0; MAX_SYSCALL_NUM],
            first_run_time: 0,
            time: 0,
        }
    }
    /// 将该task的某个系统调用号+1
    /// syscall_num:  系统调用号
    /// 无返回值
    pub fn plus1_syscall_times(&mut self, syscall_num: usize) {
        if syscall_num >= MAX_SYSCALL_NUM {
            panic!("syscall_num out of range");
        }
        self.syscall_times[syscall_num] += 1;
    }

    /// 功能: 得到运行时间
    /// 返回值: 运行时间ms(usize)
    pub fn update_get_runtime(&mut self) -> usize {
        self.time = (get_time_us() - self.first_run_time) / 1000;
        self.time
    }

    /// function: set the first run time
    /// return: ()
    pub fn set_runtime(&mut self) {
        //if time == 0, task had run
        if self.first_run_time == 0 {
            self.first_run_time = get_time_us() / 1000;
        }
    }
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
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    TASK_MANAGER.update_task_info_runtime();
    let result = TASK_MANAGER.get_task_info();
    TASK_MANAGER.show_task_info();
    if result.is_err() {
        return -1;
    }
    unsafe {
        *ti = result.expect("Don't have task info");
    }
    0
}

//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus,get_current_status,get_syscall_times,get_current_start_time,current_user_token,mmap,munmap
    },
    timer::get_time_us,
};
use crate::{
mm::{VirtAddr,PageTable,PhysAddr},
};
use crate::mm::MemorySet;
use crate::config::PAGE_SIZE;
use crate::mm::MapPermission;
use crate::mm::VPNRange;
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
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
pub fn current_translated_physical_address(ptr:*const u8)->usize{//将虚拟地址转换为物理地址
        let token = current_user_token();
	let page_table=PageTable::from_token(token);//根据token找pagetable
	let mut va=VirtAddr::from(ptr as usize);//从虚拟地址中提取出一级地址，二级地址，三级地址
	let mut vpn = va.floor();//va->virt page num
	let ppn = page_table.translate(vpn).unwrap().ppn();//ppn
	PhysAddr::from(ppn).0+va.page_offset()//物理地址
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

pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    let _us = get_time_us();
    let ts=current_translated_physical_address(_ts as *const u8) as *mut TimeVal;
    unsafe {
        *ts = TimeVal {
            sec: _us / 1_000_000,
            usec: _us % 1_000_000,
        };
    }
    0
}

pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let _ti = current_translated_physical_address(ti as *const u8) as *mut TaskInfo;
    trace!("kernel: sys_task_info");
    unsafe{
        *_ti = TaskInfo {
            status: get_current_status(),
            syscall_times: get_syscall_times(),
            time:(get_time_us()-get_current_start_time())/1000
        };
    }
    0
}


// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, len: usize, port: usize) -> isize {
    	mmap(_start, len, port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    munmap(_start, _len)
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

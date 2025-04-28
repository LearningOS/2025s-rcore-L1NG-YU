//! Process management syscalls
use crate::{
       
        task::{get_single_sys_call_time,change_program_brk, exit_current_and_run_next, suspend_current_and_run_next,current_user_token}, 
        timer::get_time_us,
        mm::translated_struct_ptr
        };

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
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
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    let us =get_time_us();
    let ts = translated_struct_ptr(current_user_token(), _ts);
    *ts = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(_trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match _trace_request {
        0 => {
            let addr = id as *const u8;
            unsafe {
                *addr as isize
            }
        }

        1 => {
            let addr = id as *mut u8;

            unsafe  {
                *addr = data as u8;
            }
            0
        }
        2 => {
            get_single_sys_call_time(id)
        }
        _ => -1

    }
    
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

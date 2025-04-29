//! Process management syscalls
use crate::{
       
        config::PAGE_SIZE, 
                mm::{translated_byte_buffer, MapPermission, PageTable, VirtAddr}, 
                task::{change_program_brk, current_user_token, exit_current_and_run_next,
                     get_single_sys_call_time, suspend_current_and_run_next,
                    mmap, munmap}, 
                timer::get_time_us
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

    let ts = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    let buffers = translated_byte_buffer(current_user_token(), _ts as *const u8, 16);
    let mut ts_ptr = &ts as *const TimeVal as *const u8;
    for buffer in buffers {
        let len = buffer.len();
        unsafe {
            buffer.copy_from_slice(core::slice::from_raw_parts(ts_ptr, len));
            ts_ptr = ts_ptr.add(len);
        }
    }
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(_trace_request: usize, id: usize, data: usize) -> isize {

    let pagetable = PageTable::from_token(current_user_token());
    let virtaddr =VirtAddr::from(id);
    let vpn = VirtAddr::from(id).floor();
    match _trace_request {
        0 => match pagetable.translate(vpn) {
            Some(pte) => {
                if pte.is_valid() && pte.readable() && pte.user() {
                    let ppn = pte.ppn();
                    let offset = virtaddr.page_offset();
                    let page_ptr = ppn.get_bytes_array().as_ptr();
                    let byte = unsafe { 
                        *(page_ptr.add(offset)) 
                        };
                    byte as isize
                }else{
                    return -1;
                }
            }
            None => return -1,
                
            
        }

        1 => match pagetable.translate(vpn){
            Some(pte) => {
                if pte.is_valid() && pte.writable() && pte.user() {
                    let ppn = pte.ppn();
                    let offset = virtaddr.page_offset();
                    let page_ptr = ppn.get_bytes_array().as_mut_ptr();
                    unsafe {
                        *(page_ptr.add(offset)) = data as u8;
                    }
                    0
                }else{
                    return -1;
                }
            }
            None => return -1,

            
        }
        2 => {
            get_single_sys_call_time(id)
        }
        _ => -1

    }
    
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {

    if len == 0 {
        return 0;
    }
    if start % PAGE_SIZE !=0 || (port & !0x7 !=0) || (port & 0x7 == 0) {
        return -1;
    }

    let start_va = VirtAddr::from(start);
    let end_va = VirtAddr::from(start+len);
    let mut map_perm =MapPermission::from_bits((port<<1) as u8).unwrap();
    map_perm |= MapPermission::U;

    match mmap(start_va, end_va, map_perm) {
        Ok(()) => 0,
        Err(()) => -1,
        
    }


}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    if len == 0 {
        return 0;
    }
    if start % PAGE_SIZE !=0 {
        return -1;
    }
    let start_va = VirtAddr::from(start);
    let end_va = VirtAddr::from(start+len);
    match munmap(start_va,end_va) {
        Ok(()) => 0,
        Err(()) => -1,
    }

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

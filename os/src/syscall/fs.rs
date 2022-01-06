use crate::batch::{get_app_boundary, get_user_sp};

const FD_STDOUT: usize = 1;
const USER_STACK_SIZE: usize = 4096;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let start = buf as usize;
            if !address_is_valid(start, start + len) {
                let (app_addr_start, app_addr_end) = get_app_boundary();
                // panic!("start is {}, end is {}, app start is {}, app end is {}",start, start+len, app_addr_start, app_addr_end);
                -1
            } else {
                let slice = unsafe { core::slice::from_raw_parts(buf, len) };
                let str = core::str::from_utf8(slice).unwrap();
                print!("{}", str);
                len as isize
            }
        },
        _ => {
            -1
            //panic!("Unsupported fd in sys_write!");
        }
    }
}

pub fn address_is_valid(addr_start: usize, addr_end: usize) -> bool {
    let (app_addr_start, app_addr_end) = get_app_boundary();
    let app_sp = get_user_sp();

    if (addr_start >= (app_addr_start + 5000) && addr_end < (app_addr_end + 5000))
//        || (addr_start >= (app_sp - USER_STACK_SIZE) && addr_end < app_sp)
    {
        return true;
    }
    false

}
use std::env;
use std::ffi::OsString;
use std::os::unix::ffi::OsStrExt;

#[allow(dead_code)]
fn abracadabra() {
    println!("Abracadabra! Function called!");
}

#[repr(C)]
#[allow(dead_code)]
struct Hackvist {
    buffer: [u8; 16],
    point: *const fn(),
}

#[allow(dead_code)]
pub(crate) fn check_case() {
    let mut args: Vec<OsString> = env::args_os().into_iter().collect();
    let first_arg: OsString = args.remove(1);
    let input_bytes: &[u8] = first_arg.as_bytes();
    let mut hackvist = Hackvist {
        buffer: [0; 16],
        point: 0 as *const fn() -> (),
    };

    unsafe {
        std::ptr::copy(
            input_bytes.as_ptr(),
            hackvist.buffer.as_mut_ptr(),
            input_bytes.len(),
        )
    }

    println!("abracadabra function address: x{:0x}", abracadabra as usize);
    println!(
        "hackvist.point after strcpy: x{:0x}",
        hackvist.point as usize
    );
    println!(
        "hackvist.point after strcpy (in chars): {:?}",
        (hackvist.point as usize as u64)
            .to_le_bytes()
            .into_iter()
            .map(|b| char::from(b))
            .collect::<String>()
    );

    if hackvist.point as usize == 0 {
        println!("Try again");
    } else {
        let code: fn() = unsafe { std::mem::transmute(hackvist.point) };
        code();
    }
}

use std::io;

#[repr(C)]
struct HackDop {
    buffer: [u8; 4],
    state: u32,
    point: *mut String,
}

pub fn dop() {
    let info = String::from("Basic info.");
    let secret = String::from("You get the secret!");
    let mut hackdop = HackDop {
        buffer: [0; 4],
        state: 0,
        point: &mut String::from("HackDop."),
    };
    // 输出必要信息
    println!("info address: {:p}", &info);
    println!("secret address: {:p}", &secret);
    println!("hackdop.point address: {:p}", hackdop.point);

    // 读入信息
    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    let input_bytes = input_string.as_bytes();

    // 处理信息 buffer overflow
    unsafe {
        std::ptr::copy(
            input_bytes.as_ptr(),
            hackdop.buffer.as_mut_ptr(),
            input_bytes.len(),
        );
        // 手动覆盖代替输入覆盖
        hackdop.state = 1;
        hackdop.point = (&info) as *const String as *mut String;
        println!("hackdop.point: {:p}", hackdop.point);

        // 根据state写入信息
        if hackdop.state == 0 {
            hackdop.point.write(info.clone());
        } else {
            hackdop.point.write(secret.clone());
        }
    }
    // 输出信息
    println!("info: {}", info);
    println!("secret: {}", secret);
    unsafe {
        println!("hackdop.point: {}", *hackdop.point);
    }
}

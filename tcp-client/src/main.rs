use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let mut stream = TcpStream::connect("127.0.0.1:12000")?;

    write_msg(&mut stream, "咳咳");
    write_msg(&mut stream, "你好，我是毛毛");
    write_msg(&mut stream, "hello, 请问你几岁");

    println!("开始休眠");
    thread::sleep(Duration::from_secs(60));
    println!("休眠结束");

    Ok(())

    // program退出，os关闭fd
}

fn write_msg(stream: &mut TcpStream, msg: &str) {
    let msg_length = msg.len();

    let header_len = msg_length as usize;
    let msg_b = msg.as_bytes();

    let mut data: Vec<u8> = Vec::with_capacity(1024);
    data.extend_from_slice(header_len.to_le_bytes().as_slice());
    data.extend_from_slice(msg_b);

    let write_r = stream.write(data.as_slice());
    match write_r {
        Ok(size) => {
            println!("write size: {}", size);
        }
        Err(e) => {
            println!("err: {}", e);
        }
    }
}

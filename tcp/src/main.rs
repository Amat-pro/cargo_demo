use std::collections::HashMap;
use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:12000").unwrap();

    let peer_manager: Arc<Mutex<PeerManager>> = Arc::new(Mutex::new(PeerManager::new()));

    'Loop: loop {
        let r = listener.accept();
        match r {
            Ok((stream, addr)) => {
                println!("listener success, addr: {}", addr.ip().to_string());
                tokio::spawn(handle(Arc::clone(&peer_manager), addr, stream));
                // handle(Arc::clone(&peer_manager), addr, &mut stream);
            }
            Err(e) => {
                println!("listener err: {}", e);
                continue 'Loop;
            }
        }
    }
}

async fn handle(peer_manager: Arc<Mutex<PeerManager>>, addr: SocketAddr, mut tcp_stream: TcpStream) {
    // lock for add
    {
        // println!("1. =====> ");
        let mut manager = peer_manager.lock().unwrap();
        if let Some(_) = manager.peers.get(&addr) {
            println!("socket has been registered, ip: {}, port: {}", addr.ip().to_string(), addr.port().to_string());
            return;
        }
        // println!("2. =====> ");
        let peer = Peer::new(addr);
        manager.peers.insert(addr, peer);
    }

    // println!("3. =====> ");
    // 1024字节
    const DEFAULT_BUFFER_SIZE: usize = 1024;
    const HEADER_SIZE: usize = 8;
    let mut buffer = Vec::<u8>::with_capacity(DEFAULT_BUFFER_SIZE);

    let mut msg_size: i64 = -1;
    loop {
        let buffer_len = buffer.len();
        // println!("buffer_len: {}", buffer_len);

        // 读取header, 确定msg_size
        if buffer_len >= HEADER_SIZE {
            // println!("读取header, 确定msg_size");
            let header_data: [u8; 8] = (&buffer[0..HEADER_SIZE]).try_into().unwrap();
            // from_le_bytes: 将以小端字节顺序存储的字节数组转换为 i64 类型。在小端字节顺序中，最低有效字节位于最低内存地址处。
            // from_be_bytes: 将以大端字节顺序存储的字节数组转换为 i64 类型。在大端字节顺序中，最高有效字节位于最低内存地址处。
            // from_ne_bytes: 将以本机字节顺序存储的字节数组转换为 i64 类型。本机字节顺序是指当前运行程序所在的计算机体系结构的字节顺序。这个方法使用的是当前计算机的字节顺序，因此在不同的计算机上可能会有不同的结果。
            msg_size = i64::from_le_bytes(header_data);
        }

        // 一条msg读取完成
        if msg_size >= 0 && (buffer_len >= msg_size as usize + HEADER_SIZE) {
            // println!("一条msg读取完成");
            buffer.drain(0..HEADER_SIZE);
            let removed = buffer.drain(0..msg_size as usize);
            let msg_data = removed.as_slice();
            let msg_str = String::from_utf8_lossy(&msg_data);
            println!("=====> msg: {}", msg_str);

            // 一条消息处理完成，重新赋值msg_size
            msg_size = -1;
            continue;
        }

        // 继续向buffer中读取数据
        // println!("继续向buffer中读取数据, buffer_len:{}", buffer.len());
        let mut peek_buf = [0; 1024];
        let size = tcp_stream.peek(&mut peek_buf).unwrap();
        if size > 0 {
            // println!(">>>>");
            let mut buf_tmp = [0; 1024];
            let read_size = tcp_stream.read(&mut buf_tmp).unwrap();
            buffer.extend(&buf_tmp[0..read_size]);
        } else {
            // peek或read为0，则退出
            println!("关闭连接，ip: {}, port: {}", addr.ip().to_string(), addr.port().to_string());
            return;
        }
    }
}

struct PeerManager {
    peers: HashMap<SocketAddr, Peer>,
}

impl PeerManager {
    fn new() -> Self {
        PeerManager {
            peers: HashMap::<SocketAddr, Peer>::new()
        }
    }
}

struct Peer {
    addr: SocketAddr,
}

impl Peer {
    fn new(addr: SocketAddr) -> Self {
        Peer {
            addr,
        }
    }
}


// ========================================== TEST =================================================
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let range = 2..6;

        let removed: Vec<_> = vec.drain(range).collect();

        println!("Removed elements: {:?}", removed); // 3, 4, 5, 6
        println!("Remaining elements: {:?}", vec); // 1, 2, 8, 9, 10
    }
}

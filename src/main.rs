use std::net::UdpSocket;
use std::net::SocketAddr;
use std::process::exit;
use std::fs::File;
use std::thread;
use std::time;

fn passive_data_process(recv_data: Vec<u8>) -> (bool, Vec<u8>) {
    // let buf = &mut recv_buf[..num];
    // buf.reverse();
    let ret = true;

    (ret, recv_data)
}
fn active_data_process() -> (bool, Vec<u8>) {
    // let buf = &mut recv_buf[..num];
    // buf.reverse();
    let ret = true;
    let recv_data = Vec::new();

    (ret, recv_data)
}

fn active_mode_proc() -> bool {
    
    true
}

fn main() -> std::io::Result<()>{
    // 日志系统
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    // 解析json文件
    let protocol = load_protocol("protocol.json".to_string());
    log::trace!("json data {:#?}", protocol);
    // 主动模式和被动模式分开执行
    let mut active_prot: Vec<serde_json::Value> = Vec::new();
    let mut passive_prot: Vec<serde_json::Value> = Vec::new();
    for prot in protocol {
        if prot["mode"] == "active" {
            active_prot.push(prot);
        }
        else if prot["mode"] == "passive" {
            passive_prot.push(prot);
        }
        else{
            log::warn!("Unknown mode [{}]", prot["mode"])
        }
    }
    log::trace!("found active: {:?}", active_prot);
    log::trace!("found passive: {:?}", passive_prot);
    // UDP
    let (client_addr, socket) = udp_init("127.0.0.1:33366".to_string(), "127.0.0.1:33368".to_string());

    let mut data_buf = [0u8; 1024];
    loop {
        let (number_of_bytes, addr) = loop {
            match  socket.recv_from(&mut data_buf) {
                Ok(n) => break n,
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // wait until network socket is ready, typically implemented
                    // via platform-specific APIs such as epoll or IOCP
                    let ret = active_mode_proc(); // 需要发送即返回true
                    if ret {
                        break (0, client_addr);
                    }
                    else {
                        thread::sleep(time::Duration::from_millis(10));
                    }
                }
                Err(e) => {
                    log::error!("encountered IO error: {e}");
                    thread::sleep(time::Duration::from_secs(10));
                },
            }
        };
        if addr != client_addr {
            log::error!("Unexpected socket addr : {:#?}", addr);
            exit(1);
        }
        let process_ret;
        let mut send_data = Vec::new();
        if number_of_bytes == 0 {
            let (ret, mut data) = active_data_process();
            process_ret = ret;
            send_data.append(&mut data);
        }
        else{
            let recv_data = &mut data_buf[..number_of_bytes];
            let recv_data_v = recv_data.to_vec();
            let (ret, mut data) = passive_data_process(recv_data_v);
            process_ret = ret;
            send_data.append(&mut data);
        }
        let num = send_data.len();
        let send_buf = &mut data_buf[..num];
        for (place, element) in send_buf.iter_mut().zip(send_data.iter()) {
            *place = *element;
        }

        if process_ret {
            socket.send_to(send_buf, client_addr).expect("Couldn't send data");
        }
        else{
            log::error!("Process occur error, no data to send.")
        }
    }
}

fn udp_init(server_addr: String, client_addr: String) -> (SocketAddr, UdpSocket) {
    let socket = UdpSocket::bind(server_addr).expect("couldn't bind to address");
    let client_addr: SocketAddr = client_addr.parse().expect("Unable to parse socket address");
    socket.set_nonblocking(true).unwrap();
    (client_addr, socket)
}

fn load_protocol(json_path: String) -> Vec<serde_json::Value> {
    let json_file = File::open(json_path).unwrap();
    let json_value: serde_json::Value = serde_json::from_reader(json_file)
                                                        .expect("Load json failed!");
    let protocol = json_value["protocol"]
                                    .as_array()
                                    .expect("Json format error! protocol not exist.");
    protocol.to_vec()
}
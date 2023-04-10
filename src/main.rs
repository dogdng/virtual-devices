use std::net::UdpSocket;
use std::net::SocketAddr;
use std::process::exit;
use std::fs::File;
use std::thread;
use std::time;

fn canopen_data_process(recv_v: Vec<u8>) -> Vec<u8> {
    let mut send_v: Vec<u8> = Vec::new();

    let canopen_tdata_ok = 0x60;
    let canopen_rdata_1b = 0x4f;
    let canopen_rdata_2b = 0x4b;
    let index = ((recv_v[2] as u16) << 8) & 0xff00 + (recv_v[1] as u16) & 0x00ff;
    if index == 0x6040 { // READ_ALL
        send_v.push(canopen_rdata_1b);
        send_v.push(recv_v[1]);
        send_v.push(recv_v[2]);
        send_v.push(recv_v[3]);
    }
    else if index == 0x6041 {
        send_v.push(canopen_tdata_ok);
        send_v.push(recv_v[1]);
        send_v.push(recv_v[2]);
        send_v.push(recv_v[3]);
    }
    send_v
}
fn passive_data_process(recv_data: Vec<u8>) -> (bool, Vec<u8>) {
    // let buf = &mut recv_buf[..num];
    // buf.reverse();
    // if 
    let mut ret = false;
    let mut resp: Vec<u8> = Vec::new();
    let head: u8 = 0xfa;
    let tail: u8 = 0xff;

    log::info!("------ >>>> {:02X?}", recv_data);
    let mut index = 0;
    if head != recv_data[index] {
        log::warn!("recv_data format error : {:02x?}", recv_data);
    }
    index += 1;
    resp.push(head);
    while 0xff != recv_data[index] {
        ret = true;
        // let null_vec = Vec::new();
        // return (false, null_vec);
    
        let type_seq = recv_data[index];
        let cmd_type = (type_seq >> 4) & 0x0f;
        
        // let sequence = type_seq & 0x0f;
        index += 1;
        // let cmd_length = recv_data[index];

        // log::trace!("cmd_length : {:02X?}", cmd_length);
        // log::trace!("cmd_length : {}", cmd_length);
        let mut ret_data: Vec<u8> = Vec::new();
        if cmd_type == 0x00 {
            // log::info!("test cmd.");
        }
        else if cmd_type == 0x01 {
            // log::info!("system cmd.");
            index += 12;
        }
        else if cmd_type == 0x02 {
            // log::info!("redirect can.")
            index += 1;
            let wait_rsp = (recv_data[index] >> 7) & 0x01;
            let send_rsp = (recv_data[index] >> 6) & 0x01;
            let remote_frame = (recv_data[index] >> 4) & 0x03; // 0:CAN_RTR_REMOTE, 1:CAN_RTR_DATA
            let dlc = recv_data[index] & 0x0f;
            index += 1;
            let controller_id = recv_data[index];
            index += 1;
            let can_bit0 = recv_data[index];
            index += 1;
            let can_bit1 = recv_data[index];
            let temp = (((can_bit0 as u16) << 8) & 0xff00) | ((can_bit1 as u16) & 0x00ff);
            let is_canopen = (temp >> 15) & 0x0001;
            let canopen_timeout = (temp >> 11) & 0x000f;
            let can_id = temp & 0x07ff;
            let mut can_data = Vec::new();
            for i in 0..8 {
                index += 1;
                let temp = recv_data[index];
                can_data.push(temp);
            }
            // log::info!("can data : {:02x?}", can_data);
            ////// 
            ret_data.push(dlc);
            ret_data.push(controller_id);
            let can_id_h = ((can_id >> 8) & 0x00ff) as u8;
            let can_id_l = (can_id & 0x00ff) as u8;
            ret_data.push(can_id_h);
            ret_data.push(can_id_l);
            let mut canopen_data = canopen_data_process(can_data);
            ret_data.append(&mut canopen_data);
        }
        else if cmd_type == 0x03 {
            log::info!("redirect IO.");
            index += 1;
            let group_id = recv_data[index];
            index += 1;
            // let timeout = (recv_data[index] >> 4) & 0x0f;
            // let op_type = recv_data[index] & 0x01;
            index += 1;
            let data_bit0 = recv_data[index];
            index += 1;
            let data_bit1 = recv_data[index];
            index += 1;
            let data_bit2 = recv_data[index];
            index += 1;
            let data_bit3 = recv_data[index];
            // 引脚状态（bit0~bit15分别对应GPIO_PIN0~GPIO_PIN15，1表示SET）
            let pin_data: u32 = ((data_bit0 as u32) << 24) | ((data_bit1 as u32) << 16) | ((data_bit2 as u32) << 8) | (data_bit3 as u32);
            // log::trace!("pin_data: {:b}", pin_data);
            index += 1;
            let mask_bit0 = recv_data[index];
            index += 1;
            let mask_bit1 = recv_data[index];
            index += 1;
            let mask_bit2 = recv_data[index];
            index += 1;
            let mask_bit3 = recv_data[index];
            // 引脚选择（bit0~bit15分别对应GPIO_PIN0~GPIO_PIN15，1表示ENB）
            let pin_mask: u32 = ((mask_bit0 as u32) << 24) | ((mask_bit1 as u32) << 16) | ((mask_bit2 as u32) << 8) | (mask_bit3 as u32);
            // log::trace!("pin_mask: {:b}", pin_mask);
            
            ret_data.push(group_id);
            ret_data.push(data_bit0);
            ret_data.push(data_bit1);
            ret_data.push(data_bit2);
            ret_data.push(data_bit3);
            ret_data.push(mask_bit0);
            ret_data.push(mask_bit1);
            ret_data.push(mask_bit2);
            ret_data.push(mask_bit3);
        }
        else if cmd_type == 0x04 {
            // log::info!("redirect com.");
            index += 1;
            let wait_rsp = (recv_data[index] >> 7) & 0x01;
            let send_rsp = (recv_data[index] >> 6) & 0x01;
            let data_len = recv_data[index] & 0x3f;
            index += 1;
            let controller_id = recv_data[index];
            index += 1;
            let wait_unit = (recv_data[index] >> 7) & 0x01; // 0x0：1ms, 0x1：10ms
            let wait_time = recv_data[index] & 0x7f;
            index += 1;
            let flush_rx = (recv_data[index] >> 7) & 0x01;
            let flush_rx_keep_last = (recv_data[index] >> 6) & 0x01;
            let reserved = recv_data[index] & 0x3f;
            let mut data = Vec::new();
            for i in 0..data_len {// data[8]
                index += 1;
                data.push(recv_data[index]);
            }
        }
        else if cmd_type == 0x05 {
            // log::info!("motor.");
            index += 6;
        }
        else if cmd_type == 0x06 {
            // log::info!("battery.");
            index += 4;
        }
        else if cmd_type == 0x07 {
            // log::info!("flash.");
            index += 1;
            let addr_bit0 = recv_data[index];
            index += 1;
            let addr_bit1 = recv_data[index];
            index += 1;
            let addr_bit2 = recv_data[index];
            index += 1;
            let addr_bit3 = recv_data[index];
            let flash_address = ((addr_bit0 as u32) << 24) | ((addr_bit1 as u32) << 16) | ((addr_bit2 as u32) << 8) | (addr_bit3 as u32);
            index += 1;
            let op_type = recv_data[index];
            index += 1;
            let op_len = recv_data[index];
            let mut data = Vec::new();
            for i in 0..op_len {
                index += 1;
                data.push(recv_data[index]);
            }
        }
        else if cmd_type == 0x08 {
            // log::info!("msg group setting.");
            index += 4;
        }
        else if cmd_type == 0x09 {
            // log::info!("cmd manager set.");
            index += 1;
            let group_id = recv_data[index] & 0x7f;
            let enable = (recv_data[index] >> 7) & 0x01;
            index += 1;
            let cmd_id = recv_data[index];
            index += 1;
            let cmd_len = recv_data[index];
            index += 1;
            let check_type = recv_data[index];
            index += 1;
            let check_type = recv_data[index];
            index += 1;
            let check_type = recv_data[index];
            index += 1;
            let extract_len = recv_data[index];
            let mut data = Vec::new();
            for i in 0..extract_len {
                index += 1;
                data.push(recv_data[index]);
            }
        }
        else if cmd_type == 0x0a {
            // log::info!("safety exchange.");
            index += 1;
            let master_status = recv_data[index] & 0x0f;
            match master_status {
                0x00 => {// log::info!("初始状态");
                        },
                0x01 => {// log::info!("复位状态");
                        },
                0x02 => {// log::info!("空闲状态");
                        },
                0x03 => {// log::info!("锁定状态");
                        },
                0x04 => {// log::info!("运动状态");
                        },
                0x05 => {// log::info!("故障0状态");
                        },
                0x06 => {// log::info!("故障1状态");
                        },
                0x07 => {// log::info!("故障2状态");
                        },
                0x08 => { // log::info!("故障3状态");
                        },
                0x0f => {// log::info!("故障恢复状态");
                        },
                _ => log::error!("reserved"),
            }
            // let linear_motion_direction = (recv_data[index] >> 4) & 0x03;
            // let rotate_motion_direction = (recv_data[index] >> 6) & 0x03;
            index += 1;
            // let oam_1_setting = recv_data[index];
            index += 1;
            // let oam_2_setting = recv_data[index];
            index += 1;
            // let light_status = recv_data[index];
            index += 1;
            // let sound_status = recv_data[index];
            index += 4; // reserved
            ////// response ///////
            ret_data.push(0x00); // 0x0: 正常模式 0x1: 恢复模式 0x2: 调试模式 bit3对应0类停止， bit4对应1类停止， bit5对应2类停止
            for i in 0..8 {
                ret_data.push(0x00);
            }
            for i in 0..8 {
                ret_data.push(0x00);
            }
        }
        else if cmd_type == 0x0b {
            // log::info!("msg group response.");
        }
        else if cmd_type == 0x0c {
            // log::info!("read count.");
        }
        else if cmd_type == 0x0d {
            // log::info!("Charging station.");
            index += 1;
            let cmd_id  = recv_data[index];
            index += 1;
            let cmd_len  = recv_data[index];
            let mut data = Vec::new();
            for i in 0..cmd_len {
                index += 1;
                data.push(recv_data[index]);
            }
        }
        else {
            log::warn!("recv_data has a error type! >>> {:02X?}", recv_data);
        }

        index += 1;
        // log::warn!("index[{}]", index);


        resp.push(type_seq);
        resp.push(ret_data.len() as u8); // 载荷部分长度
        resp.push(0x00); // 0：成功，1：失败

        resp.append(&mut ret_data);
    }

    resp.push(tail);

    if ret {
        log::info!("send data : {:02x?}", resp);
    }

    (ret, resp)
}
fn active_data_process() -> (bool, Vec<u8>) {
    // let buf = &mut recv_buf[..num];
    // buf.reverse();
    let ret = true;
    let recv_data = Vec::new();

    (ret, recv_data)
}

fn active_mode_proc() -> bool {
    false
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
    // let (client_addr, socket) = udp_init("192.168.0.6:33366".to_string(), "192.168.0.100:33368".to_string());
    let (client_addr, socket) = udp_init("127.0.0.1:33366".to_string(), "127.0.0.1:33368".to_string());

    let mut data_buf = [0u8; 1024];
    loop {
        let (number_of_bytes, addr) = loop {
            match  socket.recv_from(&mut data_buf) {
                Ok(n) => break n,
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // wait until network socket is ready, typically implemented via platform-specific APIs such as epoll or IOCP
                    let ret = active_mode_proc(); // 需要发送即返回true
                    if ret {
                        break (0, client_addr);
                    }
                    // else {
                    //     thread::sleep(time::Duration::from_micros(10));
                    // }
                }
                Err(e) => {
                    log::error!("encountered IO error: {e}");
                    thread::sleep(time::Duration::from_millis(1));
                },
            }
        };
        // let (number_of_bytes, addr) = socket.recv_from(&mut data_buf)?;
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
            socket.send_to(send_buf, client_addr).unwrap();//expect("Couldn't send data");
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
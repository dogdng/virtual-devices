use std::fs::File;
use std::error::Error;
use std::net::UdpSocket;
use std::net::SocketAddr;
use std::thread;
use std::time;
use std::process::exit;

static mut MOTOR_WORK_MODE: [i8; 16] = [0; 16];

fn canopen_data_process(node_id: u16, recv_v: Vec<u8>) -> Vec<u8> {
    let mut send_v: Vec<u8> = Vec::new();
    let canopen_tdata_ok = 0x60;
    let canopen_rdata_1b = 0x4f;
    let canopen_rdata_2b = 0x4b;
    let canopen_rdata_4b = 0x43;
    let canopen_rdata_var = 0x41;
    let index = ((recv_v[2] as u16) << 8) & 0xff00 | (recv_v[1] as u16) & 0x00ff;
    match index {
        0x1000 => { // GetDeviceModel
            send_v.push(canopen_rdata_4b); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x2ff7 => { // GetDeviceTime
            send_v.push(canopen_rdata_4b); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x100c => { // SetNodeHeartbeatProtectInterval
            send_v.push(canopen_tdata_ok); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x100d => { // SetNodeHeartbeatProtectThreshols
            send_v.push(canopen_tdata_ok); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x6007 => { // SetHeartbeatEnable Enable
            send_v.push(canopen_tdata_ok); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x6040 => { // control word SetServoEnable
            if recv_v[0] == 0x2f || recv_v[0] == 0x2b || recv_v[0] == 0x23 {
                send_v.push(canopen_tdata_ok);
            }
            send_v.push(recv_v[1]); // index
            send_v.push(recv_v[2]); // index
            send_v.push(recv_v[3]); // sub index
            // if recv_v[4] == 0x0b && recv_v[5] == 0x00 { // QuickStop
                // log::info!("QuickStop");
                send_v.push(0x00);
                send_v.push(0x00);
                send_v.push(0x00);
                send_v.push(0x00);
            // }
        },
        0x605a => { // SetQuickStopMode
            send_v.push(canopen_tdata_ok); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x605e => { // SetFaultStopMode
            send_v.push(canopen_tdata_ok); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x6060 => { // SetControlMode
            let id = node_id as usize;
            unsafe{
                log::info!("MOTOR_WORK_MODE: {:?}", MOTOR_WORK_MODE);
                MOTOR_WORK_MODE[id] = recv_v[4] as i8;
                log::info!("MOTOR_WORK_MODE: {:?}", MOTOR_WORK_MODE);
            }
            send_v.push(canopen_tdata_ok); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x6061 => { // GetRunningMode
            send_v.push(canopen_rdata_1b); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            let id = node_id as usize;
            unsafe{let work_mode = MOTOR_WORK_MODE[id]; send_v.push(work_mode as u8);}
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x6063 => { // GetCurrentSpeed
            send_v.push(canopen_rdata_4b); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x606c => { // GetCurrentPosition
            send_v.push(canopen_rdata_4b); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x6075 => { // GetRateCurrent
            send_v.push(canopen_rdata_4b); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x6078 => { // GetLoadingRates
            send_v.push(canopen_rdata_4b); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x6085 => { // SetQuickStopDeceleration
            send_v.push(canopen_tdata_ok); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x60f6 => { // GET_PEAK_DRIVER_UTILIZATION
            send_v.push(canopen_rdata_2b); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x60ff => { // SetTargetSpeed
            send_v.push(canopen_tdata_ok); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x6410 => { // GetMotorSn
            send_v.push(canopen_rdata_var); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x6510 => { // GET_PEAK_CURRENT
            send_v.push(canopen_rdata_2b); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x3000 => { // GetCfg1BusVoltage
            send_v.push(canopen_rdata_2b); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x1008 => { // GetDeviceName
            send_v.push(canopen_rdata_var); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x100a => { // GetSoftwareVersion
            send_v.push(canopen_rdata_var); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x2ff8 => { // GetSerialNum
            send_v.push(canopen_rdata_var); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x1005 => { // SyncID
            send_v.push(canopen_tdata_ok); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x1600 => { // SetRPDO1
            send_v.push(canopen_tdata_ok); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x1400 => { // SetRPDO1
            send_v.push(canopen_tdata_ok); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x1a00 => { // SetRPDO1 off
            send_v.push(canopen_tdata_ok); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x1a01 => { // SetRPDO2
            send_v.push(canopen_tdata_ok); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x1800 => { // SetTPDO1ID
            send_v.push(canopen_tdata_ok); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        0x1801 => { // SetRPDO2
            send_v.push(canopen_tdata_ok); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
        },
        _ => {
            send_v.push(0xcc); send_v.push(recv_v[1]); send_v.push(recv_v[2]); send_v.push(recv_v[3]);
            send_v.push(0x00); send_v.push(0x00); send_v.push(0x00); send_v.push(0x00);
            log::error!("unknown canopen command : {:02x?}", send_v);
        },
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

    // log::info!("---- >>>> {:02x?}", recv_data);
    let mut index = 0;
    if head != recv_data[index] {
        log::warn!("recv_data format error : {:02x?}", recv_data);
    }
    index += 1;
    resp.push(head);
    while 0xff != recv_data[index] {
        ret = true;
        let type_seq = recv_data[index];
        let cmd_type = (type_seq >> 4) & 0x0f;
        // let sequence = type_seq & 0x0f;
        index += 1;
        // let cmd_length = recv_data[index];
        let mut ret_data: Vec<u8> = Vec::new();
        if cmd_type == 0x00 { // log::info!("test cmd.");
            let pro_type = 0x11;
            let ver_major = 0x22;
            let ver_minor = 0x33;
            let ver_revision = 0x44;
            ret_data.push(pro_type);
            ret_data.push(ver_major);
            ret_data.push(ver_minor);
            ret_data.push(ver_revision);
        }
        else if cmd_type == 0x01 { // log::info!("system cmd.");
            index += 12;
        }
        else if cmd_type == 0x02 {// log::info!("redirect can.")
            // log::info!("---- >>>> {:02x?}", recv_data);
            index += 1;
            let _wait_rsp = (recv_data[index] >> 7) & 0x01;
            let _send_rsp = (recv_data[index] >> 6) & 0x01;
            let _remote_frame = (recv_data[index] >> 4) & 0x03; // 0:CAN_RTR_REMOTE, 1:CAN_RTR_DATA
            let dlc = recv_data[index] & 0x0f;
            index += 1;
            let controller_id = recv_data[index];
            index += 1;
            let can_bit0 = recv_data[index];
            index += 1;
            let can_bit1 = recv_data[index];
            let temp = (((can_bit0 as u16) << 8) & 0xff00) | ((can_bit1 as u16) & 0x00ff);
            let _is_canopen = (temp >> 15) & 0x0001;
            let _canopen_timeout = (temp >> 11) & 0x000f;
            let can_id = temp & 0x07ff;
            let mut can_data = Vec::new();
            for _i in 0..dlc {
                index += 1;
                let temp = recv_data[index];
                can_data.push(temp);
            }
            if (can_id & 0x0f00) == 0x0700 {
                // log::info!("heart beat can data : {:02x?}", can_data);
            }
            else if (can_id & 0x0f00) == 0x0600 {
                let ret_can_id = can_id - 0x0600 + 0x0580;
                ret_data.push(dlc);
                ret_data.push(controller_id);
                let can_id_h = ((ret_can_id >> 8) & 0x00ff) as u8;
                let can_id_l = (ret_can_id & 0x00ff) as u8;
                ret_data.push(can_id_h);
                ret_data.push(can_id_l);
                let mut canopen_data = canopen_data_process(can_id - 0x0600, can_data);
                ret_data.append(&mut canopen_data);
            }
            else if (can_id & 0xff80) == 0x080 {
                let ret_can_id = can_id | 0x0200; // TPDO1 0x0100, TPDO2 0x0200
                ret_data.push(8);
                ret_data.push(controller_id);
                let can_id_h = ((ret_can_id >> 8) & 0x00ff) as u8;
                let can_id_l = (ret_can_id & 0x00ff) as u8;
                ret_data.push(can_id_h);
                ret_data.push(can_id_l);
                for _i in 0..8 {
                    ret_data.push(0x01);
                }
            }
            else if can_id == 0x0000 {
                let ret_can_id = can_id;
                ret_data.push(0);
                ret_data.push(controller_id);
                let can_id_h = ((ret_can_id >> 8) & 0x00ff) as u8;
                let can_id_l = (ret_can_id & 0x00ff) as u8;
                ret_data.push(can_id_h);
                ret_data.push(can_id_l);
                // log::info!("---- >>>> {:02x?}", recv_data);
                // return (false, vec![])
            }
            log::info!("ret data : {:02x?}", ret_data);
        }
        else if cmd_type == 0x03 {// log::info!("redirect IO.");
        // log::info!("---- >>>> {:02x?}", recv_data);
        index += 1;
        let group_id = recv_data[index];
            index += 1;
            // let timeout = (recv_data[index] >> 4) & 0x0f;
            let _op_type = recv_data[index] & 0x01; // 0x0：ReadPin 0x1：WritePin
            index += 1;
            let data_bit0 = recv_data[index];
            index += 1;
            let data_bit1 = recv_data[index];
            index += 1;
            let data_bit2 = recv_data[index];
            index += 1;
            let data_bit3 = recv_data[index];
            // 引脚状态（bit0~bit15分别对应GPIO_PIN0~GPIO_PIN15，1表示SET）
            let _pin_data: u32 = ((data_bit0 as u32) << 24) | ((data_bit1 as u32) << 16) | ((data_bit2 as u32) << 8) | (data_bit3 as u32);
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
            let _pin_mask: u32 = ((mask_bit0 as u32) << 24) | ((mask_bit1 as u32) << 16) | ((mask_bit2 as u32) << 8) | (mask_bit3 as u32);
            // log::trace!("pin_mask: {:b}", pin_mask);
            
            ret_data.push(group_id);
            // if op_type == 0x01 {
                // ret_data.push(data_bit0);
                // ret_data.push(data_bit1);
                // ret_data.push(data_bit2);
                // ret_data.push(data_bit3);
                ret_data.push(mask_bit0);
                ret_data.push(mask_bit1);
                ret_data.push(mask_bit2);
                ret_data.push(mask_bit3);
                ret_data.push(mask_bit0);
                ret_data.push(mask_bit1);
                ret_data.push(mask_bit2);
                ret_data.push(mask_bit3);
            // }
        }
        else if cmd_type == 0x04 {// log::info!("redirect com.");
            index += 1;
            let wait_rsp = (recv_data[index] >> 7) & 0x01;
            let send_rsp = (recv_data[index] >> 6) & 0x01;
            let data_len = recv_data[index] & 0x3f;
            index += 1;
            let controller_id = recv_data[index];
            index += 1;
            let _wait_unit = (recv_data[index] >> 7) & 0x01; // 0x0：1ms, 0x1：10ms
            let _wait_time = recv_data[index] & 0x7f;
            index += 1;
            let _flush_rx = (recv_data[index] >> 7) & 0x01;
            let _flush_rx_keep_last = (recv_data[index] >> 6) & 0x01;
            let _reserved = recv_data[index] & 0x3f;
            let mut data = Vec::new();
            for _i in 0..data_len {// data[8]
                index += 1;
                data.push(recv_data[index]);
            }
            //////// IMU ////////
            if data_len > 0 {
            if data[0] == 0x77 {
                    if wait_rsp > 0 && send_rsp > 0 {
                        let ret_dete_len = 14;
                        ret_data.push(ret_dete_len);
                        ret_data.push(controller_id);
                        ret_data.push(0x77); ret_data.push(0x0d); ret_data.push(0x00); ret_data.push(0x84); ret_data.push(0x00); ret_data.push(0x02); ret_data.push(0x01);
                        ret_data.push(0x10); ret_data.push(0x00); ret_data.push(0x51); ret_data.push(0x00); ret_data.push(0x00); ret_data.push(0x00); ret_data.push(0xf5);
                    }
                    else {
                        ret_data.push(0x00);
                        ret_data.push(controller_id);
                    }
                }
            }
            else { // AGV codereader Dahua
                if wait_rsp > 0 && send_rsp > 0 {
                    let ret_dete_len = 21;
                    ret_data.push(ret_dete_len);
                    ret_data.push(controller_id);
                    ret_data.push(0x00); ret_data.push(0x44); ret_data.push(0x00); ret_data.push(0x00); ret_data.push(0x00); ret_data.push(0x04); ret_data.push(0x7f);
                    ret_data.push(0x5c); ret_data.push(0x00); ret_data.push(0x00); ret_data.push(0x00); ret_data.push(0x59); ret_data.push(0x00); ret_data.push(0x00);
                    ret_data.push(0x00); ret_data.push(0x07); ret_data.push(0x77); ret_data.push(0x54); ret_data.push(0x00); ret_data.push(0x00); ret_data.push(0x4a);
                }
                else {
                    ret_data.push(0x00);
                    ret_data.push(controller_id);
                }
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
            let _flash_address = ((addr_bit0 as u32) << 24) | ((addr_bit1 as u32) << 16) | ((addr_bit2 as u32) << 8) | (addr_bit3 as u32);
            index += 1;
            let _op_type = recv_data[index];
            index += 1;
            let op_len = recv_data[index];
            let mut data = Vec::new();
            for _i in 0..op_len {
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
            let _group_id = recv_data[index] & 0x7f;
            let _enable = (recv_data[index] >> 7) & 0x01;
            index += 1;
            let _cmd_id = recv_data[index];
            index += 1;
            let _cmd_len = recv_data[index];
            index += 1;
            let _check_type = recv_data[index];
            index += 1;
            let _check_type = recv_data[index];
            index += 1;
            let _check_type = recv_data[index];
            index += 1;
            let extract_len = recv_data[index];
            let mut data = Vec::new();
            for _i in 0..extract_len {
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
            for _i in 0..8 {
                ret_data.push(0x00);
            }
            for _i in 0..8 {
                ret_data.push(0x00);
            }
        }
        else if cmd_type == 0x0b {
            // log::info!("msg group response.");
        }
        else if cmd_type == 0x0c {// log::info!("read count.");
            // log::info!("---- >>>> {:02x?}", recv_data);
            let ticks_system_running = 0x00;
            let cnt_processed_frames = 0x00;
            let cnt_succ_instruction = 0xff;
            let cnt_fail_instruction = 0xff;
            ret_data.push(ticks_system_running); ret_data.push(ticks_system_running); ret_data.push(ticks_system_running); ret_data.push(ticks_system_running);
            ret_data.push(cnt_processed_frames); ret_data.push(cnt_processed_frames); ret_data.push(cnt_processed_frames); ret_data.push(cnt_processed_frames);
            ret_data.push(cnt_succ_instruction); ret_data.push(cnt_succ_instruction); ret_data.push(cnt_succ_instruction); ret_data.push(cnt_succ_instruction);
            ret_data.push(cnt_fail_instruction); ret_data.push(cnt_fail_instruction); ret_data.push(cnt_fail_instruction); ret_data.push(cnt_fail_instruction);
        }
        else if cmd_type == 0x0d {// log::info!("Charging station.");
            index += 1;
            let _cmd_id  = recv_data[index];
            index += 1;
            let cmd_len  = recv_data[index];
            let mut data = Vec::new();
            for _i in 0..cmd_len {
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

    // if ret {
    //     log::info!("send data : {:02x?}", resp);
    // }

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

pub fn run() -> Result<(), Box<dyn Error>> {
    // UDP
    // let (client_addr, socket) = udp_init("192.168.0.6:33366".to_string(), "192.168.0.100:33368".to_string());
    let (client_addr, socket) = udp_init("127.0.0.1:33366", "127.0.0.1:33368");

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
    }
    // Ok(())
}

fn udp_init(server_addr: &str, client_addr: &str) -> (SocketAddr, UdpSocket) {
    let socket = UdpSocket::bind(server_addr).expect("couldn't bind to address");
    let client_addr: SocketAddr = client_addr.parse().expect("Unable to parse socket address");
    socket.set_nonblocking(true).unwrap();
    (client_addr, socket)
}

pub fn load_protocol(json_path: String) -> Vec<serde_json::Value> {
    let json_file = File::open(json_path).unwrap();
    let json_value: serde_json::Value = serde_json::from_reader(json_file)
                                                        .expect("Load json failed!");
    let protocol = json_value["protocol"]
                                    .as_array()
                                    .expect("Json format error! protocol not exist.");
    protocol.to_vec()
}
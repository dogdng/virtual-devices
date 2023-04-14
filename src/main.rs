use virtual_devices;

// enum CmdType {
//     Test(u8) = 0x00,
// }

fn main() {
    // 日志系统
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    // 解析json文件
    // let protocol = virtual_devices::load_protocol("protocol.json".to_string());
    // log::trace!("json data {:#?}", protocol);
    // // 主动模式和被动模式分开执行
    // let mut active_prot: Vec<serde_json::Value> = Vec::new();
    // let mut passive_prot: Vec<serde_json::Value> = Vec::new();
    // for prot in protocol {
    //     if prot["mode"] == "active" {
    //         active_prot.push(prot);
    //     }
    //     else if prot["mode"] == "passive" {
    //         passive_prot.push(prot);
    //     }
    //     else{
    //         log::warn!("Unknown mode [{}]", prot["mode"])
    //     }
    // }
    // log::trace!("found active: {:?}", active_prot);
    // log::trace!("found passive: {:?}", passive_prot);
    if let Err(e) = virtual_devices::run() {
        log::error!("vd run error: {}", e);
        std::process::exit(1);
    }
}


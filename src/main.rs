use std::net::UdpSocket;
use std::net::SocketAddr;
use std::process::exit;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};

fn data_process(recv_data: Vec<u8>) -> (bool, Vec<u8>) {
    // let buf = &mut recv_buf[..num];
    // buf.reverse();
    let ret = true;


    (ret, recv_data)
}


fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let server_addr = "127.0.0.1:33366";
    let client_addr = "127.0.0.1:33368";
    let client_addr: SocketAddr = client_addr.parse().expect("Unable to parse socket address");
    let mut data_buf = [0u8; 1024];

    let socket = UdpSocket::bind(server_addr).expect("couldn't bind to address");
    loop {
        let (number_of_bytes, addr) = socket.recv_from(&mut data_buf).expect("Didn't receive data");
        if addr != client_addr {
            log::error!("Unexpected socket addr : {:#?}", addr);
            exit(1);
        }
        let recv_data = &mut data_buf[..number_of_bytes];
        let recv_data_v = recv_data.to_vec();

        let (process_ret, send_data) = data_process(recv_data_v);

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
    // loop {
    //     let mut buf = [1, 2, 3, 4, 5, 6, 7, 8];
    //     socket.send_to(buf, client_addr)?;
        
    //     let buf = &mut buf[..amt];
    //     buf.reverse();
    //     let (amt, src) = socket.recv_from(&mut buf)?;
    // }
}
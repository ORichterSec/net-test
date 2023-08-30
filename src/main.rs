use std::env;
use service::service;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let config = service::Config::new(&args);
    service::run(&config).unwrap();

    Ok(())
}

// use tokio::net::UdpSocket;
// use std::io;

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let sock = UdpSocket::bind("0.0.0.0:8080").await?;
//     let mut buf = [0; 1024];
//     loop {
//         let (len, addr) = sock.recv_from(&mut buf).await?;
//         println!("{:?} bytes received from {:?}", len, addr);

//         let len = sock.send_to(&buf[..len], addr).await?;
//         println!("{:?} bytes sent", len);
//     }
// }
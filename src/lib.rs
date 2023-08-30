pub mod service{

    #[derive(Debug)]
    struct CustomError;
    impl std::error::Error for CustomError{}
    impl std::fmt::Display for CustomError{
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
            write!(f, "custom error")
        }
    }
    static TIME_OUT: u64 = 60;
    use std::{
        net::{UdpSocket, TcpListener, TcpStream, IpAddr, SocketAddr},
        io::{prelude::*},
        time,
        error::Error,
        str,
    };

    #[derive(Debug)]
    pub enum Protocoll{
        TCP,
        UDP,
        None
    }

    impl std::fmt::Display for Protocoll{
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    pub struct Config{
        address: IpAddr,
        port: u16,
        protocoll: Protocoll,
    }

    impl Config{

        pub fn new(args: &[String]) -> Config {
            if args.len() < 3 {
                panic!("not enough arguments");
            }
            let address: IpAddr = args[1].clone().parse().unwrap_or_else( |_| panic!("invalid value for ip address '{}'", args[1]));
            let port = args[2].clone().parse().unwrap_or_else( |_| panic!("invalid value for port '{}'", args[2]));
            let protocoll = 
                if args.len() == 4{
                    match args[3].as_str(){
                        "tcp" => Protocoll::TCP,
                        "udp" => Protocoll::UDP,
                        _ => Protocoll::None,
                    }
                }else{
                    Protocoll::None
                };
            Config { address, port, protocoll }
        }

    }

    fn sent_answer(mut stream: TcpStream) -> Result<String, Box<dyn Error>>{
        println!("inside sent_answer");
        let mut read_size;
        let mut write_size = 0;
        loop{
            let mut buf = [0;512];
            // let buf_reader = BufReader::new(&mut stream);
            read_size =  stream.read(&mut buf)?;
            println!("content of buf:{}", str::from_utf8(&buf).unwrap());

            if read_size == 0 {
                break;
            }
            write_size = stream.write(&buf)?;
        }
        let output = format!("got {} bytes from tcp stream wrote {} bytes", read_size, write_size);
        Ok(output)
    }

    fn start_tcp_listener(socket: SocketAddr, timeout: Option<u64>) -> Result<String, Box<dyn Error>> {
        println!("called start_tcp_listener");
        let listener = TcpListener::bind(socket)?;
        let start = time::Instant::now();
        for stream in listener.incoming() {
            let stream = stream?;
            println!("inside for loop");
            sent_answer(stream)?;
            let quit =
                match timeout{
                    Some(value) => start.elapsed().as_secs() > value,
                    None => false,
                };
            if quit{
                break;
            }
        }

        let output = match timeout {
            Some(value) => value.to_string(),
            None => String::from("THIS SHOULD NOT HAVE BEEN PRINTED. REPLACE THIS WITH PANIC!/UNWRAP() LATER."),
        };
        Ok(format!("closed UDPListener after {}",output))
    }

    fn start_upd_listener(socket: SocketAddr, timeout: Option<u64>) -> Result<String, Box<dyn Error>> {
        let socket = UdpSocket::bind(socket)?;
        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 10];
        let start = time::Instant::now();        
        loop {
            let (amt, src) = socket.recv_from(&mut buf)?;
            
            // Redeclare `buf` as slice of the received data and send reverse data back to origin.
            let buf = &mut buf[..amt];
            buf.reverse();
            println!("{:?} bytes received from {:?}", amt, src);
            let len = socket.send_to(buf, &src)?;
            println!("{:?} bytes sent", len);

            let quit =
                match timeout{
                    Some(value) => start.elapsed().as_secs() > value,
                    None => false,
                };

            if quit{
                break;
            }
        }

        let output = match timeout {
            Some(value) => value.to_string(),
            None => String::from("THIS SHOULD NOT HAVE BEEN PRINTED. REPLACE THIS WITH PANIC!/UNWRAP() LATER."),
        };
        Ok(format!("closed UDPListener after {}",output))
        // the socket is closed here
    }

    fn start_listener(socket: SocketAddr, protocoll: &Protocoll, timeout: Option<u64>) -> Result<String, Box<dyn Error>>{
        match protocoll{
            Protocoll::TCP => return start_tcp_listener(socket, timeout),
            Protocoll::UDP => return start_upd_listener(socket, timeout),
            Protocoll::None => println!("invalid choice '{:?}' for protocoll choose either: tcp|udp", protocoll),
        };
        return Ok(String::from("pass"));
    }

    pub fn run(config: &Config) -> Result<String, Box<dyn Error>>{
        let addr = config.address;
        let port = config.port;
        let protocoll = &config.protocoll;
        let socket_addr: SocketAddr = SocketAddr::from((addr,port));
        
        return start_listener(socket_addr, protocoll, Some(TIME_OUT));

        // Ok(String::from("pass"))
    }
}


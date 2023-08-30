pub mod service{

    #[derive(Debug)]
    struct CustomError;
    impl std::error::Error for CustomError{}
    impl std::fmt::Display for CustomError{
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
            write!(f, "custom error")
        }
    }
    static TIME_OUT: u64 = 3600;
    use std::{
        net::{UdpSocket, TcpListener, TcpStream, IpAddr, SocketAddr},
        io::{prelude::*},
        time,
        error::Error,
        str,
    };
    use configparser::ini::Ini;

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

    #[derive(Debug)]
    pub struct Config{
        address: IpAddr,
        port: u16,
        protocoll: Protocoll,
    }

    impl Config{

        fn from_file(path: &String) -> Config{
            let section = "default";
            let mut config = Ini::new();
            config.load(path).expect("Error while loading file");
            let address = config.get(section, "address").unwrap();
            let port = config.get(section, "port").unwrap();
            let protocoll = config.get(section, "protocoll").unwrap();
            Config::parse_string_triple(&address, &port, &protocoll)
        }

        fn parse_string_triple(address: &String, port: &String, protocoll: &String) -> Config {
            let address: IpAddr = address.clone().parse().unwrap_or_else( |_| panic!("invalid value for ip address '{}'", address));
            let port = port.clone().parse().unwrap_or_else( |_| panic!("invalid value for port '{}'", port));
            let protocoll =
                match protocoll.as_str() {
                    "tcp" => Protocoll::TCP,
                    "udp" => Protocoll::UDP,
                    _ => Protocoll::None,
                };
            Config { address, port, protocoll }
        }

        pub fn new(args: &[String]) -> Config {

            //try to read from file if we pass -f flag as first arg
            if args.len() >= 3{
                if args[1] == "-f" {
                    return Config::from_file(&args[2]);
                }
            }

            if args.len() < 3 {
                panic!("not enough arguments");
            }
            let addr = &args[1];
            let port = &args[2];
            if args.len() == 4{
                Config::parse_string_triple(addr, port, &args[3].to_string())
            }else{
                Config::parse_string_triple(addr, port, &"".to_string())
            }
            
        }

    }

    fn sent_answer(mut stream: TcpStream) -> Result<String, Box<dyn Error>>{
        let mut read_size;
        let mut write_size = 0;
        loop{
            let mut buf = [0;512];
            // let buf_reader = BufReader::new(&mut stream);
            read_size =  stream.read(&mut buf)?;
            println!("content of buf:{}", str::from_utf8(&buf).unwrap());

            let output = format!("got {} bytes from tcp stream wrote {} bytes", read_size, write_size);
            println!("{}",output);
            if read_size == 0 {
                break;
            }
            write_size = stream.write(&buf)?;
        }
        Ok(format!("sent answer of {} bytes", write_size))
    }
    
    fn start_tcp_listener(socket: SocketAddr, timeout: Option<u64>) -> Result<String, Box<dyn Error>> {
        let start = time::Instant::now();
        let listener = TcpListener::bind(socket)?;
        loop{
            let (stream, _) = listener.accept()?;
            sent_answer(stream)?;
            let quit =
                match timeout {
                    Some(value) => start.elapsed().as_secs() > value,
                    None => false,
                };
            if quit{
                break;
            }else{
                continue;
            }
        }
        
        let output = match timeout {
            Some(value) => value.to_string(),
            None => String::from("THIS SHOULD NOT HAVE BEEN PRINTED. REPLACE THIS WITH PANIC!/UNWRAP() LATER."),
        };
        Ok(format!("closed TCPListener after {} seconds (timeout)",output))
    }

    // fn ready_to_answer(socket: SocketAddr) -> Result<String, Box<dyn Error>>{
    //     let listener = TcpListener::bind(socket)?;
    //     for stream in listener.incoming() {
    //         let stream = stream?;
    //         sent_answer(stream)?;
    //     }
    //     Ok(String::from("pass"))
    // }

    // fn start_tcp_listener(socket: SocketAddr, timeout: Option<u64>) -> Result<String, Box<dyn Error>> {
    //     let start = time::Instant::now();
    //     // loop{
    //     // let stream = listener.accept();
    //     let(tx, rx) = mpsc::channel();

        
    //     let join = thread::spawn(move || tx.send(ready_to_answer(socket).unwrap()));
    //     let recieved = rx.try_recv();
    //     match recieved {
    //         Ok(value) => println!("Recieved from worker:{}", value),
    //         Err(error_str) => println!("Error while trying to receive: {}", error_str),
    //     }
        
    //     loop{
    //         thread::sleep(std::time::Duration::new(1, 0));
    //         let quit =
    //             match timeout {
    //                 Some(value) => start.elapsed().as_secs() > value,
    //                 None => false,
    //             };
    //         if quit{
    //             break;
    //         }else{
    //             continue;
    //         }
    //     }

    //     let output = match timeout {
    //         Some(value) => value.to_string(),
    //         None => String::from("THIS SHOULD NOT HAVE BEEN PRINTED. REPLACE THIS WITH PANIC!/UNWRAP() LATER."),
    //     };
    //     Ok(format!("closed TCPListener after {}",output))
    // }
    
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
                match timeout {
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
        Ok(format!("closed UDPListener after {} seconds (timeout)",output))
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


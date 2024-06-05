use {
    bincode::{self, config::{self, Configuration}, Encode, Decode},
    mdns_sd::{DaemonEvent, Receiver, ServiceDaemon, ServiceEvent, ServiceInfo},
    std::{
        collections::HashMap,
        io::{self, Write},
        net::{IpAddr, Ipv6Addr, SocketAddr, TcpListener, TcpStream},
        sync::{mpsc, Arc, Mutex},
        thread,
        time::Duration,
    },
};

const CONFIG: Configuration = config::standard();

pub fn lan_demo() {
    let mut mdns = ServiceDaemon::new().unwrap();

    joust(&mut mdns);

    // let mut input = String::new();

    // loop {
    //     println!(
    //         "Enter 'host' to host, enter 'join' to join.\n\
    //         Enter 'joust' to both host and join, like a knighte of olde!"
    //     );
    //     io::stdin().read_line(&mut input).unwrap();

    //     match input.trim().to_ascii_lowercase().as_ref() {
    //         "host" => {
    //             host(&mut mdns);
    //             break;
    //         }
    //         "join" => {
    //             join(&mut mdns);
    //             break;
    //         }
    //         "joust" => {
    //             joust(&mut mdns);
    //             break;
    //         }
    //         _ => println!("Invalid input."),
    //     }
    // }
}

fn joust(mdns: &mut ServiceDaemon) {
    let mut input = String::new();
    let name;
    loop {
        print!("\rEnter name: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let _input = input.trim();
        if _input.chars().all(|ch| ch.is_ascii_alphanumeric()) && { input.len() >= 3 } {
            name = _input.to_owned();
            break;
        } else {
            input.clear();
            print!("\rName must be at least 3 chars, and alphanumeric.");
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_secs(1));
            print!("\r                                                ");
        }
    }

    // get names, and insert those with the SocketAddr
    let mut map = Arc::new(Mutex::new(HashMap::new()));

    // 'easy peasy lemon squeezy'
    // 'why would you have anything but mac and chees?'
    // lemonvsqueezy sounds like a led zeppelin song with a sexual metaphor

    // let (tx, rx) = mpsc::sync_channel(0);

    // port 0 requests to be assigned a port
    let listener = TcpListener::bind("::1:0").unwrap();

    host(mdns, &name, listener.local_addr().unwrap().port());

    let receiver = mdns.browse("_termchess._tcp.local.").unwrap();
    let _map = map.clone();
    let _name = name.clone();
    thread::spawn(|| watch_for_endpoints(receiver, _map, _name));

    // 2 things:
    // listen in another thread for a request to contact
    // if user enters name, make request to contact
    // if other user connects here, then use the input for the y/n

    println!("\rEnter name of player to connect. (case sensitive)");

    // <(String, socketaddr?)>
    // tcpstream I think
    let req = Arc::new(Mutex::new(None::<(String, TcpStream)>));

    let _req = req.clone();
    thread::spawn(|| wait_for_connect(listener, _req));

    let mut stream;
    let con_name;
    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        // checking to see if someone else is trying to connect to us
        if let Some(name) = &*req.lock().unwrap() {
            match input.trim().to_ascii_lowercase().as_ref() {
                "y" => todo!(),
                "n" => todo!(),
                _ => println!("Invalid input."),
            }
        } else if let Some(addr) = map.lock().unwrap().get(input.trim()) {
            // just pass all 4 addresses?

            con_name = input.trim().to_owned();
            stream = dbg!(TcpStream::connect(dbg!(&addr[..]))).unwrap();
            // stream.write(
                bincode::encode_into_std_write(
                Transmission::ConnectionRequest(name.clone()),
                &mut stream,
                CONFIG,
            ).unwrap();
        // );

            break;
        } else {
            println!("Name not found.");
        }
    }
    // println!("socketaddr: {:?}", map.lock().unwrap().get(input.trim()));

    loop {
        // get input, and send that;

        // spawn thread to print input?
    }
}

fn wait_for_connect(listener: TcpListener, req: Arc<Mutex<Option<(String, TcpStream)>>>) {
    // wait for any connection

    // if we get a connection,
    // set is_req to the stream?

    // then on that end it gets moved out I guess
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let trans: Transmission = bincode::decode_from_std_read(&mut stream, CONFIG).unwrap();
        let Transmission::ConnectionRequest(name) = trans else {
            panic!("first trans should be req");
        };
        println!("{} is trying to connect. Join? (y/n)", name);
        *req.lock().unwrap() = Some((name, stream));
    }
}

#[derive(Encode, Decode)]
enum Transmission {
    ConnectionRequest(String),
    Message(String),
}

// figure out way to gracefully shutdown
fn watch_for_endpoints(
    receiver: Receiver<ServiceEvent>,
    map: Arc<Mutex<HashMap<String, Vec<SocketAddr>>>>,
    own_name: String,
) {
    // println!("test!");
    // browse for endpoints, when found,
    // add them to the hash and print them out

    // use refresh: bool to refresh

    loop {
        if let Ok(event) = receiver.try_recv() {
            match event {
                ServiceEvent::ServiceResolved(info) => {
                    // temp
                    // let ip = info
                    //     .get_addresses()
                    //     .iter()
                    //     .find(|ip| ip.to_string().starts_with("fe80"))
                    //     .unwrap();
                    let name = info
                        .get_hostname()
                        .strip_suffix(".local.")
                        .unwrap()
                        .trim()
                        .to_owned();
                    // println!("name: {}\nown_name: {}", name, own_name);
                    if name == own_name || map.lock().unwrap().contains_key(&name) {
                        continue;
                    }

                    // let Some(ip) = info.get_addresses().iter().find(|ip| {
                    //     // println!("ip: {}", ip);
                    //     match **ip {
                    //         IpAddr::V6(ip) => {
                    //             // println!(
                    //             //     "is_unicast_link_local: {}",
                    //             //     ip.is_unicast_link_local()
                    //             // );
                    //             ip.is_unicast_link_local()
                    //         }
                    //         IpAddr::V4(_) => false,
                    //     }
                    // }) else {
                    //     continue;
                    // };
                    if info.get_addresses().len() < 4 {
                        continue;
                    }

                    let addrs: Vec<SocketAddr> = info.get_addresses().iter().map(|addr| SocketAddr::new(addr.to_owned(), info.get_port())).collect();
                    // let addrs: Vec<SocketAddr> = info.get_addresses().iter().map(|addr| SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), info.get_port())).collect();

                    println!("Found player: {}.", name);
                    // for ip in info.get_addresses() {
                    //     println!("ip: {}", ip);
                    // }
                    map.lock()
                        .unwrap()
                        .insert(name, addrs);
                    // println!(
                    //     "to_string: {}\nfrom_bits: {}",
                    //     Ipv6Addr::from_bits(
                    //         info.get_addresses()
                    //             .iter()
                    //             .next()
                    //             .unwrap()
                    //             .to_string()
                    //             .as_bytes()
                    //     ),
                    // );
                    // for ip in info.get_addresses() {
                    //     if let IpAddr::V6(ip) = ip {
                    //         println!(
                    //             "ip: {} is unicast link local: {}",
                    //             ip,
                    //             ip.is_unicast_link_local()
                    //         )
                    //     }
                    // }
                }
                // ignore these for now, but name them explicity
                // as a reminder of sorts
                ServiceEvent::SearchStarted(_)
                | ServiceEvent::ServiceFound(_, _)
                | ServiceEvent::SearchStopped(_)
                | ServiceEvent::ServiceRemoved(_, _) => (),
            }
        }

        // check for refresh
    }
}

///  holds the data for another instance of termchess
// #[derive(PartialEq, Eq, Hash)]
// struct Endpoint {
//     name: String,
//     socketaddr: SocketAddr,
// }

// this was to try vand be able to input name OR uuid, but for final thinkg,
// I'm just gonna need the uuid, so for now will index with name
// #[derive(Hash, Eq)]
// struct Key {
//     uuid: String,
//     name: String,
// }
// impl Key {
//     pub fn new() -> Self {
//         Self {
//             uuid: String::new(),
//             name: String::new(),
//         }
//     }
// }

// impl PartialEq for Key {
//     fn eq(&self, other: &Self) -> bool {
//         if
//     }

//     fn ne(&self, other: &Self) -> bool {

//     }
// }

fn host(mdns: &mut ServiceDaemon, name: &str, port: u16) {
    // println!("{}", listener.local_addr().unwrap());

    // necessary because ServiceInfo is a tuple struct
    let service_type = "_termchess._tcp.local.";
    // name of person I guess? can I specifically get this property
    // when querying it?
    let instance_name = name;
    // will be automatically set by addr_auto
    let ip = "";
    let service_hostname = format!("{}.local.", &instance_name);

    // uuid and public key would go here, but that's not necessary right now
    // let properties = [("property_1", "test"), ("property_2", "1234")];

    let service_info = ServiceInfo::new(
        service_type,
        instance_name,
        &service_hostname,
        ip,
        port,
        // &properties[..],
        None,
    )
    .unwrap()
    .enable_addr_auto();

    let monitor = mdns.monitor().unwrap();
    mdns.register(service_info).unwrap();

    // while let Ok(event) = monitor.recv() {
    //     println!("Daemon event: {:?}", &event);
    //     if let DaemonEvent::Error(e) = event {
    //         println!("Failed: {}", e);
    //         break;
    //     }
    // }
}

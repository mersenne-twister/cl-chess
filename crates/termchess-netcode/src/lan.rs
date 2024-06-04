use {
    mdns_sd::{DaemonEvent, Receiver, ServiceDaemon, ServiceEvent, ServiceInfo},
    std::{
        collections::HashMap,
        io::{self, Write},
        net::{TcpListener, TcpStream},
        sync::{mpsc, Arc, Mutex},
        thread,
        time::Duration,
    },
};

pub fn lan_demo() {
    let mut mdns = ServiceDaemon::new().unwrap();

    // joust(&mut mdns);
    // return;

    let mut input = String::new();

    loop {
        println!(
            "Enter 'host' to host, enter 'join' to join.\n\
            Enter 'joust' to both host and join, like a knighte of olde!"
        );
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().to_ascii_lowercase().as_ref() {
            "host" => {
                host(&mut mdns);
                break;
            }
            "join" => {
                join(&mut mdns);
                break;
            }
            "joust" => {
                joust(&mut mdns);
                break;
            }
            _ => println!("Invalid input."),
        }
    }
}

fn joust(mdns: &mut ServiceDaemon) {
    let mut input = String::new();
    let name;
    loop {
        print!("\rEnter name: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input.chars().all(|ch| ch.is_ascii_alphanumeric()) && { input.len() >= 3 } {
            name = input.to_owned();
            break;
        } else {
            // println!("{}", input);
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

    let _map = map.clone();
    let receiver = mdns.browse("_termchess._tcp.local.").unwrap();
    thread::spawn(|| watch_for_endpoints(receiver, _map));
}

// figure out way to gracefully shutdown
fn watch_for_endpoints(
    receiver: Receiver<ServiceEvent>,
    map: Arc<Mutex<HashMap<String, Endpoint>>>,
) {
    // browse for endpoints, when found,
    // add them to the hash and print them out

    // use refresh: bool to refresh

    loop {
        if let Ok(event) = receiver.try_recv() {
            match event {
                ServiceEvent::ServiceResolved(info) => {
                    //
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
#[derive(PartialEq, Eq, Hash)]
struct Endpoint {
    // name
    // socketaddr
}

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

fn host(mdns: &mut ServiceDaemon) {
    // port 0 requests to be assigned a port
    let listener = TcpListener::bind("::1:0").unwrap();
    println!("{}", listener.local_addr().unwrap());

    // necessary because ServiceInfo is a tuple struct
    let service_type = "_termchess._tcp.local.";
    // name of person I guess? can I specifically get this property
    // when querying it?
    let instance_name = "alice";
    // will be automatically set by addr_auto
    let ip = "";
    let service_hostname = format!("{}.local.", &instance_name);
    // get port from tcp-listener
    let port = listener.local_addr().unwrap().port();
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

    while let Ok(event) = monitor.recv() {
        println!("Daemon event: {:?}", &event);
        if let DaemonEvent::Error(e) = event {
            println!("Failed: {}", e);
            break;
        }
    }

    mdns.shutdown().unwrap();
}

fn join(mdns: &mut ServiceDaemon) {
    let service_type = "_termchess._tcp.local.";
}

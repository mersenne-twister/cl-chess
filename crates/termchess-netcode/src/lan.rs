use {
    mdns_sd::{DaemonEvent, ServiceDaemon, ServiceEvent, ServiceInfo},
    std::{
        collections::HashMap,
        io,
        net::{TcpListener, TcpStream},
        thread,
        time::Duration,
    },
};

pub fn lan_demo() {
    let mut mdns = ServiceDaemon::new().unwrap();

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
                todo!();
                break;
            }
            _ => println!("Invalid input."),
        }
    }
}

fn host(mdns: &mut ServiceDaemon) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
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

fn join(mdns: &mut ServiceDaemon) {}

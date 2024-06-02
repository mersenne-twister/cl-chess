use {
    mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo},
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
    let listener = TcpListener::bind(0).unwrap();

    // necessary because ServiceInfo is a tuple struct
    let service_type = "_termchess._tcp.local.";
    let instance_name = "termchess";
    // will be automatically set by addr_auto
    let ip = "";
    let service_hostname = format!("{}{}", &instance_name, &service_type);
    // get port from tcp-listener
    let port = 0;
    // id would go here, but that's not necessary right now
    // let properties = [("property_1", "test"), ("property_2", "1234")];

    let info = ServiceInfo::new(
        service_type,
        instance_name,
        &service_hostname,
        ip,
        port,
        // set status here?
        // but it's also instance specific
        // &properties[..],
        None,
    )
    .unwrap()
    .enable_addr_auto();

    // println!("{:?}", info.get_addresses());

    mdns.register(info).unwrap();

    // println!("{:?}", mdns.status());

    thread::sleep(Duration::from_secs(10));
    mdns.shutdown().unwrap();
}

fn join(mdns: &mut ServiceDaemon) {}

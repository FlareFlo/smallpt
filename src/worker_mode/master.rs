use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::thread::sleep;
use std::time::Duration;

use get_port::Ops;
use mdns_sd::ServiceInfo;

use crate::worker_mode::SERVICE_NAME;

pub fn master() {

	let slaves = connect_slaves(Duration::from_secs(1));
	loop {

	}
}

pub fn connect_slaves(timeout: Duration) -> Vec<(TcpStream, SocketAddr)> {
	// Discover open configurations
	let ip = local_ip_address::local_ip().unwrap();
	let port = get_port::tcp::TcpPort::any(ip.to_string().as_str()).unwrap();

	// Reserve port on machine
	let listener = TcpListener::bind((ip, port)).unwrap();

	// Advertise services on localhost and LAN
	let dns = mdns_sd::ServiceDaemon::new().unwrap();

	let full_service_name = "smallpt_master";
	let host_name = ip.to_string() + ".local.";
	let service = ServiceInfo::new(
		SERVICE_NAME,
		full_service_name,
		&host_name,
		ip.to_string(),
		port,
		None,
	).unwrap()
		.enable_addr_auto();
	dns.register(service).unwrap();
	println!("Advertising master service at {ip}:{port}");

	let mut socks = vec![];

	socks.push(listener.accept().unwrap());
	// TODO: Await connections until timeout is reached, or key is pressed

	socks
}
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::thread::sleep;
use std::time::Duration;

use get_port::Ops;
use mdns_sd::ServiceInfo;

use crate::worker_mode::SERVICE_NAME;

pub fn master() {

	let slaves = connect_slaves(Duration::from_secs(1));

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
	let service_local = ServiceInfo::new(
		SERVICE_NAME,
		full_service_name,
		&host_name,
		Ipv4Addr::new(127,0,0,1),
		port,
		None,
	).unwrap();
	dns.register(service).unwrap();
	dns.register(service_local).unwrap();
	println!("Advertising master service at {ip}:{port}");

	let mut socks = vec![];

	// TODO: Await connections until timeout is reached, or key is pressed

	socks
}
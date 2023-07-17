use std::net::TcpListener;
use std::thread::sleep;
use std::time::Duration;

use get_port::Ops;
use mdns_sd::ServiceInfo;

use crate::worker_mode::SERVICE_NAME;

pub fn master() {
	let ip = local_ip_address::local_ip().unwrap();
	let port = get_port::tcp::TcpPort::any(ip.to_string().as_str()).unwrap();
	let dns = mdns_sd::ServiceDaemon::new().unwrap();
	let service = ServiceInfo::new(
		SERVICE_NAME,
		"smallpt_master",
		&(ip.to_string() + ".local."),
		ip.to_string(),
		port,
		[("role", "master")].as_slice(),
	).unwrap()
		.enable_addr_auto();
	dns.register(service).unwrap();

	let listener = TcpListener::bind((ip, port)).unwrap();

	println!("Hosting master render at {ip}:{port}");
	let mut socks = vec![];
	loop {
		socks.push(listener.accept().unwrap());
		sleep(Duration::from_secs(1));
		println!("{:?}", socks);
	}
}
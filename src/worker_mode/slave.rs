use std::net::TcpStream;
use mdns_sd::{ServiceDaemon, ServiceEvent};
use crate::worker_mode::SERVICE_NAME;

pub fn slave() {
	let dns = ServiceDaemon::new().expect("Failed to create daemon");
	let receiver = dns.browse(SERVICE_NAME).expect("Failed to browse");
	let conn = TcpStream::connect("192.168.0.102:1024").unwrap();

	while let Ok(event) = receiver.recv() {
		match event {
			ServiceEvent::ServiceResolved(info) => {
				println!("Resolved a new service: {}", info.get_fullname());
				let sock = TcpStream::connect((info.get_addresses().iter().next().unwrap().to_string(), info.get_port())).unwrap();
				loop {

				}
			}
			other_event => {
				println!("Received other event: {:?}", &other_event);
			}
		}
	}
}
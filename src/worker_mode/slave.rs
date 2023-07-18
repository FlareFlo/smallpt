use std::net::TcpStream;
use std::process::exit;
use std::time::{Duration, Instant};
use clearscreen::clear;
use mdns_sd::{ServiceDaemon, ServiceEvent};
use crate::worker_mode::SERVICE_NAME;

pub const SLAVE_DISCOVER_TIMEOUT: u64 = 100;

pub fn slave() {
	let master = resolve_master().unwrap();
}

fn resolve_master() -> Option<TcpStream> {
	let dns = ServiceDaemon::new().expect("Failed to create daemon");
	let receiver = dns.browse(SERVICE_NAME).expect("Failed to browse");

	let begin = Instant::now();
	while let Ok(event) = receiver.recv() {
		match event {
			ServiceEvent::ServiceResolved(info) => {
				clear().unwrap();
				println!("Resolved a new service: {}", info.get_fullname());
				let sock = TcpStream::connect((info.get_addresses().iter().next().unwrap().to_string(), info.get_port())).unwrap();
				dns.shutdown().unwrap();
				return Some(sock);
			}
			_ => {
				clear().unwrap();
				println!("Searching for master {:.0}s elapsed", begin.elapsed().as_secs_f64());
				if begin.elapsed() > Duration::from_secs(SLAVE_DISCOVER_TIMEOUT) {
					eprintln!("Failed to discover master client after {} seconds", SLAVE_DISCOVER_TIMEOUT);
					exit(1);
				}
			}
		}
	}
	None
}
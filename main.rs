use std::env;
mod handler;
mod server;
mod client;


fn main() {
    let args: Vec<String> = env::args().collect();
	let as_server = handler::handler(&args, "-s", "false");
	let as_server = if as_server.parse::<bool>().is_ok() {as_server.parse::<bool>().unwrap() as bool} else {false};
	let port = handler::handler(&args, "-p", "9090");
	let host = handler::handler(&args, "-h", "127.0.0.1");
	if as_server {
		server::run(format!("{}:{}", host, port));
	} else {
		client::run(format!("{}:{}", host, port));
	}
}
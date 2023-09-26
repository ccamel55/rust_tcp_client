use std::thread;

mod helpers;

fn tcp_thread() {
    loop {
        // todo: process TCP messages here

    }
}

fn main() {
    println!("Hello, world!");

    // load config
    let config = helpers::config::load_config().expect("poop");
    println!("address: {}, port: {}", config.udp_address, config.udp_port);

    // start TCP port, is move needed here??
    let tcp_thread_handle = thread::spawn(move || tcp_thread());

    // wait until thread has finished before shutting down process
    tcp_thread_handle.join().unwrap();

    println!("TCP thread closed, shutting down");
}

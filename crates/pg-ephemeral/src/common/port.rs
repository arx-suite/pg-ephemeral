use std::{
    net::{IpAddr, SocketAddr, TcpListener},
    sync::atomic::{AtomicU16, Ordering},
};

use super::constants::{DEFAULT_DB_HOST, DEFAULT_DB_PORT};

static DB_PORT_COUNTER: AtomicU16 = AtomicU16::new(DEFAULT_DB_PORT);

pub fn random_free_port() -> u16 {
    let host = DEFAULT_DB_HOST;

    loop {
        let port = DB_PORT_COUNTER.fetch_add(1, Ordering::SeqCst);

        if port >= u16::MAX {
            DB_PORT_COUNTER.store(DEFAULT_DB_PORT, Ordering::SeqCst);
            continue;
        }

        if is_free_tcp(port, &host) {
            return port;
        }
    }
}

fn is_free_tcp(port: u16, host: &IpAddr) -> bool {
    let socket_addr: SocketAddr = SocketAddr::new(*host, port);
    TcpListener::bind(socket_addr).is_ok()
}

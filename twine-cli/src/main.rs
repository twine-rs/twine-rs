use std::net::{SocketAddr, TcpListener};
use socket2::{Socket, Domain, Type, Protocol};
use log;

fn check_af_inet6() {
    let socket = match Socket::new(Domain::IPV6, Type::DGRAM, Some(Protocol::ICMPV6)) {
        Ok(s) => s,
        Err(e) => {
            log::error!("failed to create socket: {e:?}");
            return;
        }
    };

    log::info!("socket: {:?}", socket);
}

fn main() {
    env_logger::init();
    check_af_inet6();
}

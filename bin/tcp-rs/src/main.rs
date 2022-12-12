use std::io;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1500];
    loop {
        let n = nic.recv(&mut buf)?;
        // filter out non ipv4 packets
        // use etherpars to parse the ethernet frame
        println!("Received {:?} bytes: {:?}", n, &buf[..n]);
    }
}

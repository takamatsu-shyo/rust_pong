use std::net::UdpSocket;

pub fn my_udp() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:50001")?;

        let mut buf = [0; 10];
        let (amt, src) = socket.recv_from(&mut buf)?;

        let buf = &mut buf[..amt];
        buf.reverse();
        socket.send_to(buf, &src)?;
    }
    Ok(())
}

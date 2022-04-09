mod hw;

fn main(){
    hw::hello_world();

    //{
    //    let socket = UdpSocket::bind("127.0.0.1:50001")?;
    //
    //    let mut buf = [0; 10];
    //    let (amt, src) = socket.recv_from(&mut buf)?;
    //
    //    let buf = &mut buf[..amt];
    //    buf.reverse();
    //    socket.send_to(buf, &src)?;
    //}
    //Ok(())
}

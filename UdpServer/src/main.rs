use std::net::UdpSocket;

pub trait ServerInterface
{
    fn executeServer();
}

impl ServerInterface for Server {


    fn executeServer() {
        todo!()
    }
}
pub trait Server<Protocol>
{
    fn start(&self);
    fn send(&self, protocol: Protocol )
    {

    }
    fn close(&self);
}


fn main() {
    println!("Hello, world!");
}

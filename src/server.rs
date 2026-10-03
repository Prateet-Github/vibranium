use std::io;
use std::net::TcpListener;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new(address: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(address)?;

        println!("Vibranium listening on {}", address);

        Ok(Self { listener })
    }

    pub fn run(&self) -> io::Result<()> {
        loop {
            let (stream, address) = self.listener.accept()?;

            println!("New connection from {}", address);

            drop(stream);
        }
    }
}

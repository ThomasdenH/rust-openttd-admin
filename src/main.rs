pub mod packet;

fn main() -> Result<(), Box<std::error::Error>> {
    /*
    let mut stream = TcpStream::connect("localhost:3977")?;

    let join = AdminJoin::new(
        "password".to_string(),
        "rust-openttd-admin".to_string(),
        "1.8.0".to_string()
    );
    stream.write(&Vec::<u8>::from(join))?;
    loop {
        let packet = stream.read_packet()?;
        println!("{:?}", packet);
    }*/
    Ok(())
}

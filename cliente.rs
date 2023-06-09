use std::io::{Read, Write, stdin, stdout};
use std::net::TcpStream;
use std::thread;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:49152")?;

    let mut read_stream = stream.try_clone()?;
    let handle = thread::spawn(move || {
        let mut buffer = [0; 256];
        loop {
            match read_stream.read(&mut buffer) {
                Ok(0) => {
                    // servidor cerró la conexión
                    break;
                }
                Ok(bytes) => {
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes]));
                    stdout().flush().unwrap();
                }
                Err(_) => {
                    eprintln!("Error al leer del servidor.");
                    break;
                }
            }
        }
    });

    loop {
        let mut mensaje = String::new();
        stdin().read_line(&mut mensaje).unwrap();
        stream.write_all(mensaje.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    handle.join().unwrap();
    Ok(())
}


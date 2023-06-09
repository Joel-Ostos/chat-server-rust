use std::io::{Write, Read};
use std::net::{TcpStream, TcpListener};
use std::thread;

fn mensajeria(nombre: String, mut habla: TcpStream, mut escucha: TcpStream) {
    loop {
        let mut buffer = [0; 256];
        match habla.read(&mut buffer) {
            Ok(0) => {
                // cliente desconectado
                break;
            }
            Ok(bytes) => {
                let mut mensaje = format!("{}: ", nombre);
                mensaje.push_str(&String::from_utf8_lossy(&buffer[..bytes]));
                escucha.write_all(mensaje.as_bytes()).unwrap();
                escucha.flush().unwrap();
            }
            Err(_) => {
                eprintln!("Error al leer del cliente.");
                break;
            }
        }
    }
}

fn main() {
    println!("Servidor iniciado");
    let servidor = TcpListener::bind("127.0.0.1:49152").unwrap();
    let (cliente1, _) = servidor.accept().unwrap();
    let (cliente2, _) = servidor.accept().unwrap();

    let hilo1_cliente1 = cliente1.try_clone().unwrap();
    let hilo1_cliente2 = cliente2.try_clone().unwrap();

    let conexion1 = thread::spawn(move || {
        mensajeria("Cliente 1".to_string(), hilo1_cliente1, hilo1_cliente2);
    });

    let hilo2_cliente1 = cliente1.try_clone().unwrap();
    let hilo2_cliente2 = cliente2.try_clone().unwrap();

    let conexion2 = thread::spawn(move || {
        mensajeria("Cliente 2".to_string(), hilo2_cliente2, hilo2_cliente1);
    });

    conexion1.join().unwrap();
    conexion2.join().unwrap();
}


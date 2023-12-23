// simple_rust_server main.rs
// use: gives simple_rust_client.rs access to a shell over tcp using IP address and port given.
// authers: idky137
//

use regex::Regex;
use std::io;
use std::io::{Read, Write};
use std::net::{SocketAddr, SocketAddrV4, TcpListener, TcpStream};
use std::process::{Command, Stdio};

fn client_handler(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    while let Ok(size) = stream.read(&mut buffer) {
        if size == 0 {
            break;
        }

        let command_str = String::from_utf8_lossy(&buffer[..size]);

        if command_str == "endsession" {
            println!("Received command: {}", command_str);
            println!("Client ended session, closing program");
            std::process::exit(1);
        }

        println!("Received command: {}", command_str);

        let output = Command::new("sh")
            .args(&["-c", &command_str])
            .stdout(Stdio::piped())
            .output()
            .expect("Error: Failed to execute command");

        if output.stdout.is_empty() {
            let ret_str = "Command received: no output returned\n";
            stream
                .write_all(ret_str.as_bytes())
                .expect("Error: failed to send response");
        }

        stream
            .write_all(&output.stdout)
            .expect("Errror: Failed to send response");

        let eor_flag = "\t\t";
        stream
            .write_all(eor_flag.as_bytes())
            .expect("Error: failed to send eor_flag");

        buffer = [0; 1024];
    }
}

fn main() {
    let mut bind_add = String::new();
    loop {
        bind_add.clear();
        println!("\nPlease enter IP address and port to bind to or press enter for loopback [127.0.0.1:8080]: ");
        print!("$$ ");
        io::stdout().flush().expect("Error: failed to flush stdout");

        let _bind_add_length = std::io::stdin()
            .read_line(&mut bind_add)
            .expect("Error: failed to read address");
        let bind_add_trim = bind_add.trim();

        if bind_add_trim.len() == 0 {
            let bind_add_def = "127.0.0.1:8080";
            bind_add = bind_add_def.to_string();

            println!("Using loopback IP address and port: {}", bind_add);
            break;
        } else {
            let bind_add_pat = r"^(?:[0-9]{1,3}\.){3}[0-9]{1,3}:\d{1,5}$";
            let bind_add_reg = Regex::new(bind_add_pat).expect("Error: invalid regex pattern");

            if bind_add_reg.is_match(bind_add_trim) {
                bind_add = bind_add_trim.to_string();

                println!("Using IP address and port: {}", bind_add);
                break;
            } else {
                println!(
                    "Invalid address format entered {} use \"IPAddress:Port\", for example \"127.0.0.1:8080\"",
                    bind_add_trim
                );
            }
        }
    }
    let bind_add_tcp = bind_add.clone();
    let listener = TcpListener::bind(bind_add_tcp).unwrap();

    println!("Listening at: {}", bind_add);
    print!(".. ");
    io::stdout().flush().expect("Error: failed to flush stdout");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                client_handler(stream);
                print!(".. ");
                io::stdout().flush().expect("Error: failed to flush stdout");
            }
            Err(e) => {
                eprintln!("Error: Failed to accept connection: {}", e);
            }
        }
    }
}

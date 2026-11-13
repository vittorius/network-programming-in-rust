
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
 // This function handles a single client connection.
// It blocks the thread while waiting for data.
fn handle_client(mut stream: TcpStream) {
 let mut buffer = [0; 1024];
 // Loop continuously to read data as it arrives.
    loop {
        // The thread pauses here until the client sends data or closes the socket.
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected cleanly.");
                break;
            }
            Ok(bytes_read) => {
                println!("Received {} bytes.", bytes_read);
                // We must handle potential errors when writing back.
                if let Err(e) = stream.write_all(&buffer[..bytes_read]) {
                    println!("Failed to write to client: {}", e);
                    break;
                }
            }
            Err(e) => {
                println!("Network error occurred: {}", e);
                break;
            }
        }
    }
} // The stream object goes out of scope here.
  // Rust automatically closes the underlying file descriptor.
 fn main() -> std::io::Result<()> {
 // Bind the listener to a specific port on the local machine.
 let listener = TcpListener::bind("127.0.0.1:8080")?;
 println!("Server listening on port 8080");

for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection accepted.");
                // Spawn a dedicated operating system thread for this client.
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Failed to accept connection: {}", e);
            }
        }
    }
     Ok(())
}

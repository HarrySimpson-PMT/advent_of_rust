use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

#[allow(dead_code)]
pub async fn send_data_to_pico(lines: &Vec<String>) -> io::Result<()> {
    let host = "10.0.0.126";
    let port = 1234;
    let address = format!("{}:{}", host, port);
    // println!(
    //     "Connecting to {}:{} to send {} lines",
    //     host,
    //     port,
    //     lines.len()
    // );

    let mut stream = match timeout(Duration::from_secs(5), TcpStream::connect(&address)).await {
        Ok(Ok(stream)) => {
            // println!("Successfully connected to the server!");
            stream
        }
        Ok(Err(e)) => {
            eprintln!("Connection failed: {}", e);
            return Err(e);
        }
        Err(_) => {
            eprintln!("Connection timed out.");
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "Connection timed out",
            ));
        }
    };

    use tokio::time::{sleep, Duration};

    async fn read_ack() {
        sleep(Duration::from_millis(80)).await;
    }

    //combine lines into a single string
    let lines = lines.join("\n");
    let transmission_size = lines.len();
    // println!("Transmission size: {}", transmission_size);
    stream.write_all(format!("LEN:{}", transmission_size).as_bytes()).await?;
        read_ack().await;

    //send the data
    stream.write_all(lines.as_bytes()).await?;

    let mut buffer = [0; 4096];
    println!("Waiting for final response...");
    let n = match stream.read(&mut buffer).await {
        Ok(0) => {
            eprintln!("Server closed the connection.");
            return Err(io::Error::new(
                io::ErrorKind::ConnectionReset,
                "Server closed the connection",
            ));
        },
        Ok(n) => n,
        Err(e) => {
            eprintln!("Failed to read data from the server: {}", e);
            return Err(e);
        }
    };
    println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));

    // println!("Data sent successfully!");
    Ok(())
}
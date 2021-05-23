#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn capture_the_flag() {
        use tokio::io::AsyncBufReadExt;
        use tokio::io::AsyncWriteExt;
        use tokio::io::BufReader;
        use tokio::net::TcpStream;

        let stream = TcpStream::connect("134.122.111.232:13374").await.unwrap();
        let mut stream = BufReader::new(stream);

        let mut line = String::new();
        stream.read_line(&mut line).await.unwrap();

        stream
            .write_all("{\"option\": \"get_secret\"}".as_bytes())
            .await
            .unwrap();

        let mut line = String::new();
        stream.read_line(&mut line).await.unwrap();

        stream
            .write_all(
                format!(
                    "{{\"option\": \"sign\", \"msg\": \"{}\"}}",
                    line.strip_prefix("{\"secret\": \"")
                        .unwrap()
                        .strip_suffix("\"}\n")
                        .unwrap()
                )
                .as_bytes(),
            )
            .await
            .unwrap();

        let mut line = String::new();
        stream.read_line(&mut line).await.unwrap();

        let line = line
            .strip_prefix("{\"signature\": \"0x")
            .unwrap()
            .strip_suffix("\"}\n")
            .unwrap();

        println!("{}", String::from_utf8_lossy(&hex::decode(&line).unwrap()));
    }
}

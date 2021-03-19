#[cfg(test)]
mod tests {
    #[macro_use]
    use serde::Deserialize;
    use serde_json::{json, ser::to_string, to_vec};
    use telnet::{Telnet, TelnetEvent};

    #[derive(Deserialize, Debug)]
    struct Response {
        r#type: String,
        encoded: String,
    }
    #[test]
    fn json_converter() {
        let ascii: [u8; 55] = [
            123, 34, 116, 121, 112, 101, 34, 58, 32, 34, 114, 111, 116, 49, 51, 34, 44, 32, 34,
            101, 110, 99, 111, 100, 101, 100, 34, 58, 32, 34, 102, 103, 104, 113, 118, 114, 113,
            95, 112, 121, 118, 122, 111, 95, 105, 114, 101, 102, 118, 98, 97, 102, 34, 125, 10,
        ]; //{"type": "rot13", "encoded": "fghqvrq_pyvzo_irefvbaf"}
           // The type of `john` is `serde_json::Value
        let resp: Response = serde_json::from_slice(&ascii).unwrap();
        println!("{:?}", resp);
    }
    #[test]
    fn capture_the_flag() {
        let mut connection = Telnet::connect(("134.122.111.232", 13377), 4096)
            .expect("Couldn't connect to the server...");
        println!("Connected to server");
        let mut event = connection.read().expect("Read Error");
        match event {
            TelnetEvent::Data(buffer) => {
                let resp: Response = serde_json::from_slice(&(*buffer)).unwrap();
                println!("{:?}", resp);
            }
            _ => println!(""),
        }
        let john = json!({"decoded": "1234"});
        connection
            .write(&to_vec(&john).unwrap())
            .expect("Write Error");
        let mut event = connection.read().expect("Read Error");
        match event {
            TelnetEvent::Data(buffer) => {
                let resp: Response = serde_json::from_slice(&(*buffer)).unwrap();
                println!("{:?}", resp);
            }
            _ => println!(""),
        }
    }
}

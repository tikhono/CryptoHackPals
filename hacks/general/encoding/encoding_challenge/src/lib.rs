#[cfg(test)]
mod tests {
    use ascii::IntoAsciiString;
    use rot13::rot13;
    use serde::Deserialize;
    use serde_json::{json, to_vec};
    use telnet::{Telnet, TelnetEvent};

    #[derive(Deserialize, Debug)]
    struct Response {
        r#type: String,
        encoded: String,
    }
    #[derive(Deserialize, Debug)]
    struct ResponseUtf {
        r#type: String,
        encoded: Vec<u8>,
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

        for _ in 0..100 {
            let event = connection.read().expect("Read Error");
            match event {
                TelnetEvent::Data(buffer) => {
                    let resp: Response = serde_json::from_slice(&(*buffer)).unwrap_or(Response {
                        r#type: "utf-8".to_string(),
                        encoded: "".to_string(),
                    });
                    let string = match resp.r#type.as_str() {
                        "utf-8" => {
                            let resp: ResponseUtf = serde_json::from_slice(&buffer).unwrap();
                            resp.encoded.into_ascii_string().unwrap().to_string()
                        }
                        "rot13" => rot13(&resp.encoded),
                        "hex" => hex::decode(&resp.encoded)
                            .unwrap()
                            .into_ascii_string()
                            .unwrap()
                            .to_string(),
                        "base64" => base64::decode(&resp.encoded)
                            .unwrap()
                            .into_ascii_string()
                            .unwrap()
                            .to_string(),
                        "bigint" => hex::decode(resp.encoded.get(2..).unwrap())
                            .unwrap()
                            .into_ascii_string()
                            .unwrap()
                            .to_string(),
                        _ => "".to_string(),
                    };
                    let answ = json!({ "decoded": string.as_str()});
                    connection
                        .write(&to_vec(&answ).unwrap())
                        .expect("Write Error");
                }
                _ => println!("Error"),
            }
        }
        let event = connection.read().expect("Read Error");
        match event {
            TelnetEvent::Data(buffer) => {
                println!("{:?}", buffer.clone().into_vec().into_ascii_string());
            }

            _ => println!("Error"),
        }
    }
}

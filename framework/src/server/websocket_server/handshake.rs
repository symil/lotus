use sha1::Sha1;

fn compute_accept_key(client_key: &str) -> String {
    let mut input = client_key.to_string();
    input.push_str("258EAFA5-E914-47DA-95CA-C5AB0DC85B11");

    let mut hasher = Sha1::new();
    hasher.update(input.as_bytes());

    let bytes = hasher.digest().bytes();
    let result = base64::encode(&bytes);

    result
}

pub fn compute_websocket_server_handshake(client_handshake: String) -> Option<String> {
    if !client_handshake.trim_start().starts_with("GET / HTTP/1.1") {
        return None;
    }

    let headers : Vec<(&str, &str)> = client_handshake.split("\r\n")
        .filter(|line| line.contains(':'))
        .map(|line| {
            let parts : Vec<&str> = line.split(':').collect();

            (parts[0].trim(), parts[1].trim())
        })
        .collect();
    
    let key = headers.iter().find_map(|(name, value)| {
        match *name {
            "Sec-WebSocket-Key" => Some(value),
            _ => None
        }
    });

    match key {
        Some(client_key) => {
            let accept_key = compute_accept_key(client_key);
            let response = format!("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\n\r\n", accept_key);

            Some(response)
        },
        None => None
    }
}
use websocket::ClientBuilder;
use websocket::message::OwnedMessage;


fn connect(i : i32) {
    let mut client = ClientBuilder::new("ws://127.0.0.1:8888")
    .unwrap()
    .connect_insecure()
    .unwrap();

    while let Ok(msg) = client.recv_message() {
        if let OwnedMessage::Text(t) = msg {
            println!("Client {}, received {}", i, t);
        }
    }
}

fn main() {
    std::thread::spawn ( move || {
        connect(2);
    });
    connect(1);
}
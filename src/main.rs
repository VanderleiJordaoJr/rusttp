extern crate serde;
extern crate serde_json;

use rusttp::tcp_server::TCPServer;
use rusttp::http_enums::ResponseStatusCode;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let server = TCPServer::new("7878", 4, 8);

    server.listen(|_req, mut res| {
        res.send(ResponseStatusCode::OK);
    });
}

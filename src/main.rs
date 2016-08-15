extern crate hyper;

use hyper::server::{Server, Request, Response};

fn hello(_: Request, res: Response) {
    res.send(b"Hello world").unwrap();
}

fn main() {
    Server::http("0.0.0.0:1337").unwrap().handle(hello).unwrap();
}
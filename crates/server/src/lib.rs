extern crate futures;
extern crate hyper;

extern crate wasm_tutorial_shared;

use std::{
    fs::{read},
    io::{Result},
    path::{PathBuf},
};

use futures::future::ok;
use futures::{Future, Stream};
use hyper::{StatusCode, Error, Method};
use hyper::server::{Response, Request, Service};

use wasm_tutorial_shared::data::*;
use wasm_tutorial_shared::models::*;


const FOUR_O_FOUR: &str = include_str!("../../../dist/404.html");

struct Server;

type HyperResult = Box<Future<Item = Response, Error = Error>>;

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = HyperResult;
    fn call(&self, req: Request) -> HyperResult {
        if req.path() != "/todos" {
            return try_files(req);
        }
        match req.method() {
            &Method::Get => {
                get_todos(req)
            },
            &Method::Post => {
                todos(req)
            },
            _ => Box::new(ok(r500("Unknown route")))
        }
    }
}

pub fn try_files(req: Request) -> HyperResult {
    let base_path = if let Ok(p) = ::std::env::current_dir() {
        p.join("dist")
    } else {
        return Box::new(ok(r500("Error with file system")))
    };
    let path = req.path();
    let endpoint = if path.ends_with("/") {
        "index.html"
    } else if path.starts_with("/") {
        &path[1..]
    } else {
        &path
    };
    println!("base_path: {:?}",  &base_path);
    let file_path = base_path.join(endpoint);
    println!("file_path: {:?}", &file_path);
    Box::new(
        ok(if let Ok(r) = try_path(&file_path) {
            r
        } else if let Ok(r) = try_path(&file_path.join("/index.html")) {
            r
        } else {
            Response::new().with_status(StatusCode::NotFound).with_body(FOUR_O_FOUR.as_bytes())
        })
    )
}

fn try_path(path: &PathBuf) -> Result<Response> {
    println!("try_path {:?}", &path);
    match read(path) {
        Ok(bytes) => {
            Ok(Response::new().with_body(bytes))
        },
        Err(e) => {
            println!("Error reading file {:?}", e);
            Err(e)
        },
    }
}

pub fn start_server() {
    let addr = "127.0.0.1:8888".parse().expect("Unable to parse address");
    let http = hyper::server::Http::new().bind(&addr, move || Ok(Server)).expect("Unable to create a server");
    let _ = http.run();
}

fn get_todos(_req: Request) -> HyperResult {
    Box::new(
        ok(
            handle_todo_route(Message::get_all())
        )
    )
}

fn todos(req: Request) -> HyperResult {
    Box::new(
        req.body()
            .concat2()
            .map(move |b| {
                match Message::from_bytes(b.as_ref().to_vec()) {
                    Ok(msg) => handle_todo_route(msg),
                    Err(e) => r500(format!("{:?}", e))
                }
        })
    )
}

fn handle_todo_route(message: Message) -> Response {
    println!("todo route {:?}", message);
    let mut data = match Data::new() {
        Ok(d) => d,
        Err(e) => return r500(format!("error getting data {:?}", e))
    };
    match message {
        Message::GetAll => {
            let todos = data.get_todos();
            let body = Message::all(todos).to_bytes();
            Response::new().with_body(body)
        },
        Message::Add(ref todo) => {
            match data.add(todo) {
                Ok(updated) => {
                    let body = Message::all(updated).to_bytes();
                    Response::new().with_body(body)
                },
                Err(e) => r500(format!("Err getting updated {:?}", e))
            }
        },
        Message::Update(ref todo) => {
            match data.update(todo) {
                Ok(updated) => {
                    let body = Message::all(updated).to_bytes();
                    Response::new().with_body(body)
                },
                Err(e) => r500(format!("Err getting updated {:?}", e))
            }
        },
        Message::Remove(todo) => {
            match data.remove(todo.id) {
                Ok(updated) => {
                    let body = Message::all(updated).to_bytes();
                    Response::new().with_body(body)
                },
                Err(e) => r500(format!("Error getting updated {:?}", e))
            }
        },
        _ => r500("I don't care about client errors"),
    }
}

fn r500(msg: impl ToString) -> Response {
    Response::new().with_status(StatusCode::BadRequest).with_body(msg.to_string())
}
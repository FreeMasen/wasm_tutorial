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
        println!("REQ: {}-{}", req.method(), req.path());
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

pub fn start_server(port: &str) {
    let addr = format!("0.0.0.0:{}", port).parse().expect("Unable to parse address");
    let http = hyper::server::Http::new();
    let server = match http.bind(&addr, move || Ok(Server)) {
        Ok(s) => {
            println!("Successfully bound server to {}", &addr);
            s
        },
        Err(e) => {
            eprintln!("Unable to create a server {:?}", e);
            return
        },
    };
    println!("starting server");
    match server.run() {
        Ok(_) => (),
        Err(e) => println!("Error starting server {:?}", e)
    }
}

fn get_todos(_req: Request) -> HyperResult {
    Box::new(
        ok(
            match handle_message(Message::GetAll) {
                Ok(msg) => Response::new().with_body(msg.to_bytes()),
                Err(e) => r500(format!("Unable to get all: {}", e.msg)),
            }
        )
    )
}

fn todos(req: Request) -> HyperResult {
    Box::new(
        req.body()
            .concat2()
            .map(move |b| {
                match Message::from_bytes(b.as_ref().to_vec()) {
                    Ok(msg) => {
                        match handle_message(msg) {
                            Ok(message) => {
                                let buf = message.to_bytes();
                                Response::new().with_body(buf)
                            },
                            Err(e) => r500(e.msg),
                        }

                    },
                    Err(e) => r500(format!("{:?}", e))
                }
        })
    )
}

fn handle_message(message: Message) -> DataResult<Message> {
    println!("todo route {:?}", message);
    let mut data = match Data::new() {
        Ok(d) => d,
        Err(e) => return Err(DataError::new(format!("error getting data {:?}", e)))
    };
    match message {
        Message::GetAll => {
            let todos = data.get_todos();
            Ok(Message::All(todos))
        },
        Message::Add(ref todo) => {
            let updated = data.add(todo)?;
            Ok(Message::All(updated))
        },
        Message::Update(ref todo) => {
            let updated = data.update(todo)?;
            Ok(Message::All(updated))
        },
        Message::Remove(id) => {
            let updated = data.remove(id)?;
            Ok(Message::All(updated))
        },
        _ => Err(DataError::new("I don't care about client errors")),
    }
}

fn r500(msg: impl ToString) -> Response {
    Response::new().with_status(StatusCode::BadRequest).with_body(msg.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::remove_file;
    #[test]
    fn route() {
        let res = handle_message(Message::GetAll).unwrap();
        if let Message::All(_) = res {
            let _ = remove_file("data.bincode");
        } else {
            panic!("Incorrect message returned for get all route")
        }
        let add_res = handle_message(Message::Add(ToDo::new("write tests".into()))).unwrap();
        let add_todos = if let Message::All(todos) = add_res {
            assert!(todos.len() > 0);
            todos
        } else {
            panic!("Add did not return All")
        };
        let mut todo = add_todos[0].clone();
        todo.complete = true;
        let update_res = handle_message(Message::Update(todo)).unwrap();
        let update_todos = if let Message::All(todos) = update_res {
            assert_eq!(add_todos.len(), todos.len());
            todos
        } else {
            panic!("Update did not return All")
        };
        let todo = update_todos[0].clone();
        let remove_res = handle_message(Message::Remove(todo.id)).unwrap();
        if let Message::All(todos) = remove_res {
            assert!(todos.len() < update_todos.len());
        } else {
            panic!("Remove did not return All")
        }
        let _ = remove_file("data.bincode");
    }
}
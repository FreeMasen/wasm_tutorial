extern crate futures;
extern crate hyper;
extern crate pony;

extern crate wasm_tutorial_shared;


use futures::future::ok;
use futures::{Future, Stream};
use hyper::{StatusCode};
use hyper::server::{Response, Request, NewService};
use pony::HyperResult;
use pony::pony_builder::PonyBuilder;

use wasm_tutorial_shared::data::*;
use wasm_tutorial_shared::models::*;

pub fn start_server() {
    let addr = "127.0.0.1:8888".parse().expect("Unable to parse address");
    let mut pb = PonyBuilder::new();
    pb.use_static("dist");
    pb.get("/todos", get_todos);
    pb.post("/todos", todos);
    pb.add_known_extension(&["wasm"]);
    pb.use_static_logging();
    let server = pony::hyper::server::Http::new().bind(&addr, move || pb.new_service()).expect("Unable to bind server");
    let _ = server.run();
}

fn get_todos(_req: Request) -> HyperResult {
    Box::new(
        ok(
            handle_todo_route(Message::get_all_client())
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
    println!("todo route {:?}", message.kind);
    let mut data = match Data::new() {
        Ok(d) => d,
        Err(e) => return r500(format!("error getting data {:?}", e))
    };
    match message.kind {
        MessageType::GetAll => {
            println!("get all message");
            let todos = data.get_todos();
            println!("Attempting to send {} todos", todos.len());
            let body = Message::get_all_server(&todos).to_bytes();
            println!("response body as {} bytes", body.len());
            Response::new().with_body(body)
        },
        MessageType::Add => {
            println!("add message");
            match message.todos() {
                Ok(mut todos) => {
                    let body = if let Some(todo) = todos.get_mut(0) {
                        let updated = data.add(todo);
                        Message::add_server(updated).to_bytes()
                    } else {
                        Message::for_error("Unable to get 1st element of array".into()).to_bytes()
                    };
                    Response::new().with_body(body)
                },
                Err(e) => {
                    println!("Error getting todos {:?}", e);
                    r500(format!("{:?}", e))
                },
            }
        },
        MessageType::Update => {
            println!("update message");
            match message.todos() {
                Ok(todos) => {
                    let body = if let Some(todo) = todos.get(0) {
                        let updated = data.update(todo);
                        Message::update_server(updated).to_bytes()
                    } else {
                        Message::for_error("Unable to get 1st element of array".into()).to_bytes()
                    };
                    Response::new().with_body(body)
                },
                Err(e) => r500(format!("{:?}", e)),
            }
        },
        MessageType::Remove => {
            println!("remove message");
            match message.todos() {
                Ok(todos) => {
                    let body = if let Some(todo) = todos.get(0) {
                        let updated = data.remove(todo.id);
                        Message::remove_server(updated).to_bytes()
                    } else {
                        Message::for_error("Unable to get the 1st element of the array".into()).to_bytes()
                    };
                    Response::new().with_body(body)
                },
                Err(e) => r500(format!("{:?}", e))
            }
        },
        MessageType::Error => r500("I don't care about client errors".into()),
    }
}

fn r500(msg: String) -> Response {
    Response::new().with_status(StatusCode::BadRequest).with_body(msg.to_owned())
}
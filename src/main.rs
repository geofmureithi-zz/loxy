extern crate env_logger;

#[macro_use]
extern crate log;

extern crate simple_server;

use simple_server::{Method, Server, StatusCode, ResponseBuilder, Response};
use std::cell::RefCell;


#[derive(Clone)]
pub struct Route {
    method: Method,
    path: String,
    handler: String
}

impl Route {
    pub fn new(path: &str, handler: String) -> Route{
        Route {
            method: Method::GET,
            path: String::from(path),
            handler
        }
    }
}

#[derive(Clone)]
pub struct App<'a>{
    routes: Vec<Route>,
    server: &'a Server
}

impl <'a> App<'a> {
    pub fn new() -> App<'a>{
        App {
            routes: Vec::new(),
            server: &Server::new(|request, mut response| {
                response.status(StatusCode::IM_A_TEAPOT);
                Ok(response.body("<h1>404</h1><p>Not found!<p>".as_bytes().to_vec())?)
            })
        }
    }

    fn find_route(&mut self, method: &Method, path : &str) -> Result<Route, &str> {
        for (i, route) in self.routes.into_iter().enumerate() {
            if route.method.eq(&method) && route.path.eq(&path) {
                return Ok(route);
            }
        }
        Err("Failed")
    }

    pub fn get(&mut self, path: &str, response: String){
        let route: Route = Route::new(path, response);
        self.routes.push(route);
    }

    pub fn start<'b>(&mut self){
        let host = "127.0.0.1";
        let port = "7878";
        self.server =  &Server::new(|request, mut response| {
            info!("Request received. {} {}", request.method(), request.uri());
            info!("Request received. {} ", (self.find_route(request.method(), request.uri().path())).is_ok());
            match (request.method(), request.uri().path()) {
                (&Method::GET, "/hello") => {
                    Ok(response.body("<h1>Hi!</h1><p>Hello Rust!</p>".as_bytes().to_vec())?)
                }
                (_, _) => {
                    response.status(StatusCode::NOT_FOUND);
                    Ok(response.body("<h1>404</h1><p>Not found!<p>".as_bytes().to_vec())?)
                }
            }
        });
        self.server.listen(host, port);
    }
}


fn main() {
    let mut app: App = App::new();
    fn home() -> String{
        String::from("<h1>Hi!</h1><p>Hello Rust!</p>")
    }
    app.get("/path", home());
    app.start();

}
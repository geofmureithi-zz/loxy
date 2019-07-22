extern crate env_logger;

#[macro_use]
extern crate log;

extern crate simple_server;
extern crate tokio;
extern crate futures;


use simple_server::{Method, Server, StatusCode};
use crossbeam_utils::thread;



fn find_route<'a>(routes : &'a Vec<Route>, method: &Method, path : &str) -> Result<&'a Route, &'a str> {
    for (i, route) in routes.iter().enumerate() {
        if route.method.eq(&method) && route.path.eq(&path) {
            return Ok(route);
        }
    }
    Err("Failed")
}


#[derive(Debug, Clone)]
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

pub struct App{
    routes: Vec<Route>,
    server: Option<Server>
}

impl App {
    pub fn new() -> App{
        App {
            routes: Vec::new(),
            server: None
        }
    }


    pub fn get(&mut self, path: &str, response: String){
        let route: Route = Route::new(path, response);
        self.routes.push(route);
    }

    pub fn start(&mut self){
        let host = "127.0.0.1";
        let port = "7878";
        let routes = self.routes.clone();
        let server = Server::new( move |request, mut response| {
            info!("Request received. {} {}", request.method(), request.uri());
                thread::scope(|s| {
                s.spawn(move |_|{
                    match find_route(&routes, request.method(), request.uri().path()) {
                        Ok(h) => {
                            Ok(response.body(h.handler.as_bytes().to_vec())?)
                        }
                        Err(e) => {
                            response.status(StatusCode::NOT_FOUND);
                            Ok(response.body(format!("<h1>404</h1><p>{}<p>", e).as_bytes().to_vec())?)
                        }
                    }
                }).join()
            }).unwrap()

        });
        self.server = Some(server);
        self.server.as_ref().unwrap().listen(host, port);
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
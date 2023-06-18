//! web_server is a small, dependency-less crate for creating HTTP servers.
//! When you coding the backend using Rust,
//! the most annoying thing could be the size of a freamwork
//! and the time needed to compile the application
//!
//! The web_server package fixes these problems.
//! web_server has no dependencies, but allows you to create full-fledged servers
//!
//!
//! First server using web_server
//! ```
//! use web_server::route;
//!
//! web_server::new()
//!    .get("/", Box::new(|request: web_server::Request, mut response: web_server::Response|
//!         "Hello World!".into()))
//!    .launch(80)
//!    .unwrap();
//! ```
//!
//! It's easy!
//! First you must create instance of HttpServer
//! ```
//! web_server::new()
//! ```
//! then you can declare your endpoints. E.g.
//! ```
//! .get("/your/path", Box::new(|request, default_response| {
//!     // There place your logic
//!     // This function returns Response
//!     "response text".into()
//! }))
//! .post("/your/path", Box::new(|_, _| "Handler for POST method".into()))
//! .route(web_server::HttpMethod::DELETE, "/your/path", Box::new(|_, _| "Handler for DELETE method".into()))
//! .any("/your/path", Box::new(|_, _| "Handler for any method"))
//! ```
//!
//! Now you must run server by launch method
//! ```
//! .launch(PORT).unwrap()
//! ```
//!
//! You can send files to client e.g.
//! ```
//! .get("/image.png", Box::new(|_, _| {
//!     std::path::Path::new("path to your file").into();
//! }))
//! ```
mod http_code;
mod http_route;
mod http_version;
mod request;
mod response;
mod server;
mod server_utils;

/// Decoders for http request body
pub mod decoders {
    pub use crate::request::x_www_form_urlencoded;
}

pub mod utils {
    pub use crate::server_utils::redirect;
}

pub use http_code::HttpCode;
pub use http_route::{HttpMethod, HttpRoute};
use http_version::HttpVersion;
pub use request::Request;
pub use response::{Body, Response};
pub use server::HttpServer;
pub use server::RequestHandler;

/// Create new instance of HttpServer
pub fn new<'a>() -> HttpServer<'a> {
    HttpServer::new()
}

/// Create new instance of HttpServer with predefined body
pub fn create_server<'a>(default_repsonse: Response) -> HttpServer<'a> {
    let mut ret = HttpServer::new();
    ret.default_repsonse = default_repsonse;
    ret
}

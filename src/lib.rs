#![allow(unused_features)]
#![feature(core)]

extern crate iron;

use iron::prelude::*;
use iron::Handler;

pub struct Defer<H1, H2> {
    handler_a: Box<H1>,
    handler_b: Box<H2>,
}

impl<H1: Handler, H2: Handler> Defer<H1, H2> {
	pub fn using(handler_a: H1, handler_b: H2) -> Defer<H1, H2> {
		Defer {
			handler_a: Box::new(handler_a),
			handler_b: Box::new(handler_b),
		}
	}
}

impl<H1: Handler, H2: Handler> Handler for Defer<H1, H2> {
	fn handle(&self, request: &mut Request) -> IronResult<Response> {
		self.handler_a.handle(request).or(
			self.handler_b.handle(request))
	}
}

#[cfg(test)]
mod tests {
	use super::Defer;

	use std::error::Error;
	use std::fmt;

	use iron::prelude::*;

	#[derive(Debug)]
	struct DummyError;

	impl Error for DummyError {
	    fn description(&self) -> &'static str { "DummyError" }
	}

	impl fmt::Display for DummyError {
	    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	        f.write_str(self.description())
	    }
	}

	#[test]
	fn it_works() {
		use iron::status;

		fn hello_world(_: &mut Request) -> IronResult<Response> {
			// Ok(Response::with((status::Ok, "Hello world")))
			Err(IronError::new(DummyError, status::NotFound))
		}

		fn about(_: &mut Request) -> IronResult<Response> {
			Ok(Response::with((status::Ok, "About")))
		}

	    let defer = Defer::using(hello_world, about);

	    Iron::new(defer).listen("localhost:3000").unwrap();
	}
}
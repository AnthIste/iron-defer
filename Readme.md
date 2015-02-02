# iron-defer

Attempt at combining handlers for the Rust web framework [iron](https://github.com/iron/iron).

## Usage

    use std::error::Error;
    use std::fmt;

    use iron::prelude::*;
    use iron::status;

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
        /// Returning `Ok` uses the original response
        /// Returning `Err` makes the `about` handler take over.
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
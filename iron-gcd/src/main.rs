extern crate iron;

use iron::prelude::*;
use iron::{status,mime};

fn main() {
    let mut router = router::Router::new();

    router.get("/", get_form, "root");

    println!("Serving on http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mime: mime::Mime = "text/html".parse().unwrap();

    let body = r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="POST">
            <input type="number" name="n">
            <input type="number" name="m">
            <button>Compute GCD</button>
        </form>
    "#;

    Ok(Response::with((status::Ok, body, mime)))
}

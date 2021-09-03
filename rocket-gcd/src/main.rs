#[macro_use] extern crate rocket;
use rocket::form;
use rocket::response::content;

#[get("/")]
fn index() -> content::Html<&'static str> {
    let body = r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="POST">
            <input type="number" name="m">
            <input type="number" name="n">
            <button>Compute GCD</button>
        </form>
    "#;

    content::Html(body)
}

#[derive(Debug, FromForm)]
struct GcdOperands {
    m: u64,
    n: u64,
}

#[post("/gcd", data = "<operands>")]
fn post_gcd(operands: form::Form<GcdOperands>) -> String {
    format!("m = {}, n = {}", operands.m, operands.n)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, post_gcd])
}

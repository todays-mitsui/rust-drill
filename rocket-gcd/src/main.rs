#[macro_use] extern crate rocket;
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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

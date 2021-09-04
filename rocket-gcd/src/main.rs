#[macro_use] extern crate rocket;
use rocket::form;
use rocket::response::content;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, post_gcd])
}

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

fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(m != 0 && n != 0);

    while n != 0 {
        if n < m {
            // swap
            let t = m;
            m = n;
            n = t;
        }
        n = n % m;
    }
    m
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(
        gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19),
        3 * 11
    );
}

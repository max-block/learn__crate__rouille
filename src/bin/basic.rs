#[macro_use]
extern crate rouille;

use rouille::Response;

fn main() {
    println!("starting server on localhost:3000");

    rouille::start_server("localhost:3000", move |request| {
        router!(request,
        (GET) (/) => {
            Response::redirect_302("/hello")
        },
        (GET) (/hello) => {
            Response::text("hi!")
        },
        (GET) (/name/{name: String}) => {
            Response::text(format!("hi, {}", name))
        },
        (GET) (/panic) => {
            panic!("AAAA!!!")
        },
        _ => Response::empty_404()
        )
    });
}

use rouille::Response;

fn main() {
    println!("starting server on localhost:3000");
    rouille::start_server("localhost:3000", move |_request| {
        println!("a new request");
        Response::text("it works")
    })
}

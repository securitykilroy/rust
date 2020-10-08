use warp::Filter;
use std::fs;

#[tokio::main]
async fn main() {

let bacon_contents = fs::read_to_string("bacon.txt")
    .expect("Unable to open bacon.txt file");

let bacon = warp::path("bacon").map(move || format!("{}", &bacon_contents));
let hello = warp::path!("hello" / "you").map(|| "Hello, you\n"); 

let bye = warp::path("bye")
        .and(warp::path::param())
        .map(|name: String| format!("Good bye, {}!\n", name));

let routes = warp::get().and(bacon.or(hello).or(bye));

warp::serve(routes)
    .run(([127, 0, 0, 1], 8080))
    .await;

}

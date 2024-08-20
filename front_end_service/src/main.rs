#[macro_use] extern crate rocket;

use reqwest;

#[get("/")]
async fn index() -> &'static str {
    "Hello, I am working!"
}

#[get("/single_request")]
async fn single_request() -> String {
	let response = reqwest::get("http://localhost:8000")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    response
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, single_request])
}

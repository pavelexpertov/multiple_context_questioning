#[macro_use] extern crate rocket;

use core::future::Future;

use rocket::form::Form;
use rocket::State;
use reqwest::{self, Error, Response, Client};

#[derive(FromForm)]
struct UserRequest {
    question: String
}

#[post("/prompt", data="<user_input>")]
async fn prompt(user_input: Form<UserRequest>) -> String {
    let response = send_single_prompt(&user_input.question).await.unwrap().text().await.unwrap();
    response
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
    rocket::build().mount("/", routes![prompt, single_request])
}

fn send_single_prompt(prompt: &str) -> impl Future<Output = Result<Response, Error>> {
    let params = [("question", prompt)];
    let client = Client::new();
	client.post("http://localhost:8000/single_prompt")
        .form(&params)
        .send()
}


//GET /single_prompt HTTP/1.1
//Accept: *[>
//Accept-Encoding: gzip, deflate
//Connection: keep-alive
//Content-Length: 28
//Content-Type: application/x-www-form-urlencoded; charset=utf-8
//Host: localhost:8000
//User-Agent: HTTPie/2.6.0

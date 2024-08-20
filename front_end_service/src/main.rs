#[macro_use] extern crate rocket;

use core::future::Future;

use rocket::form::Form;
use rocket::State;
use rocket::tokio;
use reqwest::{self, Error, Response, Client};

#[derive(FromForm)]
struct MultipleContextPromptRequest {
    question: String,
    contexts: String
}

#[post("/multiple_context_prompt", data="<user_input>")]
async fn multiple_context_prompt(user_input: Form<MultipleContextPromptRequest>) -> String {
    let contexts: Vec<&str> = user_input.contexts
        .split(',')
        .map(|context| { context.trim() })
        .collect();

    let question = if let Some(removed_question_mark) = user_input.question.strip_suffix("?") {
        removed_question_mark
    } else {
        &user_input.question
    };

    let futures = vec![];
    for context in contexts.iter() {
        let context_prompt: String = format!("{question} in a context of {context}");
        let future = tokio::spawn(async move {
            send_single_prompt(&context_prompt)
        });
        futures.push(future);
    }

    let responses: Vec<String> = vec![];
    for future in futures {
        let response = future.await.unwrap().await;
        responses.push(response);
    }

    let response: String = String::from("");
    for (returned_response, context) in responses.iter().chain(contexts.iter()) {
        let formatted_string = format!("Context {context}: {returned_response}\n\n");
        response.push_str(&formatted_string);
    }

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
    rocket::build().mount("/", routes![multiple_context_prompt, single_request])
}

async fn send_single_prompt(prompt: &str) -> String {
    let params = [("question", prompt)];
    let client = Client::new();
	client.post("http://localhost:8000/single_prompt")
        .form(&params)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}



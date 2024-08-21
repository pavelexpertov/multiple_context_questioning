#[macro_use] extern crate rocket;

use core::future::Future;

use rocket::form::Form;
use rocket::State;
use rocket::tokio;
use rocket_dyn_templates::{Template, context};
use reqwest::{self, Error, Response, Client};

#[derive(FromForm)]
struct MultipleContextPromptRequest {
    question: String,
    contexts: String
}

#[post("/multiple_context_prompt", data="<user_input>")]
async fn multiple_context_prompt(user_input: Form<MultipleContextPromptRequest>) -> String {
    let contexts: Vec<String> = user_input.contexts
        .split(',')
        .map(|context| { context.trim().to_string() })
        .collect();

    let question = if let Some(removed_question_mark) = user_input.question.strip_suffix("?") {
        removed_question_mark
    } else {
        &user_input.question
    };

    let mut futures = vec![];
    for context in contexts.iter() {
        let context_prompt: String = format!("{question} in a context of {context}");
        let future = tokio::spawn(async move {
            let context_prompt = context_prompt;
            send_single_prompt(&context_prompt).await
        });
        futures.push(future);
    }

    let mut responses: Vec<String> = vec![];
    for future in futures {
        let response = future.await.unwrap();
        responses.push(response);
    }

    let mut response: String = String::from("");
    for (returned_response, context) in responses.iter().zip(contexts.iter()) {
        let formatted_string = format!("Context {context}: {returned_response}\n\n");
        response.push_str(&formatted_string);
    }

    response
}

#[get("/")]
async fn index() -> Template {
    Template::render("index", context! {})
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
    rocket::build()
        .mount("/", routes![multiple_context_prompt, single_request, index])
        .attach(Template::fairing())
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



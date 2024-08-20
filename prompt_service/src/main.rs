mod prompter;

use rocket::form::Form;
use rocket::State;

use prompter::Prompter;

#[macro_use] extern crate rocket;

#[derive(FromForm)]
struct PromptRequest {
    question: String
}

#[post("/single_prompt", data="<request_items>")]
fn single_request_post(request_items: Form<PromptRequest>, prompter: &State<Prompter>) -> String {
    let response = prompter.prompt(&request_items.question);
    let t = format!("Prompt's reseponse: {response}");
    println!("{}", t);
    t
}

#[get("/single_prompt", data="<request_items>")]
fn single_request(request_items: Form<PromptRequest>, prompter: &State<Prompter>) -> String {
    let response = prompter.prompt(&request_items.question);
    let t = format!("Prompt's reseponse: {response}");
    println!("{}", t);
    t
}

#[launch]
fn rocket() -> _ {
    let p = Prompter::initialise();
    rocket::build()
        .manage(p)
        .mount("/", routes![single_request, single_request_post])
}

mod prompter;

use rocket::State;

use prompter::Prompter;

#[macro_use] extern crate rocket;

#[get("/")]
fn index(prompter: &State<Prompter>) -> String {
    let response = prompter.prompt();
    format!("Prompt's reseponse: {response}")
}

#[launch]
fn rocket() -> _ {
    let p = Prompter::initialise();
    //println!("{}", p.prompt());
    rocket::build()
        .manage(p)
        .mount("/", routes![index])
}

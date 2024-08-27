#[macro_use]




extern crate rocket;

#[get("/helloworld")]
fn hello() -> String {
    format!("Hello!")
}






#
[launch]
fn rocket() -> _ {

   rocket::build().mount("/", routes![hello])
}

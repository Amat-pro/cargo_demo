use rocket::get;

#[get("/hello-world")]
pub fn index() -> &'static str {
    "Hello, World !"
}
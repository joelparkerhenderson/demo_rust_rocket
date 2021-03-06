// Demo of the simplest route and handler.
// When a browser does a HTTP GE request to "/"
// then this handler prints "hello world".

#[get("/hello")]
pub fn hello() -> &'static str {
    "hello world"
}

#[cfg(test)]
mod tests {

    use crate::rocketeer;
    use rocket::local::Client;
    use rocket::http::{ContentType, Status};

    #[test]
    fn hello() {
        let client = Client::new(rocketeer()).expect("rocketeer");
        let mut response = client.get("/hello").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.body_string(), Some("hello world".into()));
    }

}
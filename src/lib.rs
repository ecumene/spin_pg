use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
    outbound_http::send_request,
};

fn get_animals() -> String {
    std::str::from_utf8(
        send_request(
            http::Request::builder()
                .method("GET")
                .uri("http://localhost:3000/animals")
                .body(None)
                .expect("Postgrest: Request build error."),
        )
        .expect("Postgrest: Network error.")
        .body()
        .as_ref()
        .expect("Postgrest: Empty body.")
        .as_ref(),
    )
    .expect("Not utf8")
    .into()
}

/// A simple Spin HTTP component.
#[http_component]
fn hello_world(_: Request) -> Result<Response> {
    Ok(http::Response::builder()
        .status(200)
        .body(Some(get_animals().into()))?)
}

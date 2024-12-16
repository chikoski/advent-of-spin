mod bindings;

use std::error::Error;

use percent_encoding::percent_decode_str;
use spin_sdk::http::{IntoResponse, Params, Request, Response, Router};
use spin_sdk::http_component;
use bindings::deps::chikoski::advent_of_spin::naughty_or_nice_calculatorable::calculate;

/// A simple Spin HTTP component.
#[http_component]
fn handle_naughty_or_nice(req: Request) -> Response {
    let mut router = Router::new();
    router.get("/api/naughty-or-nice/:name", get_naughty_or_nice_score);    
    router.handle(req)
}

fn get_naughty_or_nice_score(
    _: Request,
    params: Params,
) -> anyhow::Result<impl IntoResponse, Box<dyn Error>> {
    let name = params
        .get("name")
        .ok_or::<Box<dyn Error>>("No ID is specified".into())?;
    let name = percent_decode_str(name).decode_utf8()?;
    let score = calculate(&name);
    let json = serde_json::to_string(&score)?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(json)
        .build())
}

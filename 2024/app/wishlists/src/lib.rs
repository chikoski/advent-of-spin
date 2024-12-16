mod model;

use model::Kid;
use spin_sdk::http::{IntoResponse, Params, Request, Response};
use spin_sdk::key_value::Store;
use spin_sdk::{http_component, http_router};
use serde_json;

/// A simple Spin HTTP component.
#[http_component]
fn handle_whishlists(req: Request) -> Response {
    let method = req.method().clone();
    let path = req.path();
    println!("request {method:?}: {path:?}");
    let router = http_router! {
        GET "/api/wishlists" => get_wishlists,
        POST "/api/wishlists" => post_wishlists
    };
    router.handle(req)
}

fn get_wishlists(_: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let store = Store::open_default()?;
    let kids:Vec<Kid> = store.get_keys()?.iter().map(|key|{
        match store.get_json::<Kid>(key) {
            Ok(kid) => kid,
            _ => None
        }
    }).filter(|i| i.is_some()).map(|i| i.unwrap()).collect();
    let json = serde_json::to_string(&kids)?;
        Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(json)
        .build())
}

fn post_wishlists(req: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let store = Store::open_default()?;
    let kid:Kid = serde_json::from_slice(req.body())?;
    store.set_json(&kid.name, &kid)?;
    let response = Response::builder().status(201).build();
    Ok(response)
}



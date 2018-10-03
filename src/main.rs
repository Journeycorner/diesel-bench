#[macro_use]
extern crate serde_derive;

use actix_web::{server, App, HttpRequest, Json, Responder};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use rayon::prelude::ParallelIterator;
use std::iter;

fn recipies(_req: &HttpRequest) -> impl Responder {
    let results: Vec<Recipe> = generate();
    Json(results)
}

fn main() {
    server::new(|| App::new().resource("/", |r| r.f(recipies)))
        .bind("127.0.0.1:8000")
        .expect("Can not bind to port 8000")
        .run();
}

fn generate() -> Vec<Recipe> {
    rayon::iter::repeatn((), 1_000_000)
        .map(|()| {
            iter::repeat(())
                .take(100)
                .map(|()| thread_rng().sample(Alphanumeric))
                .collect()
        })
        .map(|text| Recipe { text })
        .collect()
}

#[derive(Serialize)]
struct Recipe {
    text: String,
}

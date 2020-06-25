use async_std::prelude::*;
use async_std::task;
use database::postgres::Postgres;
use dotenv::dotenv;
use std::fs;
use tide::{log, Response, StatusCode};

mod error_response;
mod handlers;
mod state;

use state::State;

fn main() -> tide::Result<()> {
    task::block_on(async {
        femme::ndjson::Logger::new()
            .start(log::Level::Info.to_level_filter())
            .unwrap();
        dotenv().ok();

        let (db, ()) = Postgres::new().join(database::migration::run()).await;

        let state = State::new(Box::new(db)).await?;
        let mut app = tide::with_state(state);

        app.at("/pets").get(handlers::get_pets);
        app.at("/pet").post(handlers::create_pet);
        app.at("/pet/:id").get(handlers::get_pet);

        app.at("/healthz")
            .get(|_| async { Ok(Response::new(StatusCode::Ok)) });
        app.at("/oas")
            .get(|_| async { Ok(fs::read_to_string("files/oas/v1.yaml")?) });

        app.listen("127.0.0.1:8181").await?;
        Ok(())
    })
}

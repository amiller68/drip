use std::convert::TryFrom;

mod app;

use app::{AppState, Args, Op, Parser};

#[tokio::main]
async fn main() {
    // Run the app and capture any errors
    let args = Args::parse();
    let state = AppState::try_from(&args).expect("valid state");
    let op = args.command.clone();
    match op.execute(&state).await {
        Ok(r) => println!("{}", r),
        Err(e) => {
            eprintln!("{}", e);
        }
    };
}

mod app;

use app::{Args, Op, Parser};

#[tokio::main]
async fn main() {
    // Run the app and capture any errors
    let args = Args::parse();
    let op = args.command.clone();
    match op.execute(&args).await {
        Ok(r) => println!("{}", r),
        Err(e) => {
            eprintln!("{}", e);
        }
    };
}

use env_logger::Env;
use log::info;
use structopt::StructOpt;
use warp::Filter;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "notes",
    about = "An App for keeping track of my thoughts in a semi-structued way"
)]
struct Args {
    /// Make the logging loud and annoying
    #[structopt(short, long)]
    debug: bool,
    /// Port to listen on
    #[structopt(short, long, default_value = "9001")]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::from_args();

    // Setup Logging
    if args.debug {
        // If both RUST_LOG env variable and debug are given, choose env variable
        env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    } else {
        env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    }

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    info!("Running server on port {}", args.port);

    // Start Server
    warp::serve(hello).run(([127, 0, 0, 1], args.port)).await;
}

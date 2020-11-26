use anyhow::{Context, Result};
use clap::arg_enum;
use env_logger::Env;
use lib::routes::build_warp_routes;
use lib::service::{NotesService, RequestHandler};
use lib::storage::{MemoryNoteStore, MysqlNoteStore, NoteStore};
use log::info;
use structopt::StructOpt;

mod lib;

arg_enum! {
    #[derive(StructOpt, PartialEq, Debug)]
    pub enum Storage {
        Mysql,
        Memory,
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "notes",
    about = "An App for keeping track of my thoughts in a semi-structued way"
)]
struct Args {
    /// Database URL to connect to
    #[structopt(
        long,
        default_value = "mysql://(host=db,port=3306,user=root,password=password)/test"
    )]
    database_url: String,
    /// Make the logging loud and annoying
    #[structopt(short, long)]
    debug: bool,
    /// Port to listen on
    #[structopt(short, long, default_value = "9001")]
    port: u16,
    #[structopt(long, possible_values = &Storage::variants(), case_insensitive = true, default_value="mysql")]
    storage_type: Storage,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::from_args();
    // Setup Logging
    if args.debug {
        // If both RUST_LOG env variable and debug are given, choose env variable
        env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    } else {
        env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    }

    // Initialize Storage Layer
    let storage = match args.storage_type {
        Storage::Mysql => {
            info!("Connecting to database at url: {}", args.database_url);
            let note_store =
                MysqlNoteStore::new(args.database_url).context("Initializing Database")?;
            Box::new(note_store) as Box<dyn NoteStore>
        }
        Storage::Memory => {
            info!("Using Memory Storage. Note, no notes will be saved!");
            Box::new(MemoryNoteStore::new()) as Box<dyn NoteStore>
        }
    };

    let handler = Box::new(RequestHandler::new(storage)) as Box<dyn NotesService>;
    let routes = build_warp_routes(handler);

    info!("Running server on port {}", args.port);
    // Start Server
    warp::serve(routes).run(([127, 0, 0, 1], args.port)).await;
    Ok(())
}

mod ua;
mod job;
mod pool;
mod worker;
mod stratum;
mod algorithm;

use std::error;

use clap::Parser;
use pool::Pool;

#[derive(Clone, Parser)]
struct AppArgs {
  #[arg(short='o', long)]
  pool: String,

  #[arg(short='u', long)]
  user: String,

  #[arg(short='p', long)]
  password: String
}

const DONATION: f32 = 0.25;
const DONATION_WALLET: &str = "42oMNQv295HEEFh7zHC22bKPeNnhBXY3bKyr3m6w7RucWod31gf8upvZt4HXwxaGmGh5e84CbH1kWSEgavojVtd3CRMqqP2";

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
  let args = AppArgs::parse();
  let pool = Pool::new(&args.pool, &args.user, &args.password).await?;

  Ok(())
}
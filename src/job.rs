use tokio::io;

use crate::{algorithm::Algorithm, stratum::{Stratum, StratumLoginResponse}};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct StratumSumbitParams {
  id: String,
  job_id: String,
  nonce: String,
  result: String
}

pub struct Job<'s> {
  pub(crate) stratum: &'s mut Stratum,
  pub(crate) blob: String,
  pub(crate) result_id: String,
  pub(crate) id: String,
  pub(crate) target: String,
  pub(crate) height: i32,
  pub(crate) seed_hash: String,
  pub(crate) algorithm: Algorithm
}

impl<'s> Job<'s> {
  pub async fn solve(&mut self, threads: u32) -> io::Result<()> {
     
    
    Ok(())
  }

  pub async fn submit(&mut self, nonce: &str, result: &str) -> io::Result<()> {
    let params = StratumSumbitParams { id: self.result_id.clone(), job_id: self.id.clone(), nonce: nonce.into(), result: result.into() };
    let params = serde_json::to_string(&params)?;

    let response = self.stratum.invoke_method("submit", &params).await?;
    let response = serde_json::from_str::<StratumLoginResponse>(&response)?;

    if let Some(error) = response.error {
      return Err(io::Error::new(io::ErrorKind::Other, format!("Error while sumbiting: {}", error.message)));
    }

    Ok(())
  }
}
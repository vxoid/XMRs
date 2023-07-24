use tokio::io;

use crate::{stratum::Stratum, ua, algorithm};

pub struct Pool {
  url: String,
  user: String,
  stratum: Stratum,
  password: String
}

impl Pool {
  pub async fn new(url: &str, user: &str, password: &str) -> io::Result<Self> {
    let user_agent = ua::create_user_agent();
    let algos = algorithm::ALGOS
      .iter()
      .cloned()
      .map(|(algo_name, algo)| (algo_name.to_string(), algo))
      .collect();

    let stratum = Stratum::new(url, &user_agent, algos).await?;
    let mut pool = Self { stratum, url: url.into(), user: user.into(), password: password.into() };

    pool.stratum.login(&pool.user, &pool.password).await?;
   
    Ok(pool)
  }
}
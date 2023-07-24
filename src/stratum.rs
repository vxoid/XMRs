use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net;
use tokio::io;

use crate::algorithm::Algorithm;
use crate::job::Job;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct StratumLoginParams {
  login: String,
  pass: String,
  agent: String,
  algo: Vec<String>
}

/// Struct which represents pool's response on login
/// 
/// # Example
/// {"id":1,
/// "jsonrpc":"2.0",
/// "error":null,
/// "result":{"id":"0",
/// "job":{"blob":"1010e197f3a506de9d4bbbdd9a776136ed4875ea530f78fffc81282f5ef8b54a0c6763a5a5a5ac00000000e76e87ff6c2bfba88cc7b626b2c064c1ca91528ef1475d67bab6894874c6fbf303",
/// "job_id":"0",
/// "target":"d3740100",
/// "height":2935911,
/// "seed_hash":"9141623a9ff9ea2e7fd1e521afe089e6c2e84d5c7e96c46e41159bfab044faf7",
/// "algo":"rx/0"},
/// "status":"OK"}}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StratumLoginResponse {
  pub(crate) id: usize,
  pub(crate) error: Option<StratumError>,
  pub(crate) result: Option<StratumResponseJobResult>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct StratumError {
  pub(crate) code: i32,
  pub(crate) message: String
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct StratumJobResponse {
  pub(crate) method: String,
  pub(crate) result: StratumResponseJobResult
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct StratumResponseJobResult {
  id: String,
  job: StratumResponseJob
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct StratumResponseJob {
  blob: String,
  job_id: String,
  target: String,
  height: i32,
  seed_hash: String,
  algo: String
}

pub struct Stratum {
  connection: net::TcpStream,
  user_agent: String,
  algos: Vec<(String, Algorithm)>,
  id: usize
}

impl Stratum {
  pub async fn new(url: &str, user_agent: &str, algos: Vec<(String, Algorithm)>) -> io::Result<Self> {
    let connection = net::TcpStream::connect(url).await?;
    
    Ok(Self { connection, user_agent: user_agent.into(), algos, id: 1 })
  }

  pub async fn login(&mut self, username: &str, password: &str) -> io::Result<Job> {
    let algo = self.algos.iter().cloned().map(|(algo_name, _)| algo_name).collect();
    let params = StratumLoginParams { login: username.into(), pass: password.into(), agent: self.user_agent.clone(), algo };

    let response = self.invoke_method("login", &serde_json::to_string(&params)?).await?;
    let response = serde_json::from_str::<StratumLoginResponse>(&response)?;

    if let Some(error) = response.error {
      return Err(io::Error::new(io::ErrorKind::Other, format!("Error while loging in: {}", error.message)));
    }

    let result = response.result.unwrap();
 
    let (_, algorithm) = self.algos.iter()
      .find(|(algo_name, _)| &result.job.algo == algo_name)
      .ok_or(io::Error::new(
        io::ErrorKind::NotFound,
        format!("Server responeded with {} algorithm which is currently not supported by {}.", result.job.algo, self.user_agent)
      ))?.clone();

    Ok(Job {
      stratum: self,
      blob: result.job.blob,
      result_id: result.id,
      id: result.job.job_id,
      target: result.job.target,
      height: result.job.height,
      seed_hash: result.job.seed_hash,
      algorithm
    })
  }

  pub async fn wait_for_job(&mut self) -> io::Result<Job> {
    let mut reader = io::BufReader::new(&mut self.connection);
    let mut buffer = Vec::new();

    reader.read_until(b'\n', &mut buffer).await?;
    let response = String::from_utf8_lossy(&buffer);
    println!("< {response}");

    let response = serde_json::from_str::<StratumJobResponse>(&response)?;

    if response.method != "job" {
      return Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!(r#"Got "{}" from server, while expecting job"#, response.method)
      ));
    }

    let result = response.result;

    let (_, algorithm) = self.algos.iter()
      .find(|(algo_name, _)| &result.job.algo == algo_name)
      .ok_or(io::Error::new(
        io::ErrorKind::NotFound,
        format!("Server responeded with {} algorithm which is currently not supported by {}.", result.job.algo, self.user_agent)
      ))?.clone();

    Ok(Job {
      stratum: self,
      blob: result.job.blob,
      result_id: result.id,
      id: result.job.job_id,
      target: result.job.target,
      height: result.job.height,
      seed_hash: result.job.seed_hash,
      algorithm
    })
  }

  pub async fn invoke_method(&mut self, method: &str, params: &str) -> io::Result<String> {
    let request = format!(r#"{{"id": {}, "jsonrpc": "2.0", "method": "{method}", "params": {params}}}{}"#, self.id, '\n');
    println!("> {request}");

    self.connection.write_all(request.as_bytes()).await?;

    let mut reader = io::BufReader::new(&mut self.connection);
    let mut buffer: Vec<_> = Vec::new();

    reader.read_until(b'\n', &mut buffer).await?;
    let response = String::from_utf8_lossy(&buffer);
    println!("< {response}");

    self.id += 1;

    Ok(response.to_string())
  }
}
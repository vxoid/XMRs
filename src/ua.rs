const APP_NAME: &str = "XMRig";
const APP_VERSION: &str = "6.20.0";

pub fn create_user_agent() -> String {
  format!("{APP_NAME}/{APP_VERSION}")
}
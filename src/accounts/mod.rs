pub mod predeployed_dev_accounts{
use serde::Deserialize;
use reqwest::Error;
#[derive(Deserialize, Debug)]
pub struct DevnetAccounts{
    pub address:String,
   pub initial_balance:f32,
   pub  private_key: String,
   pub  public_key:String
}
    pub async fn get_predeployed_devnet_accouns(url_ip: &str, port:&str)-> Result<(Vec<DevnetAccounts>), Error>{
        let request_url = format!("http://{url_ip}:{port}/predeployed_accounts");
        
        println!("requesting items from {}",request_url);

        let response  = reqwest::get(&request_url).await?;
        let accounts: Vec<DevnetAccounts> = response.json().await?;
        println!("{:#?}",accounts);

        Ok(accounts)
    }
}
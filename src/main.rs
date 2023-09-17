// use inquire::Text;

pub mod accounts;
use accounts::predeployed_dev_accounts::get_predeployed_devnet_accouns;
#[tokio::main]
async fn main(){
 let url_ip = "127.0.0.1";
 let port = "5051";
  let response =  get_predeployed_devnet_accouns(url_ip,port).await;

  println!("{:#?}",response)
}
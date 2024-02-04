use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;

error_chain!{
    foreign_links{
        RegError(reqwest::Error);
        IoError(std::io::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("https://www.rust-lang.org/en-US/")
      .await?
      .text()
      .await?;
  
    
  }
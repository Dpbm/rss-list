use std::path;
use std::fs;
use rss::Channel;
use clap::Parser;
use std::error::Error;

#[derive(Parser)]
struct Cli {
    feed: path::PathBuf,
}


#[tokio::main]
async fn main() {
    let args = Cli::parse();
    
    let feed: path::PathBuf = args.feed;

    let feed_content = fs::read_to_string(&feed).expect("Feed File not found!!!");
    let feed_list = feed_content.lines();
    
    for url in feed_list{
        let result = get_rss(url.to_string()).await;
        match result{
            Ok(channel) => {
                println!("{}",&channel.title);
                for item in &channel.items{
                    match &item.title{
                        Some(item_title) => println!("-> {}", &item_title),
                        None => println!("-> NULL")
                    } 
                }
            }
            Err(_) => println!("Error")
        }
    }
}

async fn get_rss(url:String) -> Result<Channel, Box<dyn Error>> {
    println!("getting from: {}",  url);
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?; 
    Ok(channel)
}

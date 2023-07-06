use std::path;
use std::fs;
use rss::Channel;
use clap::Parser;
use rss::Item;
use std::error::Error;

#[derive(Parser)]
struct Cli {
    #[arg(short='u', long="url")]
    url: Option<String>,
    #[arg(short='f', long="feed")]
    feed: Option<path::PathBuf>,
}

#[tokio::main]
async fn main(){
    let args = Cli::parse();
    
    match &args.url{
        Some(url) => get_one(url.to_string()).await,
        None => {}
    }
    
    match &args.feed{
        Some(feed) => get_multiple(feed.to_path_buf()).await,
        None => {}
    }
}    

    

async fn get_one(url:String){
    let result = get_rss(url).await;

    match result{
        Ok(channel) => {
            println!("{}",&channel.title);

            for item in &channel.items{
                show_item(&item);
            }
        },
        Err(_) => println!("Error")
    }

}

async fn get_multiple(feed: path::PathBuf){
    let feed_content = fs::read_to_string(feed).expect("Feed File not found!!!");
    let feed_list = feed_content.lines();
    
    for url in feed_list{
        get_one(url.to_string()).await;
    }
}

fn show_item(item:&Item){
    match &item.title{
        Some(title) => println!("----{}----", title),
        None => println!("NULL")
    }

    match &item.link{
        Some(url) => println!("{}", url),
        None => println!("NO LINK!")
    }

    println!("");
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

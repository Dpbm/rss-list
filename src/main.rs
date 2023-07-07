use std::path;
use std::fs;
use rss::Channel;
use clap::Parser;
use rss::Item;
use std::error::Error;
use colored::*;

#[derive(Parser)]
struct Cli {
    #[arg(short='u', long="url")]
    url: Option<String>,
    #[arg(short='f', long="feed")]
    feed: Option<path::PathBuf>,
    #[arg(short='l', long="limit", default_value_t=10)]
    limit: u8
}

#[tokio::main]
async fn main(){
    let args = Cli::parse();
    let limit = args.limit;

    match &args.url{
        Some(url) => get_one(url.to_string(), &limit).await,
        None => {}
    }
    
    match &args.feed{
        Some(feed) => get_multiple(feed.to_path_buf(), &limit).await,
        None => {}
    }
}    

    

async fn get_one(url:String, limit:&u8){
    let result = get_rss(url).await;
    match result{
        Ok(channel) => {
            println!("{}\n",&channel.title.magenta().underline());
            let mut i = 0;
            for item in &channel.items{
                if limit == &i{
                    break;
                }
                show_item(&item);
                i+=1;
            }
        },
        Err(_) => println!("{}", String::from("No data found!").red().bold())
    }

}

async fn get_multiple(feed: path::PathBuf, limit:&u8){
    let feed_content = fs::read_to_string(feed).expect("Feed File not found!!!");
    let feed_list = feed_content.lines();
    
    for url in feed_list{
        get_one(url.to_string(), limit).await;
    }
}

fn show_item(item:&Item){
    match &item.title{
        Some(title) => println!("{}", title.green().bold()),
        None => println!("{}",String::from("NULL").red().bold())
    }

    match &item.description{
        Some(description) => println!("{}", description.cyan()),
        None => println!("{}", String::from("NO DESCRIPTION").red().bold())
    }

    match &item.link{
        Some(url) => println!("{}", url.blue().italic()),
        None => println!("{}",String::from("NO LINK!").red().bold())
    }

    println!("");
}

async fn get_rss(url:String) -> Result<Channel, Box<dyn Error>> {
    println!("Getting data from: {}\n",  url.underline());
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;

    let channel = Channel::read_from(&content[..])?; 

    Ok(channel)
}

//use rss::Channel;
//use std::error::Error;

use clap::Parser;

mod cli;
mod database;

use crate::cli::Cli;
use crate::database::connect;

// TODO
// - Fetching
// -- Simple `$ rs-s` run will fetch all the urls or return error like no
//      added please add the links 


// POST PROCESSING
// -- Please set the real file location for the database path


fn main() {

    let cli = Cli::parse();
    //println!("rss: {:?}",&cli.links);

    if let Some(links) = &cli.links {
        let _ = connect::add_links(links);
    }

    if let Some(update) = &cli.update {
        let _ = connect::update_links(update);
    }   

    if let Some(delete) = &cli.delete {
        let _ = connect::delete_links(delete);
    }

    // ERROR ERROR THIS IS NOT WORKING
    if let Some(_views) = cli.view {
        let _ = connect::view_links();
    }


}






/*
async fn exam() -> Result<Channel, Box<dyn Error>> {
    let url = "https://techcrunch.com/feed";
    let content = reqwest::get(url).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}


#[tokio::main]
async fn main() {
    let ch = exam().await.unwrap();
    ch.items
        .iter()
        .for_each(|x| println!("{:#?}", x.title.as_ref().unwrap()));
}


use owo_colors::OwoColorize;

fn main() {
    println!("My number is {:#x}!", 10.green());
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1,2), 3);
    }

    #[test]
    #[should_panic]
    fn fail_add() {
        assert_ne!(add(1,99), 100);
    }
}

*/

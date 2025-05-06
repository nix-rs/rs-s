use clap::{Parser, Args, Subcommand, ValueEnum};

// Features
// -- help
// -- add
// -- remove
// -- edit
//

// Fetch
// -t titles
// -d dates wise
// -c catogries wise
//

#[derive(Parser)]
#[command(name= "rs-s")]
#[command(version = "0.1")]
#[command(about = "Fetch all given rss links", long_about = None)]
pub struct Cli {

    #[arg(num_args(0..9), short = 'a', help = "description", value_name= "websites")]
    //#[arg(default_value_t = 2020)]
    pub links: Option<Vec<String>>,


    // ERROR ERROR THIS IS NOT WORKING
    #[arg(num_args(0), short = 'v', help = "description", value_name= "websites")]
    //#[arg(default_value_t = 2020)]
    pub view: Option<bool>,

    #[arg(num_args(2), short = 'u', help = "<id> <new link>", value_name= "update")]
    //#[arg(default_value_t = 2020)]
    pub update: Option<Vec<String>>,

    #[arg(short = 'd', help = "delete the link", value_name= "update")]
    //#[arg(default_value_t = 2020)]
    pub delete: Option<i32>,

}



#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}


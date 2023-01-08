mod args;

use clap::Parser;
use args::Cli;
use args::Commands;

fn main() {

    let top = reqwest::blocking::get("https://myanimelist.net/topanime.php")
        .unwrap()
        .text()
        .unwrap();

    let top_anime_document = scraper::Html::parse_fragment(&top);

    let title_selector = scraper::Selector::parse("h3.hoverinfo_trigger").unwrap();

    let mut v = Vec::new();
    for x in top_anime_document.select(&title_selector){
        let x = x.text().collect::<Vec<_>>();
        v.push(x[0]);
    }

    let _args = Cli::parse();

    match &_args.command {
        Commands::TopTen(_top_ten) => {
            println!("Top Ten Anime");
            for i in 0..10 {
                println!("{}-{}",i+1,v[i]);
            }
        },
        Commands::TopFifty(_top_fifty) => {
            println!("Top Fifty Anime");
            for i in 0..50 {
                println!("{}-{}",i+1,v[i]);
            }
        },
        Commands::TopAiring(_) => {}
        Commands::TopUpcoming(_) => {}
    }
}

mod args;

use args::Cli;
use args::Commands;
use clap::Parser;

fn main() {
    let _args = Cli::parse();
    let v = get_top_anime();
    let y = get_top_airing();
    let z = get_top_upcoming();
    match &_args.command {
        Commands::TopTen(_top_ten) => {
            println!("Top Ten Anime");
            for i in 0..10 {
                println!("{}-{}", i + 1, v[i]);
            }
        }
        Commands::TopFifty(_top_fifty) => {
            println!("Top Fifty Anime");
            for i in 0..50 {
                println!("{}-{}", i + 1, v[i]);
            }
        }
        Commands::TopAiring(_) => {
            println!("Top Airing Anime");
            for i in 0..10 {
                println!("{}-{}", i + 1, y[i]);
            }
        }
        Commands::TopUpcoming(_) => {
            println!("Top Upcoming Anime");
            for i in 0..10 {
                println!("{}-{}", i + 1, z[i]);
            }
        }
    }

    fn get_top_anime() -> Vec<String> {
        let top = reqwest::blocking::get("https://myanimelist.net/topanime.php")
            .unwrap()
            .text()
            .unwrap();

        let top_anime_document = scraper::Html::parse_fragment(&top);

        let title_selector = scraper::Selector::parse("h3.hoverinfo_trigger").unwrap();
        let mut v: Vec<String> = Vec::new();

        for x in top_anime_document.select(&title_selector) {
            let x = x.text().collect::<Vec<_>>();
            v.push(x[0].to_string());
        }
        v
    }

    fn get_top_airing() -> Vec<String> {
        let top = reqwest::blocking::get("https://myanimelist.net/topanime.php?type=airing")
            .unwrap()
            .text()
            .unwrap();

        let top_anime_document = scraper::Html::parse_fragment(&top);

        let title_selector = scraper::Selector::parse("h3.hoverinfo_trigger").unwrap();
        let mut v: Vec<String> = Vec::new();

        for x in top_anime_document.select(&title_selector) {
            let x = x.text().collect::<Vec<_>>();
            v.push(x[0].to_string());
        }
        v
    }

    fn get_top_upcoming() -> Vec<String> {
        let top = reqwest::blocking::get("https://myanimelist.net/topanime.php?type=upcoming")
            .unwrap()
            .text()
            .unwrap();

        let top_anime_document = scraper::Html::parse_fragment(&top);

        let title_selector = scraper::Selector::parse("h3.hoverinfo_trigger").unwrap();
        let mut v: Vec<String> = Vec::new();

        for x in top_anime_document.select(&title_selector) {
            let x = x.text().collect::<Vec<_>>();
            v.push(x[0].to_string());
        }
        v
    }


}

mod args;
use clap::Parser;
use args::AniCLIArgs;

fn main() {
    let mut count = 1;
    let response = reqwest::blocking::get("https://myanimelist.net/topanime.php")
        .unwrap()
        .text()
        .unwrap();

    //println!("response = {:?}", response);

    let document = scraper::Html::parse_fragment(&response);

    let title_selector = scraper::Selector::parse("h3.hoverinfo_trigger").unwrap();

    //println!("test -> {:?}",document);
    let mut v = Vec::new();
    for x in document.select(&title_selector){
        let x = x.text().collect::<Vec<_>>();
        v.push(x[0]);
        println!("{}-{}",count,x[0]);
        count = count + 1;
    }
    let _args = AniCLIArgs::parse();
}

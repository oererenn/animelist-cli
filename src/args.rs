use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}


#[derive(Subcommand)]
pub enum Commands {
    /// Get the top ten anime
    TopTen(TopTen),
    /// Get the top fifty anime
    TopFifty(TopFifty),
    ///Get top airing anime
    TopAiring(TopAiring),
    ///Get top upcoming anime
    TopUpcoming(TopUpcoming),
}

#[derive(Args)]
pub struct TopTen {
    top10: Vec<String>,
}

#[derive(Args)]
pub struct TopFifty {
    top50: Vec<String>,
}

#[derive(Args)]
pub struct TopAiring {
    top_airing: Vec<String>,
}

#[derive(Args)]
pub struct TopUpcoming {
    topupcoming: Vec<String>,
}




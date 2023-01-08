use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug,Parser)]
#[clap(author,version,about)]
pub struct AniCLIArgs {
    pub first_arg: String,
    pub second_arg: String,
    pub third_arg: String,
}




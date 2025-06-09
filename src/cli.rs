use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub city: Option<String>,

    #[arg(short = 'D', long, value_name = "N_DAYS")]
    pub days: Option<i32>,

    #[arg(short = 'H', long, value_name = "N_HOURS")]
    pub hours: Option<i32>,
}

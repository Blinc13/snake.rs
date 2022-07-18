use clap::Parser;

#[derive(Parser)]
#[clap(about, author, version)]
pub struct Args {
    /// Width of the field
    #[clap(short, long, value_parser, default_value_t=50)]
    pub width: u32,

    /// Height of the field
    #[clap(short, long, value_parser, default_value_t=50)]
    pub height: u32,

    ///Size of pixel (if field size is 50x50 and pixel size is 10, then window size will be 500x500)
    #[clap(short, long, value_parser, default_value_t=10)]
    pub pixel_size: u32
}
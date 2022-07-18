use snake::{Game, Args};
use clap::Parser;

fn main() {
    let args = Args::parse();

    Game::new(args.width, args.height, args.pixel_size).start();
}

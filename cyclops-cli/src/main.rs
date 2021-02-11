use clap::Clap;
use cyclops;

#[derive(Clap)]
#[clap(version = "0.1", author = "Michael Simpson")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    SIFTDescriptors(SIFTDescriptors),
}

/// Calculate the SIFT descriptors for an image.
#[derive(Clap)]
struct SIFTDescriptors {
    /// The image filepath to calculate descriptors for.
    image: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::SIFTDescriptors(t) => sift_descriptors(t),
    }
}

fn sift_descriptors(opts: SIFTDescriptors) {
    let img = cyclops::load_img(&opts.image).unwrap();
    let descriptors = cyclops::get_descriptors(&img);
    println!("{:?}", descriptors)
}

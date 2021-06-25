use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1", author = "Michael Simpson")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Descriptors(Descriptors),
}

/// Calculate the descriptors for an image.
#[derive(Clap)]
struct Descriptors {
    /// The image filepath to calculate descriptors for.
    image: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Descriptors(t) => descriptors(t),
    }
}

fn descriptors(opts: Descriptors) {
    let descriptors = cyclops::get_descriptors(&opts.image).unwrap();
    println!("{:?}", descriptors)
}

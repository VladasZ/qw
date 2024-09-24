use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Args {
    New { name: String },
}

fn main() {
    let args = Args::from_args();

    dbg!(&args);

    dbg!("A");
}

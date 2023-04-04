use clap::Parser;


#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    //get arguments from clap parser
    let args = Cli::parse();
    //display args
    // println!("{} {}", args.pattern, args.path.display());

    // get the contents of the file

    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

}

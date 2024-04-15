use git2::opts;
use git2::Repository;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let url = args.get(1).unwrap();
    let into = args.get(2).unwrap();

    println!("Cloning {} => {}", url, into);

    Repository::clone(url, into).unwrap();

    println!("Clone finished.")
}

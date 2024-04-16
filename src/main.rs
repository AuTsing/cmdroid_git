use git2::build::RepoBuilder;
use git2::Cred;
use git2::FetchOptions;
use git2::RemoteCallbacks;
use std::env;
use std::path::Path;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let url = args.get(1).unwrap();
    let into = args.get(2).unwrap();

    println!("Cloning {} => {}", &url, &into);

    let mut remote_callbacks = RemoteCallbacks::new();
    remote_callbacks.credentials(|_, _, _| Cred::userpass_plaintext("", ""));

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(remote_callbacks);

    let mut repo_builder = RepoBuilder::new();
    repo_builder.fetch_options(fetch_options);

    repo_builder.clone(url, Path::new(into)).unwrap();

    println!("Clone finished.")
}

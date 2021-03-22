use git2::{Oid, Repository};
use std::env::current_dir;

/// Get timestamp from epoch by commit
pub fn epoch(id: &str) {
    let dir = current_dir().expect("Could not find current directory");
    let repo = Repository::open(&dir).expect(&format!("{:?} doesn't contain .git", dir));
    let commmit = repo
        .find_commit(Oid::from_str(id).expect(&format!("Doesn't have commit {}", id)))
        .expect(&format!("Could not find commit {}", id));

    println!("{:?}", commmit.time().seconds());
}

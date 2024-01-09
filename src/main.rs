use std::{fs::canonicalize, path::Path};

use ignore::{Walk, WalkBuilder, WalkState};

fn main() {
    let from_file = std::env::args().nth(1).expect("no start path given");
    let from_file = canonicalize(Path::new(&from_file)).unwrap();
    assert!(from_file.is_file());
    let from_dir = from_file.parent().unwrap();
    assert!(from_dir.is_dir());

    let pwd = &std::env::current_dir().unwrap();

    for ret in Walk::new(from_dir) {
        let entry = ret.unwrap();
        if entry.file_type().unwrap().is_file() {
            println!("{}", entry.path().strip_prefix(pwd).unwrap().display());
        }
    }

    let walker = WalkBuilder::new(pwd).build_parallel();
    walker.run(|| {
        Box::new(move |ret| match ret {
            Ok(entry) => {
                if entry.file_type().unwrap().is_file() && !entry.path().starts_with(from_dir) {
                    println!("{}", entry.path().strip_prefix(pwd).unwrap().display());
                }
                WalkState::Continue
            }
            Err(err) => {
                eprintln!("{err:?}");
                WalkState::Quit
            }
        })
    });
}

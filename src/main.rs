use std::{fs::canonicalize, path::Path};

use ignore::{WalkBuilder, WalkState};

fn main() {
    let from_file = std::env::args().nth(1).expect("no start path given");
    let from_file = canonicalize(from_file).unwrap();
    assert!(from_file.is_file());
    let from_dir = from_file.parent().unwrap();
    assert!(from_dir.is_dir());
    let git_dir = &canonicalize(".git").ok();

    let pwd = &std::env::current_dir().unwrap();

    for ret in WalkBuilder::new(from_dir)
        .hidden(false)
        .sort_by_file_path(Path::cmp)
        .build()
    {
        let entry = ret.unwrap();
        if entry.file_type().unwrap().is_file() {
            match git_dir {
                Some(git_dir) if entry.path().starts_with(git_dir) => {}
                _ => println!("{}", entry.path().strip_prefix(pwd).unwrap().display()),
            }
        }
    }

    let walker = WalkBuilder::new(pwd).hidden(false).build_parallel();
    walker.run(|| {
        Box::new(move |ret| match ret {
            Ok(entry) => {
                if entry.file_type().unwrap().is_file() && !entry.path().starts_with(from_dir) {
                    match git_dir {
                        Some(git_dir) if entry.path().starts_with(git_dir) => {}
                        _ => println!("{}", entry.path().strip_prefix(pwd).unwrap().display()),
                    }
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

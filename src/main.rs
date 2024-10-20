use clap::Parser;
use std::{collections::HashMap, fs, path::PathBuf};

use diff::{
    diff,
    model::{Edit, EditKind, Slice},
};

mod diff;

fn print_edits(a: Slice<'_>, b: Slice<'_>, edits: &[Edit<'_>]) {
    for edit in edits {
        match edit.kind {
            EditKind::DELETE => println!("{} -- {}", edit.pos, edit.line),
            EditKind::INSERT => println!("{} ++ {}", edit.pos, edit.line),
        }
    }

    // let edit_map = edits
    //     .into_iter()
    //     .map(|edit| ((edit.kind.clone(), edit.pos), edit))
    //     .collect::<HashMap<_, _>>();

    // for index in 0..a.len().max(b.len()) {
    //     if let Some(edit) = edit_map.get(&(EditKind::DELETE, index)) {
    //         println!("{index} -- {}", edit.line);
    //     }
    // }
}

#[derive(Parser)]
struct Cli {
    first: PathBuf,
    second: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let source = fs::read_to_string("test/notes-1.org").unwrap();
    let source_lines = source.lines().collect::<Vec<_>>();
    let target = fs::read_to_string("test/notes-2.org").unwrap();
    let target_lines = target.lines().collect::<Vec<_>>();

    let edits = diff(&source_lines, &target_lines);
    print_edits(&source_lines, &target_lines, &edits);
}

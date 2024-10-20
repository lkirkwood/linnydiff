use clap::Parser;
use colored::Colorize;
use std::{fs, path::PathBuf};

use diff::{
    diff,
    model::{Edit, EditKind, Slice},
};

mod diff;

fn print_edits(_a: Slice<'_>, _b: Slice<'_>, edits: &[Edit<'_>]) {
    for edit in edits {
        match edit.kind {
            EditKind::Delete => println!("{}", format!("-- {}", edit.line).red()),
            EditKind::Insert => println!("{}", format!("++ {}", edit.line).green()),
        }
    }
}

#[derive(Parser)]
struct Cli {
    first: PathBuf,
    second: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let source = fs::read_to_string(cli.first).unwrap();
    let source_lines = source.lines().collect::<Vec<_>>();
    let target = fs::read_to_string(cli.second).unwrap();
    let target_lines = target.lines().collect::<Vec<_>>();

    let edits = diff(&source_lines, &target_lines);
    print_edits(&source_lines, &target_lines, &edits);
}

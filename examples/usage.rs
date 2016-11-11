extern crate emoji_commit_type;

use emoji_commit_type::CommitType;

fn main() {
    println!("The emoji commit types are:");

    for commit_type in CommitType::iter_variants() {
        println!("{}  - {}", commit_type.emoji(), commit_type.description());
    }
}

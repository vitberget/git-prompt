use crate::changes::changes_index_and_worktree;
use crate::shorthand::{shorthand, remote};

use git2::Repository;

pub(crate) mod changes;
pub(crate) mod shorthand;

fn main() {
    if let Ok(repo) = Repository::open_from_env() {
        changes_index_and_worktree(&repo);
        let head = repo.head();
        shorthand(&head);
        remote(head);
    };
}

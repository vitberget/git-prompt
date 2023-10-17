use crate::changes::changes_index_and_worktree;
use crate::shorthand::shorthand;
use git2::Repository;

pub(crate) mod changes;
pub(crate) mod shorthand;

fn main() {
    if let Ok(repo) = Repository::open_from_env() {
        changes_index_and_worktree(&repo);
        shorthand(repo.head());
    };
}

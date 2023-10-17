use crate::branch::branch;
use crate::summary::changed_index_and_worktree;
use git2::Repository;

pub(crate) mod branch;
pub(crate) mod summary;

fn main() {
    Repository::open_from_env().iter().for_each(|repo| {
        repo.head().iter()
            .for_each(branch);
        changed_index_and_worktree(&repo);
    });
}

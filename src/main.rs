use crate::branch::branch;
use crate::summary::changed_index_and_worktree;
use git2::Repository;

pub(crate) mod branch;
pub(crate) mod summary;

fn main() {
    if let Ok(repo) = Repository::open_from_env() {
        if let Ok(active_branch) = repo.head() {
            branch(&active_branch);
        }
        changed_index_and_worktree(&repo);
    };
}

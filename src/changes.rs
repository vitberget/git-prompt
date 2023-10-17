use git2::{Repository, Status, StatusEntry, StatusOptions};

struct RepoChanges {
    index: bool,
    worktree: bool,
}

impl RepoChanges {
    fn new() -> RepoChanges {
        RepoChanges {
            index: false,
            worktree: false,
        }
    }

    fn both_true(&self) -> bool {
        self.worktree && self.index
    }
}

pub(crate) fn changes_index_and_worktree(repo: &Repository) {
    let mut status_options = StatusOptions::new();
    status_options.include_untracked(true);

    let repo_changes = repo.statuses(Some(&mut status_options))
        .iter()
        .fold(RepoChanges::new(), |acc, statuses| {
            if acc.both_true() {
                acc
            } else {
                statuses.iter()
                    .filter(|entry| entry.status() != Status::CURRENT)
                    .fold(acc, |acc, entry| update_repo_changes(acc, entry))
            }
        });

    export_true_false(&repo_changes);
}

fn update_repo_changes(repo_changes: RepoChanges, entry: StatusEntry<'_>) -> RepoChanges {
    if repo_changes.both_true() {
        repo_changes
    } else {
        let status = entry.status();

        let index = repo_changes.index
            || status.contains(Status::INDEX_DELETED)
            || status.contains(Status::INDEX_MODIFIED)
            || status.contains(Status::INDEX_NEW)
            || status.contains(Status::INDEX_RENAMED)
            || status.contains(Status::INDEX_TYPECHANGE);

        let worktree = repo_changes.worktree
            || status.contains(Status::WT_DELETED)
            || status.contains(Status::WT_MODIFIED)
            || status.contains(Status::WT_NEW)
            || status.contains(Status::WT_RENAMED)
            || status.contains(Status::WT_TYPECHANGE);

        RepoChanges { index, worktree }
    }  
}

fn export_true_false(repo_changes: &RepoChanges) {
    println!("export GIT_PROMPT_INDEX_CHANGED={}", repo_changes.index);
    println!("export GIT_PROMPT_WORKTREE_CHANGED={}", repo_changes.worktree);
}

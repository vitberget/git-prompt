use git2::{Repository, Status, StatusOptions, StatusEntry};

struct RepoChanges {
    index: bool,
    worktree: bool,
}

impl RepoChanges {
    fn new() -> RepoChanges {
        RepoChanges { index: false , worktree: false }
    }
}

pub(crate) fn changed_index_and_worktree(repo: &Repository) {
    let mut repo_changes = RepoChanges::new();

    let mut status_options = StatusOptions::new();
    status_options.include_untracked(true);

    repo.statuses(Some(&mut status_options)).iter()
        .for_each(|statuses| {
            statuses.iter()
                .filter(|entry| entry.status() != Status::CURRENT) 
                .for_each(|entry| {
                    update_repo_changes(&mut repo_changes, &entry);

                    if repo_changes.index && repo_changes.worktree {
                        export_true_false(&repo_changes);
                        return;
                    }
                });
        });
    export_true_false(&repo_changes);
}

fn update_repo_changes(repo_changes: &mut RepoChanges, entry: &StatusEntry<'_>) {
    let status = entry.status();

    repo_changes.index = repo_changes.index
        || status.contains(Status::INDEX_DELETED)
        || status.contains(Status::INDEX_MODIFIED)
        || status.contains(Status::INDEX_NEW)
        || status.contains(Status::INDEX_RENAMED)
        || status.contains(Status::INDEX_TYPECHANGE);

    repo_changes.worktree = repo_changes.worktree
        || status.contains(Status::WT_DELETED)
        || status.contains(Status::WT_MODIFIED)
        || status.contains(Status::WT_NEW)
        || status.contains(Status::WT_RENAMED)
        || status.contains(Status::WT_TYPECHANGE);
}

fn export_true_false(cc: &RepoChanges) {
    println!("export GIT_PROMPT_INDEX_CHANGED={}", cc.index);
    println!("export GIT_PROMPT_WORKTREE_CHANGED={}", cc.worktree);
}

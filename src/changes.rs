use git2::{Repository, Status, StatusEntry, StatusOptions, Statuses};

struct RepoChanges {
    index: bool,
    worktree: bool,
}

impl Default for RepoChanges {
    fn default() -> Self {
        Self { 
            index: false,
            worktree: false
        }
    }
}

impl RepoChanges {
    fn both_true(&self) -> bool {
        self.worktree && self.index
    }

    fn export_true_false(&self) {
        println!("export GIT_PROMPT_INDEX_CHANGED={}", self.index);
        println!("export GIT_PROMPT_WORKTREE_CHANGED={}", self.worktree);
    }
}

pub(crate) fn changes_index_and_worktree(repo: &Repository) {
    let mut status_options = StatusOptions::new();
    status_options.include_untracked(true);

    repo.statuses(Some(&mut status_options))
        .iter()
        .fold(RepoChanges::default(), update_repo_changes)
        .export_true_false();
}

fn update_repo_changes(repo_changes: RepoChanges, statuses :&Statuses<'_>) -> RepoChanges {
    match repo_changes.both_true() {
        true => repo_changes,

        false => statuses.iter()
            .filter(|entry| entry.status() != Status::CURRENT)
            .fold(repo_changes, update_repo_changes_from_status_entry)

    }
}

fn update_repo_changes_from_status_entry(repo_changes: RepoChanges, entry: StatusEntry<'_>) -> RepoChanges {
    match repo_changes.both_true() {
        true => repo_changes,

        false => {
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
}

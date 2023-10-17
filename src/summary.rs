use git2::{Repository, Status, StatusOptions};

struct RepoChanges {
    index: bool,
    worktree: bool,
}

pub(crate) fn changed_index_and_worktree(repo: &Repository) {
    let mut status_options = StatusOptions::new();
    status_options.include_untracked(true);

    repo.statuses(Some(&mut status_options)).iter().for_each(|statuses| {
        let mut cc = RepoChanges { index: false, worktree: false };

        for entry in statuses.iter().filter(|e| e.status() != Status::CURRENT) {
            if cc.index && cc.worktree {
                export_true_false(&cc);
                return;
            }
            let status = entry.status();

            cc.index = cc.index
                || status.contains(Status::INDEX_DELETED)
                || status.contains(Status::INDEX_MODIFIED)
                || status.contains(Status::INDEX_NEW)
                || status.contains(Status::INDEX_RENAMED)
                || status.contains(Status::INDEX_TYPECHANGE);

            cc.worktree = cc.worktree
                || status.contains(Status::WT_DELETED)
                || status.contains(Status::WT_MODIFIED)
                || status.contains(Status::WT_NEW)
                || status.contains(Status::WT_RENAMED)
                || status.contains(Status::WT_TYPECHANGE);
        }

        export_true_false(&cc);
    });
}

fn export_true_false(cc: &RepoChanges) {
    println!("export GIT_PROMPT_INDEX_CHANGED={}", cc.index);
    println!("export GIT_PROMPT_WORKTREE_CHANGED={}", cc.worktree);
}

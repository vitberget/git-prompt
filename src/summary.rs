use git2::{Repository, Status, StatusOptions};

pub(crate) fn changed_index_and_worktree(repo: &Repository) {
    let mut status_options = StatusOptions::new();
    status_options.include_untracked(true);

    repo.statuses(Some(&mut status_options)).iter().for_each(|statuses| {
        let mut index_deleted = 0;
        let mut index_modified = 0;
        let mut index_new = 0;
        let mut index_renamed = 0;
        let mut index_typechange = 0;
        let mut worktree_deleted = 0;
        let mut worktree_modified = 0;
        let mut worktree_new = 0;
        let mut worktree_renamed = 0;
        let mut worktree_typechange = 0;

        for entry in statuses.iter().filter(|e| e.status() != Status::CURRENT) {
            match entry.status() {
                s if s.contains(Status::INDEX_DELETED) => index_deleted += 1,
                s if s.contains(Status::INDEX_MODIFIED) => index_modified += 1,
                s if s.contains(Status::INDEX_NEW) => index_new += 1,
                s if s.contains(Status::INDEX_RENAMED) => index_renamed += 1,
                s if s.contains(Status::INDEX_TYPECHANGE) => index_typechange += 1,
                s if s.contains(Status::WT_DELETED) => worktree_deleted += 1,
                s if s.contains(Status::WT_MODIFIED) => worktree_modified += 1,
                s if s.contains(Status::WT_NEW) => worktree_new += 1,
                s if s.contains(Status::WT_RENAMED) => worktree_renamed += 1,
                s if s.contains(Status::WT_TYPECHANGE) => worktree_typechange +=1,
                _ => {}
            }
        }

        let index_changed = index_deleted + index_modified + index_new + index_renamed + index_typechange > 0;
        println!("export GIT_PROMPT_INDEX_CHANGED={index_changed}");
        let worktree_changed = worktree_deleted + worktree_modified + worktree_new + worktree_renamed + worktree_typechange > 0;
        println!("export GIT_PROMPT_WORKTREE_CHANGED={worktree_changed}");
    }
    );
}

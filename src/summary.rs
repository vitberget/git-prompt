use git2::{Repository, Status, StatusOptions};

struct ChangeCounter {
    index_deleted: i32,
    index_modified: i32,
    index_new: i32,
    index_renamed: i32,
    index_typechange: i32,
    worktree_deleted: i32,
    worktree_modified: i32,
    worktree_new: i32,
    worktree_renamed: i32,
    worktree_typechange: i32,
}

impl ChangeCounter {
    fn sum_index(&self) -> i32 {
        self.index_deleted + self.index_modified + self.index_new + self.index_renamed + self.index_typechange
    }

    fn sum_worktree(&self) -> i32 {
        self.worktree_deleted + self.worktree_modified + self.worktree_new + self.worktree_renamed + self.worktree_typechange
    }
}

pub(crate) fn changed_index_and_worktree(repo: &Repository) {
    let mut status_options = StatusOptions::new();
    status_options.include_untracked(true);

    repo.statuses(Some(&mut status_options)).iter().for_each(|statuses| {
        let mut cc = ChangeCounter { index_deleted: 0, index_modified: 0, index_new: 0, index_renamed: 0, index_typechange: 0, worktree_deleted: 0, worktree_modified: 0, worktree_new: 0, worktree_renamed: 0, worktree_typechange: 0 };

        for entry in statuses.iter().filter(|e| e.status() != Status::CURRENT) {
            match entry.status() {
                s if s.contains(Status::INDEX_DELETED) => cc.index_deleted += 1,
                s if s.contains(Status::INDEX_MODIFIED) => cc.index_modified += 1,
                s if s.contains(Status::INDEX_NEW) => cc.index_new += 1,
                s if s.contains(Status::INDEX_RENAMED) => cc.index_renamed += 1,
                s if s.contains(Status::INDEX_TYPECHANGE) => cc.index_typechange += 1,
                s if s.contains(Status::WT_DELETED) => cc.worktree_deleted += 1,
                s if s.contains(Status::WT_MODIFIED) => cc.worktree_modified += 1,
                s if s.contains(Status::WT_NEW) => cc.worktree_new += 1,
                s if s.contains(Status::WT_RENAMED) => cc.worktree_renamed += 1,
                s if s.contains(Status::WT_TYPECHANGE) => cc.worktree_typechange +=1,
                _ => {}
            }
        }

        export_true_false(&cc);
    });
}

fn export_true_false(cc: &ChangeCounter) {
    let index_changed = cc.sum_index() > 0;
    println!("export GIT_PROMPT_INDEX_CHANGED={index_changed}");

    let worktree_changed = cc.sum_worktree() > 0;
    println!("export GIT_PROMPT_WORKTREE_CHANGED={worktree_changed}");
}

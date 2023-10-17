use git2::{Repository, Status, StatusOptions};

struct ChangeCounterChanges {
    deleted: i32,
    modified: i32,
    new: i32,
    renamed: i32,
    typechange: i32,
}

impl ChangeCounterChanges {
    fn new() -> ChangeCounterChanges {
        ChangeCounterChanges { deleted: 0, modified: 0, new: 0, renamed: 0, typechange: 0 }
    }

    fn sum(&self) -> i32 {
        self.deleted + self.modified + self.new + self.renamed + self.typechange
    }

    fn changed(&self) -> bool {
        self.deleted > 0 
            || self.modified  > 0 
            || self.new  > 0 
            || self.renamed  > 0 
            || self.typechange > 0 
    }
}

struct ChangeCounter {
    index: ChangeCounterChanges,
    worktree: ChangeCounterChanges,
}
impl ChangeCounter {
    fn new() -> ChangeCounter {
        ChangeCounter {
            index: ChangeCounterChanges::new(),
            worktree: ChangeCounterChanges::new()
        }
    }
}

pub(crate) fn changed_index_and_worktree(repo: &Repository) {
    let mut status_options = StatusOptions::new();
    status_options.include_untracked(true);

    repo.statuses(Some(&mut status_options)).iter().for_each(|statuses| {
        let mut cc = ChangeCounter::new();

        for entry in statuses.iter().filter(|e| e.status() != Status::CURRENT) {
            match entry.status() {
                s if s.contains(Status::INDEX_DELETED) => cc.index.deleted += 1,
                s if s.contains(Status::INDEX_MODIFIED) => cc.index.modified += 1,
                s if s.contains(Status::INDEX_NEW) => cc.index.new += 1,
                s if s.contains(Status::INDEX_RENAMED) => cc.index.renamed += 1,
                s if s.contains(Status::INDEX_TYPECHANGE) => cc.index.typechange += 1,
                s if s.contains(Status::WT_DELETED) => cc.worktree.deleted += 1,
                s if s.contains(Status::WT_MODIFIED) => cc.worktree.modified += 1,
                s if s.contains(Status::WT_NEW) => cc.worktree.new += 1,
                s if s.contains(Status::WT_RENAMED) => cc.worktree.renamed += 1,
                s if s.contains(Status::WT_TYPECHANGE) => cc.worktree.typechange +=1,
                _ => {}
            }
        }

        export_true_false(&cc);
    });
}

fn export_true_false(cc: &ChangeCounter) {
    let index_changed = cc.index.changed();
    println!("export GIT_PROMPT_INDEX_CHANGED={index_changed}");

    let worktree_changed = cc.worktree.changed();
    println!("export GIT_PROMPT_WORKTREE_CHANGED={worktree_changed}");
}

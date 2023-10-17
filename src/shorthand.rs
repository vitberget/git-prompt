use git2::{Error, Reference, Branch};

pub(crate) fn shorthand(head: Result<Reference<'_>, Error>) {
    const UNKNOWN: &str = "?";

    if let Ok(reference) = head {
        let shorthand = reference.shorthand().unwrap_or(UNKNOWN);
        println!("export GIT_PROMPT_BRANCH={shorthand}");


        let branch = Branch::wrap(reference);
        let local_target = branch.get().target();

        let remote = if let Ok(upstream) = branch.upstream() {
            if local_target == upstream.get().target() {
                "same"
            } else {
                "different"
            }
        } else {
            "not_a_branch"
        };

        println!("export GIT_PROMPT_REMOTE={remote}");

    } else {
        println!("export GIT_PROMPT_BRANCH={UNKNOWN}");
    }
}

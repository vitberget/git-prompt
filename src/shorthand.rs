use git2::{Error, Reference, Branch};

pub(crate) fn shorthand(head: Result<Reference<'_>, Error>) {
    const UNKNOWN: &str = "?";

    let shorthand = if let Ok(reference) = &head {
        reference.shorthand().unwrap_or(UNKNOWN)
    } else {
        UNKNOWN
    };
    println!("export GIT_PROMPT_BRANCH={shorthand}");


    let remote = if let Ok(reference) = head {
        let branch = Branch::wrap(reference);
        if let Ok(upstream) = branch.upstream() {
            if branch.get().target() == upstream.get().target() {
                "same"
            } else {
                "different"
            }
        } else {
            "not_a_branch"
        }
    } else {
        UNKNOWN
    };
    println!("export GIT_PROMPT_REMOTE={remote}");
}

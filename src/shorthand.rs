use git2::{Error, Reference, Branch};

const UNKNOWN: &str = "?";

pub(crate) fn shorthand(head: &Result<Reference<'_>, Error>) {
    let shorthand = match head {
        Ok(reference) => reference.shorthand().unwrap_or(UNKNOWN),
        Err(_) => UNKNOWN
    };

    println!("export GIT_PROMPT_BRANCH={shorthand}");
}

pub(crate) fn remote(head: Result<Reference<'_>, Error>) {
    let remote = match head {
        Ok(reference) => branch_equaliness(reference),
        Err(_) => UNKNOWN,
    };
    println!("export GIT_PROMPT_REMOTE={remote}");
}

fn branch_equaliness(reference: Reference<'_>) -> &str {
    let branch = Branch::wrap(reference);
    match branch.upstream() {
        Ok(upstream) => {
            match branch.get().target() == upstream.get().target() {
                true => "same",
                false => "different",
            }
        }
        Err(_) => "not_a_branch",
    }
}

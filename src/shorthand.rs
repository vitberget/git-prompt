use git2::{Error, Reference};

pub(crate) fn shorthand(head: Result<&Reference<'_>, &Error>) {
    const UNKNOWN: &str = "?";

    let branch: &str = match head {
        Ok(reference) => match reference.shorthand() {
            Some(shorthand) => shorthand,
            _ => UNKNOWN,
        },
        Err(_) => UNKNOWN,
    };

    println!("export GIT_PROMPT_BRANCH={branch}");
}

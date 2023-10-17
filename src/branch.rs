use git2::Reference;

pub(crate) fn branch(head: &Reference<'_>) {
    head.shorthand().iter()
        .for_each(|branch| { println!("export GIT_PROMPT_BRANCH={branch}"); });
}

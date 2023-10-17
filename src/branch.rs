use git2::Reference;

pub(crate) fn branch(head: &Reference<'_>) {
    if let Some(branch) = head.shorthand() {
        println!("export GIT_PROMPT_BRANCH={branch}");
    };
}

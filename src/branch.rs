use git2::Reference;

pub(crate) fn branch(head: &Reference<'_>) {
    head.shorthand().and_then(|branch| { 
        println!("export GIT_PROMPT_BRANCH={branch}"); 
        Some(())
    });
}

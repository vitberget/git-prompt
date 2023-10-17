# Git prompt

## Install

```sh
cargo install --path .
```

Should be installed to `~/.cargo/bin` on Linux.

## Use

Git prompt gives output like output:

```
export GIT_PROMPT_BRANCH=main
export GIT_PROMPT_INDEX_CHANGED=false
export GIT_PROMPT_WORKTREE_CHANGED=false
export GIT_PROMPT_REMOTE=different
```

To use it in Bash/Zsh/other:

```sh
eval $(git-prompt)
```

### Zsh example

```zsh
function git-prompt-prompt() {
    eval $(git-prompt)
    if [[ ! -z "$GIT_PROMPT_BRANCH" ]]; then
        if [[ "$GIT_PROMPT_REMOTE" == "same" ]]; then
            echo -n "%F{249}\uE0A0${GIT_PROMPT_BRANCH}"
        else
            echo -n "%F{160}\uE0A0%F{249}${GIT_PROMPT_BRANCH}"
        fi
        if [[ "$GIT_PROMPT_INDEX_CHANGED" == "true" ]]; then
            echo -n "%F{red}✗"
        else
            echo -n "%F{green}✓"
        fi
        if [[ "$GIT_PROMPT_WORKTREE_CHANGED" == "true" ]]; then
            echo -n "%F{red}✗"
        else
            echo -n "%F{green}✓"
        fi
        echo " "
    fi
}
```

## References

### Crate: git2

* <https://crates.io/crates/git2>
* <https://docs.rs/git2/0.18.0/git2/>
* <https://github.com/rust-lang/git2-rs>

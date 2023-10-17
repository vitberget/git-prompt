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
        echo -n " %F{249}\uE0A0%{[94m%}${GIT_PROMPT_BRANCH} "
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
    fi
}
```

## References

### Crate: git2

* <https://crates.io/crates/git2>
* <https://docs.rs/git2/0.18.0/git2/>
* <https://github.com/rust-lang/git2-rs>

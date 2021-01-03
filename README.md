# Purs

A [Pure](https://github.com/sindresorhus/pure)-inspired prompt in [Rust](https://www.rust-lang.org/).

Even more minimal, definitively faster and at least as pretty as the original Pure by [Sindre Sohrus](https://github.com/sindresorhus).

Fork of https://github.com/xcambar/purs to suit my tastes :)

![Screenshot of Purs in action](./static/imgs/prompt.png)

## Installation — Usage

1. `$ cargo build --release`
1. Add the following to your ZSH configuration:

```
function zle-line-init zle-keymap-select {
  PROMPT=`/PATH/TO/PURS/target/release/purs prompt -k "$KEYMAP" -r "$?" --venv "${${VIRTUAL_ENV:t}%-*}"`
  zle reset-prompt
}
zle -N zle-line-init
zle -N zle-keymap-select

autoload -Uz add-zsh-hook

function _prompt_purs_precmd() {
  /PATH/TO/PURS/target/release/purs precmd
}
add-zsh-hook precmd _prompt_purs_precmd

```

# License

MIT, see LICENSE file.

#!/bin/bash
curl https://raw.githubusercontent.com/git/git/master/contrib/completion/git-prompt.sh -o ~/.git-prompt.sh
curl https://gist.githubusercontent.com/babymotte/08d7f17a1aa70e372d745352cd3aabff/raw/c413544f1e0854ba6b34975836192e2a60dac662/01-git-prompt -o ~/.bashrc.d/01-git-prompt
cargo build
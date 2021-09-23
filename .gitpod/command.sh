#!/bin/bash
curl https://raw.githubusercontent.com/git/git/master/contrib/completion/git-prompt.sh -o ~/.git-prompt.sh
curl https://gist.githubusercontent.com/babymotte/08d7f17a1aa70e372d745352cd3aabff/raw/2bc2e486decf4703647db88c2b3f003333ce33a3/01-git-prompt -o ~/.bashrc.d/01-git-prompt
cargo make test
echo "Your workspace is ready!"
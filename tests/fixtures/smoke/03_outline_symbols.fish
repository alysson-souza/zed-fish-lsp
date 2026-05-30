# Expected in Zed:
# - The outline includes "say hello", valid_function, ll, la, EDITOR, GOPATH,
#   fish_color, gs, co, and -C.
# - The outline does not include not_an_outline_symbol.

function "say hello"
    echo hi
end

function valid_function
    echo ok
end

if false
    "say hello"
    valid_function

    alias ll ls
    alias --save la "ls -la"

    set --global --export EDITOR nvim
    set -x -g GOPATH $GOPATH
    set --global fish_color normal

    abbr -a gs git status
    abbr --command git co checkout
    abbr -a --position anywhere -- -C --color
end

echo not_an_outline_symbol

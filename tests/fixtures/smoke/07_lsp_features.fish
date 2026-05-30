# Expected in Zed with fish-lsp running:
# - Hover and completion work on builtins such as function, set, echo, and
#   string.
# - Go to definition and find references work for helper.
# - Formatting indents the body of helper.

function helper --argument-names name
echo "hello $name"
end

function main
    set -l user_name "zed"
    helper $user_name
    string upper -- $user_name
end

if false
    main
end

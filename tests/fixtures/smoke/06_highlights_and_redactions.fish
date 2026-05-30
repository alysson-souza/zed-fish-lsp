# Expected in Zed:
# - Syntax highlighting covers commands, flags, variables, strings, numbers,
#   operators, escapes, comments, control flow, and punctuation.
# - Screen-sharing redaction covers comments, strings, variable expansions, and
#   command substitutions.

# token used by the demo service
function highlight_redaction_demo --argument-names value
    set -gx PATH $PATH
    set -l secret "token-123"

    if not test -n "$value"
        echo "line\n" $secret (pwd)
        echo yes; or echo no
        echo done && echo next
    else
        return 0
    end
end

if false
    highlight_redaction_demo demo
end

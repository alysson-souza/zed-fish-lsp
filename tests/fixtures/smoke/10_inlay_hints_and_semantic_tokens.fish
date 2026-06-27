# Expected in Zed with fish-lsp running:
# - With inlay_hints.enabled set to true, return and exit lines show status
#   descriptions inline.
# - With semantic_tokens set to combined or full, fish-lsp contributes richer
#   highlighting for function names, variable names, builtins, keywords, and
#   command subcommands.

function semantic_status_demo --argument-names input
    set -l normalized (string lower -- $input)
    set -q PATH

    if test -z "$normalized"
        return 127
    end

    echo $normalized
    return 0
end

function exit_status_demo
    exit 130
end

semantic_status_demo "ZED"

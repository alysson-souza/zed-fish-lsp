#!/usr/bin/env fish
#
# Expected in Zed:
# - A script runnable appears on this shebang line.
# - A function runnable appears on top_level.
# - No standalone function runnable appears on nested.

function top_level --description "top-level runnable"
    echo top-level

    function nested
        echo nested
    end

    if false
        nested
    end
end

if false
    top_level
end

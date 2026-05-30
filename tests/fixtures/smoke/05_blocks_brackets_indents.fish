# Expected in Zed:
# - Matching pairs work for function/end, if/end, while/end, for/end,
#   switch/end, begin/end, brackets, braces, parentheses, and quotes.
# - Auto-indent keeps block bodies one level deeper.
# - Vim and Helix text objects select function and block bodies.

function exercise_blocks --argument-names item
    echo $argv[1] {alpha,beta} (pwd) "double" 'single'

    if test -n "$item"
        echo "has item"
    else if test $status = 1
        echo "retry"
    else
        echo "fallback"
    end

    while false
        break
    end

    for value in alpha beta
        echo $value
        continue
    end

    switch $item
        case alpha
            echo "alpha"
        case '*'
            echo "other"
    end

    begin
        echo "begin block"
    end

    {
        echo "brace begin block"
    }
end

if false
    exercise_blocks alpha
end

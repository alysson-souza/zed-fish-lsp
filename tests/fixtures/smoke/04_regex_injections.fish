# Expected in Zed:
# - Regex highlighting appears inside supported arguments for string match -r,
#   string replace --regex, grep, egrep, rg, sed, and awk.
# - The strings passed to echo, string match, and string replace at the bottom
#   are not highlighted as regex.

if false
    set branch_name feature/123
    set ref_name refs/heads/main

    string match -r '^feature/[0-9]+$' $branch_name
    string replace --regex "refs/heads/(.*)" '$1' $ref_name
    grep "TODO|FIXME" README.md
    egrep 'one|two|three' README.md
    rg "task-[0-9]+" .
    sed 's/foo/bar/g' README.md
    awk '/error|warning/ { print $0 }' app.log

    echo "^not_regex"
    string match "literal" $branch_name
    string replace "literal" replacement $branch_name
end

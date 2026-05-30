# Expected in Zed:
# - The file is handled as Fish because of the .fish suffix.
# - No script runnable appears because there is no shebang.
# - A function runnable appears on plain_file_function.

function plain_file_function
    echo "plain .fish file"
end

if false
    plain_file_function
end

function fun1
    echo "output1"
end

function fun2
    echo "output2"
end

if false
    fun1
    fun2
end

# Expected in Zed:
# - Function runnables appear on fun1 and fun2.
# - Running fun1 prints output1.
# - Running fun2 prints output2.
# - No file-level script runnable appears on line 1.
# - If line 1 runs the whole file as a script, it prints nothing because this
#   file only defines functions. That is the runnable/task mismatch this file
#   is meant to expose.

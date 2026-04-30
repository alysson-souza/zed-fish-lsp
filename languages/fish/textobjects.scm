(function_definition) @function.around

(function_definition
    "function"
    name: (_)
    [
        (begin_statement)
        (break)
        (command)
        (conditional_execution)
        (continue)
        (for_statement)
        (function_definition)
        (if_statement)
        (negated_statement)
        (pipe)
        (redirect_statement)
        (return)
        (switch_statement)
        (while_statement)
        (comment)
    ] @function.inside
    "end")

(if_statement) @class.around
(if_statement
    "if"
    condition: (_)
    (_)* @class.inside
    "end")

(while_statement) @class.around
(while_statement
    "while"
    condition: (_)
    (_)* @class.inside
    "end")

(for_statement) @class.around
(for_statement
    "for"
    variable: (_)
    "in"
    [
        (begin_statement)
        (break)
        (command)
        (conditional_execution)
        (continue)
        (for_statement)
        (function_definition)
        (if_statement)
        (negated_statement)
        (pipe)
        (redirect_statement)
        (return)
        (switch_statement)
        (while_statement)
        (comment)
    ] @class.inside
    "end")

(switch_statement) @class.around
(switch_statement
    "switch"
    value: (_)
    (_)* @class.inside
    "end")

(begin_statement) @class.around
(begin_statement
    "begin"
    (_)* @class.inside
    "end")
(begin_statement
    "{"
    (_)* @class.inside
    "}")

(comment)+ @comment.around

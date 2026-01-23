(function_definition) @function.around

(function_definition
    "function"
    name: [(word) (concatenation)]
    (_)* @function.inside
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
    (_)* @class.inside
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

(comment)+ @comment.around

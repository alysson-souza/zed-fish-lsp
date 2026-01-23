[(double_quote_string) (single_quote_string)] @string

(escape_sequence) @string.escape

(comment) @comment

[(integer) (float)] @number

[
    "not"
    "!"
    "and"
    "or"
    "&&"
    "||"
    (direction)
    (file_redirect)
    (stream_redirect)
] @operator

(pipe ["|" "2>|" "&|"] @operator)

[";" "&"] @punctuation.delimiter

(command name: (word) @function)

(command argument: (word) @constant (#match? @constant "^-."))

(command
    name: (word) @function (#eq? @function "test")
    argument: (word) @constant (#match? @constant "^(-[A-Za-z]+|!?=|!)$"))

(command
    name: (word) @punctuation.bracket (#eq? @punctuation.bracket "[")
    argument: (word) @constant (#match? @constant "^(-[A-Za-z]+|!?=|!)$"))

[(variable_expansion) (list_element_access)] @variable.special

(variable_name) @variable

(for_statement variable: (variable_name) @variable)

(command_substitution "$" @punctuation.special)
(command_substitution "(" @punctuation.bracket)
(command_substitution ")" @punctuation.bracket)

["{" "}" ","] @punctuation.bracket
[(home_dir_expansion) (glob)] @constant

(if_statement ["if" "end"] @keyword.control)
(switch_statement ["switch" "end"] @keyword.control)
(case_clause ["case"] @keyword.control)
(else_clause ["else"] @keyword.control)
(else_if_clause ["else" "if"] @keyword.control)

(while_statement ["while" "end"] @keyword.control)
(for_statement ["for" "in" "end"] @keyword.control)
(begin_statement ["begin" "end"] @keyword.control)

(function_definition ["function" "end"] @keyword.function)
(function_definition name: [(word) (concatenation)] @function)
(function_definition option: (word) @constant (#match? @constant "^-."))

[(return) (break) (continue)] @keyword.control

(conditional_execution ["&&" "||"] @operator)
(negated_statement "not" @keyword.control)

(ERROR) @error

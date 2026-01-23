(if_statement) @indent
(if_statement "end" @end)

(while_statement) @indent
(while_statement "end" @end)

(for_statement) @indent
(for_statement "end" @end)

(switch_statement) @indent
(switch_statement "end" @end)

(begin_statement) @indent
(begin_statement "end" @end)

(function_definition) @indent
(function_definition "end" @end)

(else_clause) @indent
(else_if_clause) @indent
(case_clause) @indent

(_ "[" "]" @end) @indent
(_ "{" "}" @end) @indent
(_ "(" ")" @end) @indent

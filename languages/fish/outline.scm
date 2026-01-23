(function_definition
    "function" @context
    name: [(word) (concatenation)] @name) @item

(command
    name: (word) @context
    argument: (word) @name
    (#eq? @context "alias")) @item

; Global/universal variables: set -g/-gx/-U VARNAME
(command
    name: (word) @context
    .
    (word) @_flag
    .
    (word) @name
    (#eq? @context "set")
    (#match? @_flag "^-[gUx]+$")) @item

; Abbreviations: abbr -a NAME expansion
(command
    name: (word) @context
    .
    (word) @_flag
    .
    (word) @name
    (#eq? @context "abbr")
    (#eq? @_flag "-a")) @item

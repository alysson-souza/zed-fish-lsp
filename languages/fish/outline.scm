(function_definition
    "function" @context
    name: (_) @name
    (#not-match? @name "^-|/")) @item

(command
    name: (word) @context
    .
    (word) @name
    (#eq? @context "alias")
    (#not-match? @name "^-|/")) @item

(command
    name: (word) @context
    .
    (word) @_flag
    .
    (word) @name
    (#eq? @context "alias")
    (#any-of? @_flag "-s" "--save")
    (#not-match? @name "^-|/")) @item

; Global/universal variables: set -g/-gx/-U VARNAME
(command
    name: (word) @context
    .
    (word) @_flag
    .
    (word) @name
    (#eq? @context "set")
    (#match? @_flag "^(-[gUxu]*[gU][gUxu]*|--global|--universal)$")
    (#match? @name "^[A-Za-z0-9_]+$")) @item

(command
    name: (word) @context
    .
    (word) @_scope_flag
    .
    (word) @_modifier_flag
    .
    (word) @name
    (#eq? @context "set")
    (#match? @_scope_flag "^(-[gU]+|--global|--universal)$")
    (#match? @_modifier_flag "^(-[xu]+|--export|--unexport)$")
    (#match? @name "^[A-Za-z0-9_]+$")) @item

(command
    name: (word) @context
    .
    (word) @_modifier_flag
    .
    (word) @_scope_flag
    .
    (word) @name
    (#eq? @context "set")
    (#match? @_modifier_flag "^(-[xu]+|--export|--unexport)$")
    (#match? @_scope_flag "^(-[gU]+|--global|--universal)$")
    (#match? @name "^[A-Za-z0-9_]+$")) @item

; Abbreviations: abbr -a NAME expansion
(command
    name: (word) @context
    .
    (word) @_flag
    .
    (word) @name
    (#eq? @context "abbr")
    (#any-of? @_flag "-a" "--add")
    (#not-match? @name "^-")) @item

(command
    name: (word) @context
    .
    (word) @_flag
    .
    (word) @_option_flag
    .
    (word) @_option_value
    .
    (word) @name
    (#eq? @context "abbr")
    (#any-of? @_flag "-a" "--add")
    (#any-of? @_option_flag "-c" "--command" "--position")
    (#not-match? @name "^-")) @item

(command
    name: (word) @context
    .
    (word) @_flag
    .
    (word) @_option_flag
    .
    (word) @_option_value
    .
    (word) @_separator
    .
    (word) @name
    (#eq? @context "abbr")
    (#any-of? @_flag "-a" "--add")
    (#any-of? @_option_flag "-c" "--command" "--position")
    (#eq? @_separator "--")) @item

(command
    name: (word) @context
    .
    (word) @_option_flag
    .
    (word) @_option_value
    .
    (word) @name
    (#eq? @context "abbr")
    (#any-of? @_option_flag "-c" "--command")
    (#not-match? @name "^-")) @item

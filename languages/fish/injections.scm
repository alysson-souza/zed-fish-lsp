; string match -r 'pattern' or string replace -r 'pattern'
((command
  name: (word) @_command
  argument: (word) @_flag
  argument: [(single_quote_string) (double_quote_string)] @content)
  (#eq? @_command "string")
  (#any-of? @_flag "-r" "--regex")
  (#set! "language" "regex"))

; grep, egrep, rg - first string argument is regex
((command
  name: (word) @_command
  argument: [(single_quote_string) (double_quote_string)] @content)
  (#any-of? @_command "grep" "egrep" "rg")
  (#set! "language" "regex"))

; sed - pattern argument
((command
  name: (word) @_command
  argument: [(single_quote_string) (double_quote_string)] @content)
  (#eq? @_command "sed")
  (#set! "language" "regex"))

; awk - pattern argument
((command
  name: (word) @_command
  argument: [(single_quote_string) (double_quote_string)] @content)
  (#eq? @_command "awk")
  (#set! "language" "regex"))

(double_quote_string) @redact
(single_quote_string) @redact

(variable_expansion) @redact

(command
  name: (word) @_cmd
  (#any-of? @_cmd "set" "export")
  argument: (double_quote_string) @redact)

(command
  name: (word) @_cmd
  (#any-of? @_cmd "set" "export")
  argument: (single_quote_string) @redact)

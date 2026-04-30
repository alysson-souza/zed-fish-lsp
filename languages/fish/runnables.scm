; Run button on shebang line (run whole script)
(
  (program . (comment) @run)
  (#match? @run "^#!")
  (#set! tag fish-script)
)

; Run button on functions
(
  (program
    (function_definition
      name: (_) @run))
  (#set! tag fish-function)
)

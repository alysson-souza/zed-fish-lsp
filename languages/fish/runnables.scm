; Add support for running fish scripts
(
  (program . (_) @run) @_fish-script
  (#set! tag fish-script)
)

; ... and for individual functions
(
    (function_definition . (_) @run) @_fish-function
  (#set! tag fish-function)
)

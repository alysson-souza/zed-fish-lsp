; Standard brackets
("[" @open "]" @close)
("{" @open "}" @close)
("(" @open ")" @close)

; Quotes (exclude from rainbow)
(("\"" @open "\"" @close) (#set! rainbow.exclude))
(("'" @open "'" @close) (#set! rainbow.exclude))

; Keyword pairs - WITHOUT newline.only (that predicate filters them from highlight!)
((function_definition "function" @open "end" @close) (#set! rainbow.exclude))
((if_statement "if" @open "end" @close) (#set! rainbow.exclude))
((for_statement "for" @open "end" @close) (#set! rainbow.exclude))
((while_statement "while" @open "end" @close) (#set! rainbow.exclude))
((switch_statement "switch" @open "end" @close) (#set! rainbow.exclude))
((begin_statement "begin" @open "end" @close) (#set! rainbow.exclude))
((begin_statement "{" @open "}" @close))

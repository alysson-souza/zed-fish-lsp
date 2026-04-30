use std::path::Path;

use tree_sitter::{Parser, Query, QueryCursor};

const BRACKETS_QUERY: &str = include_str!("../languages/fish/brackets.scm");
const HIGHLIGHTS_QUERY: &str = include_str!("../languages/fish/highlights.scm");
const INDENTS_QUERY: &str = include_str!("../languages/fish/indents.scm");
const INJECTIONS_QUERY: &str = include_str!("../languages/fish/injections.scm");
const OUTLINE_QUERY: &str = include_str!("../languages/fish/outline.scm");
const OVERRIDES_QUERY: &str = include_str!("../languages/fish/overrides.scm");
const REDACTIONS_QUERY: &str = include_str!("../languages/fish/redactions.scm");
const RUNNABLES_QUERY: &str = include_str!("../languages/fish/runnables.scm");
const TEXT_OBJECTS_QUERY: &str = include_str!("../languages/fish/textobjects.scm");

const FEATURE_SAMPLE: &str = r#"#!/usr/bin/env fish
function greet --argument-names name
    set -gx PATH $PATH
    alias ll ls
    alias --save la "ls -la"
    abbr -a gs git status
    string match -r "^foo" $name
    echo "ok"
end

for item in 1 2 $values
    echo $item
    continue
end
"#;

fn parse(source: &str) -> tree_sitter::Tree {
    let language = tree_sitter_fish::language();

    let mut parser = Parser::new();
    parser
        .set_language(&language)
        .expect("failed to load fish grammar");
    let tree = parser.parse(source, None).expect("failed to parse source");

    assert!(
        !tree.root_node().has_error(),
        "fixture should parse without ERROR nodes:\n{source}"
    );

    tree
}

fn captures(query_source: &str, source: &str, capture_name: &str) -> Vec<String> {
    let language = tree_sitter_fish::language();
    let tree = parse(source);
    let query = Query::new(&language, query_source).expect("query should compile");
    let capture_names = query.capture_names();

    let mut cursor = QueryCursor::new();
    let matches = cursor.matches(&query, tree.root_node(), source.as_bytes());

    matches
        .flat_map(|query_match| query_match.captures.iter())
        .filter_map(|capture| {
            let name = capture_names.get(capture.index as usize)?;
            (*name == capture_name).then(|| {
                capture
                    .node
                    .utf8_text(source.as_bytes())
                    .unwrap()
                    .to_string()
            })
        })
        .collect()
}

fn assert_unordered_eq(mut actual: Vec<String>, expected: &[&str]) {
    let mut expected = expected
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>();
    actual.sort();
    expected.sort();

    assert_eq!(actual, expected);
}

fn assert_no_captures(query_source: &str, source: &str, capture_name: &str) {
    let actual = captures(query_source, source, capture_name);

    assert!(
        actual.is_empty(),
        "expected no @{capture_name} captures, got {actual:?}"
    );
}

fn assert_query_compiles(path: &Path) {
    let language = tree_sitter_fish::language();
    let source = std::fs::read_to_string(path).unwrap();

    Query::new(&language, &source).unwrap_or_else(|error| {
        panic!("{} should compile: {error}", path.display());
    });
}

#[test]
fn all_fish_queries_compile_against_the_pinned_grammar() {
    let query_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("languages/fish");

    for query_file in [
        "brackets.scm",
        "highlights.scm",
        "indents.scm",
        "injections.scm",
        "outline.scm",
        "overrides.scm",
        "redactions.scm",
        "runnables.scm",
        "textobjects.scm",
    ] {
        assert_query_compiles(&query_dir.join(query_file));
    }
}

#[test]
fn runnables_identify_script_and_function_entrypoints() {
    assert_eq!(
        captures(RUNNABLES_QUERY, FEATURE_SAMPLE, "run"),
        ["#!/usr/bin/env fish", "greet"]
    );
}

#[test]
fn runnables_ignore_plain_comments_and_nested_functions() {
    let source = r#"# not a shebang
function outer
    function inner
        echo nested
    end
end
"#;

    assert_eq!(captures(RUNNABLES_QUERY, source, "run"), ["outer"]);
}

#[test]
fn outline_reports_defined_names_not_command_expansions() {
    assert_eq!(
        captures(OUTLINE_QUERY, FEATURE_SAMPLE, "name"),
        ["greet", "PATH", "ll", "la", "gs"]
    );
}

#[test]
fn outline_skips_supported_options_before_defined_names() {
    let source = r#"alias --save ll ls
alias -s la ls
set -g -x PATH $PATH
set --global --export EDITOR nvim
set -x -g GOPATH $GOPATH
set -u -U fish_color normal
abbr --add gs git status
abbr -a L --set-cursor "% | less"
abbr --command git co checkout
abbr -a --position anywhere -- -C --color
"#;

    assert_eq!(
        captures(OUTLINE_QUERY, source, "name"),
        [
            "ll",
            "la",
            "PATH",
            "EDITOR",
            "GOPATH",
            "fish_color",
            "gs",
            "L",
            "co",
            "-C"
        ]
    );
}

#[test]
fn outline_follows_documented_identifier_rules() {
    let source = r#"function "say hello"
    echo hi
end
function bad/name
    echo no
end
function -bad
    echo no
end
alias good ls
alias bad/name ls
alias -s -bad ls
set -g VALID_NAME value
set -gx BAD/NAME value
set --global invalid-name value
"#;

    assert_eq!(
        captures(OUTLINE_QUERY, source, "name"),
        ["\"say hello\"", "good", "VALID_NAME"]
    );
}

#[test]
fn injections_mark_supported_regex_arguments_as_regex_content() {
    let source = r#"string match -r "^foo" $name
string replace --regex "bar+" baz $name
grep "needle.*" file.txt
egrep 'one|two' file.txt
rg "todo[0-9]+" .
sed 's/foo/bar/' file.txt
awk '/foo/ { print }' file.txt
"#;

    assert_eq!(
        captures(INJECTIONS_QUERY, source, "content"),
        [
            "\"^foo\"",
            "\"bar+\"",
            "\"needle.*\"",
            "'one|two'",
            "\"todo[0-9]+\"",
            "'s/foo/bar/'",
            "'/foo/ { print }'",
        ]
    );
}

#[test]
fn injections_ignore_non_regex_strings() {
    let source = r#"echo "^not_regex"
string match "literal" $name
string replace "literal" replacement $name
"#;

    assert_no_captures(INJECTIONS_QUERY, source, "content");
}

#[test]
fn redactions_cover_quoted_strings_and_variable_expansions() {
    let source = r#"echo "token" 'secret' $PATH
echo $status
"#;

    assert_eq!(
        captures(REDACTIONS_QUERY, source, "redact"),
        ["\"token\"", "'secret'", "$PATH", "$status"]
    );
}

#[test]
fn overrides_capture_screen_share_strings_command_substitutions_and_comments() {
    let source = r#"# private note
echo "token" 'secret' (pwd)
"#;

    assert_eq!(
        captures(OVERRIDES_QUERY, source, "string"),
        ["\"token\"", "'secret'", "(pwd)"]
    );
    assert_eq!(
        captures(OVERRIDES_QUERY, source, "comment"),
        ["# private note"]
    );
}

#[test]
fn brackets_pair_delimiters_quotes_and_block_keywords() {
    let source = r#"function pairs
    echo $argv[1] {alpha,beta} (pwd) "double" 'single'
    if true
        echo yes
    end
    while true
        break
    end
    for item in alpha
        echo $item
    end
    switch $item
        case alpha
            echo alpha
    end
    begin
        echo done
    end
end
"#;

    assert_unordered_eq(
        captures(BRACKETS_QUERY, source, "open"),
        &[
            "function", "[", "{", "(", "\"", "'", "if", "while", "for", "switch", "begin",
        ],
    );
    assert_unordered_eq(
        captures(BRACKETS_QUERY, source, "close"),
        &[
            "]", "}", ")", "\"", "'", "end", "end", "end", "end", "end", "end",
        ],
    );
}

#[test]
fn indents_mark_block_forms_and_their_end_tokens() {
    let source = r#"function blocks
    if true
        echo if
    end
    while true
        break
    end
    for item in alpha
        echo $item
    end
    switch $item
        case alpha
            echo alpha
    end
    begin
        echo done
    end
end
"#;

    assert_eq!(
        captures(INDENTS_QUERY, source, "end"),
        ["end", "end", "end", "end", "end", "end"]
    );

    assert_eq!(
        captures(INDENTS_QUERY, source, "indent"),
        [
            r#"function blocks
    if true
        echo if
    end
    while true
        break
    end
    for item in alpha
        echo $item
    end
    switch $item
        case alpha
            echo alpha
    end
    begin
        echo done
    end
end"#,
            r#"if true
        echo if
    end"#,
            r#"while true
        break
    end"#,
            r#"for item in alpha
        echo $item
    end"#,
            r#"switch $item
        case alpha
            echo alpha
    end"#,
            r#"case alpha
            echo alpha
"#,
            r#"begin
        echo done
    end"#,
        ]
    );
}

#[test]
fn indents_mark_bracketed_expressions_and_their_close_tokens() {
    let source = "echo $argv[1]\necho {alpha,beta}\necho (pwd)\n";

    assert_eq!(captures(INDENTS_QUERY, source, "end"), ["]", "}", ")"]);
    assert_eq!(
        captures(INDENTS_QUERY, source, "indent"),
        ["[1]", "{alpha,beta}", "(pwd)"]
    );
}

#[test]
fn highlights_capture_function_definitions_and_returns() {
    let source = r#"function greet --argument-names name
    return 0
end
"#;

    assert_eq!(
        captures(HIGHLIGHTS_QUERY, source, "keyword.function"),
        ["function", "end"]
    );
    assert_eq!(captures(HIGHLIGHTS_QUERY, source, "function"), ["greet"]);
    assert_eq!(
        captures(HIGHLIGHTS_QUERY, source, "constant"),
        ["--argument-names"]
    );
    assert_eq!(
        captures(HIGHLIGHTS_QUERY, source, "keyword.control"),
        ["return"]
    );
    assert_eq!(captures(HIGHLIGHTS_QUERY, source, "number"), ["0"]);
}

#[test]
fn highlights_capture_commands_flags_strings_variables_and_numbers() {
    let source = r#"set -gx PATH $PATH
echo "ok" 'fine' 1 2.5
"#;

    assert_eq!(
        captures(HIGHLIGHTS_QUERY, source, "function"),
        ["set", "echo"]
    );
    assert_eq!(captures(HIGHLIGHTS_QUERY, source, "constant"), ["-gx"]);
    assert_eq!(captures(HIGHLIGHTS_QUERY, source, "variable"), ["PATH"]);
    assert_eq!(
        captures(HIGHLIGHTS_QUERY, source, "variable.special"),
        ["$PATH"]
    );
    assert_eq!(
        captures(HIGHLIGHTS_QUERY, source, "string"),
        ["\"ok\"", "'fine'"]
    );
    assert_eq!(captures(HIGHLIGHTS_QUERY, source, "number"), ["1", "2.5"]);
}

#[test]
fn highlights_capture_control_flow_operators_and_punctuation() {
    let source = r#"if not test -n $argv
    echo yes; or echo no
else if test $status = 1
    echo retry && echo done
else
    echo fallback
end
while true
    break
end
for item in alpha beta
    continue
end
switch $item
    case alpha
        echo alpha
end
begin
    echo $(pwd) ~ * {alpha,beta}
end
"#;

    assert_unordered_eq(
        captures(HIGHLIGHTS_QUERY, source, "keyword.control"),
        &[
            "if", "not", "else", "if", "else", "end", "while", "break", "end", "for", "in",
            "continue", "end", "switch", "case", "end", "begin", "end",
        ],
    );
    assert_unordered_eq(
        captures(HIGHLIGHTS_QUERY, source, "operator"),
        &["or", "&&"],
    );
    assert_eq!(
        captures(HIGHLIGHTS_QUERY, source, "punctuation.delimiter"),
        [";"]
    );
    assert_unordered_eq(
        captures(HIGHLIGHTS_QUERY, source, "punctuation.bracket"),
        &["(", ")", "{", "}", ","],
    );
    assert_unordered_eq(
        captures(HIGHLIGHTS_QUERY, source, "punctuation.special"),
        &["$"],
    );
    assert_unordered_eq(
        captures(HIGHLIGHTS_QUERY, source, "constant"),
        &["-n", "=", "~", "*"],
    );
}

#[test]
fn highlights_capture_comments_and_escape_sequences() {
    let source = "# note\necho \"line\\n\"\n";

    assert_eq!(captures(HIGHLIGHTS_QUERY, source, "comment"), ["# note"]);
    assert_eq!(captures(HIGHLIGHTS_QUERY, source, "string.escape"), ["\\n"]);
}

#[test]
fn function_inside_skips_function_options() {
    let source = include_str!("fixtures/textobjects/function_with_options.fish");

    assert_eq!(
        captures(TEXT_OBJECTS_QUERY, source, "function.inside"),
        ["echo $name", "return 0"]
    );
}

#[test]
fn for_inside_skips_iterable_values() {
    let source = include_str!("fixtures/textobjects/for_loop_body.fish");

    assert_eq!(
        captures(TEXT_OBJECTS_QUERY, source, "class.inside"),
        ["echo $item", "continue"]
    );
}

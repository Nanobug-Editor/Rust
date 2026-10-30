; Rust syntax captures for declarations, names, literals, comments, and punctuation.

(type_identifier) @type
(primitive_type) @type.builtin
(field_identifier) @property

(function_item name: (identifier) @function)
(function_signature_item name: (identifier) @function)
(struct_item name: (type_identifier) @type)
(enum_item name: (type_identifier) @type)
(trait_item name: (type_identifier) @type)
(type_item name: (type_identifier) @type)
(mod_item name: (identifier) @namespace)

(call_expression function: (identifier) @function)
(call_expression function: (field_expression field: (field_identifier) @function.method))
(call_expression function: (scoped_identifier name: (identifier) @function))
(generic_function function: (identifier) @function)
(generic_function function: (scoped_identifier name: (identifier) @function))
(generic_function function: (field_expression field: (field_identifier) @function.method))
(macro_invocation macro: (identifier) @function.macro)
(macro_invocation macro: (scoped_identifier name: (identifier) @function.macro))
(macro_definition name: (identifier) @function.macro)

; Declaration nodes identify constants without guessing from identifier capitalization.
(const_item name: (identifier) @constant)
(static_item name: (identifier) @constant)
(lifetime) @label
(label) @label

(parameter pattern: (identifier) @variable.parameter)
(let_declaration pattern: (identifier) @variable)
(line_comment) @comment
(block_comment) @comment
(string_literal) @string
(raw_string_literal) @string
(char_literal) @string
(escape_sequence) @string.escape
(boolean_literal) @boolean
(integer_literal) @number
(float_literal) @number
(attribute_item) @attribute
(inner_attribute_item) @attribute

[
  "as" "async" "await" "break" "const" "continue" "default" "dyn" "else" "enum"
  "extern" "fn" "for" "gen" "if" "impl" "in" "let" "loop" "macro_rules!" "match" "mod"
  "move" "pub" "raw" "ref" "return" "static" "struct"
  "trait" "type" "union" "unsafe" "use" "where" "while" "yield"
] @keyword

; `mut` and `self` are named Rust grammar nodes rather than anonymous tokens.
(mutable_specifier) @keyword
(crate) @keyword
(super) @keyword
(self) @variable.builtin

[
  "(" ")" "[" "]" "{" "}"
] @punctuation.bracket

[
  "," "." ";" ":" "::"
] @punctuation.delimiter

[
  "=" "+" "-" "*" "/" "%" "!" "&" "|" "^" "<" ">" "?"
  "&&" "||" "<<" ">>" "==" "!=" "<=" ">="
  "+=" "-=" "*=" "/=" "%=" "&=" "|=" "^=" "<<=" ">>="
  ".." "..." "..=" "->" "=>" "@"
] @operator

; Doc markers alias `/` and `!`: match their exact node ranges after the generic
; operators so shorter captures cannot punch a differently colored hole in a comment.
(outer_doc_comment_marker) @comment.documentation
(inner_doc_comment_marker) @comment.documentation

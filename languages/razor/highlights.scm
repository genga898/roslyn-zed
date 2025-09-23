; inherits: [c_sharp, html]

; Razor Directives
(razor_page_directive) @keyword.directive
(razor_using_directive) @keyword.directive
(razor_model_directive) @keyword.directive
(razor_inject_directive) @keyword.directive
(razor_namespace_directive) @keyword.directive
(razor_implements_directive) @keyword.directive
(razor_preservewhitespace_directive) @keyword.directive
(razor_inherits_directive) @keyword.directive
(razor_typeparam_directive) @keyword.directive
(razor_layout_directive) @keyword.directive
(razor_attribute_directive) @keyword.directive
(razor_rendermode_directive) @keyword.directive
(razor_section) @keyword.directive

; Razor Control Flow
(razor_if) @keyword.control.conditional
(razor_else_if) @keyword.control.conditional
(razor_else) @keyword.control.conditional
(razor_for) @keyword.control.loop
(razor_foreach) @keyword.control.loop
(razor_while) @keyword.control.loop
(razor_do_while) @keyword.control.loop
(razor_switch) @keyword.control.conditional
(razor_switch_case) @keyword.control.conditional
(razor_switch_default) @keyword.control.conditional
(razor_try) @keyword.control.exception
(razor_catch) @keyword.control.exception
(razor_finally) @keyword.control.exception
(razor_lock) @keyword.control
(razor_compound_using) @keyword.control

; Razor Expressions
(razor_implicit_expression) @variable
(razor_explicit_expression) @variable
(razor_escape) @constant.character.escape

; Comments
(razor_comment) @comment
(html_comment) @comment

; Razor Transitions
(explicit_line_transition) @keyword.directive

; Razor HTML Attributes
(razor_html_attribute) @attribute.html
(razor_attribute_name) @function
(razor_attribute_modifier) @attribute.modifier
(razor_attribute_value) @string

; Razor Rendermode
(razor_rendermode) @property

(member_access_expression) @variable.member
(typeof_expression) @keyword.operator
(identifier) @variable

(invocation_expression
  function: (identifier) @function.call)

(invocation_expression
  function: (member_access_expression
   name: (identifier) @function.call))

(member_access_expression
  name: (identifier) @property)

(predefined_type) @type.builtin
(qualified_name) @type
(generic_name) @type.generic

[
  "at_page"
  "at_using"
  "at_model"
  "at_rendermode"
  "at_inject"
  "at_implements"
  "at_layout"
  "at_inherits"
  "at_attribute"
  "at_typeparam"
  "at_namespace"
  "at_preservewhitespace"
  "at_block"
  "at_at_escape"
  "at_colon_transition"
] @constant.macro

[
  "at_lock"
  "at_section"
] @keyword

[
  "at_if"
  "at_switch"
] @keyword.conditional

[
  "at_for"
  "at_foreach"
  "at_while"
  "at_do"
] @keyword.repeat

[
  "at_try"
  "catch"
  "finally"
] @keyword.exception

[
  "at_implicit"
  "at_explicit"
] @variable

"at_await" @keyword.coroutine

; C# Keywords
[
  "public"
  "private"
  "protected"
  "internal"
  "static"
  "readonly"
  "const"
  "virtual"
  "override"
  "abstract"
  "sealed"
  "partial"
  "async"
  "await"
  "var"
  "typeof"
] @keyword

; C# Control Flow
[
  "if"
  "else"
  "for"
  "foreach"
  "while"
  "do"
  "switch"
  "case"
  "default"
  "break"
  "continue"
  "return"
  "goto"
] @keyword.control

; C# Exception Handling
[
  "try"
  "catch"
  "finally"
  "throw"
] @keyword.control.exception

; Operators
[
  "="
  "=="
  "!="
  "<"
  ">"
  "<="
  ">="
  "&&"
  "||"
  "!"
  "+"
  "-"
  "*"
  "/"
  "%"
] @operator

; Punctuation
[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

[
  ";"
  ","
  "."
] @punctuation.delimiter

; Razor @ symbol
"@" @operator

; Variable declarations
(variable_declarator
  (identifier) @variable.declaration)

; Parameters
(parameter
  name: (identifier) @parameter)

(string_literal) @string
(string_literal_content) @string
(character_literal) @character
(integer_literal) @number
(real_literal) @number.float
(boolean_literal) @constant.builtin.boolean
(null_literal) @constant.builtin.null

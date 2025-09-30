; inherits: c_sharp

[
  (razor_comment)
  (html_comment)
] @comment

; HTML elements (basic highlighting)
(element) @none

; Razor HTML attributes - these are the exposed nodes
(razor_html_attribute
  (razor_attribute_name) @attribute
  (razor_attribute_value) @string)
(razor_attribute_modifier) @function.method
(attribute_list) @attribute
(modifier) @keyword.modifier

; Directive nodes
[
  (razor_page_directive)
  (razor_using_directive)
  (razor_model_directive)
  (razor_rendermode_directive)
  (razor_inject_directive)
  (razor_implements_directive)
  (razor_layout_directive)
  (razor_inherits_directive)
  (razor_attribute_directive)
  (razor_typeparam_directive)
  (razor_namespace_directive)
  (razor_preservewhitespace_directive)
  (razor_block)
  (razor_escape)
  (explicit_line_transition)
] @constant.macro

; Control flow directives
[
  (razor_lock)
  (razor_section)
] @keyword

; Conditional directives
[
  (razor_if)
  (razor_switch)
] @keyword.conditional

; Loop directives
[
  (razor_for)
  (razor_foreach)
  (razor_while)
  (razor_do_while)
] @keyword.repeat

; Exception handling
[
  (razor_try)
  (razor_catch)
  (razor_finally)
] @keyword.exception

; Expressions
[
  (razor_implicit_expression)
  (razor_explicit_expression)
] @variable

(razor_await_expression) @keyword.coroutine

; Special nodes
(razor_rendermode) @property

[
  (boolean_literal)
  (null_literal)
] @constant.builtin

[
  (integer_literal)
  (real_literal)
] @number

[
  (character_literal)
  (string_literal)
  (verbatim_string_literal)
  (raw_string_literal)
] @string

(string_literal_content) @string
(escape_sequence) @string.escape
 
;; Types
(predefined_type) @type.builtin
(generic_name) @type
(qualified_name) @type
(typeof_expression) @type
(interface_declaration name: (identifier) @type)
(class_declaration name: (identifier) @type)
(enum_declaration name: (identifier) @type)
(struct_declaration (identifier) @type)
(record_declaration (identifier) @type)

; Method and property declarations within Razor blocks
(method_declaration name: (identifier) @function)
(property_declaration name: (identifier) @property)
(field_declaration) @variable.member

[
  "--"
  "-"
  "-="
  "&"
  "&="
  "&&"
  "+"
  "++"
  "+="
  "<"
  "<="
  "<<"
  "<<="
  "="
  "=="
  "!"
  "!="
  "=>"
  ">"
  ">="
  ">>"
  ">>="
  ">>>"
  ">>>="
  "|"
  "|="
  "||"
  "?"
  "??"
  "??="
  "^"
  "^="
  "~"
  "*"
  "*="
  "/"
  "/="
  "%"
  "%="
  ":"
] @operator

[
 "."
 ","
 ";"
 ":"
 "::"
 ] @punctuation.delimiter

[
 "("
 ")"
 "["
 "]"
 "{"
 "}"
 ] @punctuation.bracket

; C# preprocessor support
[
  (preproc_if)
  (preproc_define)
  (preproc_region)
  (preproc_endregion)
] @constant.macro

(attribute name: (identifier) @attribute)

; Functions
(local_function_statement name: (identifier) @function)
(invocation_expression function:
    (member_access_expression
        expression: (identifier) @type
        name: (identifier) @method.call)
)

; Pattern matching
(is_pattern_expression) @keyword.operator
(negated_pattern) @operator
(constant_pattern) @pattern

; Object creation
[
  (implicit_object_creation_expression)
  (object_creation_expression)
] @constructor

; Variable declarations
(variable_declarator name: (identifier) @variable)
(variable_declaration type: (identifier) @type)

; Parameter and argument lists
(parameter_list) @punctuation.bracket
(argument_list) @punctuation.bracket

; Assignment operators (more specific)
(assignment_expression "=" @operator.assignment)

(member_access_expression
    expression: (identifier) @variable
    name: (identifier) @property
)

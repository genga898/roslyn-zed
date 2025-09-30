; Comments
([
  (html_comment)
  (razor_comment)
] @comment
  (#set! injection.language "comment"))

((element) @injection.content
  (#set! injection.language "html"))
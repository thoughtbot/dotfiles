" Language:    CoffeeScript
" Maintainer:  Mick Koch <kchmck@gmail.com>
" URL:         http://github.com/kchmck/vim-coffee-script
" License:     WTFPL

" Bail if our syntax is already loaded.
if exists('b:current_syntax') && b:current_syntax == 'coffee'
  finish
endif

if version < 600
  syn clear
endif

" Include JavaScript for coffeeEmbed.
syn include @coffeeJS syntax/javascript.vim

" Highlight long strings.
syn sync minlines=100

" CoffeeScript identifiers can have dollar signs.
setlocal isident+=$

" These are `matches` instead of `keywords` because vim's highlighting
" priority for keywords is higher than matches. This causes keywords to be
" highlighted inside matches, even if a match says it shouldn't contain them --
" like with coffeeAssign and coffeeDot.
syn match coffeeStatement /\<\%(return\|break\|continue\|throw\)\>/ display
hi def link coffeeStatement Statement

syn match coffeeRepeat /\<\%(for\|while\|until\|loop\)\>/ display
hi def link coffeeRepeat Repeat

syn match coffeeConditional /\<\%(if\|else\|unless\|switch\|when\|then\)\>/
\                           display
hi def link coffeeConditional Conditional

syn match coffeeException /\<\%(try\|catch\|finally\)\>/ display
hi def link coffeeException Exception

syn match coffeeKeyword /\<\%(new\|in\|of\|by\|and\|or\|not\|is\|isnt\|class\|extends\|super\|own\|do\)\>/
\                       display
hi def link coffeeKeyword Keyword

syn match coffeeOperator /\<\%(instanceof\|typeof\|delete\)\>/ display
hi def link coffeeOperator Operator

" The first case matches symbol operators only if they have an operand before.
syn match coffeeExtendedOp /\%(\S\s*\)\@<=[+\-*/%&|\^=!<>?.]\+\|--\|++\|::/
\                          display
syn match coffeeExtendedOp /\%(and\|or\)=/ display
hi def link coffeeExtendedOp coffeeOperator

" This is separate from `coffeeExtendedOp` to help differentiate commas from
" dots.
syn match coffeeSpecialOp /[,;]/ display
hi def link coffeeSpecialOp SpecialChar

syn match coffeeBoolean /\<\%(true\|on\|yes\|false\|off\|no\)\>/ display
hi def link coffeeBoolean Boolean

syn match coffeeGlobal /\<\%(null\|undefined\)\>/ display
hi def link coffeeGlobal Type

" A special variable
syn match coffeeSpecialVar /\<\%(this\|prototype\|arguments\)\>/ display
" An @-variable
syn match coffeeSpecialVar /@\%(\I\i*\)\?/ display
hi def link coffeeSpecialVar Special

" A class-like name that starts with a capital letter
syn match coffeeObject /\<\u\w*\>/ display
hi def link coffeeObject Structure

" A constant-like name in SCREAMING_CAPS
syn match coffeeConstant /\<\u[A-Z0-9_]\+\>/ display
hi def link coffeeConstant Constant

" A variable name
syn cluster coffeeIdentifier contains=coffeeSpecialVar,coffeeObject,
\                                     coffeeConstant

" A non-interpolated string
syn cluster coffeeBasicString contains=@Spell,coffeeEscape
" An interpolated string
syn cluster coffeeInterpString contains=@coffeeBasicString,coffeeInterp

" Regular strings
syn region coffeeString start=/"/ skip=/\\\\\|\\"/ end=/"/
\                       contains=@coffeeInterpString
syn region coffeeString start=/'/ skip=/\\\\\|\\'/ end=/'/
\                       contains=@coffeeBasicString
hi def link coffeeString String

" A integer, including a leading plus or minus
syn match coffeeNumber /\i\@<![-+]\?\d\+\%([eE][+-]\?\d\+\)\?/ display
" A hex number
syn match coffeeNumber /\<0[xX]\x\+\>/ display
hi def link coffeeNumber Number

" A floating-point number, including a leading plus or minus
syn match coffeeFloat /\i\@<![-+]\?\d*\.\@<!\.\d\+\%([eE][+-]\?\d\+\)\?/
\                     display
hi def link coffeeFloat Float

" An error for reserved keywords
if !exists("coffee_no_reserved_words_error")
  syn match coffeeReservedError /\<\%(case\|default\|function\|var\|void\|with\|const\|let\|enum\|export\|import\|native\|__hasProp\|__extends\|__slice\|__bind\|__indexOf\)\>/
  \                             display
  hi def link coffeeReservedError Error
endif

" This is separate from `coffeeExtendedOp` since assignments require it.
syn match coffeeAssignOp /:/ contained display
hi def link coffeeAssignOp coffeeOperator

" Strings used in string assignments, which can't have interpolations
syn region coffeeAssignString start=/"/ skip=/\\\\\|\\"/ end=/"/ contained
\                             contains=@coffeeBasicString
syn region coffeeAssignString start=/'/ skip=/\\\\\|\\'/ end=/'/ contained
\                             contains=@coffeeBasicString
hi def link coffeeAssignString String

" A normal object assignment
syn match coffeeObjAssign /@\?\I\i*\s*:\@<!::\@!/
\                         contains=@coffeeIdentifier,coffeeAssignOp
hi def link coffeeObjAssign Identifier

" An object-string assignment
syn match coffeeObjStringAssign /\("\|'\)[^\1]*\1\s*;\@<!::\@!'\@!/
\                               contains=coffeeAssignString,coffeeAssignOp
" An object-integer assignment
syn match coffeeObjNumberAssign /\d\+\%(\.\d\+\)\?\s*:\@<!::\@!/
\                               contains=coffeeNumber,coffeeAssignOp

syn keyword coffeeTodo TODO FIXME XXX contained
hi def link coffeeTodo Todo

syn match coffeeComment /#.*/ contains=@Spell,coffeeTodo
hi def link coffeeComment Comment

syn region coffeeBlockComment start=/####\@!/ end=/###/
\                             contains=@Spell,coffeeTodo
hi def link coffeeBlockComment coffeeComment

" A comment in a heregex
syn region coffeeHeregexComment start=/#/ end=/\ze\/\/\/\|$/ contained
\                               contains=@Spell,coffeeTodo
hi def link coffeeHeregexComment coffeeComment

" Embedded JavaScript
syn region coffeeEmbed matchgroup=coffeeEmbedDelim
\                      start=/`/ skip=/\\\\\|\\`/ end=/`/
\                      contains=@coffeeJS
hi def link coffeeEmbedDelim Delimiter

syn region coffeeInterp matchgroup=coffeeInterpDelim start=/#{/ end=/}/ contained
\                       contains=@coffeeAll
hi def link coffeeInterpDelim PreProc

" A string escape sequence
syn match coffeeEscape /\\\d\d\d\|\\x\x\{2\}\|\\u\x\{4\}\|\\./ contained display
hi def link coffeeEscape SpecialChar

" A regex -- must not follow a parenthesis, number, or identifier, and must not
" be followed by a number
syn region coffeeRegex start=/\%(\%()\|\i\@<!\d\)\s*\|\i\)\@<!\/=\@!\s\@!/
\                      skip=/\[[^\]]\{-}\/[^\]]\{-}\]/
\                      end=/\/[gimy]\{,4}\d\@!/
\                      oneline contains=@coffeeBasicString
hi def link coffeeRegex String

" A heregex
syn region coffeeHeregex start=/\/\/\// end=/\/\/\/[gimy]\{,4}/
\                        contains=@coffeeInterpString,coffeeHeregexComment
\                        fold
hi def link coffeeHeregex coffeeRegex

" Heredoc strings
syn region coffeeHeredoc start=/"""/ end=/"""/ contains=@coffeeInterpString
\                        fold
syn region coffeeHeredoc start=/'''/ end=/'''/ contains=@coffeeBasicString
\                        fold
hi def link coffeeHeredoc String

" An error for trailing whitespace, as long as the line isn't just whitespace
if !exists("coffee_no_trailing_space_error")
  syn match coffeeSpaceError /\S\@<=\s\+$/ display
  hi def link coffeeSpaceError Error
endif

" An error for trailing semicolons, for help transitioning from JavaScript
if !exists("coffee_no_trailing_semicolon_error")
  syn match coffeeSemicolonError /;$/ display
  hi def link coffeeSemicolonError Error
endif

" Ignore reserved words in dot accesses.
syn match coffeeDotAccess /\.\@<!\.\s*\I\i*/he=s+1 contains=@coffeeIdentifier
hi def link coffeeDotAccess coffeeExtendedOp

" Ignore reserved words in prototype accesses.
syn match coffeeProtoAccess /::\s*\I\i*/he=s+2 contains=@coffeeIdentifier
hi def link coffeeProtoAccess coffeeExtendedOp

" This is required for interpolations to work.
syn region coffeeCurlies matchgroup=coffeeCurly start=/{/ end=/}/
\                        contains=@coffeeAll
syn region coffeeBrackets matchgroup=coffeeBracket start=/\[/ end=/\]/
\                         contains=@coffeeAll
syn region coffeeParens matchgroup=coffeeParen start=/(/ end=/)/
\                       contains=@coffeeAll

" These are highlighted the same as commas since they tend to go together.
hi def link coffeeBlock coffeeSpecialOp
hi def link coffeeBracket coffeeBlock
hi def link coffeeCurly coffeeBlock
hi def link coffeeParen coffeeBlock

" This is used instead of TOP to keep things coffee-specific for good
" embedding. `contained` groups aren't included.
syn cluster coffeeAll contains=coffeeStatement,coffeeRepeat,coffeeConditional,
\                              coffeeException,coffeeKeyword,coffeeOperator,
\                              coffeeExtendedOp,coffeeSpecialOp,coffeeBoolean,
\                              coffeeGlobal,coffeeSpecialVar,coffeeObject,
\                              coffeeConstant,coffeeString,coffeeNumber,
\                              coffeeFloat,coffeeReservedError,coffeeObjAssign,
\                              coffeeObjStringAssign,coffeeObjNumberAssign,
\                              coffeeComment,coffeeBlockComment,coffeeEmbed,
\                              coffeeRegex,coffeeHeregex,coffeeHeredoc,
\                              coffeeSpaceError,coffeeSemicolonError,
\                              coffeeDotAccess,coffeeProtoAccess,
\                              coffeeCurlies,coffeeBrackets,coffeeParens

if !exists('b:current_syntax')
  let b:current_syntax = 'coffee'
endif

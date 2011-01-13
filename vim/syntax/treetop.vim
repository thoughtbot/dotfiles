" treetop.vim
" syntax file for treetop.
" Author: NANKI Haruo.
 
let b:ruby_no_expensive=1
 
syntax match ttOperators /[&~!*+?\/]/
syntax keyword ttDefine end
syntax keyword ttKeywords include super
 
syntax match ttComment /#.*$/
syntax match ttName /\w\+/ contained
syntax match ttDefine "\<\(module\|grammar\|rule\)\>" nextgroup=ttName skipwhite skipnl
syntax match ttTag /\w\+\%(:\)/
 
syntax include @Ruby syntax/ruby.vim
syntax region BlockRuby matchgroup=ttQuote start=/{/ end=/^\s*}$/ transparent contains=@Ruby
 
syntax region ttPattern matchgroup=ttQuote start=/\[/ end=/\]/
syntax region ttString matchgroup=ttQuote start=/'/ end=/'/
syntax region ttString matchgroup=ttQuote start=/"/ end=/"/
syntax region ttClass matchgroup=ttQuote start=/</ end=/>/
 
highlight link ttQuote Delimiter
highlight link ttString String
highlight link ttPattern String
 
highlight link ttDefine Define
highlight link ttKeywords Statement
highlight link ttName Identifier
highlight link ttClass Constant
highlight link ttOperators Operator
highlight link ttComment Comment
highlight link ttTag Type


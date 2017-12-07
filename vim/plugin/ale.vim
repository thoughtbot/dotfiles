call ale#linter#Define('eruby', {
  \   'name': 'erubylint',
  \   'executable': 'erb',
  \   'output_stream': 'stderr',
  \   'command': "ruby -rerb -e \"puts ERB.new(File.read(%t, encoding: 'BINARY').gsub('<%=','<%'), nil, '-').src\" | ruby -c",
  \   'callback': 'ale#handlers#ruby#HandleSyntaxErrors',
  \})

if !exists('g:ale_linters')
  let g:ale_linters = {}
endif

let g:ale_linters['eruby'] = ['erubylint']

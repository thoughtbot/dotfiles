# Change file extensions recursively in current directory
#
#   change-extension erb haml

function change-extension() {
  foreach f (**/*.$1)
    mv $f $f:r.$2
  end
}

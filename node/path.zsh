# node modules
NODE_BINARY=`which node`
NODE_PREFIX=${NODE_BINARY%/bin/node}
NODE_PATH=${NODE_PREFIX}/lib/node_modules 

# Setting PATH for node package manager
PATH="${NODE_PREFIX}/bin:$PATH"
export PATH


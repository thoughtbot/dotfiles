# Load .env file into shell session for environment variables

function envup() {
  if [ -f .env ]; then
    export $(cat .env)
  else
    echo 'No .env file found' 1>&2
    return 1
  fi
}

# Check if the 'code' command exists
if ! command -v code &> /dev/null; then
    # If it doesn't exist, alias 'code' to 'cursor'
    alias code='cursor'
fi

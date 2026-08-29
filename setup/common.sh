# Shared helpers for platform setup scripts (mac-setup.sh, wsl-setup.sh).
# Source from the dotfiles repo root:
#   source "$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/setup/common.sh"

readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly NC='\033[0m'

log_info() {
  echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
  echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
  echo -e "${RED}[ERROR]${NC} $1"
}

command_exists() {
  command -v "$1" &>/dev/null
}

safe_source() {
  if [[ -f "$1" ]]; then
    # shellcheck disable=SC1090
    source "$1"
    return 0
  fi
  return 1
}

run_safe() {
  set +e
  "$@" >/dev/null 2>&1
  local exit_code=$?
  set -e
  return "$exit_code"
}

get_git_config() {
  git config --get "$1" 2>/dev/null || echo ""
}

prompt_for_git_identity() {
  if [[ -z "${GIT_EMAIL:-}" ]]; then
    GIT_EMAIL="$(get_git_config user.email)"
    if [[ -z "$GIT_EMAIL" ]]; then
      echo -n "Enter your Git email: "
      read -r GIT_EMAIL
    else
      log_info "Using Git email from config: $GIT_EMAIL"
    fi
    export GIT_EMAIL
  fi

  if [[ -z "${FULL_NAME:-}" ]]; then
    FULL_NAME="$(get_git_config user.name)"
    if [[ -z "$FULL_NAME" ]]; then
      echo -n "Enter your full name: "
      read -r FULL_NAME
    else
      log_info "Using Git name from config: $FULL_NAME"
    fi
    export FULL_NAME
  fi
}

dotfiles_repo_dir() {
  local script_dir
  script_dir="$(cd "$(dirname "${BASH_SOURCE[1]}")" && pwd)"
  if [[ -f "${script_dir}/rcrc" ]]; then
    printf '%s\n' "$script_dir"
    return 0
  fi
  if [[ -d "${HOME}/dotfiles" && -f "${HOME}/dotfiles/rcrc" ]]; then
    printf '%s\n' "${HOME}/dotfiles"
    return 0
  fi
  return 1
}

ensure_dotfiles_local_gitconfig() {
  local gitconfig_local="${HOME}/dotfiles-local/gitconfig.local"
  mkdir -p "${HOME}/dotfiles-local"

  if [[ -f "$gitconfig_local" ]]; then
    log_info "Keeping existing ${gitconfig_local}"
  else
    prompt_for_git_identity

    cat >"$gitconfig_local" <<EOF
[user]
  name = ${FULL_NAME}
  email = ${GIT_EMAIL}
EOF
    log_info "Wrote ${gitconfig_local}"
  fi

  # Base gitconfig includes ~/.gitconfig.local; rcup should symlink this from
  # dotfiles-local, but repair explicitly if the link still points at the removed
  # ~/dotfiles/gitconfig.local path.
  if [[ ! -e "${HOME}/.gitconfig.local" ]] \
    || [[ "$(readlink "${HOME}/.gitconfig.local" 2>/dev/null)" == "${HOME}/dotfiles/gitconfig.local" ]]; then
    ln -sf "$gitconfig_local" "${HOME}/.gitconfig.local"
    log_info "Linked ~/.gitconfig.local -> ${gitconfig_local}"
  fi
}

run_rcup() {
  local dotfiles_dir="$1"
  if ! command_exists rcup; then
    log_error "rcup not found. Install rcm (macOS: brew install rcm; Ubuntu: sudo apt install rcm)."
    return 1
  fi

  log_info "Running rcup from ${dotfiles_dir}..."
  env RCRC="${dotfiles_dir}/rcrc" rcup
}

# Prefer GitHub Release binary, else cargo build --release. Soft-fail.
install_agent_sync() {
  local dotfiles_dir="$1"
  local crate="${dotfiles_dir}/agent-sync"
  local dest="${HOME}/.local/bin/agent-sync"
  mkdir -p "${HOME}/.local/bin"

  if [[ ! -d "${crate}" ]]; then
    log_warn "agent-sync crate missing at ${crate}; skipping install"
    return 0
  fi

  if command_exists cargo; then
    log_info "Building agent-sync (release)..."
    if (cd "${crate}" && cargo build --release --quiet); then
      cp -f "${crate}/target/release/agent-sync" "${dest}"
      chmod +x "${dest}"
      log_info "Installed ${dest}"
      return 0
    fi
    log_warn "cargo build --release failed for agent-sync"
  else
    log_warn "cargo not found; cannot build agent-sync (install Rust or download a Release asset)"
  fi
  return 0
}

run_agent_sync() {
  local dotfiles_dir="$1"
  local verify="${2:-0}"
  local bin=""
  if [[ -x "${HOME}/.local/bin/agent-sync" ]]; then
    bin="${HOME}/.local/bin/agent-sync"
  elif [[ -x "${dotfiles_dir}/bin/agent-sync" ]]; then
    bin="${dotfiles_dir}/bin/agent-sync"
  elif [[ -x "${dotfiles_dir}/agent-sync/target/release/agent-sync" ]]; then
    bin="${dotfiles_dir}/agent-sync/target/release/agent-sync"
  fi
  if [[ -z "${bin}" ]]; then
    log_warn "agent-sync binary not found; skipping sync"
    return 0
  fi
  log_info "Running agent-sync pull --if-stale then sync --source hybrid..."
  DOTFILES_DIR="${dotfiles_dir}" "${bin}" pull --if-stale || log_warn "harness pull skipped"
  DOTFILES_DIR="${dotfiles_dir}" "${bin}" sync --source hybrid || log_warn "agent-sync sync failed"
  if [[ "${verify}" == "1" ]]; then
    DOTFILES_DIR="${dotfiles_dir}" "${bin}" verify || log_warn "agent-sync verify failed"
  fi
}

# Compat alias for older setup scripts.
run_sync_ai_assistants() {
  run_agent_sync "$1" 1
}

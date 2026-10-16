#!/usr/bin/env bash
# Deploy the OpenCrabs landing page and docs to production, in one command.
#
# Usage:
#   scripts/deploy.sh                build, push, deploy
#   scripts/deploy.sh --docs         rebuild the docs book first (mdbook + OG previews)
#   scripts/deploy.sh --skip-build   deploy the image already on Docker Hub
#   scripts/deploy.sh --dry-run      print every step, change nothing
#
# This replaces the hand dance:
#   crabslandlogin -> crabslanddeploy -> ssh neuraprodnew -> cldeploy
#
# The docs ship inside the image. src/docker/Dockerfile copies the committed
# docs/book into nginx, so rebuild the book locally (--docs) whenever the docs
# changed. og_previews.py needs macOS Chrome and sips, so it can only run on
# this machine and never inside the Linux image build.
set -euo pipefail

IMAGE="opencrabs/landing:latest"
DOCKERFILE="src/docker/Dockerfile"
REGISTRY_USER="opencrabs"
SERVER="${CRABSLAND_SERVER:-neuraprodnew}"
SERVER_DIR="/srv/rs/opencrabs.com"
SITE="https://opencrabs.com/"
DOCS_CHECK="https://docs.opencrabs.com/channels/telegram"

# The exact steps the cldeploy alias on neuraprodnew runs, expanded because a
# non-interactive ssh shell does not load /root/.bash_aliases (where both
# cldeploy and dockercleanup are defined).
REMOTE_CMDS="cd ${SERVER_DIR} && git pull --ff-only && cd src/docker && docker compose pull && docker compose down && docker builder prune -a --force && docker network prune --force && docker image prune -a --force && docker compose up -d"

BUILD=1
DOCS=0
DRY_RUN=0

while [ $# -gt 0 ]; do
    case "$1" in
        --docs) DOCS=1 ;;
        --skip-build) BUILD=0 ;;
        --dry-run) DRY_RUN=1 ;;
        -h|--help) sed -n '2,17p' "$0"; exit 0 ;;
        *) echo "deploy: unknown option '$1'" >&2; exit 2 ;;
    esac
    shift
done

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

step() { printf '\n==> %s\n' "$*"; }

run() {
    local cmd="$1"
    printf '+ %s\n' "$cmd"
    if [ "$DRY_RUN" = 0 ]; then
        eval "$cmd"
    fi
}

step "Preflight"
for bin in docker ssh curl; do
    command -v "$bin" >/dev/null || { echo "deploy: $bin is not on PATH" >&2; exit 1; }
done
[ -f .env ] || { echo "deploy: .env is missing, DOCKER_LOGIN_TOKEN lives there" >&2; exit 1; }
[ -f "$DOCKERFILE" ] || { echo "deploy: $DOCKERFILE is missing" >&2; exit 1; }
echo "repo:   $REPO_ROOT"
echo "commit: $(git rev-parse --short HEAD) $(git log -1 --pretty=%s)"
if [ -n "$(git status --porcelain --untracked-files=no)" ]; then
    echo "note:   uncommitted tracked changes are part of this build:"
    git status --short --untracked-files=no
fi

if [ "$DOCS" = 1 ]; then
    step "Rebuilding the docs book (mdbook + per-page OG previews)"
    run "bash docs/build-all.sh"
fi

step "Docker Hub login as $REGISTRY_USER"
set -a
# shellcheck disable=SC1091
. ./.env
set +a
[ -n "${DOCKER_LOGIN_TOKEN:-}" ] || { echo "deploy: DOCKER_LOGIN_TOKEN is empty in .env" >&2; exit 1; }
if [ "$DRY_RUN" = 0 ]; then
    printf '%s' "$DOCKER_LOGIN_TOKEN" | docker login -u "$REGISTRY_USER" --password-stdin
else
    echo "+ docker login -u $REGISTRY_USER --password-stdin  (token from .env)"
fi

if [ "$BUILD" = 1 ]; then
    step "Building and pushing $IMAGE for linux/amd64"
    run "docker buildx build --platform linux/amd64 --provenance=false --push -t $IMAGE -f $DOCKERFILE ."
else
    step "Skipping the build, deploying $IMAGE as it sits on Docker Hub"
fi

step "Deploying on $SERVER"
if [ "$DRY_RUN" = 0 ]; then
    # shellcheck disable=SC2029  # the steps are meant to be expanded here
    ssh "$SERVER" "$REMOTE_CMDS"
else
    echo "+ ssh $SERVER '$REMOTE_CMDS'"
fi

step "Verifying"
if [ "$DRY_RUN" = 1 ]; then
    echo "+ ssh $SERVER docker ps --filter name=opencrabs-landing"
    echo "+ curl $SITE"
    echo "+ curl $DOCS_CHECK"
    echo
    echo "dry run: nothing was built, pushed or restarted."
    exit 0
fi

ssh "$SERVER" "docker ps --filter name=opencrabs-landing --format '{{.Names}} | {{.Image}} | {{.Status}}'"

code=""
for _ in 1 2 3 4 5 6; do
    code="$(curl -sS -o /dev/null -m 20 -w '%{http_code}' "$SITE" || true)"
    [ "$code" = "200" ] && break
    sleep 5
done
echo "$SITE -> ${code:-no answer}"

og="$(curl -sS -m 20 "$DOCS_CHECK" | grep -oE '<meta property="og:image" content="[^"]*"' | head -1 || true)"
echo "docs preview: ${og:-no og:image found}"

if [ "$code" != "200" ]; then
    echo "deploy: $SITE answered ${code:-nothing} after the restart" >&2
    exit 1
fi

case "$og" in
    *"/og/"*) ;;
    *) echo "WARNING: $DOCS_CHECK still points at the site-wide preview, this image does not carry the new docs" >&2 ;;
esac

step "Done, $IMAGE is live on $SERVER"

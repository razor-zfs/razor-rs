#!/usr/bin/env bash

#FIXME: update once there is a runner at build server
#	note build linkage (dynamic/static - cargo deb --no-build?)

set -e

usage() {
    cat <<'EOF' >&2
Unexpected script parameter. Please call this script with following options:
release
debug
test
clippy
deb
build
EOF
    exit 1
}

BUILD_MODE="release"
CARGO_BUILD_ARGS="--release"

while [ -n "$1" ]; do
  case "$1" in
    release)
      BUILD_MODE="release"
      CARGO_BUILD_ARGS="--release"
      ;;
    debug)
      BUILD_MODE="debug"
      CARGO_BUILD_ARGS=
      ;;
    test)
      CARGOTEST=1
      ;;
    clippy)
      CARGOCLIPPY=1
      ;;
    deb)
      CARGODEB=1
      ;;
    build)
      CARGO_BUILD=1
      ;;
    *)
      echo "$1 is not allowed here"
      usage
      ;;
  esac
  shift
done

slugify() {
  echo $1 | iconv -t ascii//TRANSLIT | sed -E 's/[^a-zA-Z0-9]+/-/g' | sed -E 's/^-+\|-+$//g' | tr A-Z a-z
}

#if [ -n "$CI" ] ; then
  UNIQUE_ID_SHORT=$CI_COMMIT_REF_SLUG-$CI_COMMIT_SHORT_SHA
  UNIQUE_ID_LONG=$CI_COMMIT_REF_SLUG-$CI_COMMIT_SHA
#else
#  GIT_REF=$(git describe --exact-match 2>/dev/null || git rev-parse --abbrev-ref HEAD)
#  GIT_REF_SLUG=$(slugify ${GIT_REF})
#  UNIQUE_ID_SHORT=$GIT_REF_SLUG-$(git rev-parse HEAD | head -c 8)
#  UNIQUE_ID_LONG=$GIT_REF_SLUG-$(git rev-parse HEAD)
#fi

if [ -n "$CARGOCLIPPY" ] ; then
  cargo clippy --workspace --all-targets $CARGO_BUILD_ARGS
fi

if [ -n "$CARGOTEST" ] ; then
  cargo fmt --all -- --check
  AWS_PROFILE=unittest cargo test --workspace --all-targets $CARGO_BUILD_ARGS
  AWS_PROFILE=unittest cargo test --all-targets $CARGO_BUILD_ARGS
fi

if [ -n "$CARGO_BUILD" ] ; then
  cargo build --workspace --all-targets $CARGO_BUILD_ARGS
fi

if [ -n "$CARGODEB" ] ; then
  which cargo-deb || cargo install cargo-deb

  if [ "$BUILD_MODE" = "debug" ] ; then
      #time cargo deb --verbose --no-build --fast --no-strip --variant=debug
      time cargo deb --fast --no-strip --variant=debug
  else
      #time cargo deb --verbose --no-build --fast --no-strip
      time cargo deb --fast --no-strip
  fi
fi

echo $0 has finished


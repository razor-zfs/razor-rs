repo_dir := `git rev-parse --show-toplevel`
hooks_dir := repo_dir + "/.git/hooks"
bin_dir := `dirname $(which just)`
uname_s := `uname -s |  tr '[:upper:]' '[:lower:]'`

githook hook:
    if [ ! -x {{hooks_dir}}/{{hook}} -a -x ./scripts/{{hook}}.sh ]; then ln -s ../../scripts/{{hook}}.sh {{hooks_dir}}/{{hook}}; fi

setup: (githook "pre-commit") (githook "prepare-commit-msg")

build:
    cargo build --workspace --all-targets
clean:
    cargo clean
test:
    cargo test --workspace

clippy:
    cargo clippy --workspace --all-targets
c:
    cargo c
pedantic:
    cargo clippy --workspace --all-targets --features pedantic
update:
    cargo update
bloat:
    cargo bloat
cbuild: clean build
rustfmt:
    cargo fmt --all -- --check
alias fmt := rustfmt
check: rustfmt update test clippy
fixlock:
    rm Cargo.lock
    cargo update
    git add Cargo.lock

branch := "develop"
merge_request:
    git push -o merge_request.create -o merge_request.target={{branch}}
alias mr := merge_request

gpf:
    git push --force-with-lease

server:
    cargo build -p razor-rpc-server

client:
    cargo build -p razor-rpc-client

ztool:
    cargo build -p razor-ztool

docker:
    docker build -t statehub_razor:local --build-arg .


#syntax=docker/dockerfile:1.2

FROM ubuntu:focal as builder
ARG RUST_TOOLCHAIN
ARG ARTIFACTS=/artifacts
ARG BUILD_DIR=/build

RUN apt update \
	&& apt install -y \
	build-essential \
	curl \
	libzfslinux-dev \
	llvm-dev \
	libclang-dev \
	clang

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y --default-toolchain ${RUST_TOOLCHAIN}

COPY . ${BUILD_DIR}
WORKDIR ${BUILD_DIR}

RUN . ${HOME}/.cargo/env && cargo build --workspace

RUN mkdir -pv ${ARTIFACTS} \
	&& cp ./target/debug/razor-rpc-server ./target/debug/razor-rpc-client ./target/debug/ztool ${ARTIFACTS}/


FROM ubuntu:focal
ARG BIN_DIR=/bin/zfsrpc
ARG ARTIFACTS=/artifacts

COPY --from=builder ${ARTIFACTS}/razor-rpc-server ${ARTIFACTS}/razor-rpc-client ${ARTIFACTS}/ztool ${BIN_DIR}/

RUN apt update \
	&& apt install -y \
	zfsutils-linux

EXPOSE 50051
ENV RUST_LOG="debug"

WORKDIR ${BIN_DIR}
CMD ./razor-rpc-server


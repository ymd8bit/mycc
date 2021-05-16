FROM rust:latest

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update
RUN apt-get install --no-install-recommends -y gcc make git binutils libc6-dev
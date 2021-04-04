FROM rust:latest

RUN apt-get update
RUN apt-get install -y gcc make git binutils libc6-dev
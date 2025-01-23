FROM mcr.microsoft.com/vscode/devcontainers/base:ubuntu-22.04

RUN apt-get update \
  && apt-get install -y wget gnupg2 ca-certificates lsb-release build-essential git \
  && apt-get clean

RUN apt update && apt upgrade -y \
  && apt install -y libudev-dev pkg-config \
  && apt clean

COPY . /workspace

WORKDIR /workspace

EXPOSE 3000 8080 5000

CMD ["sleep", "infinity"]

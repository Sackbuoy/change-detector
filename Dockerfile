FROM debian:bullseye-slim

SHELL ["/bin/bash", "-c"]

# Avoid debconf errors.
ARG DEBIAN_FRONTEND=noninteractive
ARG TERM=linux

ARG app_dir=/opt/change-detector

# 0 rhyme or reason to how these are grouped
RUN apt update && \
  apt install -y curl && \
  apt install -y git && \
  apt install -y ssmtp && \
  apt install -y wget xvfb unzip gnupg pkg-config libssl-dev && \
  apt install -y cron && \
  apt install -y build-essential && \
  apt install -y libgconf-2-4

# Chrome setup
# Set up the Chrome PPA
RUN wget -q -O - https://dl-ssl.google.com/linux/linux_signing_key.pub | apt-key add -
RUN echo "deb http://dl.google.com/linux/chrome/deb/ stable main" >> /etc/apt/sources.list.d/google.list
# Update the package list and install chrome
RUN apt-get update -y
RUN apt-get install -y google-chrome-stable

RUN useradd -ms /bin/bash change-detector

# set up app workdir
RUN mkdir -p $app_dir
COPY . $app_dir
RUN chown -R change-detector $app_dir
USER change-detector
WORKDIR $app_dir

ARG RUST_VERSION=1.68.2

# install asdf
ENV HOME /home/change-detector
RUN git clone https://github.com/asdf-vm/asdf.git ~/.asdf --branch v0.10.0
RUN  echo ". $HOME/.asdf/asdf.sh" >> $HOME/.bashrc
ENV PATH "${PATH}:${HOME}/.asdf/bin:${HOME}/.asdf/shims"

# install rust/cargo
RUN asdf plugin add rust; asdf install rust $RUST_VERSION; asdf global rust $RUST_VERSION

# install chromedriver
RUN asdf plugin add chromedriver; asdf install chromedriver latest; asdf global chromedriver latest

ENV CARGO_NET_GIT_FETCH_WITH_CLI true
RUN cargo build --release

ENV RUST_LOG info

# not sure if this is necessary but idc rn
EXPOSE 9515

ENTRYPOINT ./target/release/change-detector

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

# Chromdriver setup
# Set up the Chrome PPA
RUN wget -q -O - https://dl-ssl.google.com/linux/linux_signing_key.pub | apt-key add -
RUN echo "deb http://dl.google.com/linux/chrome/deb/ stable main" >> /etc/apt/sources.list.d/google.list
# Update the package list and install chrome
RUN apt-get update -y
RUN apt-get install -y google-chrome-stable
# Set up Chromedriver Environment variables
ENV CHROMEDRIVER_VERSION 113.0.5672.24
ENV CHROMEDRIVER_DIR /chromedriver
RUN mkdir -p $CHROMEDRIVER_DIR
# Download and install Chromedriver
RUN wget -q --continue -P $CHROMEDRIVER_DIR "http://chromedriver.storage.googleapis.com/$CHROMEDRIVER_VERSION/chromedriver_linux64.zip"
RUN unzip $CHROMEDRIVER_DIR/chromedriver* -d $CHROMEDRIVER_DIR
RUN chmod +x $CHROMEDRIVER_DIR/chromedriver

RUN useradd -ms /bin/bash change-detector

# set up app workdir
RUN mkdir -p $app_dir
COPY . $app_dir
RUN chown -R change-detector $app_dir
USER change-detector
WORKDIR $app_dir

# Put Chromedriver into the PATH
ENV PATH $CHROMEDRIVER_DIR:$PATH

ARG RUST_VERSION=1.68.2

# install rust/cargo
ENV HOME /home/change-detector
RUN git clone https://github.com/asdf-vm/asdf.git ~/.asdf --branch v0.10.0
RUN  echo ". $HOME/.asdf/asdf.sh" >> $HOME/.bashrc
ENV PATH "${PATH}:${HOME}/.asdf/bin:${HOME}/.asdf/shims"
RUN asdf plugin add rust; asdf install rust $RUST_VERSION; asdf global rust $RUST_VERSION

ENV CARGO_NET_GIT_FETCH_WITH_CLI true
RUN cargo build --release

ENV RUST_LOG info

# not sure if this is necessary but idc rn
EXPOSE 9515

ENTRYPOINT ./target/release/change-detector

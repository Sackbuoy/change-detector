# Change Detector - but rusty

A simple web scraping service that notifies you via your desired method whenever a webpage changes. 

## Usage
Execution is just a single binary that relies on a central configuration file(example in `infra/local/`).
Easiest way to use it is docker-compose, but you can also run the container via docker or clone and build the
project, then execute the binary.

### Configuration
In order to get any notifications you will need an stmp server with basic auth set up, I may add some links on doing
this later but you can google's SMTP server for free if you have a gmail account, you just need to enable it and
set up some application credentials. Also rn you need to have the `sms` section of the config even just with dummy
values or it will panic lol I wanna do that later.

### Docker
To deploy with docker-compose, use this template
```
version: '3.7'

services:
  service-name:
    image: "ghcr.io/sackbuoy/change-detector"
    restart: always
    container_name: "container-name"
    volumes:
      - ${PWD}/configuration.yaml:/opt/change-detector/configuration.yaml
```
and ensure your `configuration.yaml` is in the same directory.

Alternatively, you can probably deploy directly using docker by running
```
docker run -v /path/to/configuration.yaml:/opt/change-detector/configuration.yaml ghcr.io/sackbuoy/change-detector
```
thats probably it idk I havent tried it that way.

### Execute locally
Clone the repo, set up your configuration.yaml in the root of the repo and run `cargo run`

## Future Work
- Implement SMS notifications
- add support for geckodriver

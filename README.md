## my rust cli-starter

This is how I startup a rust cli project. It's dockerized. And it uses the seahorse crate for cli.

I use docker-compose because I'm often making api calls to some service in my projects. There is a naive scripts for quickly building and testing.

### Start

```
git clone https://github.com/7db9a/cli-starter
cd cli-starter
docker volume create --name=cli-starter-cargo-data
docker build -t cli-starter:0.1.0 .
docker-compose up
```

See `dev.sh` for command usage.

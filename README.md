# ruster

[![Generic badge](https://img.shields.io/github/workflow/status/reaandrew/ruster/Rust)](https://shields.io/)

Request something from something by sending something receiving something and asserting something.

## Building

1. First you need to initiate and update thea `rusty_v8` submodule.  This will take a minute or three.

```
git submodule update --init --recursive
```

2. For the next bit I followed the `rusty_v8` instructions and installed [https://github.com/mozilla/sccache](sccache) which after the initial build (takes about 30 mins) reduces the time to seconds.

```
cargo build
```

---

## Testing

If developing inside a docker container I use the following helper function which configures various docker capabilities, without `--security-opt seccomp=unconfined` it wont run:

```shell
dockershellhere() {
    dirname=${PWD##*/}
    docker run --rm -it --entrypoint=/bin/zsh --security-opt seccomp=unconfined -v `pwd`:/${dirname} \
      -v ~/.cargo:/home/docker/.cargo \
      -v ~/.ssh:/home/docker/.ssh \
      -w /${dirname} "$@"
}
```

To test and also get code coverage, [https://github.com/xd009642/tarpaulin](Tarpaulin) is used:

1. Install `cargo install cargo-tarpaulin`
2. Run `cargo tarpaulin --all-features --coveralls TOKEN_HERE`

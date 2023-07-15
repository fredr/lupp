# lupp 🕵️

[![Crates.io](https://img.shields.io/crates/v/lupp.svg)](https://crates.io/crates/lupp)

Lupp is a small command line tool that can be used to enhance the experience when looking at logs in the terminal.

It tries to highligh important parts of the log message, and brim down the contextual fields.

![image](https://github.com/fredr/lupp/assets/762956/1accba6a-2352-4464-b57d-0c98c856c462)


## Install

Via cargo

```bash
cargo install lupp
```

## Usage

Pipe logs from example `kubectl`

```bash
kubectl logs -f pod-name | lupp
```

## Caveat

The implementation is quite naive, and this tools is very WIP. But I find it usefull. Feel free to suggest improvements!

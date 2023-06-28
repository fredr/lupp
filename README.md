# lupp 🕵️

Lupp is a small command line tool that can be used to enhance the experience when looking at logs in the terminal.

It tries to highligh important parts of the log message, and brim down the contextual fields.

## Install

Via cargo

```bash
cargo install lupp
```

## Usage

Pipe logs from example `kubectl`

```bash
kubectl logs -f -l name=grafana-agent --all-containers --max-log-requests 100 | lupp
```

## Caveat

The implementation is quite naive, and this tools is very WIP. But I find it usefull. Feel free to suggest improvements!

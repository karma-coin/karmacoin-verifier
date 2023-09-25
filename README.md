# KarmaCoin Verifier

This repo contains the code for the Karmachain 2.0 human verification service. The service is written in Rust and is using an external authentication provider.

To learn more about KarmaCoin visit https://karmaco.in

---

## Setup
- Clone this repo
- Install rust via `rustup` - stable toolchain - default installation options
- Install `cargo-nextest`. See https://nexte.st/book/pre-built-binaries 


### Ubuntu
Install the following packages via apt-get or similar:
- build-essential
- pkg-config
- libssl-dev
- protobuf-compiler
- libclang-dev
 

## Building

### Building a dev build
```cargo build```

### Building for release
```cargo build --release```

## Testing
Make sure you have a valid config.yaml. 
Check `config_template.yaml` for config file schema.

- Use [cargo-nextest](https://nexte.st/) runner.


```cargo nextest run --test-threads 1```

## Running

Create a config file `config.yaml` with the authentication tokens for the service providers used by the verifier, and provide the path to the config file to the server app.

### Running a dev build
```bash
cargo build
./target/debug/server-app
```

### Running a release build

```bash
cargo build --release
./target/release/server-app
```
---

## Dev Notes

### Protos for downstream Dart repos
This repo contains the canonical protobufs definitions for the Karma Coin APIs. To generate protos for other Karma Coin projects in Dart, first enable the dart protoc plugin:

```bash
dart pub global activate protoc_plugin
```
Next, run the following commands from `[project_root]/crates/base/proto`.

```bash
mkdir dart
protoc --dart_out=grpc:dart  karma_coin/core_types/*.proto
```

and copy over the generated files to your Dart project.

### Timestamps
All timestamps should be in milliseconds using chrono. Use milliseconds in clients when working with timestamps.

```rust
use chrono::prelude::*;
let t = Utc::now().timestamp_millis() as u64;
```

### Xactor Usage 
Xactor (unlike Actix) gives us nice and clean async syntax for actor messages. However, be aware that calling an actor message from the body of the same actor's message handler, will deadlock. It is easy to spend hours on such bugs. Just factor out impl into an async fn and call it from different message handlers....

### Code Structure
- `base` - shared types.
- `crypto` - low-level crypto lib.
- `server` - Server implementation.
- `server-app` - Simple console server app.

### Docker 
To build docker image

```bash
docker build . -t teamkarmacoin/karmacoin-verifier
```

To use configuration file from host machine, mount it to `/config.yaml` inside the container. And than run the container. Both actions can be done with the following command:

```bash
docker run -d -p 9080:9080 --name karmacoin-verifier --mount type=bind,source="$(pwd)"/config.yaml,target=/config.yaml teamkarmacoin/karmacoin-verifier
```

---

Copyright (c) 2023 by the KarmaCoin Authors. This work is licensed under the [KarmaCoin License](https://github.com/karma-coin/.github/blob/main/LICENSE).





# Aruna - API Definitions

This repo contains the definitions of the Aruna API. It is written in the [protocol-buffers](https://developers.google.com/protocol-buffers) interface definition language (IDL). This can be used to automatically generate clients in many different programming languages using the [gRPC](https://grpc.io/docs/what-is-grpc/introduction/) framework. 


## Usage

The official grpc documentation provides [Quick start](https://grpc.io/docs/languages/) sections for many programming languages. A good starting point is the [python](https://grpc.io/docs/languages/python/quickstart/#generate-grpc-code) section. You can either use the official [protoc compiler](https://grpc.io/docs/protoc-installation/) or tools like [buf](https://buf.build/) to generate stubs for the Aruna API.

We also automatically generate packages for the most common programming languages for each release:
- **Python**: [PyPI](https://pypi.org/project/Aruna-Python-API/) 
- **Rust**: [Crates.io](https://crates.io/crates/aruna-rust-api)
- **Go**: [Go package](https://github.com/ArunaStorage/go-api/releases/)
- **Java**: [Java Package](https://github.com/ArunaStorage/java-api/packages)


## Structure

The API contains three main sections:

- [Storage section](#storage): This is the main section for external use. It contains a basic set of services and models that describe the interfaces with the storage system.

- [Notification section](#notifications): This section contains a set of services and models that describe the interfaces with the notification system.

- [Hooks section](#hooks): This section contains the service that can be used to extend Aruna with external functionality or automate internal processes.


### Storage

The storage section is divided in two subsections:

- [Models](aruna/api/storage/models/v2/): This section contains the models that are used by the storage system. 

- [Storage services](aruna/api/storage/services/v2/): This section contains all services that are used to interact with the storage system. Services are defined as RPCs and are grouped by object type.


### Notifications

The Notification section provides a set of RPCs that are used to interact with the notification system. The notification system uses [nats.io](https://nats.io/) as its underlying service. The service definition can be found [here](aruna/api/notification/services/v2/notification_service.proto).


### Hooks

Hooks are the way to automate internal processes in Aruna and/or to integrate external services to extend functionality. Once created, they're available globally in Aruna, and Projects must be associated with them to be included in their trigger cycle. The action that triggers the specific hook is defined by its trigger type. The service definition can be found [here](aruna/api/hooks/services/v2/hooks_service.proto).


## License

The API is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option. Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions. 

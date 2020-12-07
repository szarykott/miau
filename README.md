# Miau

**Miau** is a *layered configuration system* for Rust Programming Lanaguage.
It has been built with **extensibility** and **async** in mind from ground up. 

It is built around `serde`, famous serialization an deserialization library for Rust. `serde` is the only non-optional heavy dependency of Miau.

It has been inspired by both Rust's `config-rs` and .NET's `IConfiguration`.

---

## Features


* out of the box support for configuration sources that do not require extra dependencies (in memory, environment, files, ...)
* out of the box support for most popular data formats (json, yaml, json5, ...)
* unit tested infrastructure for layering configurations
* unit tested infrastructure for creating strongly typed configurations from layered ones
* possibility to create your own sources or formats by implementing simple traits
* support for both sync and async runtimes (Miau is *truly* executor agnostic)
* all optional dependencies are feature flagged so that you do not pay for features you do not use

## Non-features

* heavy dependencies (this is the reason no remote source is supported out of the box, usually they require heavy dependencies and can be done in at least few ways, for instance issuing http request)

---

## Definitions

In Miau, configurations are retrieved from **sources** and stored in **builder** along with information about **format**. When **builder** is built, it transforms its content into collection of **configuration nodes** that together form **configuration**. It can be used as is or deserialized into strongly typed struct of choice.

### Source

Source is, as name suggests, source of your configuration. It can be an in-memory source, a file or remote source, be it your homemade tool or industry standard one like consul.

Data can be retrieved from sources in both synchronous and asynchronous way using executor of choice. 

Sources that are supported out of the box are the ones that can be implemented using standard library features.

### Format

Format is way in which configuraton is preserved in file or in other source. Miau provides support for most popular formats that are also supported by `serde` like json, json5, yaml and others. 

Most popular formats are supported out of the box. If format you are looking for is not supported, worry not, as long as it is supported by `serde` it can be easily integrated with Miau.

### Configuration node

Configuration node is a tree like structure that corresponds to exactly one configuration source. It keeps all the information for the source in a strongly typed manner.

### Configuration

Configuration is an ordered collection of configuration nodes. Order of nodes corresponds to order of adding them to builder. Each node corresponds to one source.

When key from configuration is requested nodes are searched for a match in revers order, starting from most recent ones.

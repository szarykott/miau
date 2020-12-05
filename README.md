# Scriba

Scriba is a layered configuration system for Rust Programming Lanaguage.
It has been built with async in mind.

Its principle is extensibility. Therefore it exposes traits that can be used to plug into it with your configuration sources and formats.

Scriba is built around `serde`, famous serialization an deserialization library for Rust. It is the only non-optional heavy dependency of Scriba. 

## Overview

There are few concepts in this library :
* source
* format
* builder
* configuration
* configuration node

### Source

Source is, as name suggests, source of your configuration. It can be an in-memory source, a file or remote source, be it your homemade tool or industry one, like consul for instance.

Scriba provides utilities for all sources that can be coded with standard library functions. It means that sources like environment, file or in-memory collections are supported out of the box. However, support to remote sources will never be implemented in Scriba. Reason for it is quite simple. Let's imagine a source that returns configuration via HTTP. There's plenty of HTTP libraries, they come and go. Scriba chooses to be agnostic from those external libraries. Support for such sources might be implemented in separate libraries.

You can also implement support for it yourself, it is enough to implement `Source` (or `AsyncSource`) trait to be able to plug it in Scriba as if it was always there.

### Format

Format is way in which configuraton is preserved in file or in other source. Scriba provides support for most popular formats that are also supported by `serde` like json, json5, yaml and others.

Scriba exposes `Format` trait. It is enough to implement it for your format to be able to plug it in.

Keeping notions of source and format separate and exporting them as traits allows for code reuse and extensibility.

### Builder

To transform sources into layered configuration Scriba uses builder that keep track of them along with associated formats. Builder comes in two flavours - sync and async. Creating configurations is as easy as adding new sources and calling `build` in the end. 

### Configuration

Configuration is an ordered collection of configuration nodes (see below). Order of nodes corresponds to order of adding them to builder. Each node corresponds to one source.

When key from configuration is requested nodes are searched for a match in revers order, starting from most recent ones.


### Configuration node

Configuration node is a tree like structure that corresponds to exactly one configuration source. It keeps all the information for a source in a strongly typed way.






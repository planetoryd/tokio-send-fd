# tokio-send-fd

A library to send tokio UnixStream and raw file descriptors over tokio UnixStream connections.

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/tokio-send-fd.svg
[crates-url]: https://crates.io/crates/tokio-send-fd
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/alexander-smoktal/tokio-send-fd/blob/main/LICENSE
[actions-badge]: https://github.com/alexander-smoktal/tokio-send-fd/actions/workflows/rust.yml/badge.svg
[actions-url]: https://github.com/alexander-smoktal/tokio-send-fd/actions/workflows/rust.yml

## Overview
The crate is a library for sending and receiving Unix file descriptors over tokio UnixStream connections.
You can transfer **RawFd** or **UnixStream**. See [test_raw_fd.rs](./tests/test_raw_fd.rs) and [test_tokio_stream.rs](./tests/test_tokio_stream.rs) for examples.

//! **tokio-send-fd** is a library for sending and receiving Unix file descriptors over tokio UnixStream connections.
//! You can either transfer [RawFd] or [UnixStream](tokio::net::UnixStream).
//!
//! ## Examles
//! See [test_raw_fd.rs](https://github.com/alexander-smoktal/tokio-send-fd/blob/main/tests/test_raw_fd.rs) and [test_tokio_stream.rs](https://github.com/alexander-smoktal/tokio-send-fd/blob/main/tests/test_tokio_stream.rs) for examples.
//!
//! ## Creating [UnixStream](tokio::net::UnixStream) from [RawFd]
//! If you want to create tokio [UnixStream](tokio::net::UnixStream) from a raw file descriptor created by
//! os' [UnixStream::pair](std::os::unix::net::UnixStream::pair()) call, you should make it
//! [set_nonblocking(true)](std::os::unix::net::UnixStream::set_nonblocking()), otherwise tokio stream will block
//! in async functions ⚠️
//!
//! ## Transfering socket pair ownership
//! Sending a socket of a socket pair doesn't close the local copy, which leads to having the socket being
//! opened until the sender is shut down.
//! If you want counterparties to detect peer shutdown, you have to close socket pair right after sending
//! a socket to a peer.
//! Use [close](https://docs.rs/nix/latest/nix/unistd/fn.close.html) Posix call.
//!
//! ## Async trait disclaimer
//! The crate uses [async-trait](https://crates.io/crates/async-trait) and because of this has a lot of extra
//! lifetime parameters on the trait. Once async traits RFC is merged, dependency will be removed.
use std::{
    io::{Error, ErrorKind},
    os::unix::{
        io::{AsRawFd, RawFd},
        net::UnixStream as OsUnixStream,
        prelude::{FromRawFd, IntoRawFd},
    },
};

use async_trait::async_trait;
use tokio::{io::Interest, net::UnixStream};

use passfd::FdPassingExt;

/// SendFd trait, *use* this extend [UnixStream](tokio::net::UnixStream) with sending and receiving functions
#[async_trait]
pub trait SendFd {
    /// Send RawFd
    async fn send_fd(&self, fd: RawFd) -> Result<(), Error>;
    /// Receive RawFd
    async fn recv_fd(&self) -> Result<RawFd, Error>;
    /// Send tokio UnixStream
    async fn send_stream(&self, stream: UnixStream) -> Result<(), Error>;
    /// Receive tokio UnixStream
    async fn recv_stream(&self) -> Result<UnixStream, Error>;
}

#[async_trait]
impl SendFd for UnixStream {
    async fn send_fd(&self, fd: RawFd) -> Result<(), Error> {
        loop {
            self.writable().await?;

            match self.try_io(Interest::WRITABLE, || self.as_raw_fd().send_fd(fd)) {
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    continue;
                }
                r => return r,
            }
        }
    }

    async fn recv_fd(&self) -> Result<RawFd, Error> {
        loop {
            self.readable().await?;

            match self.try_io(Interest::READABLE, || self.as_raw_fd().recv_fd()) {
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    continue;
                }
                r => return r,
            }
        }
    }

    async fn send_stream(&self, stream: UnixStream) -> Result<(), Error> {
        let fd = stream.into_std()?.into_raw_fd();

        self.send_fd(fd).await
    }

    async fn recv_stream(&self) -> Result<UnixStream, Error> {
        let fd = self.recv_fd().await?;

        let os_stream = unsafe { OsUnixStream::from_raw_fd(fd) };
        UnixStream::from_std(os_stream)
    }
}

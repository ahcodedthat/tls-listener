# Changelog

## Upcoming 0.5.0

### Added

- Added [`AsyncAccept::until`] method, that creates a new `AsyncAccept` that will stop accepting connections after another future finishes.

### Changed
- **Backwards incompatible**: `AsyncAccept::poll_accept` now returns, `Poll<Option<Result<...>>>` instead of `Poll<Result<...>>`. This allows the incoming stream of connections to stop, for example, if a graceful shutdown has been initiated. `impl`s provided by this crate have been updated, but custom implementations of `AsyncAccept`, or direct usage of the trait may break.

## 0.4.2 - 2022-04-09

### Added

- Added [TlsListener::replace_acceptor()] function to allow replacing the listener certificate at runtime.

## 0.4.1 - 2022-04-09

### Changed

- The implementation of `AsyncTls` for `tokio_native_tls::TlsAcceptor` now requires the connection type to implement `Send`. This in turn allows `TlsListener` to be `Send` when using the `native-tls` feature. Technically, this is a breaking change. However, in practice it is unlikely to break existing code and makes using `TlsListener` much easier to use when `native-tls` is enabled.

## 0.4.0 - 2022-02-22

NOTE: This release contains several breaking changes.

### Added

- Support for [`native-tls`](https://github.com/sfackler/rust-native-tls).

### Changed

- The TLS backend is now configurable. Both rustls and native-tls are supported. Other backends can also be used by implementing the `AsyncTls` trait.
  - You must now supply either the `rustls` or `native-tls` features to get support for a tls backend.
  - Unfortunately, the machinery for this required adding an additional type parameter to `TlsListener`.
- The `TlsListener` stream now returns a `tls_listener::Error` instead of `std::io::Error` type.
- Signatures of `TcpListener::new()` and `builder()` have changed to now take an argument of the TLS type rather than a `rustls::ServerConfig`,
  to update existing calls, replace `builder(config)` with `builder(Arc::new(config).into())`.

### Fixed

- Crate will now compile when linked against a target that doesn't explicitly enable the `tokio/time` and `hyper/tcp`
  features.
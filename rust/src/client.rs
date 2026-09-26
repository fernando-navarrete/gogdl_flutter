//! The HTTP client `GogdlApi` hands to `gogdl-lib`.

use std::time::Duration;

use gogdl_lib::Client;

/// How long a connection may take to open.
pub(crate) const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
/// How long a request may go without reading anything. It runs from `send()` until the response
/// headers arrive (so it also covers uploading the request body), then restarts on each read of
/// the response body. There's no overall timeout, so long transfers aren't capped.
pub(crate) const READ_TIMEOUT: Duration = Duration::from_secs(30);

/// Builds the client with `connect` and `read` timeouts, or falls back to a client with none if
/// that fails, so `GogdlApi::new` stays infallible.
pub(crate) fn build(connect: Duration, read: Duration) -> Client {
    Client::builder()
        .connect_timeout(connect)
        .read_timeout(read)
        .build()
        .unwrap_or_else(|_| Client::new())
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use futures_util::StreamExt;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    use super::*;

    const SHORT: Duration = Duration::from_millis(200);
    /// Far longer than `SHORT`, so a pass means the timeout fired, not that the test was slow.
    const BOUND: Duration = Duration::from_secs(5);

    /// A local server that accepts one connection and hands it to `serve`.
    async fn server<F, Fut>(serve: F) -> String
    where
        F: FnOnce(tokio::net::TcpStream) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send,
    {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}/", listener.local_addr().unwrap());
        tokio::spawn(async move {
            let (socket, _) = listener.accept().await.unwrap();
            serve(socket).await;
        });
        url
    }

    #[tokio::test]
    async fn a_server_that_never_answers_times_out() {
        let url = server(|mut socket| async move {
            let mut buf = [0; 4096];
            let _ = socket.read(&mut buf).await;
            std::future::pending::<()>().await;
        })
        .await;

        let start = Instant::now();
        let err = build(SHORT, SHORT).get(url).send().await.unwrap_err();
        assert!(err.is_timeout(), "{err:?}");
        assert!(start.elapsed() < BOUND);
    }

    #[tokio::test]
    async fn a_body_that_stalls_times_out() {
        let url = server(|mut socket| async move {
            let mut buf = [0; 4096];
            let _ = socket.read(&mut buf).await;
            let head = b"HTTP/1.1 200 OK\r\nContent-Length: 100\r\n\r\nhalf";
            socket.write_all(head).await.unwrap();
            std::future::pending::<()>().await;
        })
        .await;

        let response = build(SHORT, SHORT).get(url).send().await.unwrap();
        let mut body = response.bytes_stream();
        assert_eq!(&body.next().await.unwrap().unwrap()[..], b"half");
        let start = Instant::now();
        let err = body.next().await.unwrap().unwrap_err();
        assert!(err.is_timeout(), "{err:?}");
        assert!(start.elapsed() < BOUND);
    }

    #[tokio::test]
    async fn an_upload_that_takes_longer_than_the_read_timeout_fails() {
        // Never reads, so the body can't be sent past the socket buffers.
        let url = server(|socket| async move {
            let _open = socket;
            std::future::pending::<()>().await;
        })
        .await;

        let start = Instant::now();
        let err = build(SHORT, SHORT)
            .put(url)
            .body(vec![0u8; 64 * 1024 * 1024])
            .send()
            .await
            .unwrap_err();
        assert!(err.is_timeout(), "{err:?}");
        assert!(start.elapsed() < BOUND);
    }
}

use tokio::{io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt}};
use anyhow::{Context, Result};

use crate::crdt::SpatialEnvironment;

pub struct PeerConnection<S> {
    stream: S
}

impl<S: AsyncRead + AsyncWrite + Unpin> PeerConnection<S> {
    pub const fn new(stream: S) -> Self {
        Self { stream }
    }

    /// Sends a serialized spatial environment over the underlying stream.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails, if the serialized payload is
    /// larger than `u32::MAX`, or if writing to the stream fails.
    pub async fn send(&mut self, env: &SpatialEnvironment) -> Result<()> {
        // currently sending whole env,
        // in the future could be just an annotation
        let bytes = serde_json::to_vec(env)?;
        let len = u32::try_from(bytes.len())
            .context("serialized spatial environment exceeds u32::MAX bytes")?;
        self.stream.write_all(&len.to_be_bytes()).await?;
        self.stream.write_all(&bytes).await?;
        Ok(())
    }

    /// Receives and deserializes a spatial environment from the stream.
    ///
    /// # Errors
    ///
    /// Returns an error if reading from the stream fails or if the received
    /// payload cannot be deserialized as a spatial environment.
    pub async fn receive(&mut self) -> Result<SpatialEnvironment> {
        // currently sending whole env,
        // in the future could be just an annotation
        let mut len_buf = [0u8; 4];
        self.stream.read_exact(&mut len_buf).await?;
        let len = u32::from_be_bytes(len_buf) as usize;
        let mut buf = vec![0u8; len];
        self.stream.read_exact(&mut buf).await?;
        let env: SpatialEnvironment = serde_json::from_slice(&buf)?;
        Ok(env)
    }
}

/// Exchanges state with a peer and merges the received environment locally.
///
/// This sends `env` to the remote peer, receives the peer's environment, and
/// then merges the remote state into `env`.
///
/// # Errors
///
/// Returns an error if sending to the peer fails or receiving/deserializing
/// the peer environment fails.
pub async fn sync_peer<S: AsyncRead + AsyncWrite + Unpin>(conn: &mut PeerConnection<S>, env: &mut SpatialEnvironment) -> Result<()> {
    let send_result = conn.send(env).await;
    send_result?;
    let receive_result = conn.receive().await;
    let remote_env = receive_result?;

    env.merge(remote_env);
    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::crdt::{Point, SpatialAnnotation};

    use super::*;

    #[tokio::test]
    async fn test_roundtrip() {
        let (client, server) = tokio::io::duplex(1024);
        let mut sender = PeerConnection::new(client);
        let mut receiver = PeerConnection::new(server);

        let mut env = SpatialEnvironment::new();
        env.create_annotation(
            SpatialAnnotation::new(
                None,
                Point(1, 2),
                String::from("hello world")
            )
        );

        let _ = sender.send(&env).await.expect(
            "PeerConnection send should succeed"
        );
        let received_env = receiver.receive().await.expect(
            "PeerConnection recieve should get the environment that was sent"
        );

        assert_eq!(env, received_env);
    }
}
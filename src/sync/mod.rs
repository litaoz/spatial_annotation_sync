use tokio::{io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt}};
use anyhow::Result;

use super::SpatialEnvironment;

pub struct PeerConnection<S> {
    stream: S
}

impl<S: AsyncRead + AsyncWrite + Unpin> PeerConnection<S> {
    pub fn new(stream: S) -> Self {
        Self { stream }
    }

    pub async fn send(&mut self, env: &SpatialEnvironment) -> Result<()> {
        // currently sending whole env,
        // in the future could be just an annotation
        let bytes = serde_json::to_vec(env)?;
        let len = bytes.len() as u32;
        self.stream.write_all(&len.to_be_bytes()).await?;
        self.stream.write_all(&bytes).await?;
        Ok(())
    }

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

#[cfg(test)]
mod tests {

    use crate::{Point, SpatialAnnotation};

    use super::*;

    #[tokio::test]
    async fn test_roundtrip() {
        let (client, server) = tokio::io::duplex(1024);
        let mut sender = PeerConnection::new(client);
        let mut receiver = PeerConnection::new(server);

        let mut env = SpatialEnvironment::new();
        env.create_annotation(
            SpatialAnnotation{
                coord: Point(1, 2),
                text: String::from("hello world")
            }
        );
        // let annotation = SpatialAnnotationInternal::new(
        //     SpatialAnnotation{
        //         coord: Point(1, 2),
        //         text: String::from("hello world")
        //     },
        //     Uuid::new_v4()
        // );
        let _ = sender.send(&env).await.expect(
            "PeerConnection send should succeed"
        );
        let received_env = receiver.receive().await.expect(
            "PeerConnection recieve should get the environment that was sent"
        );

        assert_eq!(env, received_env);
    }
}
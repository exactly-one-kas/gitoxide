use async_trait::async_trait;
use bstr::BString;
use futures_io::{AsyncRead, AsyncWrite};
use futures_lite::AsyncWriteExt;
use git_packetline::PacketLineRef;

use crate::{
    client::{self, capabilities, git, Capabilities, SetServiceResponse},
    Protocol, Service,
};

impl<R, W> client::TransportWithoutIO for git::Connection<R, W>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    fn request(
        &mut self,
        write_mode: client::WriteMode,
        on_into_read: client::MessageKind,
    ) -> Result<client::RequestWriter<'_>, client::Error> {
        Ok(client::RequestWriter::new_from_bufread(
            &mut self.writer,
            Box::new(self.line_provider.as_read_without_sidebands()),
            write_mode,
            on_into_read,
        ))
    }
    fn to_url(&self) -> String {
        self.custom_url.as_ref().map_or_else(
            || {
                let mut possibly_lossy_url = self.path.to_string();
                possibly_lossy_url.insert_str(0, "file://");
                possibly_lossy_url
            },
            |url| url.clone(),
        )
    }

    /// We implement this in a paranoid and safe way, not allowing downgrade to V1 which
    /// could send large amounts of refs in case we didn't want to support V1.
    fn supported_protocol_versions(&self) -> &[Protocol] {
        if self.desired_version == Protocol::V1 {
            &[]
        } else {
            &self.supported_versions
        }
    }

    fn connection_persists_across_multiple_requests(&self) -> bool {
        true
    }
}

#[async_trait(?Send)]
impl<R, W> client::Transport for git::Connection<R, W>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    async fn handshake<'a>(
        &mut self,
        service: Service,
        extra_parameters: &'a [(&'a str, Option<&'a str>)],
    ) -> Result<SetServiceResponse<'_>, client::Error> {
        if self.mode == git::ConnectMode::Daemon {
            let mut line_writer = git_packetline::Writer::new(&mut self.writer).binary_mode();
            line_writer
                .write_all(&git::message::connect(
                    service,
                    self.desired_version,
                    &self.path,
                    self.virtual_host.as_ref(),
                    extra_parameters,
                ))
                .await?;
            line_writer.flush().await?;
        }

        let capabilities::recv::Outcome {
            capabilities,
            refs,
            protocol: actual_protocol,
        } = Capabilities::from_lines_with_version_detection(&mut self.line_provider).await?;
        Ok(SetServiceResponse {
            actual_protocol,
            capabilities,
            refs,
        })
    }
}

impl<R, W> git::Connection<R, W>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    /// Create a connection from the given `read` and `write`, asking for `desired_version` as preferred protocol
    /// and the transfer of the repository at `repository_path`.
    ///
    /// `virtual_host` along with a port to which to connect to, while `mode` determines the kind of endpoint to connect to.
    pub fn new(
        read: R,
        write: W,
        desired_version: Protocol,
        repository_path: impl Into<BString>,
        virtual_host: Option<(impl Into<String>, Option<u16>)>,
        mode: git::ConnectMode,
    ) -> Self {
        git::Connection {
            writer: write,
            line_provider: git_packetline::StreamingPeekableIter::new(read, &[PacketLineRef::Flush]),
            path: repository_path.into(),
            virtual_host: virtual_host.map(|(h, p)| (h.into(), p)),
            desired_version,
            custom_url: None,
            supported_versions: [desired_version],
            mode,
        }
    }
}

#[cfg(feature = "async-std")]
mod async_net {
    use crate::client::git;
    use crate::client::Error;
    use async_std::net::TcpStream;
    use std::time::Duration;

    impl git::Connection<TcpStream, TcpStream> {
        /// Create a new TCP connection using the `git` protocol of `desired_version`, and make a connection to `host`
        /// at `port` for accessing the repository at `path` on the server side.
        pub async fn new_tcp(
            host: &str,
            port: Option<u16>,
            path: bstr::BString,
            desired_version: crate::Protocol,
        ) -> Result<git::Connection<TcpStream, TcpStream>, Error> {
            let read = async_std::io::timeout(
                Duration::from_secs(5),
                TcpStream::connect(&(host, port.unwrap_or(9418))),
            )
            .await?;
            let write = read.clone();
            Ok(git::Connection::new(
                read,
                write,
                desired_version,
                path,
                None::<(String, _)>,
                git::ConnectMode::Daemon,
            ))
        }
    }
}

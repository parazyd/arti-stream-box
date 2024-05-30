use std::io;
use std::sync::Arc;

use arti_client::config::onion_service::OnionServiceConfigBuilder;
use arti_client::TorClient;
use async_trait::async_trait;
use futures::stream::BoxStream;
use futures::AsyncRead;
use futures::AsyncWrite;
use smol::lock::OnceCell;
use tor_hsservice::HsNickname;
use tor_hsservice::RendRequest;
use tor_hsservice::RunningOnionService;
use tor_rtcompat::PreferredRuntime;
use url::Url;

pub trait PtStream: AsyncRead + AsyncWrite + Unpin + Send {}

impl PtStream for arti_client::DataStream {}

#[async_trait]
pub trait PtListener: Send + Sync + Unpin {
    async fn next(&self) -> io::Result<(Box<dyn PtStream>, Url)>;
}

static TOR_CLIENT: OnceCell<TorClient<PreferredRuntime>> = OnceCell::new();

pub struct TorListener;

impl TorListener {
    pub async fn new() -> io::Result<Self> {
        Ok(Self {})
    }

    pub(crate) async fn do_listen(&self, port: u16) -> io::Result<TorListenerIntern> {
        let client = match TOR_CLIENT
            .get_or_try_init(|| async { TorClient::builder().create_bootstrapped().await })
            .await
        {
            Ok(client) => client,
            Err(e) => panic!("{}", e),
        };

        let hs_nick = HsNickname::new("tor_hs_test".to_string()).unwrap();

        let hs_config = OnionServiceConfigBuilder::default()
            .nickname(hs_nick)
            .build()
            .unwrap();

        let (onion_service, rendreq_stream) = client.launch_onion_service(hs_config).unwrap();

        Ok(TorListenerIntern {
            port,
            onion_service,
            rendreq_stream: BoxStream::new(rendreq_stream),
        })
    }
}

pub struct TorListenerIntern<'a> {
    port: u16,
    onion_service: Arc<RunningOnionService>,
    rendreq_stream: BoxStream<'a, RendRequest>,
}

#[async_trait]
impl PtListener for TorListenerIntern<'_> {
    async fn next(&self) -> io::Result<(Box<dyn PtStream>, Url)> {
        todo!()
    }
}

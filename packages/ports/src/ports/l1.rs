use std::pin::Pin;

use crate::types::{FuelBlock, FuelBlockCommittedOnL1, InvalidL1Height, L1Height, Stream, U256};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("network error: {0}")]
    Network(String),
    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<InvalidL1Height> for Error {
    fn from(err: InvalidL1Height) -> Self {
        Self::Other(err.to_string())
    }
}

#[cfg_attr(feature = "test-helpers", mockall::automock)]
#[async_trait::async_trait]
pub trait Contract: Send + Sync {
    async fn submit(&self, block: FuelBlock) -> Result<()>;
    fn event_streamer(&self, height: L1Height) -> Box<dyn EventStreamer + Send + Sync>;
}

#[cfg_attr(feature = "test-helpers", mockall::automock)]
#[async_trait::async_trait]
pub trait Api {
    async fn get_block_number(&self) -> Result<L1Height>;
    async fn balance(&self) -> Result<U256>;
}

#[cfg_attr(feature = "test-helpers", mockall::automock)]
#[async_trait::async_trait]
pub trait EventStreamer {
    async fn establish_stream<'a>(
        &'a self,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<FuelBlockCommittedOnL1>> + 'a + Send>>>;
}

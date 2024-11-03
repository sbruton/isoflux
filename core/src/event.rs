use std::fmt::Debug;

use crate::generation::FluxGeneration;
use crate::reexport::*;

pub trait FluxEventPayload: Clone + Debug {}

#[derive(Clone, Debug, CopyGetters)]
pub struct FluxEvent<P>
where
    P: FluxEventPayload,
{
    #[getset(get_copy = "pub")]
    id: Uuid,
    #[getset(get_copy = "pub")]
    timestamp: Timestamp,
    #[getset(get_copy = "pub")]
    generation: FluxGeneration,
    #[getset(get = "pub")]
    payload: P,
}

impl<P> FluxEvent<P>
where
    P: FluxEventPayload,
{
    pub fn new<G>(generation: G, payload: P) -> Self
    where
        G: Into<FluxGeneration>,
    {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            generation: generation.into(),
            payload,
        }
    }
}

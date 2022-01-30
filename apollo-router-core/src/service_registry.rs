use crate::{RouterResponse, SubgraphRequest};
use std::collections::HashMap;
use tower::buffer::Buffer;
use tower::util::{BoxCloneService, BoxService};
use tower::BoxError;
use tower::ServiceExt;

pub struct ServiceRegistry {
    services: HashMap<
        String,
        Buffer<BoxService<SubgraphRequest, RouterResponse, BoxError>, SubgraphRequest>,
    >,
}

impl ServiceRegistry {
    pub(crate) fn new(
        services: HashMap<
            String,
            Buffer<BoxService<SubgraphRequest, RouterResponse, BoxError>, SubgraphRequest>,
        >,
    ) -> Self {
        Self { services }
    }

    pub fn get(
        &self,
        name: &str,
    ) -> Option<BoxCloneService<SubgraphRequest, RouterResponse, BoxError>> {
        self.services.get(name).map(|s| s.clone().boxed_clone())
    }

    pub fn contains(&self, name: &str) -> bool {
        self.services.contains_key(name)
    }
}

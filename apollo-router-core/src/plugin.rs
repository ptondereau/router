use crate::{PlannedRequest, RouterRequest, RouterResponse, SubgraphRequest};
use futures::future::BoxFuture;
use futures::FutureExt;
use tower::util::BoxService;
use tower::BoxError;

pub trait Plugin: Send + Sync + 'static {
    //Configuration is untyped. Implementations may marshal to a strongly typed object
    fn configure(&mut self, _configuration: serde_json::Value) -> Result<(), BoxError> {
        Ok(())
    }

    // Plugins will receive a notification that they should start up and shut down.
    fn startup(&mut self) -> BoxFuture<Result<(), BoxError>> {
        async { Ok(()) }.boxed()
    }
    fn shutdown(&mut self) -> BoxFuture<Result<(), BoxError>> {
        async { Ok(()) }.boxed()
    }

    fn router_service(
        &mut self,
        service: BoxService<RouterRequest, RouterResponse, BoxError>,
    ) -> BoxService<RouterRequest, RouterResponse, BoxError> {
        service
    }

    fn query_planning_service(
        &mut self,
        service: BoxService<RouterRequest, PlannedRequest, BoxError>,
    ) -> BoxService<RouterRequest, PlannedRequest, BoxError> {
        service
    }

    fn execution_service(
        &mut self,
        service: BoxService<PlannedRequest, RouterResponse, BoxError>,
    ) -> BoxService<PlannedRequest, RouterResponse, BoxError> {
        service
    }

    fn subgraph_service(
        &mut self,
        _name: &str,
        service: BoxService<SubgraphRequest, RouterResponse, BoxError>,
    ) -> BoxService<SubgraphRequest, RouterResponse, BoxError> {
        service
    }
}

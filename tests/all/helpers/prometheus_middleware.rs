// use prometheus::Registry;
//
// pub struct MetricsEndpointMiddleware {
//     registry: Registry,
//     endpoint: String,
// }
//
// impl MetricsEndpointMiddleware {
//     pub fn new(endpoint: &str) -> Self {
//         let registry = Registry::new();
//
//         MetricsEndpointMiddleware {
//             registry,
//             endpoint: endpoint.into(),
//         }
//     }
//
//     pub fn registry(&self) -> &Registry {
//         &self.registry
//     }
// }

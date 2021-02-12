use rocket::local::Client;
use rocket_prometheus::PrometheusMetrics;

pub struct TestContext {
    client: Client,
}

impl TestContext {
    pub fn start() -> Self {
        let prometheus = PrometheusMetrics::new();
        let rocket = rocket::ignite()
            .attach(prometheus.clone())
            .mount("/metrics", prometheus);

        let client = Client::new(rocket).expect("valid rocket instance");

        Self { client }
    }

    pub fn get_metrics(&self) -> String {
        let _ = self.client.get("/metrics").dispatch();
        let mut response = self.client.get("/metrics").dispatch();
        response.body_string().expect("expect string response")
    }
}

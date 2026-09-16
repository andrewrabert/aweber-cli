//! One client, shared, with a cap on how much of it is in flight.

use std::sync::Arc;

use crate::ports::{BoxFuture, Http};

pub const IN_FLIGHT_CAP: usize = 4;

pub struct SharedHttp {
    client: aweber::client::Client,
    permits: Arc<tokio::sync::Semaphore>,
}

impl SharedHttp {
    pub fn new(client: aweber::client::Client) -> SharedHttp {
        SharedHttp {
            client,
            permits: Arc::new(tokio::sync::Semaphore::new(IN_FLIGHT_CAP)),
        }
    }
}

impl Http for SharedHttp {
    fn send(
        &self,
        plan: aweber::catalog::RequestPlan,
    ) -> BoxFuture<'static, Result<aweber::client::PlanResponse, aweber::client::ApiError>> {
        let client = self.client.clone();
        let permits = Arc::clone(&self.permits);
        Box::pin(async move {
            let _permit = permits
                .acquire()
                .await
                .expect("the in-flight semaphore is never closed");
            client.send_plan(&plan).await
        })
    }
}

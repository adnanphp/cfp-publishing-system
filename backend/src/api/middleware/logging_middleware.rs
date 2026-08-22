use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse},
    Error,
};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;

pub struct LoggingMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = Instant::now();
        let path = req.path().to_string();
        let method = req.method().to_string();
        
        let fut = self.service.call(req);
        
        Box::pin(async move {
            let res = fut.await?;
            let duration = start.elapsed();
            println!("✅ {} {} - {}ms", method, path, duration.as_millis());
            Ok(res)
        })
    }
}

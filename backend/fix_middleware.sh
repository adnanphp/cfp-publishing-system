#!/bin/bash

# Fix all middleware files
for file in src/api/middleware/*.rs; do
    if [[ $file == *"mod.rs" ]]; then
        continue
    fi
    
    echo "Fixing $file"
    
    # Add proper generics to middleware structs
    sed -i 's/pub struct \([A-Za-z]*\)Service {/pub struct \1Service<S> where S: actix_web::dev::Service<actix_web::dev::ServiceRequest, Response = actix_web::dev::ServiceResponse, Error = actix_web::Error> + '\\''static {/' $file
    sed -i 's/service: dev::Service,/service: S,/' $file
    
    # Fix Transform implementations
    sed -i 's/impl dev::Transform<dev::Service/impl dev::Transform<S/' $file
    sed -i 's/where/where S: actix_web::dev::Service<actix_web::dev::ServiceRequest, Response = actix_web::dev::ServiceResponse, Error = actix_web::Error> + '\\''static,/' $file
    
    # Fix Service implementations
    sed -i 's/impl dev::Service<dev::ServiceRequest> for \([A-Za-z]*\)Service/impl<S> dev::Service<dev::ServiceRequest> for \1Service<S> where S: actix_web::dev::Service<actix_web::dev::ServiceRequest, Response = actix_web::dev::ServiceResponse, Error = actix_web::Error> + '\\''static/' $file
done

# Fix error_middleware.rs specifically
cat > src/api/middleware/error_middleware_fixed.rs << 'FIXED'
use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ready, Ready};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use uuid::Uuid;

pub struct ErrorMiddleware;

impl<S, B> Transform<S, ServiceRequest> for ErrorMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ErrorMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ErrorMiddlewareService { service }))
    }
}

pub struct ErrorMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ErrorMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let request_id = Uuid::new_v4().to_string();
        req.extensions_mut().insert(request_id.clone());
        
        let fut = self.service.call(req);
        
        Box::pin(async move {
            match fut.await {
                Ok(res) => Ok(res),
                Err(err) => {
                    // Log error here if needed
                    Err(err)
                }
            }
        })
    }
}
FIXED

mv src/api/middleware/error_middleware.rs src/api/middleware/error_middleware.backup
mv src/api/middleware/error_middleware_fixed.rs src/api/middleware/error_middleware.rs

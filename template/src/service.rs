use crate::proto::{{project-name | snake_case}}_server::{{project-name | upper_camel_case}};
use crate::proto::{HelloReply, HelloRequest};
use async_trait::async_trait;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct {{project-name | upper_camel_case}}Service {}

#[async_trait]
impl {{project-name | upper_camel_case}} for {{project-name | upper_camel_case}}Service {
    #[tracing::instrument(skip(self))]
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let request = request.get_ref();

        Ok(Response::new(HelloReply {
            message: format!("Hello {}", request.name),
        }))
    }
}

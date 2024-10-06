use pb::{
    common::{GreetRequest, GreetResponse},
    service::foo::foo_rpc_server::{FooRpc, FooRpcServer},
};
use tonic::{Request, Response, Status};

pub(crate) mod pb {
    pub(crate) mod common {
        tonic::include_proto!("common");
    }

    pub(crate) mod service {
        pub(crate) mod foo {
            pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
                tonic::include_file_descriptor_set!("service.foo.descriptor");

            tonic::include_proto!("service.foo");
        }
    }
}

struct FooRpcImpl;

#[tonic::async_trait]
impl FooRpc for FooRpcImpl {
    async fn greet(
        &self,
        request: Request<GreetRequest>,
    ) -> Result<Response<GreetResponse>, Status> {
        let payload = request.get_ref();

        if payload.name.is_empty() {
            return Err(Status::invalid_argument(
                "The \"name\" field in the GreetRequest is empty. Please provide a valid name.",
            ));
        }

        Ok(Response::new(GreetResponse {
            message: format!("[service.foo] hello, {}!", payload.name),
        }))
    }
}

impl FooRpcImpl {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(pb::service::foo::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    tonic::transport::Server::builder()
        .add_service(reflection_service)
        .add_service(FooRpcServer::new(FooRpcImpl::new()))
        .serve("0.0.0.0:50051".parse().unwrap())
        .await?;
    Ok(())
}

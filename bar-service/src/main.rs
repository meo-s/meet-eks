use pb::{
    common::{GreetRequest, GreetResponse},
    service::bar::bar_rpc_server::{BarRpc, BarRpcServer},
};
use tonic::{Request, Response, Status};

pub(crate) mod pb {
    pub(crate) mod common {
        tonic::include_proto!("common");
    }

    pub(crate) mod service {
        pub(crate) mod bar {
            pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
                tonic::include_file_descriptor_set!("service.bar.descriptor");

            tonic::include_proto!("service.bar");
        }
    }
}

struct BarRpcImpl;

#[tonic::async_trait]
impl BarRpc for BarRpcImpl {
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
            message: format!("[service.bar] hello, {}!", payload.name),
        }))
    }
}

impl BarRpcImpl {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(pb::service::bar::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    tonic::transport::Server::builder()
        .add_service(reflection_service)
        .add_service(BarRpcServer::new(BarRpcImpl::new()))
        .serve("0.0.0.0:50051".parse().unwrap())
        .await?;
    Ok(())
}

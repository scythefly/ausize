use protos::lan_proto::{self as lp, AllocateRequest, AllocateResponse};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct LanService {}

// service进程的LanServer,只需要将请求代理到session进程
#[tonic::async_trait]
impl lp::lan_server::Lan for LanService {
    async fn allocate_conn(
        &self,
        _req: Request<AllocateRequest>,
    ) -> anyhow::Result<Response<AllocateResponse>, Status> {
        Err(Status::unimplemented("not implemented"))
    }
}

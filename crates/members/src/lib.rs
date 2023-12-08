#[allow(unused)]
use prost;

pub mod membership;
pub mod server;

pub mod protos {
    pub mod v1 {
        pub mod membership {
            tonic::include_proto!("chatter.v1.membership");
        }
    }
}

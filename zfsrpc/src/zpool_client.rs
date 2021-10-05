use super::zfsrpc_proto::tonic_zpoolrpc as proto;
use super::zfsrpc_proto::tonic_zpoolrpc::zpool_rpc_client::ZpoolRpcClient;
// ::{CreateRequest, DestroyRequest, Method, Property};

use tonic::transport::Channel;

#[allow(unused)]
use tracing::{debug, error, info, trace, warn};

#[derive(Debug)]
pub enum Property {
    Mountpoint(String),
    Ashift(u32),
}

impl From<Property> for proto::Property {
    fn from(p: Property) -> Self {
        match p {
            Property::Mountpoint(mp) => Self {
                property: Some(proto::property::Property::Mountpoint(mp)),
            },
            Property::Ashift(ashift) => Self {
                property: Some(proto::property::Property::Ashift(ashift)),
            },
        }
    }
}

#[derive(Debug)]
pub enum Method {
    Raidz,
    Mirror,
}

impl From<Method> for proto::Method {
    fn from(m: Method) -> Self {
        match m {
            Method::Raidz => Self {
                method: Some(proto::method::Method::Raidz(proto::Variant {})),
            },
            Method::Mirror => Self {
                method: Some(proto::method::Method::Mirror(proto::Variant {})),
            },
        }
    }
}

impl std::str::FromStr for Method {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, anyhow::Error> {
        let method = match s.to_lowercase().as_str() {
            "raidz" => Ok(Self::Raidz),
            "mirror" => Ok(Self::Mirror),
            _ => Err(anyhow::anyhow!("zpool method is missing")),
        };
        method
    }
}

#[derive(Debug)]
pub struct Client {
    client: ZpoolRpcClient<Channel>,
}

impl Client {
    pub async fn new(port: String) -> Self {
        let client = ZpoolRpcClient::connect(format!("http://0.0.0.0:{}", port))
            .await
            .unwrap();
        Self { client }
    }

    pub async fn create(
        &mut self,
        name: &str,
        method: Method,
        disks: Vec<String>,
        properties: Vec<Property>,
    ) -> anyhow::Result<String> {
        let name = name.to_string();
        let method = Some(method.into());
        let properties = properties.into_iter().map(From::from).collect();

        let request = proto::CreateRequest {
            name,
            method,
            disks,
            properties,
        };
        let request = tonic::Request::new(request);
        let resp = self.client.create(request).await?;
        let resp = resp.into_inner();

        let resp = format!("{:?}", resp);
        Ok(resp)
    }

    pub async fn destroy(&mut self, name: &str) -> anyhow::Result<String> {
        let name = name.to_string();

        let request = proto::DestroyRequest { name };
        let request = tonic::Request::new(request);

        let resp = self.client.destroy(request).await?;
        let resp = resp.into_inner();

        let resp = format!("{:?}", resp);
        Ok(resp)
    }
}

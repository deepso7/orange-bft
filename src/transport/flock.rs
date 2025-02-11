use std::str::FromStr;

use iroh::{protocol::Router, Endpoint, NodeAddr};
use iroh_gossip::{
    net::{Gossip, GossipReceiver, GossipSender, GOSSIP_ALPN},
    proto::TopicId,
};
use serde::{Deserialize, Serialize};

use crate::{crypto::keypair::Keypair, error::Result};

use super::ticket::Ticket;

pub struct Flock {
    ticket: Ticket,

    router: Router,

    sender: GossipSender,
    receiver: GossipReceiver,
}

impl Flock {
    // init func
    pub async fn init(keys: Keypair, ticket: String) -> Result<Self> {
        let endpoint = Endpoint::builder()
            .secret_key(keys.secretKey())
            .discovery_n0()
            .bind()
            .await?;

        let gossip = Gossip::builder().spawn(endpoint.clone()).await?;

        let router = Router::builder(endpoint.clone())
            .accept(GOSSIP_ALPN, gossip.clone())
            .spawn()
            .await?;

        let t = Ticket::from_str(&ticket)?;

        let peer_ids = t.peers.iter().map(|p| p.node_id).collect();

        let (sender, mut receiver) = gossip.subscribe_and_join(t.topic, peer_ids).await?.split();

        Ok(Self {
            ticket: t,
            router,
            sender,
            receiver,
        })
    }

    pub async fn shutdown(&self) -> Result<()> {
        self.router.shutdown().await?;
        Ok(())
    }

    pub async fn send(&self, data: BroadcastData) -> Result<()> {
        let d = serde_json::to_string(&data)?;

        self.sender
            .broadcast(postcard::to_stdvec(&d)?.into())
            .await?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct BroadcastData {}

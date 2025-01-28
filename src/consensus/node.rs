use crate::{crypto::keypair::Keypair, error::Result, transport::flock::Flock};

pub struct Node {
    keypair: Keypair,
    flock: Flock,
}

impl Node {
    pub async fn init(
        ticket: String,
        keypair: Option<Keypair>,
        flock: Option<Flock>,
    ) -> Result<Self> {
        let kp = keypair.unwrap_or_else(|| Keypair::new());

        let f = match flock {
            Some(val) => val,
            None => Flock::init(kp.clone(), ticket).await?,
        };

        Ok(Self {
            keypair: kp,
            flock: f,
        })
    }
}

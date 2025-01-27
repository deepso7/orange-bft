use crate::crypto::keypair::Keypair;

pub struct Node {
    keypair: Keypair,
}

impl Node {
    // TODO: add keypair logic

    pub fn init() -> Self {
        let keypair = Keypair::new();

        Self { keypair }
    }
}

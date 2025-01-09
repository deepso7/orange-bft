pub struct QuorumCertificate {
    block_id: String,           // Unique identifier for the block
    signatures: Vec<Signature>, // Signatures from a majority of validators
}

pub struct Block {
    id: String,
    parent_id: String,
    payload: Vec<u8>, // Transactions or data
    qc: Option<QuorumCertificate>,
}

pub enum Message {
    Propose {
        block: Block,
    },
    Vote {
        block_id: String,
        signature: Signature,
    },
    NewView {
        leader_id: String,
        qc: QuorumCertificate,
    },
}

pub struct Signature;

struct ConsensusState {
    current_round: u64,
    locked_qc: Option<QuorumCertificate>, // Highest QC for safety
    ledger: Vec<Block>,                   // Committed blocks
}

mod node;

use crate::{
    error::Result,
    types::{Block, QuorumCertificate},
};

fn select_leader(round: u64, validators: Vec<String>) -> String {
    let index = round as usize % validators.len();
    validators[index].clone()
}

fn prepare(block: Block) -> Result<QuorumCertificate> {
    // Broadcast proposal to validators
    // Collect votes and form a QC
    todo!()
}

fn pre_commit(qc: QuorumCertificate) -> Result<()> {
    // Broadcast QC and move to commit phase
    todo!()
}

fn commit(qc: QuorumCertificate) -> Result<()> {
    // Finalize the block and persist it
    todo!()
}

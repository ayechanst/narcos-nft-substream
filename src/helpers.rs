use crate::{
    pb::schema::{Approvals, Transfers, Mints},
    ADDRESS,
};
use substreams::Hex;
use substreams_entity_change::tables::Tables;

pub fn transfers_to_table_changes(tables: &mut Tables, transfers: &Transfers) {
    for transfer in transfers.transfers.iter() {
        // handle the transfer
        let key = format!("{}-{}", transfer.tx_hash, transfer.token_id);
        let row = tables.update_row("Transfer", key);
        row.set("from", &transfer.from);
        row.set("to", &transfer.to);
        row.set("tokenId", &transfer.token_id);

        // handle the accounts
        tables.create_row("Account", &transfer.from);
        tables.create_row("Account", &transfer.to);

        // handle updating the token owner
        tables
            .update_row("Token", format!("{}", &transfer.token_id))
            .set("collection", ADDRESS.to_string())
            .set("owner", &transfer.to);
    }
}

pub fn mints_to_table_changes(tables: &mut Tables, mints: &Mints) {
    for mint in mints.mints.iter() {
        //handle the mint
        let key = format!("{}-{}", &mint.token_id, mint.token_id);
        let row = tables.update_row("Mint", key);
        row.set("from", &mint.from);
        row.set("to", &mint.to);
        row.set("tokenId", &mint.token_id);
        // handle the accounts
        tables.create_row("Account", &mint.from);
        tables.create_row("Account", &mint.to);

        // handle updating the token owner
        tables
            .update_row("Token", format!("{}", &mint.token_id))
            .set("collection", ADDRESS.to_string())
            .set("owner", &mint.to);
    }
}

pub fn approvals_to_table_changes(tables: &mut Tables, approvals: &Approvals) {
    for approval in approvals.approvals.iter() {
        // handle the approval
        let key = format!("{}-{}", &approval.tx_hash, approval.token_id);
        let row = tables.update_row("Approval", key);
        row.set("owner", &approval.owner);
        row.set("approved", &approval.approved);
        row.set("tokenId", &approval.token_id);

        // handle creation of accounts
        tables.create_row("Account", &approval.owner);
        tables.create_row("Account", &approval.approved);
    }
}

pub fn format_hex(address: &[u8]) -> String {
    format!("0x{}", Hex(address).to_string())
}

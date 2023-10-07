use anchor_lang::prelude::*;

declare_id!("4bXLmkHeyEuJYWLanTNCjd9xgkzK1sy2ud4TkUAXFEbk");

mod account;
mod constant;
mod context;
mod error;

use crate::context::*;

#[program]
pub mod liberte_program {
    use super::*;

    //Initialize creates the Settings PDA to track Node Counter
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.settings.node_count = 0;
        Ok(())
    }

    //Create Node PDA
    pub fn register_node(ctx: Context<RegisterNode>, ip_addr: String) -> Result<()> {
        ctx.accounts.node.ip_addr = ip_addr;
        ctx.accounts.node.active = true;
        ctx.accounts.node.authority = ctx.accounts.node_signer.key();
        Ok(())
    }
    // TODO: Update Node PDA to switch it's Active status or IP address/Key

    // Checks ownership of DistributionNFT & Request PDA with Session and Verifier Nodes
    // Request PDA also holds tokens that can be used to debit from by the Node
    pub fn request_node(ctx: Context<RequestNode>) -> Result<()> {
        Ok(())
    }
}

/*
1. Allows nodes to register/deregister themselves by creating PDAs that point to their IP addresses (also allows updating their IP addresses)
3. Node Request Ix that checks ownership of a distribution NFT and then picks a node to assign
2. Escrow features for holding tokens (can just be SOL tokens instead of $LGN tokens) and debiting them by servers
4. Loot Deposit from Node to be minted by Player
5. Module Mint Authority Registration
*/

use anchor_lang::prelude::*;
use anchor_spl::token::{
    self, InitializeAccount, Mint, SetAuthority, Token, TokenAccount, Transfer,
};
use crate::errors::LibreteError::InvalidAuthority;
declare_id!("4bXLmkHeyEuJYWLanTNCjd9xgkzK1sy2ud4TkUAXFEbk");

mod account;
mod constant;
mod context;
mod errors;
mod event;

use crate::context::*;
use crate::errors::*;
use crate::event::*;

#[program]
pub mod liberte_program {
    use super::*;

    //Initialize creates the Settings PDA to track Node Counter
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let settings = &mut ctx.accounts.settings;
        settings.all_node = 0;
        settings.active_node = 0;
        settings.authority = ctx.accounts.authority.key();
        settings.whitelist = [0; 32];
        settings.blacklist = [0; 32];
        settings.reserved = [0; 32];
        settings.bump = *ctx.bumps.get("settings").unwrap();
        Ok(())
    }

    //Create Node PDA
    pub fn register_node(ctx: Context<RegisterNode>, ip_addr: String,listen_port :u16,signature :String) -> Result<()> {
        let node = &mut ctx.accounts.node;
        let now = ctx.accounts.clock.unix_timestamp as u64;
        node.listen_ip = ip_addr.clone();
        node.listen_port = listen_port.clone();
        node.active = true;
        node.authority = ctx.accounts.authority.key();
        node.init_stamp = now;
        emit!(NewNodeEvent{
            signer:ctx.accounts.authority.key(),
            ip:ip_addr,
            port:listen_port
        });
        Ok(())

    }

    pub fn close_node(ctx: Context<CloseNode>, signature :String) -> Result<()> {
        let node = &mut ctx.accounts.node;
        let setting = &mut ctx.accounts.settings;
        let now = ctx.accounts.clock.unix_timestamp as u64;
        require_keys_eq!(node.authority, ctx.accounts.authority.key());
        setting.active_node = setting.active_node.checked_sub(1).unwrap();
        emit!(CloseNodeEvent{
            signer:ctx.accounts.authority.key(),
            ip:node.listen_ip.clone(),
            port:node.listen_port.clone(),
            timestamp:now
        });
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

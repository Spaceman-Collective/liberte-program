use anchor_lang::prelude::*;

use crate::account::*;
use crate::constant::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
      init,
      seeds=[PREFIX_SETTINGS],
      bump,
      payer=payer,
      space=8+Settings::get_max_size()
    )]
    pub settings: Account<'info, Settings>,
}

#[derive(Accounts)]
pub struct RegisterNode<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    pub settings: Account<'info, Settings>,
    pub node_signer: Signer<'info>,
    #[account(
      init,
      seeds=[PREFIX_NODE, (settings.node_count+1).to_be_bytes().as_ref()],
      bump,
      payer=payer,
      space=8+Node::get_max_size()
    )]
    pub node: Account<'info, Node>,
}

#[derive(Accounts)]
pub struct RequestNode {}

use anchor_lang::prelude::*;

declare_id!("4bXLmkHeyEuJYWLanTNCjd9xgkzK1sy2ud4TkUAXFEbk");

#[program]
pub mod liberte_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

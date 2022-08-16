use anchor_lang::prelude::*;

declare_id!("FAU7uzegHAhsV8NYZ1BXzmCaAW8vVTKnNfknNgQcFB9u");

#[program]
pub mod pindown {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
    // Get a reference to the account.
    let base_account = &mut ctx.accounts.base_account;
    
    base_account.total_docs = 0;
    Ok(())
  }
    
    pub fn add_doc(ctx: Context<AddDoc>, doc_link: String, dest_addr: String ) -> Result <()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;
    
        // Build the struct.
        let item = ItemStruct {
          doc_link: doc_link.to_string(),
          dest_addr: dest_addr.to_string(),
          user_address: *user.to_account_info().key,
        };
            
        // Add it to the doc_list vector.
        base_account.doc_list.push(item);
        base_account.total_docs += 1;
        Ok(())
      }
}

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}
// Specify what data you want in the AddDoc Context.

#[derive(Accounts)]
pub struct AddDoc<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}
// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub doc_link: String,
    pub dest_addr: String,
    pub user_address: Pubkey,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_docs: u64,
    // Attach a Vector of type ItemStruct to the account.
    pub doc_list: Vec<ItemStruct>,
}
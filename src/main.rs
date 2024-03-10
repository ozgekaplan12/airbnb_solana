use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,
    program_pack::{Pack, IsInitialized},
    sysvar::{rent::Rent, Sysvar},
};


entrypoint!(process_instruction);

#[derive(Debug)]
struct House {
    is_initialized: bool,
    owner: Pubkey,
    price: u64,
    reserved: bool,
}

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    
    let instruction_size = instruction_data.len();
    if instruction_size == 0 {
        return Err(ProgramError::InvalidInstructionData);
    }

 
    let instruction_type = instruction_data[0];
    match instruction_type {
       
        0 => {
            msg!("Listing a house");
       
            let accounts_iter = &mut accounts.iter();
            let owner_account = next_account_info(accounts_iter)?;
            let house_account = next_account_info(accounts_iter)?;

          
            if !owner_account.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }

           
            let mut house_data = House {
                is_initialized: true,
                owner: *owner_account.key,
                price: 0,
                reserved: false,
            };
            house_data.pack_into_slice(&mut house_account.data.borrow_mut())?;

            Ok(())
        }
        
        1 => {
            msg!("Reserving a house");
          
            let accounts_iter = &mut accounts.iter();
            let guest_account = next_account_info(accounts_iter)?;
            let house_account = next_account_info(accounts_iter)?;
            let rent_sysvar_account = next_account_info(accounts_iter)?;
            
       
            let mut house_data = House::unpack(&house_account.data.borrow())?;
            if house_data.reserved {
                return Err(ProgramError::AccountAlreadyInitialized);
            }
            
      
            let rent = Rent::from_account_info(rent_sysvar_account)?;
            let rent_amount = rent.minimum_balance(house_account.data_len())?;

          
            if guest_account.lamports() >= rent_amount {
                guest_account.try_borrow_mut_lamports(rent_amount)?;
                house_data.reserved = true;
                house_data.pack_into_slice(&mut house_account.data.borrow_mut())?;
                Ok(())
            } else {
                Err(ProgramError::InsufficientFunds)
            }
        }
        _ => {
          
            Err(ProgramError::InvalidInstructionData)
        }
    }
}

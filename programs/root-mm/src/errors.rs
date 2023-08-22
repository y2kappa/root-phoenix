use anchor_lang::error_code;

#[error_code]
pub enum RootError {

    #[msg("Refresh triggered before the MIN_REFRESH_SLOT is reached")]
    RefreshedTooSoon,
    
}
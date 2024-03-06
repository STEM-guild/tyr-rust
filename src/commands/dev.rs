// these really should be exclusive to dev team roles but we cross that bridge when it comes across us

use crate::utils::base::{ Context, Error };

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn ping(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
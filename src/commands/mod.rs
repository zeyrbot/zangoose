use crate::{ZeyrContext, ZeyrError};

pub mod help;
pub mod ping;

#[poise::command(prefix_command, slash_command, ephemeral = true, rename = "register")]
pub async fn register(ctx: ZeyrContext<'_>) -> Result<(), ZeyrError> {
	print!("Registering commands... ");
	poise::builtins::register_application_commands_buttons(ctx).await?;
	
	Ok(())
}

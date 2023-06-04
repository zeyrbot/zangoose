use crate::{ZeyrContext, ZeyrError};

#[poise::command(slash_command, rename = "help")]
pub async fn help(
	ctx: ZeyrContext<'_>,
	#[description = "Specific command to show help about"]
	#[autocomplete = "poise::builtins::autocomplete_command"]
	command: Option<String>,
) -> Result<(), ZeyrError> {
	poise::builtins::help(
		ctx,
		command.as_deref(),
		poise::builtins::HelpConfiguration {
			extra_text_at_bottom: "\
			This is an example bot made to showcase features of my custom Discord bot framework",
			show_context_menu_commands: true,
			..Default::default()
		},
	)
	.await?;
	Ok(())
}

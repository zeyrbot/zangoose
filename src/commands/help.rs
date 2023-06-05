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
			Zeyr rust. version 0.1.0",
			show_context_menu_commands: false,
			..Default::default()
		},
	)
	.await?;
	Ok(())
}

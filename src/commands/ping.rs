use std::time::SystemTime;

use crate::{ZeyrContext, ZeyrError};

#[poise::command(prefix_command, slash_command, rename = "ping")]
pub async fn run(ctx: ZeyrContext<'_>) -> Result<(), ZeyrError> {
	let initial = SystemTime::now();

	let reply = ctx.send(|f| f.content("awaiting...")).await?;

	let elapsed = initial.elapsed().unwrap();

	reply
		.edit(ctx, |m| {
			m.content(format!("roundtrip: {}ms", elapsed.as_millis()))
		})
		.await?;

	Ok(())
}

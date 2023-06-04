#![warn(clippy::str_to_string)]

mod commands;
mod events;

use commands::*;
use events::Handler;
use poise::serenity_prelude::{self as serenity, RwLock};
use std::{
	collections::{HashMap, HashSet},
	env::var,
	sync::Arc,
};

type ZeyrError = Box<dyn std::error::Error + Send + Sync>;
type ZeyrContext<'a> = poise::Context<'a, Arc<RwLock<ZeyrData>>, ZeyrError>;

impl serenity::TypeMapKey for ZeyrData {
	type Value = Arc<RwLock<ZeyrData>>;
}

pub struct ZeyrData {
	pub queue: HashMap<String, String>,
}

#[tokio::main]
async fn main() {
	dotenv::dotenv().ok();

	tracing_subscriber::fmt::init();

	let mut owners = HashSet::new();
	owners.insert(serenity::UserId(1076700780175831100));

	let mut commands = vec![
		register(),
		help::help(),
		ping::run(),
		tag::tag::run()
	];
	poise::set_qualified_names(&mut commands);

	let data = Arc::new(RwLock::new(ZeyrData {
		queue: HashMap::new(),
	}));

	let handler = Arc::new(Handler::new(
		poise::FrameworkOptions {
			owners,
			commands,
			prefix_options: poise::PrefixFrameworkOptions {
				prefix: Some("+".into()),
				edit_tracker: Some(poise::EditTracker::for_timespan(
					std::time::Duration::from_secs(3600),
				)),
				case_insensitive_commands: true,
				..Default::default()
			},
			on_error: |error| {
				Box::pin(async {
					poise::samples::on_error(error)
						.await
						.unwrap_or_else(|error| tracing::error!("{}", error))
				})
			},
			event_handler: |_ctx, event, _framework, _data| {
				Box::pin(async move {
					tracing::trace!("{:?}", event.name());
					Ok(())
				})
			},
			skip_checks_for_owners: true,
			..Default::default()
		},
		data.clone(),
	));

	let mut client = serenity::Client::builder(
		var("DISCORD_TOKEN")
			.expect("Missing `DISCORD_TOKEN` env var, see README for more information."),
		serenity::GatewayIntents::non_privileged()
			| serenity::GatewayIntents::MESSAGE_CONTENT
			| serenity::GatewayIntents::GUILD_MESSAGES
			| serenity::GatewayIntents::GUILD_INTEGRATIONS,
	)
	.event_handler_arc(handler.clone())
	.await
	.unwrap();

	client.data.write().await.insert::<ZeyrData>(data);

	handler
		.set_shard_manager(client.shard_manager.clone())
		.await;

	client.start().await.unwrap();
}

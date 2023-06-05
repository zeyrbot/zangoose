use piston_rs::{Client, Executor, File};
use poise::prefix_argument::CodeBlock;
use std::time::SystemTime;

use crate::{ZeyrContext, ZeyrError};

#[poise::command(
	prefix_command,
	slash_command,
	rename = "piston",
	aliases("run", "code")
)]
pub async fn run(
	ctx: ZeyrContext<'_>,
	#[description = "Language of the code to run"] language: String,
	#[description = "Code to run"]
	#[rest]
	code: String,
) -> Result<(), ZeyrError> {
	let initial = SystemTime::now();

	let piston = Client::new();
	let engine = Executor::new()
		.set_language(&language)
		.set_version("*")
		.add_file(File::default().set_name("main.rs").set_content(&code));

	let mut final_content = String::new();

	match piston.execute(&engine).await {
		Ok(response) => {
			println!("Language: {}", response.language);
			println!("Version: {}", response.version);

			if !response.run.stderr.is_empty() {
				final_content.push_str(&format!("error: {}", response.run.stderr));
			} else {
				final_content.push_str(&format!(
					"output: {}",
					CodeBlock {
						code: response.run.output,
						language: Some(response.language.to_lowercase())
					}
				));
			}

			if let Some(c) = response.compile {
				final_content.push_str(&format!("\ncompilation: {}", c.output));
			}
		}
		Err(e) => {
			println!("Something went wrong contacting Piston.");
			println!("{}", e);

			return Err(ZeyrError::from("error"));
		}
	}

	let elapsed = initial.elapsed().unwrap();

	final_content.push_str(&format!("\nelapsed: {}ms", elapsed.as_millis()));

	ctx.send(|f| f.content(final_content)).await?;

	Ok(())
}

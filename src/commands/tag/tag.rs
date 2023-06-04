use crate::{ZeyrContext, ZeyrError};
use tagscript_rs::tagscript_rs::TemplateParser;
use std::collections::HashMap;

#[poise::command(prefix_command, slash_command, rename = "tag")]
pub async fn run(
    ctx: ZeyrContext<'_>,
    #[description = "Text to render with tagscript_rs"]
    #[rest]
	text: String,
) -> Result<(), ZeyrError> {
    let mut parser = TemplateParser::new(
        &text
    );
    parser.parse();

    let mut data = HashMap::new();
    
    data.insert("name".to_string(), ctx.author().name.to_string());

    let result = parser.render(&data);

	ctx.send(|f| f.content(result)).await?;

	Ok(())
}

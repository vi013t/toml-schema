# `toml-schema`

A Rust library for generating TOML schemas that are statically typed and generate support for try-parsing, `Default::default()`, pretty-printing, and more.

## Example

```rust
toml! {
	#[name = Config]

	name = "example";

	[options]
	quiet = false;
	project_name: "project",
	output = { name = "build", format = "JSON" };
}

fn main() -> anyhow::Result<()> {
	let config = Config::default();
	assert_eq!(false, config.quiet());
	assert_eq!("JSON", config.output().format());

	let user_config: Config = include_str!("user_config.toml").parse()?;
}
```
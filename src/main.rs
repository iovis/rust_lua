use anyhow::Result;
use mlua::Lua;

fn main() -> Result<()> {
    let code = std::fs::read_to_string("test.lua").unwrap();

    let lua = Lua::new();
    let globals = lua.globals();

    globals.set("message_from_rust", "HEDDO? (in Rust)")?;

    lua.load(code).exec()?;

    Ok(())
}

use anyhow::Result;
use mlua::Lua;

fn main() -> Result<()> {
    let code = std::fs::read_to_string("test.lua").unwrap();

    let lua = Lua::new();
    lua.load(code).exec()?;

    Ok(())
}

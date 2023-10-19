use anyhow::Result;
use mlua::Lua;

fn main() -> Result<()> {
    let code = std::fs::read_to_string("test.lua").unwrap();

    let lua = Lua::new();
    let globals = lua.globals();
    let non_magical_number = 42;

    globals.set("message_from_rust", "HEDDO? (in Rust)")?;
    globals.set("non_magical_number", non_magical_number)?;

    lua.load(code).exec()?;

    let non_magical_number: i32 = globals.get("non_magical_number")?;

    eprintln!("non_magical_number = {non_magical_number:?}");

    Ok(())
}

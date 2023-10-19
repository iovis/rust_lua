use anyhow::Result;
use mlua::Lua;

fn main() -> Result<()> {
    let code = std::fs::read_to_string("test.lua").unwrap();

    let lua = Lua::new();
    let globals = lua.globals();
    let non_magical_number = 42;

    globals.set("message_from_rust", "HEDDO? (in Rust)")?;
    globals.set("non_magical_number", non_magical_number)?;
    globals.set(
        "rs_add",
        lua.create_function(|_, (a, b): (i32, i32)| Ok(a + b))?,
    )?;
    globals.set(
        "range",
        lua.create_function(|_, (start, end): (i32, i32)| Ok((start..end).collect::<Vec<_>>()))?,
    )?;
    globals.set(
        "create_table",
        lua.create_function(|lua, ()| {
            let table = lua.create_table()?;
            table.set("string", "This is a string")?;
            table.set("number", 123)?;

            Ok(table)
        })?,
    )?;

    lua.load(code).exec()?;

    let non_magical_number: i32 = globals.get("non_magical_number")?;

    eprintln!("[Rust] non_magical_number = {non_magical_number:?}");

    Ok(())
}

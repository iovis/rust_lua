use std::collections::HashMap;

use anyhow::Result;
use mlua::Lua;

fn main() -> Result<()> {
    let code = std::fs::read_to_string("test.lua").unwrap();

    let lua = Lua::new();
    let globals = lua.globals();
    let non_magical_number = 42;

    // Example string
    globals.set("message_from_rust", "HEDDO? (in Rust)")?;

    // Example number
    globals.set("non_magical_number", non_magical_number)?;

    // Example function
    globals.set(
        "rs_add",
        lua.create_function(|_, (a, b): (i32, i32)| Ok(a + b))?,
    )?;

    // Example function serializing a collection
    globals.set(
        "range",
        lua.create_function(|_, (start, end): (i32, i32)| Ok((start..end).collect::<Vec<_>>()))?,
    )?;

    // Example table
    globals.set(
        "create_table",
        lua.create_function(|lua, ()| {
            let table = lua.create_table()?;
            table.set("string", "This is a string")?;
            table.set("number", 123)?;

            Ok(table)
        })?,
    )?;

    // Example simple object
    globals.set(
        "Point",
        lua.create_function(|lua, (x, y): (f32, f32)| {
            let point = lua.create_table()?;

            // Attributes
            point.set("x", x)?;
            point.set("y", y)?;

            // Methods
            point.set(
                "distance",
                lua.create_function(|_, (myself,): (mlua::Table,)| {
                    let x: f32 = myself.get("x")?;
                    let y: f32 = myself.get("y")?;

                    Ok((x.powi(2) + y.powi(2)).sqrt())
                })?,
            )?;

            Ok(point)
        })?,
    )?;

    lua.load(code).exec()?;

    let non_magical_number: i32 = globals.get("non_magical_number")?;
    println!("\n[Rust]\tnon_magical_number = {non_magical_number:?}");

    let a_table: mlua::Table = globals.get("a_table")?;
    println!("[Rust]\ta_table = {a_table:#?}");

    let lua_list: Vec<i32> = a_table.get("lua_list")?;
    println!("[Rust]\ta_table.lua_list = {lua_list:?}");

    let lua_tbl: HashMap<String, i32> = a_table.get("lua_tbl")?;
    println!("[Rust]\ta_table.lua_tbl = {lua_tbl:?}");

    Ok(())
}

use mlua::prelude::*;

fn is_empty(_: &Lua, string: String) -> LuaResult<bool> {
    Ok(string.is_empty())
}

#[mlua::lua_module]
fn string_utils(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("is_empty", lua.create_function(is_empty)?)?;

    Ok(exports)
}

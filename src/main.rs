use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;
use mlua::{ExternalResult, FromLua, Lua, LuaSerdeExt};
use serde::{Deserialize, Serialize};

// https://reqres.in/api/users
//
// {
//     "avatar": "https://reqres.in/img/faces/1-image.jpg",
//     "email": "george.bluth@reqres.in",
//     "first_name": "George",
//     "id": 1,
//     "last_name": "Bluth"
// },
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: usize,
    avatar: String,
    email: String,
    first_name: String,
    last_name: String,
}

// TODO: Why do I need to manually `impl FromLua`?
impl<'lua> FromLua<'lua> for User {
    fn from_lua(value: mlua::Value<'lua>, _: &'lua Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::Table(user) => Ok(User {
                id: user.get("id")?,
                avatar: user.get("avatar")?,
                email: user.get("email")?,
                first_name: user.get("first_name")?,
                last_name: user.get("last_name")?,
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UsersResponse {
    data: Vec<User>,
}

fn main() -> Result<()> {
    let lua_path = Path::new("./lua");
    let code = std::fs::read_to_string(lua_path.join("init.lua")).unwrap();

    // let lua = Lua::new(); // Normally you'd use this if you don't need to require C modules
    let lua = unsafe { Lua::unsafe_new() };

    // Add ./lua/ to _G.package.path
    set_package_path(&lua, lua_path)?;

    // Passing information to Lua
    let globals = lua.globals();

    // Example string
    globals.set("message_from_rust", "HEDDO? (in Rust)")?;

    // Example number
    let non_magical_number = 42;
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
                lua.create_function(|_, myself: mlua::Table| {
                    let x: f32 = myself.get("x")?;
                    let y: f32 = myself.get("y")?;

                    Ok((x.powi(2) + y.powi(2)).sqrt())
                })?,
            )?;

            point.set(
                "scale",
                lua.create_function(|_, (myself, scale_factor): (mlua::Table, f32)| {
                    let (x, y): (f32, f32) = (myself.get("x")?, myself.get("y")?);

                    myself.set("x", x * scale_factor)?;
                    myself.set("y", y * scale_factor)?;

                    Ok(myself)
                })?,
            )?;

            point.set(
                "double_cloned",
                lua.create_function(|lua, myself: mlua::Table| {
                    let (x, y): (f32, f32) = (myself.get("x")?, myself.get("y")?);

                    let globals = lua.globals();
                    let point: mlua::Function = globals.get("Point")?;
                    let point: mlua::Table = point.call((x * 2.0, y * 2.0))?;

                    Ok(point)
                })?,
            )?;

            Ok(point)
        })?,
    )?;

    // More interesting function
    globals.set(
        "get_users",
        lua.create_function(|lua, ()| {
            let users = reqwest::blocking::get("https://reqres.in/api/users")
                .unwrap()
                .json::<UsersResponse>()
                .unwrap()
                .data;

            // .to_value() is necessary for Serialization
            Ok(lua.to_value(&users))
        })?,
    )?;

    // Object with methods and errors
    let http = lua.create_table()?;
    http.set(
        "get",
        lua.create_function(|lua, url: String| {
            let users: serde_json::Value = reqwest::blocking::get(url)
                .into_lua_err()?
                .json()
                .into_lua_err()?;

            Ok(lua.to_value(&users))
        })?,
    )?;

    globals.set("http", http)?;

    lua.load(code).exec()?;

    let non_magical_number: i32 = globals.get("non_magical_number")?;
    println!("\n[Rust]\tnon_magical_number = {non_magical_number:?}");

    let a_table: mlua::Table = globals.get("a_table")?;
    println!("[Rust]\ta_table = {a_table:#?}");

    let lua_list: Vec<i32> = a_table.get("lua_list")?;
    println!("[Rust]\ta_table.lua_list = {lua_list:?}");

    let lua_tbl: HashMap<String, i32> = a_table.get("lua_tbl")?;
    println!("[Rust]\ta_table.lua_tbl = {lua_tbl:?}");

    let users: Vec<User> = globals.get("users")?;
    println!("[Rust]\tusers = {users:#?}");

    Ok(())
}

/// Add path to _G.package.path
fn set_package_path(lua: &Lua, path: &Path) -> anyhow::Result<()> {
    let globals = lua.globals();
    let package: mlua::Table = globals.get("package")?;
    let package_path: String = package.get("path")?;
    let mut path_array: Vec<String> = package_path
        .split(';')
        .map(std::borrow::ToOwned::to_owned)
        .collect();

    path_array.insert(0, format!("{}/?.lua", path.display()));
    path_array.insert(0, format!("{}/?.so", path.display()));
    path_array.insert(1, format!("{}/?/init.lua", path.display()));

    package.set("path", path_array.join(";"))?;

    Ok(())
}

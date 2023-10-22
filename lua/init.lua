---@type string
message_from_rust = message_from_rust
---@type number
non_magical_number = non_magical_number

local log = require("log").log
local table_print = require("log").table_print

log("--- Primitives")
log("HEDDO! (from Lua)")
log(message_from_rust)
log("non_magical_number = " .. non_magical_number)

non_magical_number = non_magical_number * 2
log("non_magical_number = " .. non_magical_number)

log("--- Functions")
log("rs_add(3, 4) = " .. rs_add(3, 4))
log("range(10, 20) = { " .. table.concat(range(10, 20), ", ") .. " }")

log("--- Tables")
a_table = create_table()
log("a_table.string = " .. a_table.string)
log("a_table.number = " .. a_table.number)
a_table.lua_list = { 1, 2, 3 }
a_table.lua_tbl = {
  a = 1,
  b = 2,
  c = 3,
}

log("--- Objects")
local point = Point(3, 4)
log("point.x = " .. point.x)
log("point.y = " .. point.y)
log("point:distance() = " .. point:distance())

local point2 = point:double_cloned(2)
log("point2.x = " .. point2.x)
log("point2.y = " .. point2.y)
log("point.x = " .. point.x)
log("point.y = " .. point.y)

point:scale(2)
log("point.x = " .. point.x)
log("point.y = " .. point.y)

log("--- HTTP calls")
for _, user in ipairs(get_users()) do
  log("{")
  log("  id: " .. user.id)
  log("  avatar: " .. user.avatar)
  log("  email: " .. user.email)
  log("  first_name: " .. user.first_name)
  log("  last_name: " .. user.last_name)
  log("}")
end

log("--- HTTP calls (2)")
local response = http.get("https://reqres.in/api/users")

users = response.data

for _, user in ipairs(users) do
  log("{")
  log("  id: " .. user.id)
  log("  avatar: " .. user.avatar)
  log("  email: " .. user.email)
  log("  first_name: " .. user.first_name)
  log("  last_name: " .. user.last_name)
  log("}")
end

log("--- HTTP calls (3)")
local ok, result = pcall(http.get, "asdf")

if ok then
  log("Success: " .. table_print(result))
else
  log("Error: ")
  print(result)
end

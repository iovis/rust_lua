---@type string
message_from_rust = message_from_rust
---@type number
non_magical_number = non_magical_number

print("HEDDO! (from Lua)")
print(message_from_rust)
print("non_magical_number = " .. non_magical_number)

non_magical_number = non_magical_number * 2
print("non_magical_number = " .. non_magical_number)

print("rs_add(3, 4) = " .. rs_add(3, 4))

for i, v in ipairs(range(10, 20)) do
  print(i, v)
end

local a_table = create_table()
print("a_table.string = " .. a_table.string)
print("a_table.number = " .. a_table.number)

---@type string
message_from_rust = message_from_rust
---@type number
non_magical_number = non_magical_number

print("HEDDO! (from Lua)")
print(message_from_rust)
print("non_magical_number = " .. non_magical_number)

non_magical_number = non_magical_number * 2
print("non_magical_number = " .. non_magical_number)

local M = {}

function M.log(message)
  print("[Lua]", message)
end

function M.table_print(tbl, indent)
  if not indent then
    indent = 0
  end

  local formatted = ""

  for k, v in pairs(tbl) do
    local key = tostring(k)
    local value = tostring(v)

    if type(v) == "table" then
      value = M.table_print(v, indent + 1)
    end

    formatted = formatted .. string.rep("  ", indent) .. key .. ": " .. value .. "\n"
  end

  return formatted
end

return M

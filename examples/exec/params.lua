compiler_name = "rustc"

local result = exec(compiler_name, {"--version"})

version = result.stdout:sub(7) -- e.g. 1.85.0 (4d91de4e4 2025-02-17)

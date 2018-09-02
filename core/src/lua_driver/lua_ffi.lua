local ffi = require("ffi")
ffi.cdef[[
__BINDINGS__
]]

return ffi.load("__ENGINE_MODULE_NAME__")

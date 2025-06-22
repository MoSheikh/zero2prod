logfile = io.open("healthz.log", "w");

local counter = 1
local threads = {}

wrk.method = "GET"
wrk.path   = "http://localhost:8080/healthz"

function setup(thread)
   thread:set("id", counter)
   table.insert(threads, thread)
   counter = counter + 1
end

function init(args)
   requests = 1
end

function request()
   local headers = {}
   headers["X-Unique-Id"] = id .. "-" .. requests
   requests = requests + 1
   return wrk.format("GET", nil, headers)
end

function response(status, header, body)
   logfile:write("status:" .. status .. "\n")
end
logfile = io.open("subscribe.log", "w");

wrk.method = "POST"
wrk.path   = "/subscribe"
wrk.headers["Content-Type"] = "application/x-www-form-urlencoded"

local counter = 1
local threads = {}

wrk.method = "GET"

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
   name = "john+doe"
   email = id .. "jd" .. requests .. "%40email.com"
   requests = requests + 1
   return wrk.format("POST", nil, nil, "name=" .. name .. "&email=" .. email)
end

function response(status, header, body)
   logfile:write("status:" .. status .. "\n")
end
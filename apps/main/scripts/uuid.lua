function uuid_v0()
    local rnd = math.random
    local template = "00000000-0000-0000-0000-000000000000"

    return string.gsub(template, "[xy]", function(c)
        local v = (c == "x") and rnd(0, 15) or rnd(8, 11)
        return string.format("%x", v)
    end)
end

function uuid_v4()
    local rnd = math.random
    local template = "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx"

    return string.gsub(template, "[xy]", function(c)
        local v = (c == "x") and rnd(0, 15) or rnd(8, 11)
        return string.format("%x", v)
    end)
end

function uuid_v7(input)
    local ts = input.timestamp

    -- 48-bite timestamp split on high 32 and low 16 bits
    local ts_high = math.floor(ts / 0x10000)
    local ts_low  = ts % 0x10000

    -- random groups
    local rand12 = math.random(0, 0x0fff)       -- 12 bits
    local rand14 = math.random(0, 0x3fff)       -- 14 bits
    local rand48_a = math.random(0, 0xffffffff) -- 32 bits
    local rand48_b = math.random(0, 0xffff)     -- 16 bits

    return string.format(
        "%08x-%04x-7%03x-%04x-%08x%04x",
        ts_high,               -- 8 hex
        ts_low,                -- 4 hex
        rand12,                -- v7
        0x8000 + rand14,       -- variant 10xxxx...
        rand48_a,              -- 8 hex
        rand48_b               -- 4 hex
    )
end

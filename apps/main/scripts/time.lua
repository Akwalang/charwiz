local function is_leap(year)
    return (year % 4 == 0 and year % 100 ~= 0) or (year % 400 == 0)
end

local function days_in_year(year)
    return is_leap(year) and 366 or 365
end

local DAYS_IN_MONTH = {
    31, 28, 31, 30, 31, 30,
    31, 31, 30, 31, 30, 31
}

local function timestamp_to_utc(ts)
    local ms = ts % 1000
    local seconds = math.floor(ts / 1000)

    local sec = seconds % 60
    seconds = math.floor((seconds - sec) / 60)

    local min = seconds % 60
    seconds = math.floor((seconds - min) / 60)

    local hour = seconds % 24
    local days = math.floor((seconds - hour) / 24)

    local year = 1970

    while true do
        local dy = days_in_year(year)
        if days >= dy then
            days = days - dy
            year = year + 1
        else
            break
        end
    end

    local month = 1
    
    while true do
        local dim = DAYS_IN_MONTH[month]
        if month == 2 and is_leap(year) then
            dim = dim + 1
        end

        if days >= dim then
            days = days - dim
            month = month + 1
        else
            break
        end
    end

    local day = days + 1

    return year, month, day, hour, min, sec, ms
end

local function pad(n, len)
    local s = tostring(math.floor(n))
    local l = #s

    if l >= len then
        return s
    end

    return string.rep("0", len - l) .. s
end

function time_ms(input)
    return input.timestamp
end

function time_iso(input)
    local y, m, d, h, mi, s, ms = timestamp_to_utc(input.timestamp)

    return string.format(
        "%04d-%s-%sT%s:%s:%s.%sZ",
        y,
        pad(m, 2),
        pad(d, 2),
        pad(h, 2),
        pad(mi, 2),
        pad(s, 2),
        pad(ms, 3)
    )
end

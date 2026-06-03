local function random_item(items)
    return items[math.random(#items)]
end

local function random_name()
    local items = {
        "Alex", "John", "Mike", "David", "James",
        "Robert", "Daniel", "Chris", "Andrew", "Ryan",
        "Matthew", "Kevin", "Thomas", "Mark", "Jason",
        "Brian", "Eric", "Adam", "Scott", "Steven",
        "Nathan", "Justin", "Aaron", "Patrick", "Tyler",
        "Joshua", "Brandon", "Derek", "Nick", "Sam"
    }

    return random_item(items)
end

local function random_lname()
    local items = {
        "Smith", "Johnson", "Williams", "Brown", "Jones",
        "Garcia", "Miller", "Davis", "Wilson", "Moore",
        "Taylor", "Anderson", "Thomas", "Jackson", "White",
        "Harris", "Martin", "Thompson", "Martinez", "Clark",
        "Lewis", "Walker", "Hall", "Allen", "Young",
        "King", "Wright", "Scott", "Green", "Baker"
    }

    return random_item(items)
end

local function random_fname()
    return random_name() .. " " .. random_lname()
end

local function random_nick()
    local items = {
        "shadowfox", "bytehunter", "silentwolf", "darkcoder", "icehawk",
        "stormrider", "pixelghost", "redfalcon", "nightbyte", "zeroflux",
        "quantumnode", "cyberwolf", "ironpixel", "ghostroot", "silverfox",
        "blueorbit", "techmage", "voidrunner", "blackcomet", "skyforge",
        "codepilot", "darkmatter", "rapidfox", "pixelstorm", "cloudrider",
        "binarycat", "rustynode", "lunarbyte", "neonwolf", "echohawk"
    }

    return random_item(items)
end

local function random_date()
    local year = math.random(2000, 2035)
    local month = math.random(1, 12)
    local day = math.random(1, 28)

    return string.format(
        "%04d.%02d.%02d",
        year,
        month,
        day
    )
end

local function random_time()
    return string.format(
        "%02d:%02d:%02d",
        math.random(0, 23),
        math.random(0, 59),
        math.random(0, 59)
    )
end

local function random_email()
    local domains = {
        "gmail.com",
        "outlook.com",
        "hotmail.com",
        "yahoo.com",
        "proton.me",
        "icloud.com",
        "example.com",
        "mail.com",
        "test.com",
        "company.com"
    }

    local first = random_name():lower()
    local last = random_lname():lower()

    return string.format(
        "%s.%s@%s",
        first,
        last,
        random_item(domains)
    )
end

local ROUTES = {
    name = random_name,
    lname = random_lname,
    fname = random_fname,
    nick = random_nick,
    date = random_date,
    time = random_time,
    email = random_email,
}

function fake(input)
    local key = input.value

    if not key then
        return "plugin_not_found"
    end

    local route = ROUTES[key]

    if not route then
        return "plugin_not_found"
    end

    return route()
end

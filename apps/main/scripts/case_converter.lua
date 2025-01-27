local function trim(s)
   return s:match("^%s*(.-)%s*$")
end

local function split_into_words(input)
    local str = input

    str = str:gsub("[-_ ]+", " ")
    str = str:gsub("([A-Z])", " %1")
    str = trim(str)

    local words = {}

    for word in str:gmatch("%S+") do
        table.insert(words, word)
    end

    return words
end

function to_snake_case(input)
    local words = split_into_words(input.value)

    return table.concat(words, "_"):lower()
end

function to_pascal_case(input)
    local words = split_into_words(input.value)

    for i, word in ipairs(words) do
        words[i] = word:sub(1, 1):upper() .. word:sub(2):lower()
    end

    return table.concat(words)
end

function to_camel_case(input)
    local words = split_into_words(input.value)

    for i, word in ipairs(words) do
        if i > 1 then
            words[i] = word:sub(1, 1):upper() .. word:sub(2):lower()
        else
            words[i] = word:lower()
        end
    end

    return table.concat(words)
end

function to_kebab_case(input)
    local words = split_into_words(input.value)

    return table.concat(words, "-"):lower()
end

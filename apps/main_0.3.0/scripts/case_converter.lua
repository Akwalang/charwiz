local function trim(s)
    return s:match("^%s*(.-)%s*$")
end

local function split_into_words(input)
    local str = input

    str = str:gsub("[-_ ]+", " ")
    str = str:gsub("(%l)(%u)", "%1 %2")
    str = str:gsub("(%u+)(%u%l)", "%1 %2")

    str = trim(str)

    local words = {}

    for word in str:gmatch("%S+") do
        table.insert(words, word)
    end

    return words
end

function to_text_case(input)
    local result = {}

    for line in input.value:gmatch("([^\n]*)\n?") do
        if line == "" then
            table.insert(result, "")
        else
            local words = split_into_words(line)
            table.insert(result, table.concat(words, " "):lower())
        end
    end

    return table.concat(result, "\n")
end

function to_snake_case(input)
    local result = {}

    for line in input.value:gmatch("([^\n]*)\n?") do
        if line == "" then
            table.insert(result, "")
        else
            local words = split_into_words(line)
            table.insert(result, table.concat(words, "_"):lower())
        end
    end

    return table.concat(result, "\n")
end

function to_pascal_case(input)
    local result = {}

    for line in input.value:gmatch("([^\n]*)\n?") do
        if line == "" then
            table.insert(result, "")
        else
            local words = split_into_words(line)

            for i, word in ipairs(words) do
                words[i] = word:sub(1, 1):upper() .. word:sub(2):lower()
            end

            table.insert(result, table.concat(words, ""))
        end
    end

    return table.concat(result, "\n")
end

function to_camel_case(input)
    local result = {}

    for line in input.value:gmatch("([^\n]*)\n?") do
        if line == "" then
            table.insert(result, "")
        else
            local words = split_into_words(line)

            for i, word in ipairs(words) do
                if i > 1 then
                    words[i] = word:sub(1, 1):upper() .. word:sub(2):lower()
                else
                    words[i] = word:lower()
                end
            end

            table.insert(result, table.concat(words, ""))
        end
    end

    return table.concat(result, "\n")
end

function to_kebab_case(input)
    local result = {}

    for line in input.value:gmatch("([^\n]*)\n?") do
        if line == "" then
            table.insert(result, "")
        else
            local words = split_into_words(line)
            table.insert(result, table.concat(words, "-"):lower())
        end
    end

    return table.concat(result, "\n")
end

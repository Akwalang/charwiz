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

local function capitalize(word)
    return word:sub(1, 1):upper() .. word:sub(2):lower()
end

local function format_words(words, mode)
    if mode == "title" then
        local result = {}

        for _, word in ipairs(words) do
            table.insert(result, capitalize(word))
        end

        return table.concat(result, " ")
    end

    if mode == "upper_text" then
        return table.concat(words, " "):upper()
    end

    if mode == "text" then
        return table.concat(words, " "):lower()
    end

    if mode == "upper_snake" then
        return table.concat(words, "_"):upper()
    end

    if mode == "snake" then
        return table.concat(words, "_"):lower()
    end

    if mode == "kebab" then
        return table.concat(words, "-"):lower()
    end

    if mode == "pascal" then
        local result = {}

        for _, word in ipairs(words) do
            table.insert(result, capitalize(word))
        end

        return table.concat(result, "")
    end

    if mode == "camel" then
        local result = {}

        for i, word in ipairs(words) do
            if i == 1 then
                table.insert(result, word:lower())
            else
                table.insert(result, capitalize(word))
            end
        end

        return table.concat(result, "")
    end

    return table.concat(words, " ")
end

local function detect_case(context)
    if context == nil or context == "" then
        return "text"
    end

    context = trim(context)

    if context == "" then
        return "text"
    end

    -- TitleCase
    if context:match("^%u%l+ %u+") then
        return "title"
    end

    -- UPPER_TEXT_CASE
    if context:match("^[A-Z0-9]+ [A-Z0-9 ]+$") or context:match("^[A-Z0-9]+$") then
        return "upper_text"
    end

    -- UPPER_SNAKE_CASE
    if context:match("^[A-Z0-9]+_[A-Z0-9_]+$") or context:match("^[A-Z0-9]+$") then
        return "upper_snake"
    end

    -- snake_case
    if context:match("^[a-z0-9]+_[a-z0-9_]+$") then
        return "snake"
    end

    -- kebab-case
    if context:match("^[a-z0-9]+%-[a-z0-9%-]+$") then
        return "kebab"
    end

    -- PascalCase
    if context:match("^%u%w*$") and context:match("%l") then
        return "pascal"
    end

    -- camelCase
    if context:match("^%l%w*$") and context:match("%u") then
        return "camel"
    end

    -- plain text
    if context:match("%s") then
        return "text"
    end

    return "text"
end

local function convert_lines(input, mode)
    local result = {}

    for line in input.value:gmatch("([^\n]*)\n?") do
        if line == "" then
            table.insert(result, "")
        else
            local words = split_into_words(line)
            table.insert(result, format_words(words, mode))
        end
    end

    return table.concat(result, "\n")
end

local function split_lines(str)
    local result = {}

    if str == nil then
        return result
    end

    for line in str:gmatch("([^\n]*)\n?") do
        table.insert(result, line)
    end

    if #result > 0 and result[#result] == "" then
        table.remove(result, #result)
    end

    return result
end



function smart_case(input)
    local input_lines = split_lines(input.value)
    local context_lines = split_lines(input.context or "")

    local result = {}

    if #input_lines == #context_lines and #input_lines > 1 then
        for i, context_line in ipairs(context_lines) do
            local words = split_into_words(input_lines[i])
            local mode = detect_case(context_line)

            table.insert(
                result,
                format_words(words, mode)
            )
        end

        return table.concat(result, "\n")
    end

    local words = split_into_words(input.value)

    for _, context_line in ipairs(context_lines) do
        local mode = detect_case(context_line)

        table.insert(
            result,
            format_words(words, mode)
        )
    end

    return table.concat(result, "\n")
end

function to_title_case(input)
    return convert_lines(input, "title")
end

function to_upper_text_case(input)
    return convert_lines(input, "upper_text")
end

function to_text_case(input)
    return convert_lines(input, "text")
end

function to_snake_case(input)
    return convert_lines(input, "snake")
end

function to_pascal_case(input)
    return convert_lines(input, "pascal")
end

function to_camel_case(input)
    return convert_lines(input, "camel")
end

function to_kebab_case(input)
    return convert_lines(input, "kebab")
end

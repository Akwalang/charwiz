local CONSTANTS = {
    pi = math.pi,
    e = math.exp(1),
}

local FUNCTIONS = {
    sqrt = math.sqrt,
    sin = math.sin,
    cos = math.cos,
    tan = math.tan,
    abs = math.abs,
    floor = math.floor,
    ceil = math.ceil,
    log = math.log,
    exp = math.exp,

    round = function(x)
        return math.floor(x + 0.5)
    end,
}

local function trim(value)
    return value:match("^%s*(.-)%s*$")
end

local function tokenize(expr)
    local tokens = {}
    local i = 1

    while i <= #expr do
        local c = expr:sub(i, i)

        if c:match("%s") then
            i = i + 1

        elseif c:match("[%+%-%*/%^%(%),]") then
            table.insert(tokens, c)
            i = i + 1

        elseif c:match("[%d%.]") then
            local start = i

            while i <= #expr and expr:sub(i, i):match("[%d%.]") do
                i = i + 1
            end

            local value = tonumber(expr:sub(start, i - 1))

            if not value then
                error("invalid number")
            end

            table.insert(tokens, value)

        elseif c:match("[%a_]") then
            local start = i

            while i <= #expr and expr:sub(i, i):match("[%w_]") do
                i = i + 1
            end

            table.insert(tokens, expr:sub(start, i - 1))

        else
            error("unexpected character: " .. c)
        end
    end

    return tokens
end

local Parser = {}
Parser.__index = Parser

function Parser:new(tokens, variables)
    return setmetatable({
        tokens = tokens,
        variables = variables or {},
        pos = 1,
    }, self)
end

function Parser:peek()
    return self.tokens[self.pos]
end

function Parser:consume()
    local token = self.tokens[self.pos]
    self.pos = self.pos + 1
    return token
end

function Parser:parse()
    local result = self:expression()

    if self:peek() ~= nil then
        error("unexpected token")
    end

    return result
end

function Parser:expression()
    local value = self:term()

    while true do
        local token = self:peek()

        if token == "+" then
            self:consume()
            value = value + self:term()

        elseif token == "-" then
            self:consume()
            value = value - self:term()

        else
            break
        end
    end

    return value
end

function Parser:term()
    local value = self:power()

    while true do
        local token = self:peek()

        if token == "*" then
            self:consume()
            value = value * self:power()

        elseif token == "/" then
            self:consume()
            value = value / self:power()

        else
            break
        end
    end

    return value
end

function Parser:power()
    local value = self:unary()

    if self:peek() == "^" then
        self:consume()
        value = value ^ self:power()
    end

    return value
end

function Parser:unary()
    if self:peek() == "-" then
        self:consume()
        return -self:unary()
    end

    return self:primary()
end

function Parser:primary()
    local token = self:consume()

    if type(token) == "number" then
        return token
    end

    if token == "(" then
        local value = self:expression()

        if self:consume() ~= ")" then
            error("missing )")
        end

        return value
    end

    if type(token) == "string" then
        if self:peek() == "(" then
            self:consume()

            local arg = self:expression()

            if self:consume() ~= ")" then
                error("missing )")
            end

            local fn = FUNCTIONS[token]

            if not fn then
                error("unknown function: " .. token)
            end

            return fn(arg)
        end

        local constant = CONSTANTS[token]

        if constant ~= nil then
            return constant
        end

        local variable = self.variables[token]

        if variable ~= nil then
            return variable
        end

        error("unknown identifier: " .. token)
    end

    error("unexpected token")
end

local function evaluate(expr, variables)
    local parser = Parser:new(tokenize(expr), variables)
    return parser:parse()
end

local function is_nan(value)
    return value ~= value
end

local function nearly_equal(a, b)
    if is_nan(a) or is_nan(b) then
        return false
    end

    return math.abs(a - b) < 1e-9
end

local function format_result(value)
    if is_nan(value) then
        return "Not Defined"
    end

    if value == math.huge then
        return "Infinity"
    end

    if value == -math.huge then
        return "-Infinity"
    end

    local integer = math.floor(value)

    if integer == value then
        return tostring(integer)
    end

    return string.format("%.10g", value)
end

local function process_equation(line, variables)
    local left, right = line:match("^(.-)%s*[=≠]%s*(.-)$")

    if not left then
        return nil, nil
    end

    left = trim(left)
    right = trim(right)

    local ok_left, value_left = pcall(evaluate, left, variables)
    local ok_right, value_right = pcall(evaluate, right, variables)

    if not ok_left or not ok_right then
        return line .. " = ERROR", nil
    end

    local operator = nearly_equal(value_left, value_right)
        and "="
        or "≠"

    return left .. " " .. operator .. " " .. right, value_left
end

local function process_expression(line, variables)
    local ok, value = pcall(evaluate, line, variables)

    if not ok then
        return line .. " = ERROR", nil
    end

    return line .. " = " .. format_result(value), value
end

local function process_assignment(line, variables)
    local name, expression = line:match("^([%a_][%w_]*)%s*:%s*(.+)$")

    if not name then
        return nil
    end

    expression = trim(expression)

    local text
    local value

    if expression:find("[=≠]") then
        text, value = process_equation(expression, variables)
    else
        text, value = process_expression(expression, variables)
    end

    if value ~= nil then
        variables[name] = value
    end

    return name .. ": " .. text
end

local function process_line(line, variables)
    local expression = trim(line)

    if expression == "" then
        return nil
    end

    local assignment = process_assignment(expression, variables)

    if assignment then
        return assignment
    end

    local equation = process_equation(expression, variables)

    if equation then
        return equation
    end

    return process_expression(expression, variables)
end

function calculate(input)
    local result = {}
    local variables = {}

    for line in (input.value .. "\n"):gmatch("(.-)\n") do
        local processed = process_line(line, variables)

        if processed then
            table.insert(result, processed)
        else
            table.insert(result, "")
        end
    end

    return table.concat(result, "\n")
end

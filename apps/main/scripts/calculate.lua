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

local EPSILON = 1e-9

local function trim(value)
    return value:match("^%s*(.-)%s*$")
end

local function is_nan(value)
    return value ~= value
end

local function nearly_equal(a, b)
    if is_nan(a) or is_nan(b) then
        return false
    end

    return math.abs(a - b) < EPSILON
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

    if nearly_equal(value, 0) then
        value = 0
    end

    local integer = math.floor(value)

    if nearly_equal(integer, value) then
        return tostring(integer)
    end

    return string.format("%.10g", value)
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

local function extract_unknowns(expr, variables)
    local found = {}
    local unknowns = {}

    for identifier in expr:gmatch("[%a_][%w_]*") do
        if CONSTANTS[identifier] == nil
            and FUNCTIONS[identifier] == nil
            and variables[identifier] == nil
            and not found[identifier]
        then
            found[identifier] = true
            table.insert(unknowns, identifier)
        end
    end

    return unknowns
end

local function clone_variables_with(variables, name, value)
    local cloned = {}

    for key, item in pairs(variables) do
        cloned[key] = item
    end

    cloned[name] = value

    return cloned
end

local function evaluate_with_unknown(expr, variables, name, value)
    return evaluate(expr, clone_variables_with(variables, name, value))
end

local function evaluate_equation_delta(left, right, variables, name, value)
    return evaluate_with_unknown(left, variables, name, value)
        - evaluate_with_unknown(right, variables, name, value)
end

local function is_bad_number(value)
    return value == nil or is_nan(value) or value == math.huge or value == -math.huge
end

local function solve_polynomial_equation(left, right, variables)
    local unknowns = extract_unknowns(left .. " " .. right, variables)

    if #unknowns ~= 1 then
        return nil
    end

    local name = unknowns[1]

    local ok0, y0 = pcall(evaluate_equation_delta, left, right, variables, name, 0)
    local ok1, y1 = pcall(evaluate_equation_delta, left, right, variables, name, 1)
    local ok2, y2 = pcall(evaluate_equation_delta, left, right, variables, name, 2)
    local ok3, y3 = pcall(evaluate_equation_delta, left, right, variables, name, 3)

    if not ok0 or not ok1 or not ok2 or not ok3 then
        return nil
    end

    if is_bad_number(y0) or is_bad_number(y1) or is_bad_number(y2) or is_bad_number(y3) then
        return nil
    end

    -- f(x) = ax^2 + bx + c
    local c = y0
    local a = (y2 - 2 * y1 + y0) / 2
    local b = y1 - y0 - a

    -- Проверяем, что выражение действительно не выше второй степени.
    if not nearly_equal(y3, 9 * a + 3 * b + c) then
        return nil
    end

    if nearly_equal(a, 0) then
        if nearly_equal(b, 0) then
            if nearly_equal(c, 0) then
                return { kind = "identity", name = name }
            end

            return { kind = "no_solution", name = name }
        end

        return {
            kind = "linear",
            name = name,
            value = -c / b,
        }
    end

    local discriminant = b * b - 4 * a * c

    if discriminant < -EPSILON then
        return {
            kind = "no_real_roots",
            name = name,
        }
    end

    if nearly_equal(discriminant, 0) then
        return {
            kind = "quadratic_one_root",
            name = name,
            value = -b / (2 * a),
        }
    end

    local sqrt_d = math.sqrt(discriminant)
    local x1 = (-b - sqrt_d) / (2 * a)
    local x2 = (-b + sqrt_d) / (2 * a)

    if x1 > x2 then
        x1, x2 = x2, x1
    end

    return {
        kind = "quadratic_two_roots",
        name = name,
        x1 = x1,
        x2 = x2,
    }
end


local function parse_user_solution(solution)
    local result = {}

    for name, value in solution:gmatch("([%a_][%w_]*)%s*=%s*([%+%-]?%d+%.?%d*)") do
        result[name] = tonumber(value)
    end

    for name, value in solution:gmatch("([%a_][%w_]*)%s*=%s*([%+%-]?%d*%.%d+)") do
        result[name] = tonumber(value)
    end

    return result
end

local function build_actual_solution(solved)
    local result = {}

    if solved.kind == "linear" then
        result[solved.name] = solved.value
    elseif solved.kind == "quadratic_one_root" then
        result[solved.name] = solved.value
    elseif solved.kind == "quadratic_two_roots" then
        result[solved.name .. "1"] = solved.x1
        result[solved.name .. "2"] = solved.x2
    end

    return result
end

local function solutions_match(actual, user)
    for name, value in pairs(actual) do
        local user_value = user[name]

        if user_value == nil then
            return false
        end

        if not nearly_equal(value, user_value) then
            return false
        end
    end

    return true
end

local function restore_solved_variables(line, variables)
    local equation, solution =
        line:match("^(.-)%s*[≠]?=>%s*(.-)%s*$")

    if not equation then
        return false
    end

    local left, right =
        equation:match("^(.-)%s*=%s*(.-)$")

    if not left then
        return false
    end

    left = trim(left)
    right = trim(right)

    local solved =
        solve_polynomial_equation(
            left,
            right,
            variables
        )

    if not solved then
        return false
    end

    local actual =
        build_actual_solution(solved)

    for name, value in pairs(actual) do
        variables[name] = value
    end

    local user =
        parse_user_solution(solution)

    return solutions_match(actual, user)
end

local function process_already_solved(line, variables)
    local equation, solution =
        line:match("^(.-)%s*[≠]?=>%s*(.-)%s*$")

    if not equation then
        return nil
    end

    local is_valid =
        restore_solved_variables(
            line,
            variables
        )

    local operator =
        is_valid and "=>" or "≠>"

    return equation
        .. " "
        .. operator
        .. " "
        .. solution
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

    if ok_left and ok_right then
        local operator = nearly_equal(value_left, value_right)
            and "="
            or "≠"

        return left .. " " .. operator .. " " .. right, value_left
    end

    local solved = solve_polynomial_equation(left, right, variables)

    if solved then
        if solved.kind == "linear" then
            variables[solved.name] = solved.value

            return line .. " => " .. solved.name .. " = " .. format_result(solved.value), solved.value
        end

        if solved.kind == "quadratic_one_root" then
            variables[solved.name] = solved.value

            return line .. " => " .. solved.name .. " = " .. format_result(solved.value), solved.value
        end

        if solved.kind == "quadratic_two_roots" then
            local first_name = solved.name .. "1"
            local second_name = solved.name .. "2"

            variables[first_name] = solved.x1
            variables[second_name] = solved.x2

            return line
                .. " => "
                .. first_name .. " = " .. format_result(solved.x1)
                .. ", "
                .. second_name .. " = " .. format_result(solved.x2),
                nil
        end

        if solved.kind == "identity" then
            return line .. " => Any value", nil
        end

        if solved.kind == "no_solution" then
            return line .. " => No solution", nil
        end

        if solved.kind == "no_real_roots" then
            return line .. " => No real roots", nil
        end
    end

    return line .. " = ERROR", nil
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

        if value ~= nil then
            variables[name] = value
        end

        return name .. ": " .. text
    end

    local ok, result = pcall(evaluate, expression, variables)

    if not ok then
        return name .. ": " .. expression .. " = ERROR"
    end

    variables[name] = result

    if nearly_equal(result, tonumber(expression) or math.huge) then
        return name .. ": " .. format_result(result)
    end

    return name .. ": " .. expression .. " = " .. format_result(result)
end

local function process_line(line, variables)
    local expression = trim(line)

    if expression == "" then
        return nil
    end

    local already_solved = process_already_solved(expression, variables)

    if already_solved then
        return already_solved
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
        table.insert(
            result,
            process_line(line, variables) or ""
        )
    end

    return table.concat(result, "\n")
end

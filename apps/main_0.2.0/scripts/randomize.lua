function randomize_numbers(input)
  local result = input.value:gsub("%d+", function(match)
    local num_digits = #match

    local min_value = 10^(num_digits - 1)
    local max_value = 10^num_digits - 1

    local random_number = math.random(min_value, max_value)

    return tostring(random_number)
  end)

  return result
end

function randomize_symbols(input)
  local result = input.value:gsub("[%w]", function(match)
    local random_choice = math.random(1, 3)

    if random_choice == 1 then
      return string.char(math.random(48, 57)) -- Random digit (0-9)
    elseif random_choice == 2 then
      return string.char(math.random(65, 90)) -- Random uppercase letter (A-Z)
    else
      return string.char(math.random(97, 122)) -- Random lowercase letter (a-z)
    end
  end)

  return result
end

function js_log(input)
    return "console.log('" .. input.value .. " =>', " .. "JSON.stringify(" .. input.value .. ", null, 2));"
end

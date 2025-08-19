function json_log(input)
    return "console.log('" .. input.value .. " =>', " .. "JSON.stringify(" .. input.value .. ", null, 2));"
end

function js_breakpoints(input)
    return "console.log('### " .. input.value .. " => " .. input.index .. "');"
end

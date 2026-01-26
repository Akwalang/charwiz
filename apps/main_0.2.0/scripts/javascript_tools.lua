function js_json_log(input)
    return "console.log('" .. input.value .. " =>', " .. "JSON.stringify(" .. input.value .. ", null, 2));"
end

function js_promise_catch()
    return ".catch((error) => { console.log(error); throw error; })"
end

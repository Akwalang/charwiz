local function capitalize(str)
    return (str:gsub("^%l", string.upper))
end

function react_view(input)
    local name = capitalize(input.value)

    return string.format([[
interface %sProps {};

export const %s: React.FC<%sProps> = (props) => {
  return (
    <div>
    </div>
  );
};
]], name, name, name)
end

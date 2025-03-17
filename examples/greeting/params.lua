local hour = tonumber(os.date("%H"))

if hour >= 0 and hour < 5 then
    greeting = "Good night..."
elseif hour >= 5 and hour < 12 then
    greeting = "Good morning!"
elseif hour >= 12 and hour < 18 then
    greeting = "Good afternoon"
elseif hour >= 18 and hour < 24 then
    greeting = "Good evening"
end

username = "John Doe"
today = os.date("%Y-%m-%d")

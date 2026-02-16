import sys

path = "components/swarm-node/src/main.rs"
with open(path, "r") as f:
    content = f.read()

# The specific fix for the CSS color inside the format macro
# We must double the curly braces for CSS and use backslashes for quotes
content = content.replace(
    'log.innerHTML += \'<br><span style="color:#38bdf8;">> Initiating broadcast...</span>\';',
    'log.innerHTML += "<br><span style=\\"color:#38bdf8;\\">> Initiating broadcast...</span>";'
)

# Fix the Error log line as well
content = content.replace(
    'log.innerHTML += \'<br><span style="color:#ef4444;">> Error: \' + err + \'</span>\';',
    'log.innerHTML += "<br><span style=\\"color:#ef4444;\\">> Error: " + err + "</span>";'
)

with open(path, "w") as f:
    f.write(content)

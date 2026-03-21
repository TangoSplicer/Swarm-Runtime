with open('components/swarm-node/src/worker.rs', 'r') as f:
    lines = f.readlines()

out = []
found_rx = False
skip = 0

for line in lines:
    if skip > 0:
        skip -= 1
        continue
        
    if "cmd = worker_rx.recv() => {" in line:
        if found_rx:
            # We found a duplicate! Skip this line and the 10 lines that follow it
            skip = 10
            continue
        else:
            # Keep the first one
            found_rx = True
            
    out.append(line)

with open('components/swarm-node/src/worker.rs', 'w') as f:
    f.writelines(out)

print("✅ Duplicate channel receiver removed!")

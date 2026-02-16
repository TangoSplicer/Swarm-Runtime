import sys

path = "components/swarm-node/src/main.rs"
with open(path, "r") as f:
    content = f.read()

# Fix the JavaScript: Clean up the handle and broadcast logic
fixed_js = """
        <script>
            let b64Data = '';
            
            // Clean file handling
            document.getElementById('wasm').addEventListener('change', function(e) {
                const file = e.target.files[0];
                if (!file) return;
                
                document.getElementById('f').innerText = file.name;
                document.getElementById('f').style.borderColor = '#10b981';
                document.getElementById('f').style.color = '#10b981';
                
                const reader = new FileReader();
                reader.onload = function(ev) {
                    b64Data = ev.target.result.split(',')[1];
                    console.log('Wasm loaded, length:', b64Data.length);
                };
                reader.readAsDataURL(file);
            });

            async function broadcast() {
                const log = document.getElementById('log');
                if(!b64Data) {
                    alert('Please select a .wasm file first!');
                    return;
                }
                
                log.innerHTML += '<br><span style="color:#38bdf8;">> Initiating broadcast...</span>';
                
                // Animate progress bars based on peer count
                const peerCount = document.querySelectorAll('.peer-entry').length;
                for(let i = 0; i < peerCount; i++) {
                    const bar = document.getElementById('b-' + i);
                    if(bar) setTimeout(() => bar.style.width = '100%', 300 + (i * 150));
                }

                try {
                    const response = await fetch('/deploy/shard', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({
                            wasm_base64: b64Data,
                            range_size: 300
                        })
                    });
                    const result = await response.text();
                    log.innerHTML += '<br>> ' + result;
                    log.scrollTop = log.scrollHeight;
                } catch (err) {
                    log.innerHTML += '<br><span style="color:#ef4444;">> Error: ' + err + '</span>';
                }
            }

            // Auto-refresh peers every 5s, but only if we aren't mid-upload
            setInterval(() => {
                if(!b64Data) location.reload();
            }, 5000);
        </script>
"""

# Replace the old script block with the clean version
import re
content = re.sub(r'<script>.*?</script>', fixed_js, content, flags=re.DOTALL)

with open(path, "w") as f:
    f.write(content)

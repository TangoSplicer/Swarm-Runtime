# Swarm Runtime: Enterprise Integration & Merging Guide

Running the Swarm via Termux is excellent for testing, but enterprise deployment requires deeply embedding the architecture into existing proprietary software. Below are the three primary architectural patterns for merging the Swarm Runtime into commercial applications.

## 1. The Mobile Native Integration (Android/iOS SDK)
**Goal:** Remove Termux entirely. Embed the Swarm Worker directly inside a company's existing consumer or employee app (e.g., an airline's crew app or a delivery driver's app).

* **The Rust FFI Bridge:** You will compile the `components/swarm-node` Rust crate as a native C-library (`cdylib`) instead of a binary. 
* **JNI / Swift Bindings:** Use tools like `uniffi-rs` or `jni` to generate bindings. This allows the Android Kotlin code or iOS Swift code to boot the Libp2p mesh and `wasmi` engine directly in the app's background thread.
* **The Flow:**
    1. Driver opens the proprietary logistics app.
    2. The app calls `SwarmNode.start(shard_id, keys)`.
    3. The Swarm runs invisibly in the background, receiving jobs from the Oracle Gateway and executing them without the user ever seeing a terminal.

## 2. The Sidecar Daemon Pattern (Desktop & IoT)
**Goal:** Run the Swarm on factory machines, point-of-sale registers, or desktop computers without modifying the legacy software running on them.

* **The Architecture:** Compile the Swarm Worker as a standalone system service (`systemd` on Linux, Windows Service on PC).
* **Local IPC:** The company's legacy application (e.g., a Python POS system) does not need to know how Libp2p works. It simply makes a local HTTP request to `http://localhost:4000/submit_data`.
* **The Flow:**
    1. The legacy app generates a transaction and sends it to the local Swarm Sidecar.
    2. The Sidecar wraps the data, packages it into a WASI payload, and broadcasts it to the global mesh for consensus.
    3. The Sidecar receives the BFT consensus hash and returns it to the legacy app.

## 3. The Backend API Replacement (Cloud Infrastructure)
**Goal:** Replace expensive, centralized AWS Lambda or Kubernetes microservices with the Swarm.

* **The Architecture:** The company keeps their existing frontend web apps (React/Vue) and their primary user database. However, they swap their backend computational engine.
* **The Integration:**
    Instead of the company's Node.js backend sending heavy computational tasks (like image processing or data sorting) to an AWS Lambda function, the backend formats the request and POSTs it directly to the Swarm Oracle Gateway.
    ```javascript
    // Legacy Code:
    // const result = await awsLambda.invoke({ FunctionName: 'ProcessData', Payload: myData });

    // Swarm Merged Code:
    const form = new FormData();
    form.append('wasm', fs.createReadStream('process_data.wasm'));
    form.append('metadata', JSON.stringify({ dataset: [myData] }));
    const result = await axios.post('[http://145.241.192.79:3000/api/v1/jobs](http://145.241.192.79:3000/api/v1/jobs)', form);
    ```
* **The Flow:** The company's existing infrastructure treats the Swarm Gateway exactly like a standard REST API, completely unaware that the actual computation is being offloaded to thousands of Android phones around the world.

## Summary: The Path to Production
1. **Define the Target:** Are you embedding into a Mobile App (SDK), an IoT device (Sidecar), or a Cloud Backend (REST API)?
2. **Strip the CLI:** Remove `clap` and the terminal outputs from `main.rs`. 
3. **Expose the Engine:** Wrap the `worker::run_worker` function in an API that your target language (Kotlin, Swift, Python, or JS) can natively call.

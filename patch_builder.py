import os

path = "components/judge/src/lib.rs"
with open(path, "r") as f:
    code = f.read()

target = """        let mut builder = WasiCtxBuilder::new();
        builder.inherit_stdout().inherit_stderr().args(&wasi_args).unwrap();
        
        if polyglot_id == "POLYGLOT:PYTHON" {
            builder.env("PYTHONPATH", "/python-wasi.zip").unwrap().env("PYTHONHOME", "/").unwrap();
        }

        let wasi_ctx = builder.preopened_dir(root_dir, "/").unwrap().build();"""

replacement = """        let mut builder = WasiCtxBuilder::new()
            .inherit_stdout()
            .inherit_stderr()
            .args(&wasi_args)
            .unwrap();
        
        if polyglot_id == "POLYGLOT:PYTHON" {
            builder = builder.env("PYTHONPATH", "/python-wasi.zip").unwrap().env("PYTHONHOME", "/").unwrap();
        }

        let wasi_ctx = builder.preopened_dir(root_dir, "/").unwrap().build();"""

if target in code:
    code = code.replace(target, replacement)
    with open(path, "w") as f:
        f.write(code)
    print("✅ Judge WASI Builder memory ownership resolved.")
else:
    print("⚠️ Target code block not found. Double-check indentation.")

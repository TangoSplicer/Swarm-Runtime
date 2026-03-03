(module
  (import "wasi_snapshot_preview1" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))
  (memory 1)
  (export "memory" (memory 0))
  
  ;; Store the string at memory offset 8
  (data (i32.const 8) "Phase 5.6: Compiled Wasm Mesh Active.\n")
  
  (func $main (export "_start")
    ;; Setup the IO Vector at offset 0
    (i32.store (i32.const 0) (i32.const 8))   ;; Pointer to string
    (i32.store (i32.const 4) (i32.const 38))  ;; Length of string (38 bytes)
    
    ;; Call WASI fd_write to print to standard output
    (call $fd_write
      (i32.const 1)  ;; 1 = stdout
      (i32.const 0)  ;; IO Vector offset
      (i32.const 1)  ;; 1 Vector
      (i32.const 4)  ;; Where to store bytes written
    )
    drop
  )
)

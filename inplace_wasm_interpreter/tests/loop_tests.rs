use inplace_wasm_interpreter::{run, Value};

#[test]
fn as_if_then() {
    let wasm = wat::parse_str(
        "(func (result i32)
    (if (result i32) (i32.const 1) (then (loop (result i32) (i32.const 1))) (else (i32.const 2)))
  )",
    )
    .unwrap();
    assert_eq!(run(&wasm, vec![]), vec![Value::I32(1)]);
}



#[test]
fn nested() {
    let wasm = wat::parse_str(
        "
(func
(local $index i32)
(local $total i32)

;; Set starting value of $index
i32.const 0
local.set $index

;; for (var $index = 0; $index < 100; $index++)
loop $loop_index
block $block_index
  ;; Check if $index is odd
  local.get $index
  i32.const 1
  i32.and

  ;; If odd
  if
    ;; Add index to total
    local.get $total
    local.get $index
    i32.add
    local.set $total
  end

  ;; Check $index is over 100
  local.get $index
  i32.const 100
  i32.ge_s
  br_if $block_index

  ;; Increase $index
  local.get $index
  i32.const 0x01
  i32.add
  local.set $index

  ;; Loop again
  br $loop_index
end
end  
)

",
    )
    .unwrap();
    assert_eq!(run(&wasm, vec![]), vec![]);
}




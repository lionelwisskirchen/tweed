(module
  (type (;0;) (func))
  (type (;1;) (func (param i32 i32) (result i32)))
  (type (;2;) (func (result i32)))
  (type (;3;) (func (param i32 i64 i64 i64 i64)))
  (func (;0;) (type 0))
  (func (;1;) (type 0)
    block  ;; label = @1
      global.get 1
      i32.const 1024
      i32.add
      i32.load
      i32.eqz
      br_if 0 (;@1;)
      unreachable
    end
    global.get 1
    i32.const 1024
    i32.add
    i32.const 1
    i32.store
    call 0)
  (func (;2;) (type 1) (param i32 i32) (result i32)
    (local i32 i32 i64 i64 i32 i64 i64 i64 i64 i32 i64 i64 i64 i64 i64 i64 i64)
    global.get 0
    i32.const 176
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        br_if 0 (;@2;)
        br 1 (;@1;)
      end
      i32.const 0
      local.set 3
      local.get 2
      i32.const 144
      i32.add
      i64.const 380896260630216687
      i64.const 0
      i32.const 0
      i32.load8_u offset=1048
      select
      local.tee 4
      i64.const 0
      i64.const 1473642379452024179
      i64.const 0
      i32.const 0
      i32.load8_u offset=1040
      select
      local.tee 5
      i64.const 0
      call 10
      i64.const 366962936819156833
      i64.const 0
      i32.const 0
      i32.load8_u offset=1032
      local.tee 6
      select
      local.set 7
      i64.const -366962936819156833
      i64.const 0
      local.get 6
      select
      local.set 8
      local.get 2
      i32.const 144
      i32.add
      i32.const 8
      i32.add
      i64.load
      local.set 9
      local.get 2
      i64.load offset=144
      local.set 10
      loop  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          local.set 11
          loop  ;; label = @4
            i64.const 64
            local.set 12
            local.get 9
            local.set 13
            local.get 10
            local.set 14
            loop  ;; label = @5
              local.get 13
              i64.const 1
              i64.shl
              local.get 14
              i64.const 63
              i64.shr_u
              i64.or
              local.tee 15
              local.get 8
              i64.const 0
              local.get 15
              local.get 13
              i64.const 63
              i64.shr_s
              i64.or
              local.get 8
              i64.ge_u
              local.tee 6
              select
              i64.sub
              local.set 13
              local.get 14
              i64.const 1
              i64.shl
              local.get 6
              i64.extend_i32_u
              i64.or
              local.set 14
              local.get 12
              i64.const -1
              i64.add
              local.tee 12
              i64.const 0
              i64.ne
              br_if 0 (;@5;)
            end
            local.get 2
            i32.const 128
            i32.add
            local.get 13
            i64.const 0
            local.get 13
            i64.const 0
            call 10
            local.get 2
            i32.const 128
            i32.add
            i32.const 8
            i32.add
            i64.load
            local.set 13
            i64.const 64
            local.set 12
            local.get 2
            i64.load offset=128
            local.set 14
            loop  ;; label = @5
              local.get 13
              i64.const 1
              i64.shl
              local.get 14
              i64.const 63
              i64.shr_u
              i64.or
              local.tee 15
              local.get 8
              i64.const 0
              local.get 15
              local.get 13
              i64.const 63
              i64.shr_s
              i64.or
              local.get 8
              i64.ge_u
              local.tee 6
              select
              i64.sub
              local.set 13
              local.get 14
              i64.const 1
              i64.shl
              local.get 6
              i64.extend_i32_u
              i64.or
              local.set 14
              local.get 12
              i64.const -1
              i64.add
              local.tee 12
              i64.const 0
              i64.ne
              br_if 0 (;@5;)
            end
            local.get 2
            i32.const 112
            i32.add
            local.get 13
            i64.const 0
            local.get 13
            i64.const 0
            call 10
            local.get 2
            i32.const 112
            i32.add
            i32.const 8
            i32.add
            i64.load
            local.set 12
            i64.const 64
            local.set 14
            local.get 2
            i64.load offset=112
            local.set 13
            loop  ;; label = @5
              local.get 12
              i64.const 1
              i64.shl
              local.get 13
              i64.const 63
              i64.shr_u
              i64.or
              local.tee 15
              local.get 8
              i64.const 0
              local.get 15
              local.get 12
              i64.const 63
              i64.shr_s
              i64.or
              local.get 8
              i64.ge_u
              local.tee 6
              select
              i64.sub
              local.set 12
              local.get 13
              i64.const 1
              i64.shl
              local.get 6
              i64.extend_i32_u
              i64.or
              local.set 13
              local.get 14
              i64.const -1
              i64.add
              local.tee 14
              i64.const 0
              i64.ne
              br_if 0 (;@5;)
            end
            i64.const -9223372036854775808
            local.set 14
            i64.const 1
            local.set 13
            i64.const 0
            local.set 15
            loop  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 13
                  i32.wrap_i64
                  i32.const 1
                  i32.and
                  br_if 0 (;@7;)
                  local.get 15
                  i64.const 1
                  i64.shr_u
                  local.set 15
                  local.get 13
                  i64.const 1
                  i64.shr_u
                  local.set 13
                  br 1 (;@6;)
                end
                local.get 13
                local.get 8
                i64.and
                local.get 13
                local.get 8
                i64.xor
                i64.const 1
                i64.shr_u
                i64.add
                local.set 13
                local.get 15
                i64.const 1
                i64.shr_u
                i64.const -9223372036854775808
                i64.or
                local.set 15
              end
              local.get 14
              i64.const 1
              i64.gt_u
              local.set 6
              local.get 14
              i64.const 1
              i64.shr_u
              local.set 14
              local.get 6
              br_if 0 (;@5;)
            end
            local.get 2
            local.get 13
            i64.store offset=168
            local.get 2
            local.get 15
            i64.store offset=160
            local.get 2
            i64.load offset=168
            drop
            i64.const 0
            local.set 14
            i64.const 64
            local.set 15
            local.get 2
            i64.load offset=160
            local.set 16
            local.get 4
            local.set 13
            loop  ;; label = @5
              local.get 13
              i64.const 1
              i64.shl
              local.get 14
              i64.const 63
              i64.shr_u
              i64.or
              local.tee 17
              local.get 8
              i64.const 0
              local.get 17
              local.get 13
              i64.const 63
              i64.shr_s
              i64.or
              local.get 8
              i64.ge_u
              local.tee 6
              select
              i64.sub
              local.set 13
              local.get 14
              i64.const 1
              i64.shl
              local.get 6
              i64.extend_i32_u
              i64.or
              local.set 14
              local.get 15
              i64.const -1
              i64.add
              local.tee 15
              i64.const 0
              i64.ne
              br_if 0 (;@5;)
            end
            i64.const 0
            local.set 15
            i64.const 64
            local.set 17
            local.get 5
            local.set 14
            loop  ;; label = @5
              local.get 14
              i64.const 1
              i64.shl
              local.get 15
              i64.const 63
              i64.shr_u
              i64.or
              local.tee 18
              local.get 8
              i64.const 0
              local.get 18
              local.get 14
              i64.const 63
              i64.shr_s
              i64.or
              local.get 8
              i64.ge_u
              local.tee 6
              select
              i64.sub
              local.set 14
              local.get 15
              i64.const 1
              i64.shl
              local.get 6
              i64.extend_i32_u
              i64.or
              local.set 15
              local.get 17
              i64.const -1
              i64.add
              local.tee 17
              i64.const 0
              i64.ne
              br_if 0 (;@5;)
            end
            local.get 2
            i32.const 96
            i32.add
            local.get 14
            i64.const 0
            local.get 13
            i64.const 0
            call 10
            local.get 2
            i32.const 80
            i32.add
            local.get 2
            i64.load offset=160
            local.get 2
            i64.load offset=96
            local.tee 13
            i64.mul
            i64.const 0
            local.get 8
            i64.const 0
            call 10
            local.get 2
            i32.const 64
            i32.add
            local.get 2
            i32.const 80
            i32.add
            i32.const 8
            i32.add
            i64.load
            local.get 2
            i32.const 96
            i32.add
            i32.const 8
            i32.add
            i64.load
            local.tee 14
            i64.add
            local.get 13
            i64.const -1
            i64.xor
            local.get 2
            i64.load offset=80
            i64.lt_u
            local.tee 6
            i64.extend_i32_u
            i64.add
            local.tee 13
            local.get 8
            local.get 8
            local.get 8
            i64.const 0
            local.get 13
            local.get 14
            i64.eq
            select
            i64.const 0
            local.get 6
            select
            local.get 13
            local.get 14
            i64.lt_u
            select
            local.get 13
            local.get 8
            i64.ge_u
            select
            i64.sub
            local.tee 13
            i64.const 0
            local.get 13
            i64.const 0
            call 10
            local.get 2
            i32.const 48
            i32.add
            local.get 2
            i64.load offset=160
            local.get 2
            i64.load offset=64
            local.tee 13
            i64.mul
            i64.const 0
            local.get 8
            i64.const 0
            call 10
            local.get 2
            i32.const 32
            i32.add
            local.get 2
            i32.const 48
            i32.add
            i32.const 8
            i32.add
            i64.load
            local.get 2
            i32.const 64
            i32.add
            i32.const 8
            i32.add
            i64.load
            local.tee 14
            i64.add
            local.get 13
            i64.const -1
            i64.xor
            local.get 2
            i64.load offset=48
            i64.lt_u
            local.tee 6
            i64.extend_i32_u
            i64.add
            local.tee 13
            local.get 8
            local.get 8
            local.get 8
            i64.const 0
            local.get 13
            local.get 14
            i64.eq
            select
            i64.const 0
            local.get 6
            select
            local.get 13
            local.get 14
            i64.lt_u
            select
            local.get 13
            local.get 8
            i64.ge_u
            select
            i64.sub
            local.tee 13
            i64.const 0
            local.get 13
            i64.const 0
            call 10
            local.get 2
            i32.const 16
            i32.add
            local.get 2
            i64.load offset=160
            local.get 2
            i64.load offset=32
            local.tee 13
            i64.mul
            i64.const 0
            local.get 8
            i64.const 0
            call 10
            local.get 2
            local.get 2
            i32.const 16
            i32.add
            i32.const 8
            i32.add
            i64.load
            local.get 2
            i32.const 32
            i32.add
            i32.const 8
            i32.add
            i64.load
            local.tee 14
            i64.add
            local.get 13
            i64.const -1
            i64.xor
            local.get 2
            i64.load offset=16
            i64.lt_u
            local.tee 6
            i64.extend_i32_u
            i64.add
            local.tee 13
            local.get 8
            local.get 8
            local.get 8
            i64.const 0
            local.get 13
            local.get 14
            i64.eq
            select
            i64.const 0
            local.get 6
            select
            local.get 13
            local.get 14
            i64.lt_u
            select
            local.get 13
            local.get 8
            i64.ge_u
            select
            i64.sub
            i64.const 0
            local.get 2
            i64.load offset=168
            i64.const 0
            call 10
            local.get 2
            i32.const 8
            i32.add
            i64.load
            local.set 13
            i64.const 64
            local.set 15
            local.get 2
            i64.load
            local.set 14
            loop  ;; label = @5
              local.get 13
              i64.const 1
              i64.shl
              local.get 14
              i64.const 63
              i64.shr_u
              i64.or
              local.tee 17
              local.get 8
              i64.const 0
              local.get 17
              local.get 13
              i64.const 63
              i64.shr_s
              i64.or
              local.get 8
              i64.ge_u
              local.tee 6
              select
              i64.sub
              local.set 13
              local.get 14
              i64.const 1
              i64.shl
              local.get 6
              i64.extend_i32_u
              i64.or
              local.set 14
              local.get 15
              i64.const -1
              i64.add
              local.tee 15
              i64.const 0
              i64.ne
              br_if 0 (;@5;)
            end
            local.get 11
            i32.const 1
            i32.add
            local.tee 11
            local.get 1
            i32.ne
            br_if 0 (;@4;)
          end
          local.get 13
          local.get 12
          i64.ne
          local.get 16
          local.get 7
          i64.mul
          i64.const 1
          i64.ne
          i32.or
          local.set 6
        end
        local.get 3
        i32.const 1
        i32.add
        local.tee 3
        local.get 0
        i32.ne
        br_if 0 (;@2;)
      end
    end
    local.get 2
    i32.const 176
    i32.add
    global.set 0
    local.get 6)
  (func (;3;) (type 2) (result i32)
    i32.const 472
    i32.const 1
    call 2)
  (func (;4;) (type 0)
    i32.const 0
    i32.const 1
    i32.store8 offset=1040
    i32.const 0
    i32.const 1
    i32.store8 offset=1032
    i32.const 0
    i32.const 1
    i32.store8 offset=1048)
  (func (;5;) (type 1) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    call 6
    call 4
    call 8
    local.get 2
    call 3
    i32.store offset=12
    call 9
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    i32.const 0)
  (func (;6;) (type 0))
  (func (;7;) (type 2) (result i32)
    i32.const 0
    i32.const 0
    call 5)
  (func (;8;) (type 0))
  (func (;9;) (type 0))
  (func (;10;) (type 3) (param i32 i64 i64 i64 i64)
    (local i64)
    local.get 0
    local.get 4
    local.get 1
    i64.mul
    local.get 2
    local.get 3
    i64.mul
    i64.add
    local.get 3
    i64.const 32
    i64.shr_u
    local.tee 2
    local.get 1
    i64.const 32
    i64.shr_u
    local.tee 4
    i64.mul
    i64.add
    local.get 3
    i64.const 4294967295
    i64.and
    local.tee 3
    local.get 1
    i64.const 4294967295
    i64.and
    local.tee 1
    i64.mul
    local.tee 5
    i64.const 32
    i64.shr_u
    local.get 3
    local.get 4
    i64.mul
    i64.add
    local.tee 3
    i64.const 32
    i64.shr_u
    i64.add
    local.get 3
    i64.const 4294967295
    i64.and
    local.get 2
    local.get 1
    i64.mul
    i64.add
    local.tee 1
    i64.const 32
    i64.shr_u
    i64.add
    i64.store offset=8
    local.get 0
    local.get 1
    i64.const 32
    i64.shl
    local.get 5
    i64.const 4294967295
    i64.and
    i64.or
    i64.store)
  (memory (;0;) 1)
  (global (;0;) (mut i32) (i32.const 17056))
  (global (;1;) i32 (i32.const 0))
  (global (;2;) i32 (i32.const 1049))
  (global (;3;) i32 (i32.const 17056))
  (export "memory" (memory 0))
  (export "_initialize" (func 1))
  (export "_run" (func 7))
  (export "__data_end" (global 2))
  (export "__heap_base" (global 3)))

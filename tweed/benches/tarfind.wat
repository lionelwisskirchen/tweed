(module
  (type (;0;) (func))
  (type (;1;) (func (param i32 i32) (result i32)))
  (type (;2;) (func (result i32)))
  (type (;3;) (func (param i32 i32)))
  (type (;4;) (func (param i32) (result i32)))
  (type (;5;) (func (param i32)))
  (type (;6;) (func (param i32 i32 i32) (result i32)))
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
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      i32.const 0
      return
    end
    i32.const 0
    local.set 2
    loop  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        local.set 3
        loop  ;; label = @3
          i32.const 1028
          i32.const 8996
          call 7
          i32.const 5
          local.set 4
          i32.const 8995
          call 8
          local.tee 5
          local.set 6
          i32.const 0
          local.set 7
          loop  ;; label = @4
            i32.const 0
            local.set 8
            local.get 5
            local.get 7
            i32.const 257
            i32.mul
            i32.add
            i32.const 0
            i32.const 257
            call 14
            local.tee 9
            i32.const 48
            i32.store8 offset=156
            loop  ;; label = @5
              local.get 6
              local.get 8
              i32.add
              call 6
              i32.const 26
              i32.rem_s
              i32.const 65
              i32.add
              i32.store8
              local.get 4
              local.get 8
              i32.const 1
              i32.add
              local.tee 8
              i32.ne
              br_if 0 (;@5;)
            end
            local.get 9
            i32.const 48
            i32.store8 offset=124
            local.get 6
            i32.const 257
            i32.add
            local.set 6
            local.get 4
            i32.const 1
            i32.add
            local.set 4
            local.get 7
            i32.const 1
            i32.add
            local.tee 7
            i32.const 35
            i32.ne
            br_if 0 (;@4;)
          end
          local.get 5
          i32.const 1
          i32.add
          local.set 10
          local.get 5
          i32.const 4369
          i32.add
          local.set 11
          i32.const 0
          local.set 12
          i32.const 0
          local.set 13
          loop  ;; label = @4
            local.get 11
            local.get 12
            i32.const 257
            i32.mul
            i32.add
            local.set 14
            local.get 10
            local.set 9
            i32.const 0
            local.set 7
            block  ;; label = @5
              block  ;; label = @6
                loop  ;; label = @7
                  local.get 14
                  local.set 8
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 5
                      local.get 7
                      i32.const 257
                      i32.mul
                      i32.add
                      i32.load8_u
                      local.tee 4
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 9
                      local.set 6
                      local.get 14
                      local.set 8
                      loop  ;; label = @10
                        local.get 4
                        i32.const 255
                        i32.and
                        local.get 8
                        i32.load8_u
                        i32.ne
                        br_if 2 (;@8;)
                        local.get 8
                        i32.const 1
                        i32.add
                        local.set 8
                        local.get 6
                        i32.load8_u
                        local.set 4
                        local.get 6
                        i32.const 1
                        i32.add
                        local.set 6
                        local.get 4
                        br_if 0 (;@10;)
                      end
                    end
                    local.get 8
                    i32.load8_u
                    i32.eqz
                    br_if 2 (;@6;)
                  end
                  local.get 9
                  i32.const 257
                  i32.add
                  local.set 9
                  local.get 7
                  i32.const 1
                  i32.add
                  local.tee 7
                  i32.const 35
                  i32.ne
                  br_if 0 (;@7;)
                  br 2 (;@5;)
                end
              end
              local.get 13
              i32.const 1
              i32.add
              local.set 13
            end
            local.get 12
            i32.const 1
            i32.add
            local.tee 12
            i32.const 5
            i32.ne
            br_if 0 (;@4;)
          end
          local.get 5
          call 9
          local.get 3
          i32.const 1
          i32.add
          local.tee 3
          local.get 1
          i32.ne
          br_if 0 (;@3;)
        end
      end
      local.get 2
      i32.const 1
      i32.add
      local.tee 2
      local.get 0
      i32.ne
      br_if 0 (;@1;)
    end
    local.get 13
    i32.const 5
    i32.eq)
  (func (;3;) (type 2) (result i32)
    i32.const 46
    i32.const 1
    call 2)
  (func (;4;) (type 0))
  (func (;5;) (type 1) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    call 10
    call 4
    call 12
    local.get 2
    call 3
    i32.store offset=12
    call 13
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    i32.const 0)
  (func (;6;) (type 2) (result i32)
    (local i32)
    i32.const 0
    i32.const 0
    i32.load offset=10024
    i32.const 1103515245
    i32.mul
    i32.const 12345
    i32.add
    i32.const 2147483647
    i32.and
    local.tee 0
    i32.store offset=10024
    local.get 0
    i32.const 16
    i32.shr_u)
  (func (;7;) (type 3) (param i32 i32)
    block  ;; label = @1
      local.get 1
      i32.const 3
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      loop  ;; label = @2
        br 0 (;@2;)
      end
    end
    i32.const 0
    local.get 0
    i32.store offset=10028
    i32.const 0
    local.get 0
    local.get 1
    i32.add
    i32.store offset=10032
    i32.const 0
    i32.const 0
    i32.store offset=10036)
  (func (;8;) (type 4) (param i32) (result i32)
    (local i32 i32 i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      i32.const 0
      return
    end
    i32.const 0
    local.set 1
    i32.const 0
    i32.const 0
    i32.load offset=10036
    local.get 0
    i32.add
    i32.const 4
    i32.const 0
    i32.load offset=10028
    local.tee 2
    local.get 0
    i32.add
    local.tee 0
    i32.const 3
    i32.and
    local.tee 3
    i32.sub
    i32.const 0
    local.get 3
    select
    local.tee 3
    i32.add
    i32.store offset=10036
    block  ;; label = @1
      local.get 0
      local.get 3
      i32.add
      local.tee 0
      i32.const 0
      i32.load offset=10032
      i32.gt_u
      br_if 0 (;@1;)
      i32.const 0
      local.get 0
      i32.store offset=10028
      local.get 2
      local.set 1
    end
    local.get 1)
  (func (;9;) (type 5) (param i32))
  (func (;10;) (type 0))
  (func (;11;) (type 2) (result i32)
    i32.const 0
    i32.const 0
    call 5)
  (func (;12;) (type 0))
  (func (;13;) (type 0))
  (func (;14;) (type 6) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i64)
    block  ;; label = @1
      local.get 2
      i32.const 33
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      local.get 2
      memory.fill
      local.get 0
      return
    end
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.store8
      local.get 0
      local.get 2
      i32.add
      local.tee 3
      i32.const -1
      i32.add
      local.get 1
      i32.store8
      local.get 2
      i32.const 3
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.store8 offset=2
      local.get 0
      local.get 1
      i32.store8 offset=1
      local.get 3
      i32.const -3
      i32.add
      local.get 1
      i32.store8
      local.get 3
      i32.const -2
      i32.add
      local.get 1
      i32.store8
      local.get 2
      i32.const 7
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.store8 offset=3
      local.get 3
      i32.const -4
      i32.add
      local.get 1
      i32.store8
      local.get 2
      i32.const 9
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      i32.const 0
      local.get 0
      i32.sub
      i32.const 3
      i32.and
      local.tee 4
      i32.add
      local.tee 5
      local.get 1
      i32.const 255
      i32.and
      i32.const 16843009
      i32.mul
      local.tee 3
      i32.store
      local.get 5
      local.get 2
      local.get 4
      i32.sub
      i32.const 60
      i32.and
      local.tee 1
      i32.add
      local.tee 2
      i32.const -4
      i32.add
      local.get 3
      i32.store
      local.get 1
      i32.const 9
      i32.lt_u
      br_if 0 (;@1;)
      local.get 5
      local.get 3
      i32.store offset=8
      local.get 5
      local.get 3
      i32.store offset=4
      local.get 2
      i32.const -8
      i32.add
      local.get 3
      i32.store
      local.get 2
      i32.const -12
      i32.add
      local.get 3
      i32.store
      local.get 1
      i32.const 25
      i32.lt_u
      br_if 0 (;@1;)
      local.get 5
      local.get 3
      i32.store offset=24
      local.get 5
      local.get 3
      i32.store offset=20
      local.get 5
      local.get 3
      i32.store offset=16
      local.get 5
      local.get 3
      i32.store offset=12
      local.get 2
      i32.const -16
      i32.add
      local.get 3
      i32.store
      local.get 2
      i32.const -20
      i32.add
      local.get 3
      i32.store
      local.get 2
      i32.const -24
      i32.add
      local.get 3
      i32.store
      local.get 2
      i32.const -28
      i32.add
      local.get 3
      i32.store
      local.get 1
      local.get 5
      i32.const 4
      i32.and
      i32.const 24
      i32.or
      local.tee 2
      i32.sub
      local.tee 1
      i32.const 32
      i32.lt_u
      br_if 0 (;@1;)
      local.get 3
      i64.extend_i32_u
      i64.const 4294967297
      i64.mul
      local.set 6
      local.get 5
      local.get 2
      i32.add
      local.set 2
      loop  ;; label = @2
        local.get 2
        local.get 6
        i64.store offset=24
        local.get 2
        local.get 6
        i64.store offset=16
        local.get 2
        local.get 6
        i64.store offset=8
        local.get 2
        local.get 6
        i64.store
        local.get 2
        i32.const 32
        i32.add
        local.set 2
        local.get 1
        i32.const -32
        i32.add
        local.tee 1
        i32.const 31
        i32.gt_u
        br_if 0 (;@2;)
      end
    end
    local.get 0)
  (table (;0;) 1 1 funcref)
  (memory (;0;) 1)
  (global (;0;) (mut i32) (i32.const 26048))
  (global (;1;) i32 (i32.const 0))
  (global (;2;) i32 (i32.const 10040))
  (global (;3;) i32 (i32.const 26048))
  (export "memory" (memory 0))
  (export "_initialize" (func 1))
  (export "_run" (func 11))
  (export "__data_end" (global 2))
  (export "__heap_base" (global 3)))

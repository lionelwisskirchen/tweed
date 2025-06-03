(module
  (type (;0;) (func))
  (type (;1;) (func (param i32 i32)))
  (type (;2;) (func (param i32 i32 i32 i32)))
  (type (;3;) (func (result i32)))
  (type (;4;) (func (param i32 i32) (result i32)))
  (type (;5;) (func (param i32) (result i32)))
  (type (;6;) (func (param i32)))
  (type (;7;) (func (param i32 i32 i32) (result i32)))
  (func (;0;) (type 0))
  (func (;1;) (type 0)
    block  ;; label = @1
      global.get 1
      i32.const 1536
      i32.add
      i32.load
      i32.eqz
      br_if 0 (;@1;)
      unreachable
    end
    global.get 1
    i32.const 1536
    i32.add
    i32.const 1
    i32.store
    call 0)
  (func (;2;) (type 1) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 7680
    i32.sub
    local.tee 2
    global.set 0
    i32.const 0
    local.set 3
    local.get 1
    i32.const 1
    i32.add
    local.tee 4
    call 9
    i32.const 0
    local.get 4
    call 16
    local.set 5
    local.get 2
    i32.const 5632
    i32.add
    i32.const 0
    i32.const 2048
    call 16
    drop
    local.get 2
    i32.const 4608
    i32.add
    i32.const 0
    i32.const 1024
    call 16
    drop
    local.get 2
    i32.const 2560
    i32.add
    i32.const 0
    i32.const 2048
    call 16
    drop
    local.get 2
    i32.const 1536
    i32.add
    i32.const 0
    i32.const 1024
    call 16
    drop
    local.get 2
    i32.const 1280
    i32.add
    i32.const 0
    i32.const 256
    call 16
    drop
    block  ;; label = @1
      local.get 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      local.set 6
      local.get 0
      local.set 4
      loop  ;; label = @2
        local.get 2
        i32.const 5632
        i32.add
        local.get 4
        i32.load8_u
        i32.const 2
        i32.shl
        i32.add
        local.tee 7
        local.get 7
        i32.load
        i32.const 1
        i32.add
        i32.store
        local.get 4
        i32.const 1
        i32.add
        local.set 4
        local.get 6
        i32.const -1
        i32.add
        local.tee 6
        br_if 0 (;@2;)
      end
    end
    local.get 2
    i32.const 4608
    i32.add
    local.set 8
    local.get 2
    i32.const 5632
    i32.add
    local.tee 9
    local.set 6
    i32.const 0
    local.set 4
    loop  ;; label = @1
      block  ;; label = @2
        local.get 6
        i32.load
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        i32.const 4608
        i32.add
        local.get 4
        i32.const 2
        i32.shl
        i32.add
        local.get 3
        i32.store
        local.get 4
        i32.const 1
        i32.add
        local.set 4
      end
      local.get 6
      i32.const 4
      i32.add
      local.set 6
      local.get 3
      i32.const 1
      i32.add
      local.tee 3
      i32.const 256
      i32.ne
      br_if 0 (;@1;)
    end
    i32.const 0
    local.set 10
    block  ;; label = @1
      local.get 4
      i32.eqz
      br_if 0 (;@1;)
      local.get 4
      local.set 6
      loop  ;; label = @2
        local.get 2
        i32.const 5632
        i32.add
        local.get 2
        i32.const 4608
        i32.add
        local.get 4
        local.get 6
        call 3
        local.get 6
        i32.const -1
        i32.add
        local.tee 6
        br_if 0 (;@2;)
      end
      block  ;; label = @2
        local.get 4
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        local.get 4
        local.set 10
        br 1 (;@1;)
      end
      i32.const -255
      local.get 4
      i32.sub
      local.set 7
      local.get 4
      i32.const 2
      i32.shl
      local.set 6
      local.get 2
      i32.const 6652
      i32.add
      local.set 11
      local.get 2
      i32.const 4608
      i32.add
      i32.const -4
      i32.add
      local.set 12
      loop  ;; label = @2
        local.get 2
        i32.load offset=4608
        local.set 13
        local.get 2
        local.get 12
        local.get 6
        i32.add
        i32.load
        i32.store offset=4608
        i32.const 1
        local.set 10
        local.get 2
        i32.const 5632
        i32.add
        local.get 2
        i32.const 4608
        i32.add
        local.get 4
        i32.const -1
        i32.add
        local.tee 3
        i32.const 1
        call 3
        local.get 2
        i32.const 2560
        i32.add
        local.get 13
        i32.const 2
        i32.shl
        local.tee 13
        i32.add
        local.get 4
        i32.const 255
        i32.add
        local.tee 4
        i32.store
        local.get 2
        i32.const 2560
        i32.add
        local.get 2
        i32.load offset=4608
        i32.const 2
        i32.shl
        local.tee 14
        i32.add
        local.get 7
        i32.store
        local.get 11
        local.get 6
        i32.add
        local.get 2
        i32.const 5632
        i32.add
        local.get 13
        i32.add
        i32.load
        local.get 2
        i32.const 5632
        i32.add
        local.get 14
        i32.add
        i32.load
        i32.add
        i32.store
        local.get 2
        local.get 4
        i32.store offset=4608
        local.get 9
        local.get 8
        local.get 3
        i32.const 1
        call 3
        local.get 7
        i32.const 1
        i32.add
        local.set 7
        local.get 6
        i32.const -4
        i32.add
        local.set 6
        local.get 3
        local.set 4
        local.get 3
        i32.const 1
        i32.gt_u
        br_if 0 (;@2;)
      end
    end
    local.get 2
    i32.const 2560
    i32.add
    local.get 10
    i32.const 2
    i32.shl
    i32.add
    i32.const 1024
    i32.add
    i32.const 0
    i32.store
    i32.const 0
    local.set 8
    i32.const 0
    local.set 9
    i32.const 0
    local.set 14
    loop  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.const 5632
              i32.add
              local.get 14
              i32.const 2
              i32.shl
              local.tee 10
              i32.add
              i32.load
              i32.eqz
              br_if 0 (;@5;)
              local.get 2
              i32.const 2560
              i32.add
              local.get 10
              i32.add
              i32.load
              local.tee 4
              br_if 1 (;@4;)
              i32.const 0
              local.set 7
              i32.const 0
              local.set 13
              br 2 (;@3;)
            end
            local.get 2
            i32.const 1280
            i32.add
            local.get 14
            i32.add
            i32.const 0
            i32.store8
            local.get 2
            i32.const 1536
            i32.add
            local.get 10
            i32.add
            i32.const 0
            i32.store
            br 2 (;@2;)
          end
          i32.const 1
          local.set 6
          i32.const 0
          local.set 13
          i32.const 0
          local.set 7
          loop  ;; label = @4
            local.get 7
            i32.const 1
            i32.add
            local.set 7
            local.get 4
            i32.const 31
            i32.shr_s
            local.tee 3
            local.get 6
            i32.and
            local.get 13
            i32.add
            local.set 13
            local.get 6
            i32.const 1
            i32.shl
            local.set 6
            local.get 2
            i32.const 2560
            i32.add
            local.get 4
            local.get 3
            i32.xor
            local.get 3
            i32.sub
            i32.const 2
            i32.shl
            i32.add
            i32.load
            local.tee 4
            br_if 0 (;@4;)
          end
        end
        local.get 2
        i32.const 1280
        i32.add
        local.get 14
        i32.add
        local.get 7
        i32.store8
        local.get 2
        i32.const 1536
        i32.add
        local.get 10
        i32.add
        local.get 13
        i32.store
        local.get 7
        local.get 8
        local.get 7
        local.get 8
        i32.gt_u
        select
        local.set 8
        local.get 13
        local.get 9
        local.get 13
        local.get 9
        i32.gt_u
        select
        local.set 9
      end
      local.get 14
      i32.const 1
      i32.add
      local.tee 14
      i32.const 256
      i32.ne
      br_if 0 (;@1;)
    end
    block  ;; label = @1
      local.get 8
      i32.const 32
      i32.gt_u
      br_if 0 (;@1;)
      local.get 9
      i32.eqz
      br_if 0 (;@1;)
      i32.const 0
      local.set 13
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          br_if 0 (;@3;)
          i32.const 0
          local.set 8
          i32.const 0
          local.set 4
          br 1 (;@2;)
        end
        i32.const -1
        local.set 6
        i32.const 0
        local.set 7
        i32.const 0
        local.set 8
        i32.const 0
        local.set 11
        local.get 0
        local.set 9
        loop  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.const 1280
            i32.add
            local.get 9
            i32.load8_u
            local.tee 10
            i32.add
            i32.load8_u
            local.tee 4
            i32.eqz
            br_if 0 (;@4;)
            i32.const 1
            local.get 4
            i32.const -1
            i32.add
            i32.shl
            local.set 4
            i32.const 0
            local.set 3
            loop  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 6
                  i32.const 7
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 5
                  local.get 8
                  i32.add
                  local.get 7
                  i32.store8
                  local.get 8
                  i32.const 1
                  i32.add
                  local.tee 8
                  local.get 1
                  i32.eq
                  br_if 6 (;@1;)
                  local.get 9
                  i32.load8_u
                  local.set 10
                  i32.const 0
                  local.set 7
                  i32.const 0
                  local.set 6
                  br 1 (;@6;)
                end
                local.get 7
                i32.const 1
                i32.shl
                local.set 7
                local.get 6
                i32.const 1
                i32.add
                local.set 6
              end
              local.get 7
              local.get 2
              i32.const 1536
              i32.add
              local.get 10
              i32.const 255
              i32.and
              local.tee 14
              i32.const 2
              i32.shl
              i32.add
              i32.load
              local.get 4
              i32.and
              i32.const 0
              i32.ne
              i32.or
              local.set 7
              local.get 4
              i32.const 1
              i32.shr_u
              local.set 4
              local.get 3
              i32.const 1
              i32.add
              local.tee 3
              local.get 2
              i32.const 1280
              i32.add
              local.get 14
              i32.add
              i32.load8_u
              i32.lt_u
              br_if 0 (;@5;)
            end
          end
          local.get 9
          i32.const 1
          i32.add
          local.set 9
          local.get 11
          i32.const 1
          i32.add
          local.tee 11
          local.get 1
          i32.ne
          br_if 0 (;@3;)
        end
        local.get 7
        i32.const 255
        i32.and
        i32.const 7
        local.get 6
        i32.sub
        i32.shl
        local.set 4
      end
      local.get 5
      local.get 8
      i32.add
      local.get 4
      i32.store8
      local.get 2
      i32.const 256
      i32.add
      i32.const 0
      i32.const 1024
      call 16
      drop
      local.get 2
      local.set 14
      loop  ;; label = @2
        local.get 14
        local.get 13
        i32.store8
        block  ;; label = @3
          local.get 2
          i32.const 1536
          i32.add
          local.get 13
          i32.const 2
          i32.shl
          local.tee 10
          i32.add
          i32.load
          local.tee 7
          local.get 2
          i32.const 1280
          i32.add
          local.get 13
          i32.add
          i32.load8_u
          local.tee 6
          i32.or
          i32.eqz
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 6
              br_if 0 (;@5;)
              i32.const 0
              local.set 3
              br 1 (;@4;)
            end
            i32.const 1
            local.get 6
            i32.const -1
            i32.add
            i32.shl
            local.set 4
            i32.const 0
            local.set 3
            loop  ;; label = @5
              local.get 3
              i32.const 1
              i32.shl
              local.tee 3
              i32.const 2
              i32.add
              local.get 3
              i32.const 1
              i32.or
              local.get 4
              local.get 7
              i32.and
              select
              local.set 3
              local.get 4
              i32.const 1
              i32.shr_u
              local.set 4
              local.get 6
              i32.const -1
              i32.add
              local.tee 6
              br_if 0 (;@5;)
            end
          end
          local.get 2
          i32.const 256
          i32.add
          local.get 10
          i32.add
          local.get 3
          i32.store
        end
        local.get 14
        i32.const 1
        i32.add
        local.set 14
        local.get 13
        i32.const 1
        i32.add
        local.tee 13
        i32.const 256
        i32.ne
        br_if 0 (;@2;)
      end
      i32.const 0
      local.set 14
      local.get 2
      i32.const 256
      i32.add
      local.set 10
      i32.const 1
      local.set 13
      loop  ;; label = @2
        local.get 2
        i32.const 256
        i32.add
        local.get 13
        i32.const 2
        i32.shl
        i32.add
        i32.load
        local.set 7
        local.get 2
        local.get 13
        i32.add
        i32.load8_u
        local.set 8
        local.get 10
        local.set 4
        local.get 14
        local.set 6
        block  ;; label = @3
          block  ;; label = @4
            loop  ;; label = @5
              local.get 4
              i32.load
              local.tee 3
              local.get 7
              i32.le_u
              br_if 1 (;@4;)
              local.get 4
              i32.const 4
              i32.add
              local.get 3
              i32.store
              local.get 2
              local.get 6
              i32.add
              local.tee 3
              i32.const 1
              i32.add
              local.get 3
              i32.load8_u
              i32.store8
              local.get 4
              i32.const -4
              i32.add
              local.set 4
              local.get 6
              i32.const -1
              i32.add
              local.tee 6
              i32.const -1
              i32.ne
              br_if 0 (;@5;)
            end
            i32.const 0
            local.set 4
            br 1 (;@3;)
          end
          local.get 6
          i32.const 1
          i32.add
          local.set 4
        end
        local.get 2
        local.get 4
        i32.add
        local.get 8
        i32.store8
        local.get 2
        i32.const 256
        i32.add
        local.get 4
        i32.const 2
        i32.shl
        i32.add
        local.get 7
        i32.store
        local.get 10
        i32.const 4
        i32.add
        local.set 10
        local.get 14
        i32.const 1
        i32.add
        local.set 14
        local.get 13
        i32.const 1
        i32.add
        local.tee 13
        i32.const 256
        i32.ne
        br_if 0 (;@2;)
      end
      i32.const -1
      local.set 14
      local.get 2
      i32.const 256
      i32.add
      local.set 4
      loop  ;; label = @2
        local.get 14
        i32.const 1
        i32.add
        local.set 14
        local.get 4
        i32.load
        local.set 6
        local.get 4
        i32.const 4
        i32.add
        local.set 4
        local.get 6
        i32.eqz
        br_if 0 (;@2;)
      end
      block  ;; label = @2
        local.get 1
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        local.set 8
        local.get 14
        local.set 4
        local.get 5
        local.set 10
        i32.const 128
        local.set 13
        i32.const 0
        local.set 7
        loop  ;; label = @3
          local.get 7
          i32.const 1
          i32.shl
          local.tee 6
          i32.const 2
          i32.add
          local.get 6
          i32.const 1
          i32.or
          local.get 13
          local.get 10
          i32.load8_u
          i32.and
          select
          local.set 7
          local.get 4
          i32.const -1
          i32.add
          local.set 6
          local.get 2
          i32.const 256
          i32.add
          local.get 4
          i32.const 2
          i32.shl
          i32.add
          local.set 4
          loop  ;; label = @4
            local.get 6
            i32.const 1
            i32.add
            local.set 6
            local.get 4
            i32.load
            local.set 3
            local.get 4
            i32.const 4
            i32.add
            local.set 4
            local.get 3
            local.get 7
            i32.lt_u
            br_if 0 (;@4;)
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 7
              local.get 3
              i32.eq
              br_if 0 (;@5;)
              local.get 6
              local.set 4
              br 1 (;@4;)
            end
            local.get 0
            local.get 2
            local.get 6
            i32.add
            i32.load8_u
            i32.store8
            local.get 8
            i32.const 1
            i32.add
            local.set 8
            local.get 0
            i32.const 1
            i32.add
            local.set 0
            i32.const 0
            local.set 7
            local.get 14
            local.set 4
          end
          i32.const 128
          local.get 13
          i32.const 1
          i32.shr_u
          local.get 13
          i32.const 2
          i32.lt_u
          local.tee 6
          select
          local.set 13
          local.get 10
          local.get 6
          i32.add
          local.set 10
          local.get 8
          local.get 1
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 5
      call 10
    end
    local.get 2
    i32.const 7680
    i32.add
    global.set 0)
  (func (;3;) (type 2) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    local.get 1
    i32.const -4
    i32.add
    local.tee 4
    local.get 3
    i32.const 2
    i32.shl
    i32.add
    i32.load
    local.set 5
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const 2
        i32.div_s
        local.tee 6
        local.get 3
        i32.lt_s
        br_if 0 (;@2;)
        local.get 0
        local.get 5
        i32.const 2
        i32.shl
        i32.add
        local.set 7
        loop  ;; label = @3
          block  ;; label = @4
            local.get 3
            i32.const 1
            i32.shl
            local.tee 8
            local.get 2
            i32.ge_s
            br_if 0 (;@4;)
            local.get 8
            local.get 0
            local.get 4
            local.get 8
            i32.const 2
            i32.shl
            local.tee 9
            i32.add
            i32.load
            i32.const 2
            i32.shl
            i32.add
            i32.load
            local.get 0
            local.get 1
            local.get 9
            i32.add
            i32.load
            i32.const 2
            i32.shl
            i32.add
            i32.load
            i32.gt_u
            i32.or
            local.set 8
          end
          local.get 7
          i32.load
          local.get 0
          local.get 4
          local.get 8
          i32.const 2
          i32.shl
          i32.add
          i32.load
          local.tee 9
          i32.const 2
          i32.shl
          i32.add
          i32.load
          i32.lt_u
          br_if 1 (;@2;)
          local.get 4
          local.get 3
          i32.const 2
          i32.shl
          i32.add
          local.get 9
          i32.store
          local.get 8
          local.set 3
          local.get 8
          local.get 6
          i32.le_s
          br_if 0 (;@3;)
          br 2 (;@1;)
        end
      end
      local.get 3
      local.set 8
    end
    local.get 4
    local.get 8
    i32.const 2
    i32.shl
    i32.add
    local.get 5
    i32.store)
  (func (;4;) (type 0))
  (func (;5;) (type 1) (param i32 i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      i32.const 0
      local.set 2
      loop  ;; label = @2
        local.get 1
        local.set 3
        block  ;; label = @3
          local.get 1
          i32.eqz
          br_if 0 (;@3;)
          loop  ;; label = @4
            i32.const 2064
            i32.const 8192
            call 8
            i32.const 1552
            i32.const 1024
            i32.const 500
            call 15
            i32.const 500
            call 2
            local.get 3
            i32.const -1
            i32.add
            local.tee 3
            br_if 0 (;@4;)
          end
        end
        local.get 2
        i32.const 1
        i32.add
        local.tee 2
        local.get 0
        i32.ne
        br_if 0 (;@2;)
      end
    end)
  (func (;6;) (type 3) (result i32)
    i32.const 11
    i32.const 1
    call 5
    i32.const 0)
  (func (;7;) (type 4) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    call 11
    call 4
    call 13
    local.get 2
    call 6
    i32.store offset=12
    call 14
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    i32.const 0)
  (func (;8;) (type 1) (param i32 i32)
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
    i32.store offset=10256
    i32.const 0
    local.get 0
    local.get 1
    i32.add
    i32.store offset=10260
    i32.const 0
    i32.const 0
    i32.store offset=10264)
  (func (;9;) (type 5) (param i32) (result i32)
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
    i32.load offset=10264
    local.get 0
    i32.add
    i32.const 4
    i32.const 0
    i32.load offset=10256
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
    i32.store offset=10264
    block  ;; label = @1
      local.get 0
      local.get 3
      i32.add
      local.tee 0
      i32.const 0
      i32.load offset=10260
      i32.gt_u
      br_if 0 (;@1;)
      i32.const 0
      local.get 0
      i32.store offset=10256
      local.get 2
      local.set 1
    end
    local.get 1)
  (func (;10;) (type 6) (param i32))
  (func (;11;) (type 0))
  (func (;12;) (type 3) (result i32)
    i32.const 0
    i32.const 0
    call 7)
  (func (;13;) (type 0))
  (func (;14;) (type 0))
  (func (;15;) (type 7) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.const 32
          i32.gt_u
          br_if 0 (;@3;)
          local.get 1
          i32.const 3
          i32.and
          i32.eqz
          br_if 1 (;@2;)
          local.get 2
          i32.eqz
          br_if 1 (;@2;)
          local.get 0
          local.get 1
          i32.load8_u
          i32.store8
          local.get 2
          i32.const -1
          i32.add
          local.set 3
          local.get 0
          i32.const 1
          i32.add
          local.set 4
          local.get 1
          i32.const 1
          i32.add
          local.tee 5
          i32.const 3
          i32.and
          i32.eqz
          br_if 2 (;@1;)
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          local.get 1
          i32.load8_u offset=1
          i32.store8 offset=1
          local.get 2
          i32.const -2
          i32.add
          local.set 3
          local.get 0
          i32.const 2
          i32.add
          local.set 4
          local.get 1
          i32.const 2
          i32.add
          local.tee 5
          i32.const 3
          i32.and
          i32.eqz
          br_if 2 (;@1;)
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          local.get 1
          i32.load8_u offset=2
          i32.store8 offset=2
          local.get 2
          i32.const -3
          i32.add
          local.set 3
          local.get 0
          i32.const 3
          i32.add
          local.set 4
          local.get 1
          i32.const 3
          i32.add
          local.tee 5
          i32.const 3
          i32.and
          i32.eqz
          br_if 2 (;@1;)
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          local.get 1
          i32.load8_u offset=3
          i32.store8 offset=3
          local.get 2
          i32.const -4
          i32.add
          local.set 3
          local.get 0
          i32.const 4
          i32.add
          local.set 4
          local.get 1
          i32.const 4
          i32.add
          local.set 5
          br 2 (;@1;)
        end
        local.get 0
        local.get 1
        local.get 2
        memory.copy
        local.get 0
        return
      end
      local.get 2
      local.set 3
      local.get 0
      local.set 4
      local.get 1
      local.set 5
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 4
        i32.const 3
        i32.and
        local.tee 2
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 3
            i32.const 16
            i32.ge_u
            br_if 0 (;@4;)
            local.get 3
            local.set 2
            br 1 (;@3;)
          end
          block  ;; label = @4
            local.get 3
            i32.const -16
            i32.add
            local.tee 2
            i32.const 16
            i32.and
            br_if 0 (;@4;)
            local.get 4
            local.get 5
            i64.load align=4
            i64.store align=4
            local.get 4
            local.get 5
            i64.load offset=8 align=4
            i64.store offset=8 align=4
            local.get 4
            i32.const 16
            i32.add
            local.set 4
            local.get 5
            i32.const 16
            i32.add
            local.set 5
            local.get 2
            local.set 3
          end
          local.get 2
          i32.const 16
          i32.lt_u
          br_if 0 (;@3;)
          local.get 3
          local.set 2
          loop  ;; label = @4
            local.get 4
            local.get 5
            i64.load align=4
            i64.store align=4
            local.get 4
            local.get 5
            i64.load offset=8 align=4
            i64.store offset=8 align=4
            local.get 4
            local.get 5
            i64.load offset=16 align=4
            i64.store offset=16 align=4
            local.get 4
            local.get 5
            i64.load offset=24 align=4
            i64.store offset=24 align=4
            local.get 4
            i32.const 32
            i32.add
            local.set 4
            local.get 5
            i32.const 32
            i32.add
            local.set 5
            local.get 2
            i32.const -32
            i32.add
            local.tee 2
            i32.const 15
            i32.gt_u
            br_if 0 (;@4;)
          end
        end
        block  ;; label = @3
          local.get 2
          i32.const 8
          i32.lt_u
          br_if 0 (;@3;)
          local.get 4
          local.get 5
          i64.load align=4
          i64.store align=4
          local.get 5
          i32.const 8
          i32.add
          local.set 5
          local.get 4
          i32.const 8
          i32.add
          local.set 4
        end
        block  ;; label = @3
          local.get 2
          i32.const 4
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.get 5
          i32.load
          i32.store
          local.get 5
          i32.const 4
          i32.add
          local.set 5
          local.get 4
          i32.const 4
          i32.add
          local.set 4
        end
        block  ;; label = @3
          local.get 2
          i32.const 2
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.get 5
          i32.load16_u align=1
          i32.store16 align=1
          local.get 4
          i32.const 2
          i32.add
          local.set 4
          local.get 5
          i32.const 2
          i32.add
          local.set 5
        end
        local.get 2
        i32.const 1
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 4
        local.get 5
        i32.load8_u
        i32.store8
        local.get 0
        return
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 3
                i32.const 32
                i32.lt_u
                br_if 0 (;@6;)
                local.get 4
                local.get 5
                i32.load
                local.tee 3
                i32.store8
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 2
                    i32.const -1
                    i32.add
                    br_table 3 (;@5;) 0 (;@8;) 1 (;@7;) 3 (;@5;)
                  end
                  local.get 4
                  local.get 3
                  i32.const 8
                  i32.shr_u
                  i32.store8 offset=1
                  local.get 4
                  local.get 5
                  i32.const 6
                  i32.add
                  i64.load align=2
                  i64.store offset=6 align=4
                  local.get 4
                  local.get 5
                  i32.load offset=4
                  i32.const 16
                  i32.shl
                  local.get 3
                  i32.const 16
                  i32.shr_u
                  i32.or
                  i32.store offset=2
                  local.get 4
                  i32.const 18
                  i32.add
                  local.set 2
                  local.get 5
                  i32.const 18
                  i32.add
                  local.set 1
                  i32.const 14
                  local.set 6
                  local.get 5
                  i32.const 14
                  i32.add
                  i32.load align=2
                  local.set 5
                  i32.const 14
                  local.set 3
                  br 3 (;@4;)
                end
                local.get 4
                local.get 5
                i32.const 5
                i32.add
                i64.load align=1
                i64.store offset=5 align=4
                local.get 4
                local.get 5
                i32.load offset=4
                i32.const 24
                i32.shl
                local.get 3
                i32.const 8
                i32.shr_u
                i32.or
                i32.store offset=1
                local.get 4
                i32.const 17
                i32.add
                local.set 2
                local.get 5
                i32.const 17
                i32.add
                local.set 1
                i32.const 13
                local.set 6
                local.get 5
                i32.const 13
                i32.add
                i32.load align=1
                local.set 5
                i32.const 15
                local.set 3
                br 2 (;@4;)
              end
              block  ;; label = @6
                block  ;; label = @7
                  local.get 3
                  i32.const 16
                  i32.ge_u
                  br_if 0 (;@7;)
                  local.get 4
                  local.set 2
                  local.get 5
                  local.set 1
                  br 1 (;@6;)
                end
                local.get 4
                local.get 5
                i32.load8_u
                i32.store8
                local.get 4
                local.get 5
                i32.load offset=1 align=1
                i32.store offset=1 align=1
                local.get 4
                local.get 5
                i64.load offset=5 align=1
                i64.store offset=5 align=1
                local.get 4
                local.get 5
                i32.load16_u offset=13 align=1
                i32.store16 offset=13 align=1
                local.get 4
                local.get 5
                i32.load8_u offset=15
                i32.store8 offset=15
                local.get 4
                i32.const 16
                i32.add
                local.set 2
                local.get 5
                i32.const 16
                i32.add
                local.set 1
              end
              local.get 3
              i32.const 8
              i32.and
              br_if 2 (;@3;)
              br 3 (;@2;)
            end
            local.get 4
            local.get 3
            i32.const 16
            i32.shr_u
            i32.store8 offset=2
            local.get 4
            local.get 3
            i32.const 8
            i32.shr_u
            i32.store8 offset=1
            local.get 4
            local.get 5
            i32.const 7
            i32.add
            i64.load align=1
            i64.store offset=7 align=4
            local.get 4
            local.get 5
            i32.load offset=4
            i32.const 8
            i32.shl
            local.get 3
            i32.const 24
            i32.shr_u
            i32.or
            i32.store offset=3
            local.get 4
            i32.const 19
            i32.add
            local.set 2
            local.get 5
            i32.const 19
            i32.add
            local.set 1
            i32.const 15
            local.set 6
            local.get 5
            i32.const 15
            i32.add
            i32.load align=1
            local.set 5
            i32.const 13
            local.set 3
          end
          local.get 4
          local.get 6
          i32.add
          local.get 5
          i32.store
        end
        local.get 2
        local.get 1
        i64.load align=1
        i64.store align=1
        local.get 2
        i32.const 8
        i32.add
        local.set 2
        local.get 1
        i32.const 8
        i32.add
        local.set 1
      end
      block  ;; label = @2
        local.get 3
        i32.const 4
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 1
        i32.load align=1
        i32.store align=1
        local.get 2
        i32.const 4
        i32.add
        local.set 2
        local.get 1
        i32.const 4
        i32.add
        local.set 1
      end
      block  ;; label = @2
        local.get 3
        i32.const 2
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 1
        i32.load16_u align=1
        i32.store16 align=1
        local.get 2
        i32.const 2
        i32.add
        local.set 2
        local.get 1
        i32.const 2
        i32.add
        local.set 1
      end
      local.get 3
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 1
      i32.load8_u
      i32.store8
    end
    local.get 0)
  (func (;16;) (type 7) (param i32 i32 i32) (result i32)
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
  (global (;0;) (mut i32) (i32.const 26272))
  (global (;1;) i32 (i32.const 0))
  (global (;2;) i32 (i32.const 10268))
  (global (;3;) i32 (i32.const 26272))
  (export "memory" (memory 0))
  (export "_initialize" (func 1))
  (export "_run" (func 12))
  (export "__data_end" (global 2))
  (export "__heap_base" (global 3))
  (data (;0;) (i32.const 1024) "J2OZF50FYLD5UTVYYRMT0VXO01VC5FNIB1CG12MTIPT2CIV00BOUWFDRAYTA3AI42KFXHRKPA3LCGA3ABLUYQXJRQ2RN2ZMYERPLC00CXFE3GB3HMS53JIOZE5HBYTZ2EJHGDBI0HMYNOVU0HUXR2FKBERC3E1ZIEBOHCWCJD0WRPLLX5DI1IS2NE4KI0DR4E5GHWIQZCHKRSVIRYQMBDJOHHYPB1AAAAGHWOXPQ4ZBQOKBH0OI3XWE4OUAJUAJUGQKUIZEGSFXBPYIKGQH3GM2UA23U2HJCXTW5N0G553APVIZ2YAZ4MVSMRQBNXKPO3FOK5UK5RKOGTHCLH2KUR2ADMBQDLASJFATFU3EFISL1ZOGAKQU1NV4ZWP3CPPLUP4ZD23IEPT5IBFJLW3HDSF2JUZLDIWYXUR0QPCU4WTHXZQDPNKSAPOJEIUHQK5I4RCPAFD41XFSQVV5D5RDP5MTHA0YK0AILCXLH1JCSPVCEKBHKSKZR"))

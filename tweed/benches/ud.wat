(module
  (type (;0;) (func))
  (type (;1;) (func (param i32 i32) (result i32)))
  (type (;2;) (func (result i32)))
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
  (func (;2;) (type 0))
  (func (;3;) (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      i32.const 0
      local.set 2
      loop  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          local.set 3
          loop  ;; label = @4
            i32.const 1120
            local.set 4
            i32.const 0
            local.set 5
            loop  ;; label = @5
              local.get 5
              i32.const 1
              i32.add
              local.set 6
              i32.const 0
              local.set 7
              local.get 4
              local.set 8
              i32.const 0
              local.set 9
              loop  ;; label = @6
                local.get 8
                local.get 5
                local.get 9
                i32.add
                i32.const 2
                i32.add
                local.get 5
                local.get 9
                i32.eq
                i32.shl
                local.tee 10
                i32.store
                local.get 10
                local.get 7
                i32.add
                local.set 7
                local.get 8
                i32.const 4
                i32.add
                local.set 8
                local.get 9
                i32.const 1
                i32.add
                local.tee 9
                i32.const 6
                i32.ne
                br_if 0 (;@6;)
              end
              local.get 5
              i32.const 2
              i32.shl
              i32.const 2720
              i32.add
              local.get 7
              i32.store
              local.get 4
              i32.const 80
              i32.add
              local.set 4
              local.get 6
              local.set 5
              local.get 6
              i32.const 6
              i32.ne
              br_if 0 (;@5;)
            end
            local.get 9
            i32.const 5
            call 4
            drop
            i32.const 0
            i32.const 0
            i32.store offset=2800
            local.get 3
            i32.const 1
            i32.add
            local.tee 3
            local.get 1
            i32.ne
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
    end
    i32.const 0
    i32.load offset=2800)
  (func (;4;) (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 400
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.const 1
          i32.lt_s
          br_if 0 (;@3;)
          i32.const 0
          local.set 3
          i32.const 1200
          local.set 4
          i32.const 1124
          local.set 5
          i32.const 1120
          local.set 6
          i32.const 1
          local.set 7
          br 1 (;@2;)
        end
        local.get 2
        i32.const 0
        i32.load offset=2720
        i32.store
        br 1 (;@1;)
      end
      loop  ;; label = @2
        local.get 3
        local.tee 8
        i32.const 80
        i32.mul
        local.get 8
        i32.const 2
        i32.shl
        local.tee 9
        i32.add
        i32.const 1120
        i32.add
        local.set 10
        local.get 4
        local.set 11
        local.get 8
        i32.const 1
        i32.add
        local.tee 3
        local.set 12
        loop  ;; label = @3
          local.get 12
          i32.const 80
          i32.mul
          local.get 9
          i32.add
          i32.const 1120
          i32.add
          local.tee 13
          i32.load
          local.set 14
          block  ;; label = @4
            local.get 8
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            local.set 15
            local.get 11
            local.set 16
            local.get 8
            local.set 17
            loop  ;; label = @5
              local.get 14
              local.get 15
              i32.load
              local.get 16
              i32.load
              i32.mul
              i32.sub
              local.set 14
              local.get 15
              i32.const 80
              i32.add
              local.set 15
              local.get 16
              i32.const 4
              i32.add
              local.set 16
              local.get 17
              i32.const -1
              i32.add
              local.tee 17
              br_if 0 (;@5;)
            end
          end
          local.get 13
          local.get 14
          local.get 10
          i32.load
          i32.div_s
          i32.store
          local.get 11
          i32.const 80
          i32.add
          local.set 11
          local.get 12
          local.get 1
          i32.ne
          local.set 15
          local.get 12
          i32.const 1
          i32.add
          local.set 12
          local.get 15
          br_if 0 (;@3;)
        end
        local.get 5
        local.set 11
        local.get 3
        local.set 12
        loop  ;; label = @3
          local.get 3
          i32.const 80
          i32.mul
          local.get 12
          i32.const 2
          i32.shl
          i32.add
          i32.const 1120
          i32.add
          local.tee 13
          i32.load
          local.set 14
          local.get 11
          local.set 15
          local.get 4
          local.set 16
          local.get 7
          local.set 17
          loop  ;; label = @4
            local.get 14
            local.get 15
            i32.load
            local.get 16
            i32.load
            i32.mul
            i32.sub
            local.set 14
            local.get 15
            i32.const 80
            i32.add
            local.set 15
            local.get 16
            i32.const 4
            i32.add
            local.set 16
            local.get 17
            i32.const -1
            i32.add
            local.tee 17
            br_if 0 (;@4;)
          end
          local.get 13
          local.get 14
          i32.store
          local.get 11
          i32.const 4
          i32.add
          local.set 11
          local.get 12
          local.get 1
          i32.eq
          local.set 15
          local.get 12
          i32.const 1
          i32.add
          local.set 12
          local.get 15
          i32.eqz
          br_if 0 (;@3;)
        end
        local.get 5
        i32.const 4
        i32.add
        local.set 5
        local.get 6
        i32.const 4
        i32.add
        local.set 6
        local.get 4
        i32.const 80
        i32.add
        local.set 4
        local.get 7
        i32.const 1
        i32.add
        local.set 7
        local.get 3
        local.get 1
        i32.ne
        br_if 0 (;@2;)
      end
      local.get 2
      i32.const 0
      i32.load offset=2720
      i32.store
      i32.const 1200
      local.set 11
      i32.const 1
      local.set 12
      loop  ;; label = @2
        local.get 12
        i32.const 2
        i32.shl
        local.tee 13
        i32.const 2720
        i32.add
        i32.load
        local.set 14
        local.get 2
        local.set 15
        local.get 11
        local.set 16
        local.get 12
        local.set 17
        loop  ;; label = @3
          local.get 14
          local.get 15
          i32.load
          local.get 16
          i32.load
          i32.mul
          i32.sub
          local.set 14
          local.get 16
          i32.const 4
          i32.add
          local.set 16
          local.get 15
          i32.const 4
          i32.add
          local.set 15
          local.get 17
          i32.const -1
          i32.add
          local.tee 17
          br_if 0 (;@3;)
        end
        local.get 2
        local.get 13
        i32.add
        local.get 14
        i32.store
        local.get 11
        i32.const 80
        i32.add
        local.set 11
        local.get 12
        local.get 1
        i32.ne
        local.set 15
        local.get 12
        i32.const 1
        i32.add
        local.set 12
        local.get 15
        br_if 0 (;@2;)
      end
    end
    local.get 1
    i32.const 2
    i32.shl
    local.tee 15
    i32.const 1040
    i32.add
    local.tee 8
    local.get 2
    local.get 15
    i32.add
    i32.load
    local.get 1
    i32.const 80
    i32.mul
    local.get 15
    i32.add
    i32.const 1120
    i32.add
    i32.load
    i32.div_s
    i32.store
    block  ;; label = @1
      local.get 1
      i32.const 1
      i32.lt_s
      br_if 0 (;@1;)
      local.get 1
      i32.const 84
      i32.mul
      i32.const 1040
      i32.add
      local.set 3
      i32.const 1
      local.set 4
      local.get 1
      local.set 12
      loop  ;; label = @2
        local.get 2
        local.get 12
        i32.const -1
        i32.add
        local.tee 11
        i32.const 2
        i32.shl
        local.tee 13
        i32.add
        i32.load
        local.set 14
        block  ;; label = @3
          local.get 12
          local.get 1
          i32.gt_s
          br_if 0 (;@3;)
          local.get 3
          local.set 15
          local.get 8
          local.set 16
          local.get 4
          local.set 17
          loop  ;; label = @4
            local.get 14
            local.get 16
            i32.load
            local.get 15
            i32.load
            i32.mul
            i32.sub
            local.set 14
            local.get 15
            i32.const 4
            i32.add
            local.set 15
            local.get 16
            i32.const 4
            i32.add
            local.set 16
            local.get 17
            i32.const -1
            i32.add
            local.tee 17
            br_if 0 (;@4;)
          end
        end
        local.get 13
        i32.const 1040
        i32.add
        local.get 14
        local.get 11
        i32.const 80
        i32.mul
        local.get 13
        i32.add
        i32.const 1120
        i32.add
        i32.load
        i32.div_s
        i32.store
        local.get 3
        i32.const -84
        i32.add
        local.set 3
        local.get 8
        i32.const -4
        i32.add
        local.set 8
        local.get 4
        i32.const 1
        i32.add
        local.set 4
        local.get 12
        i32.const 1
        i32.gt_s
        local.set 15
        local.get 11
        local.set 12
        local.get 15
        br_if 0 (;@2;)
      end
    end
    local.get 2
    i32.const 400
    i32.add
    global.set 0
    i32.const 0)
  (func (;5;) (type 2) (result i32)
    i32.const 1785
    i32.const 1
    call 3)
  (func (;6;) (type 1) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    call 7
    call 2
    call 9
    local.get 2
    call 5
    i32.store offset=12
    call 10
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    i32.const 0)
  (func (;7;) (type 0))
  (func (;8;) (type 2) (result i32)
    i32.const 0
    i32.const 0
    call 6)
  (func (;9;) (type 0))
  (func (;10;) (type 0))
  (table (;0;) 1 1 funcref)
  (memory (;0;) 1)
  (global (;0;) (mut i32) (i32.const 18816))
  (global (;1;) i32 (i32.const 0))
  (global (;2;) i32 (i32.const 2804))
  (global (;3;) i32 (i32.const 18816))
  (export "memory" (memory 0))
  (export "_initialize" (func 1))
  (export "_run" (func 8))
  (export "__data_end" (global 2))
  (export "__heap_base" (global 3)))

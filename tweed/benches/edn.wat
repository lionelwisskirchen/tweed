(module
  (type (;0;) (func))
  (type (;1;) (func (param i32 i32 i32)))
  (type (;2;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;3;) (func (param i32 i32 i32 i32)))
  (type (;4;) (func (param i32 i32)))
  (type (;5;) (func (result i32)))
  (type (;6;) (func (param i32 i32) (result i32)))
  (type (;7;) (func (param i32 i32 i32) (result i32)))
  (func (;0;) (type 0))
  (func (;1;) (type 0)
    block  ;; label = @1
      global.get 1
      i32.const 1824
      i32.add
      i32.load
      i32.eqz
      br_if 0 (;@1;)
      unreachable
    end
    global.get 1
    i32.const 1824
    i32.add
    i32.const 1
    i32.store
    call 0)
  (func (;2;) (type 1) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    local.get 0
    i32.const 4
    i32.add
    local.set 3
    i32.const 0
    local.set 4
    loop  ;; label = @1
      local.get 0
      local.get 4
      i32.const 1
      i32.shl
      i32.add
      i32.load16_u
      local.set 5
      local.get 3
      local.set 6
      local.get 1
      local.set 7
      i32.const -2
      local.set 8
      i32.const 0
      local.set 9
      i32.const 0
      local.set 10
      loop  ;; label = @2
        local.get 7
        i32.load16_s
        local.tee 11
        local.get 5
        i32.extend16_s
        i32.mul
        local.get 9
        i32.add
        local.get 7
        i32.const 2
        i32.add
        i32.load16_s
        local.tee 12
        local.get 6
        i32.const -2
        i32.add
        i32.load16_s
        local.tee 5
        i32.mul
        i32.add
        local.set 9
        local.get 11
        local.get 5
        i32.mul
        local.get 10
        i32.add
        local.get 12
        local.get 6
        i32.load16_s
        local.tee 5
        i32.mul
        i32.add
        local.set 10
        local.get 6
        i32.const 4
        i32.add
        local.set 6
        local.get 7
        i32.const 4
        i32.add
        local.set 7
        local.get 8
        i32.const 2
        i32.add
        local.tee 8
        i32.const 30
        i32.lt_u
        br_if 0 (;@2;)
      end
      local.get 2
      local.get 4
      i32.const 2
      i32.shl
      i32.add
      local.tee 6
      local.get 9
      i32.const 15
      i32.shr_s
      i32.store
      local.get 6
      i32.const 4
      i32.add
      local.get 10
      i32.const 15
      i32.shr_s
      i32.store
      local.get 3
      i32.const 4
      i32.add
      local.set 3
      local.get 4
      i32.const 98
      i32.lt_u
      local.set 6
      local.get 4
      i32.const 2
      i32.add
      local.set 4
      local.get 6
      br_if 0 (;@1;)
    end)
  (func (;3;) (type 2) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32)
    local.get 3
    local.get 1
    local.get 2
    i32.const 1
    i32.shl
    local.tee 4
    i32.const -2
    i32.add
    local.tee 5
    i32.add
    i32.load16_s
    local.get 0
    local.get 5
    i32.add
    i32.load16_s
    i32.mul
    i32.sub
    local.set 3
    block  ;; label = @1
      local.get 2
      i32.const 2
      i32.lt_s
      br_if 0 (;@1;)
      local.get 2
      i32.const -1
      i32.add
      local.set 5
      local.get 0
      local.get 4
      i32.const -4
      i32.add
      local.tee 4
      i32.add
      local.set 2
      local.get 1
      local.get 4
      i32.add
      local.set 1
      loop  ;; label = @2
        local.get 2
        i32.const 2
        i32.add
        local.get 2
        i32.load16_s
        local.tee 4
        local.get 3
        local.get 4
        local.get 1
        i32.load16_s
        local.tee 6
        i32.mul
        i32.sub
        local.tee 3
        i32.const 16
        i32.shr_s
        local.get 6
        i32.mul
        i32.const 16
        i32.shr_u
        i32.add
        i32.store16
        local.get 2
        i32.const -2
        i32.add
        local.set 2
        local.get 1
        i32.const -2
        i32.add
        local.set 1
        local.get 5
        i32.const -1
        i32.add
        local.tee 5
        br_if 0 (;@2;)
      end
    end
    local.get 0
    local.get 3
    i32.const 16
    i32.shr_u
    i32.store16
    local.get 3)
  (func (;4;) (type 3) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32)
    local.get 1
    i32.load16_s
    local.set 4
    i32.const 0
    local.set 1
    loop  ;; label = @1
      local.get 3
      local.get 1
      i32.add
      local.tee 5
      i32.const 4
      i32.add
      local.tee 6
      i32.load
      local.set 7
      local.get 6
      local.get 5
      i32.load
      local.tee 8
      i32.store
      local.get 5
      local.get 7
      local.get 0
      local.get 1
      i32.add
      local.tee 6
      i32.const 6
      i32.add
      i32.load16_s
      i32.mul
      local.get 8
      local.get 6
      i32.const 4
      i32.add
      i32.load16_s
      i32.mul
      i32.add
      i32.const 15
      i32.shr_s
      local.get 4
      i32.add
      local.tee 4
      i32.store
      local.get 7
      local.get 6
      i32.const 2
      i32.add
      i32.load16_s
      i32.mul
      local.get 8
      local.get 6
      i32.load16_s
      i32.mul
      i32.add
      i32.const 15
      i32.shr_s
      local.get 4
      i32.add
      local.set 4
      local.get 1
      i32.const 8
      i32.add
      local.tee 1
      i32.const 400
      i32.ne
      br_if 0 (;@1;)
    end
    local.get 2
    local.get 4
    i32.store)
  (func (;5;) (type 4) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 2
    i32.const 28
    i32.add
    local.set 3
    i32.const 13
    local.set 4
    i32.const 0
    local.set 5
    i32.const -2
    local.set 6
    i32.const 14
    local.set 7
    i32.const 2
    local.set 8
    i32.const 16
    local.set 9
    i32.const 1
    local.set 10
    loop  ;; label = @1
      local.get 0
      local.get 7
      i32.add
      local.set 11
      local.get 10
      i32.const 6
      i32.mul
      local.set 12
      local.get 10
      i32.const 10
      i32.mul
      local.set 13
      local.get 10
      i32.const 14
      i32.mul
      local.set 14
      local.get 10
      i32.const 12
      i32.mul
      local.set 15
      local.get 10
      i32.const 2
      i32.shl
      local.set 16
      local.get 10
      i32.const 3
      i32.shl
      local.set 17
      i32.const 0
      local.set 18
      loop  ;; label = @2
        i32.const 0
        local.set 19
        local.get 11
        local.set 20
        local.get 3
        local.set 21
        local.get 0
        local.set 22
        loop  ;; label = @3
          local.get 2
          local.get 19
          i32.add
          local.get 20
          i32.load16_s
          local.tee 23
          local.get 22
          i32.load16_s
          local.tee 24
          i32.add
          i32.store
          local.get 21
          local.get 24
          local.get 23
          i32.sub
          i32.store
          local.get 20
          local.get 6
          i32.add
          local.set 20
          local.get 21
          i32.const -4
          i32.add
          local.set 21
          local.get 22
          local.get 8
          i32.add
          local.set 22
          local.get 19
          i32.const 4
          i32.add
          local.tee 19
          i32.const 16
          i32.ne
          br_if 0 (;@3;)
        end
        local.get 0
        local.get 2
        i32.load offset=8
        local.tee 19
        local.get 2
        i32.load offset=4
        local.tee 20
        i32.add
        local.tee 21
        local.get 2
        i32.load offset=12
        local.tee 22
        local.get 2
        i32.load
        local.tee 23
        i32.add
        local.tee 24
        i32.add
        local.get 5
        i32.shr_s
        i32.store16
        local.get 0
        local.get 17
        i32.add
        local.get 24
        local.get 21
        i32.sub
        local.get 5
        i32.shr_s
        i32.store16
        local.get 0
        local.get 16
        i32.add
        local.get 23
        local.get 22
        i32.sub
        local.tee 21
        local.get 1
        i32.load16_s offset=18
        i32.mul
        local.get 4
        i32.shr_s
        local.get 20
        local.get 19
        i32.sub
        local.tee 19
        local.get 21
        i32.add
        i32.extend16_s
        local.get 1
        i32.load16_s offset=20
        i32.mul
        local.tee 20
        i32.add
        i32.store16
        local.get 0
        local.get 15
        i32.add
        local.get 19
        local.get 1
        i32.load16_s offset=22
        i32.mul
        local.get 4
        i32.shr_s
        local.get 20
        i32.add
        i32.store16
        local.get 2
        i32.load offset=20
        local.set 19
        local.get 2
        i32.load offset=24
        local.set 20
        local.get 1
        i32.load16_s offset=16
        local.set 25
        local.get 1
        i32.load16_s offset=2
        local.set 26
        local.get 2
        local.get 2
        i32.load offset=28
        local.tee 21
        local.get 2
        i32.load offset=16
        local.tee 22
        i32.add
        i32.extend16_s
        local.get 1
        i32.load16_s offset=4
        i32.mul
        local.tee 23
        i32.store
        local.get 2
        local.get 20
        local.get 19
        i32.add
        i32.extend16_s
        local.get 1
        i32.load16_s
        i32.mul
        local.tee 24
        i32.store offset=4
        local.get 2
        local.get 26
        local.get 20
        local.get 22
        i32.add
        local.tee 27
        i32.extend16_s
        i32.mul
        local.get 25
        local.get 27
        local.get 19
        local.get 21
        i32.add
        local.tee 26
        i32.add
        i32.extend16_s
        i32.mul
        local.tee 27
        i32.add
        local.tee 25
        i32.store offset=8
        local.get 1
        i32.load16_s offset=6
        local.set 28
        local.get 0
        local.get 14
        i32.add
        local.get 25
        local.get 23
        i32.add
        local.get 22
        local.get 1
        i32.load16_u offset=8
        i32.mul
        i32.add
        i32.extend16_s
        local.get 4
        i32.shr_s
        i32.store16
        local.get 2
        local.get 28
        local.get 26
        i32.extend16_s
        i32.mul
        local.get 27
        i32.add
        local.tee 22
        i32.store offset=12
        local.get 0
        local.get 13
        i32.add
        local.get 22
        local.get 24
        i32.add
        local.get 19
        local.get 1
        i32.load16_u offset=12
        i32.mul
        i32.add
        i32.extend16_s
        local.get 4
        i32.shr_s
        i32.store16
        local.get 0
        local.get 12
        i32.add
        local.get 25
        local.get 24
        i32.add
        local.get 20
        local.get 1
        i32.load16_u offset=10
        i32.mul
        i32.add
        i32.extend16_s
        local.get 4
        i32.shr_s
        i32.store16
        local.get 0
        local.get 10
        i32.const 1
        i32.shl
        i32.add
        local.get 22
        local.get 23
        i32.add
        local.get 21
        local.get 1
        i32.load16_u offset=14
        i32.mul
        i32.add
        i32.extend16_s
        local.get 4
        i32.shr_s
        i32.store16
        local.get 11
        local.get 9
        i32.add
        local.set 11
        local.get 0
        local.get 9
        i32.add
        local.set 0
        local.get 18
        i32.const 65535
        i32.and
        local.set 19
        local.get 18
        i32.const 1
        i32.add
        local.set 18
        local.get 19
        i32.const 7
        i32.lt_u
        br_if 0 (;@2;)
      end
      local.get 6
      i32.const -14
      i32.add
      local.set 6
      local.get 7
      i32.const 98
      i32.add
      local.set 7
      local.get 8
      i32.const 14
      i32.add
      local.set 8
      local.get 9
      i32.const -14
      i32.add
      local.set 9
      local.get 0
      i32.const -128
      i32.add
      local.set 0
      local.get 4
      i32.const 3
      i32.add
      local.set 4
      local.get 5
      i32.const 3
      i32.add
      local.set 5
      local.get 10
      i32.const 2
      i32.lt_u
      local.set 19
      local.get 10
      i32.const 7
      i32.add
      local.set 10
      local.get 19
      br_if 0 (;@1;)
    end)
  (func (;6;) (type 0))
  (func (;7;) (type 4) (param i32 i32)
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
            i32.const 2656
            i32.const 1024
            i32.const 400
            call 14
            local.set 4
            i32.const 3056
            i32.const 1424
            i32.const 400
            call 14
            local.set 5
            i32.const -300
            local.set 6
            loop  ;; label = @5
              local.get 6
              i32.const 2956
              i32.add
              local.tee 7
              local.get 7
              i32.load16_u
              local.get 6
              i32.const 3356
              i32.add
              i32.load16_s
              i32.const 3
              i32.mul
              i32.const 15
              i32.shr_u
              i32.add
              i32.store16
              local.get 6
              i32.const 2
              i32.add
              local.tee 6
              br_if 0 (;@5;)
            end
            i32.const 0
            i32.load offset=1840
            local.set 8
            i32.const 3
            local.set 9
            i32.const -300
            local.set 6
            loop  ;; label = @5
              local.get 6
              i32.const 2956
              i32.add
              i32.load16_s
              local.get 6
              i32.const 3356
              i32.add
              i32.load16_s
              local.tee 7
              i32.mul
              local.get 8
              i32.add
              local.set 8
              local.get 7
              local.get 7
              i32.mul
              local.get 9
              i32.add
              local.set 9
              local.get 6
              i32.const 2
              i32.add
              local.tee 6
              br_if 0 (;@5;)
            end
            i32.const 0
            local.set 10
            i32.const 0
            local.get 9
            i32.store16 offset=2640
            i32.const 0
            local.get 8
            i32.store offset=1840
            i32.const 2656
            local.set 8
            loop  ;; label = @5
              i32.const 0
              local.set 6
              i32.const 0
              local.set 7
              loop  ;; label = @6
                local.get 6
                local.get 5
                i32.add
                i32.load16_s
                local.get 8
                local.get 6
                i32.add
                i32.load16_s
                i32.mul
                local.get 7
                i32.add
                local.set 7
                local.get 6
                i32.const 2
                i32.add
                local.tee 6
                i32.const 100
                i32.ne
                br_if 0 (;@6;)
              end
              local.get 10
              i32.const 2
              i32.shl
              i32.const 1840
              i32.add
              local.get 7
              i32.const 15
              i32.shr_s
              i32.store
              local.get 8
              i32.const 2
              i32.add
              local.set 8
              local.get 10
              i32.const 1
              i32.add
              local.tee 10
              i32.const 50
              i32.ne
              br_if 0 (;@5;)
            end
            local.get 4
            local.get 5
            i32.const 1840
            call 2
            i32.const 0
            local.get 4
            local.get 5
            i32.const 100
            i32.const 43690
            call 3
            local.tee 6
            i32.store offset=2644
            local.get 4
            local.get 5
            i32.const 2240
            i32.const 1840
            call 4
            i32.const 0
            local.get 6
            i32.store offset=2648
            local.get 4
            local.get 5
            call 5
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
    end)
  (func (;8;) (type 5) (result i32)
    i32.const 81
    i32.const 1
    call 7
    i32.const 0)
  (func (;9;) (type 6) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    call 10
    call 6
    call 12
    local.get 2
    call 8
    i32.store offset=12
    call 13
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    i32.const 0)
  (func (;10;) (type 0))
  (func (;11;) (type 5) (result i32)
    i32.const 0
    i32.const 0
    call 9)
  (func (;12;) (type 0))
  (func (;13;) (type 0))
  (func (;14;) (type 7) (param i32 i32 i32) (result i32)
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
  (table (;0;) 1 1 funcref)
  (memory (;0;) 1)
  (global (;0;) (mut i32) (i32.const 19456))
  (global (;1;) i32 (i32.const 0))
  (global (;2;) i32 (i32.const 3456))
  (global (;3;) i32 (i32.const 19456))
  (export "memory" (memory 0))
  (export "_initialize" (func 1))
  (export "_run" (func 11))
  (export "__data_end" (global 2))
  (export "__heap_base" (global 3))
  (data (;0;) (i32.const 1024) "\00\00\ff\07\00\0c\00\08\00\02\00\f8\00\f3\00\04\00\00\ff\07\00\0c\00\08\00\02\00\f8\00\f3\00\04\00\00\ff\07\00\0c\00\08\00\02\00\f8\00\f3\00\04\00\00\ff\07\00\0c\00\08\00\02\00\f8\00\f3\00\04\00\00\ff\07\00\0c\00\08\00\02\00\f8\00\f3\00\04\00\00\ff\07\00\0c\00\08\00\02\00\f8\00\f3\00\04\00\00\ff\07\00\0c\00\08\00\02\00\f8\00\f3\00\04\00\00\ff\07\00\0c\00\08\00\02\00\f8\00\f3\00\04\00\00\ff\07\00\0c\00\08\00\02\00\f8\00\f3\00\04\00\00\ff\07\00\0c\00\08\00\02\00\f8\00\f3\00\04\00\00\ff\07\00\0c\00\08\00\02\00\f8\00\f3\00\04\00\00\ff\07\00\0c\00\08\00\02\00\f8\00\f3\00\04\00\00\ff\07\00\0c\00\08\00\02\00\f8\00\f3\00\04\00\00\ff\07\00\0c\00\08\00\02\00\f8\00\f3\00\04\00\00\ff\07\00\0c\00\08\00\02\00\f8\00\f3\00\04\00\00\ff\07\00\0c\00\08\00\02\00\f8\00\f3\00\04\00\00\ff\07\00\0c\00\08\00\02\00\f8\00\f3\00\04\00\00\ff\07\00\0c\00\08\00\02\00\f8\00\f3\00\04\00\00\ff\07\00\0c\00\08\00\02\00\f8\00\f3\00\04\00\00\ff\07\00\0c\00\08\00\02\00\f8\00\f3\00\04\00\00\ff\07\00\0c\00\08\00\02\00\f8\00\f3\00\04\00\00\ff\07\00\0c\00\08\00\02\00\f8\00\f3\00\04\00\00\ff\07\00\0c\00\08\00\02\00\f8\00\f3\00\04\00\00\ff\07\00\0c\00\08\00\02\00\f8\00\f3\00\04\00\00\ff\07\00\0c\00\08\00\02\00\f8\00\f3\00\04`\0c@\0c \0c\00\0c\00\f6\00\f4\00\f2\00\f0`\0c@\0c \0c\00\0c\00\f6\00\f4\00\f2\00\f0`\0c@\0c \0c\00\0c\00\f6\00\f4\00\f2\00\f0`\0c@\0c \0c\00\0c\00\f6\00\f4\00\f2\00\f0`\0c@\0c \0c\00\0c\00\f6\00\f4\00\f2\00\f0`\0c@\0c \0c\00\0c\00\f6\00\f4\00\f2\00\f0`\0c@\0c \0c\00\0c\00\f6\00\f4\00\f2\00\f0`\0c@\0c \0c\00\0c\00\f6\00\f4\00\f2\00\f0`\0c@\0c \0c\00\0c\00\f6\00\f4\00\f2\00\f0`\0c@\0c \0c\00\0c\00\f6\00\f4\00\f2\00\f0`\0c@\0c \0c\00\0c\00\f6\00\f4\00\f2\00\f0`\0c@\0c \0c\00\0c\00\f6\00\f4\00\f2\00\f0`\0c@\0c \0c\00\0c\00\f6\00\f4\00\f2\00\f0`\0c@\0c \0c\00\0c\00\f6\00\f4\00\f2\00\f0`\0c@\0c \0c\00\0c\00\f6\00\f4\00\f2\00\f0`\0c@\0c \0c\00\0c\00\f6\00\f4\00\f2\00\f0`\0c@\0c \0c\00\0c\00\f6\00\f4\00\f2\00\f0`\0c@\0c \0c\00\0c\00\f6\00\f4\00\f2\00\f0`\0c@\0c \0c\00\0c\00\f6\00\f4\00\f2\00\f0`\0c@\0c \0c\00\0c\00\f6\00\f4\00\f2\00\f0`\0c@\0c \0c\00\0c\00\f6\00\f4\00\f2\00\f0`\0c@\0c \0c\00\0c\00\f6\00\f4\00\f2\00\f0`\0c@\0c \0c\00\0c\00\f6\00\f4\00\f2\00\f0`\0c@\0c \0c\00\0c\00\f6\00\f4\00\f2\00\f0`\0c@\0c \0c\00\0c\00\f6\00\f4\00\f2\00\f0"))

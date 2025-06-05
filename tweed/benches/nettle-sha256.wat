(module
  (type (;0;) (func))
  (type (;1;) (func (param i32 i32 i32)))
  (type (;2;) (func (param i32 i32)))
  (type (;3;) (func (result i32)))
  (type (;4;) (func (param i32 i32) (result i32)))
  (type (;5;) (func (param i32 i32 i32) (result i32)))
  (func (;0;) (type 0))
  (func (;1;) (type 0)
    block  ;; label = @1
      global.get 1
      i32.const 1376
      i32.add
      i32.load
      i32.eqz
      br_if 0 (;@1;)
      unreachable
    end
    global.get 1
    i32.const 1376
    i32.add
    i32.const 1
    i32.store
    call 0)
  (func (;2;) (type 1) (param i32 i32 i32)
    (local i32 i32 i32)
    local.get 0
    i32.const 3
    i32.and
    local.set 3
    i32.const 0
    local.set 4
    block  ;; label = @1
      local.get 0
      i32.const 4
      i32.lt_u
      br_if 0 (;@1;)
      local.get 2
      local.set 5
      local.get 0
      i32.const 2
      i32.shr_u
      local.tee 4
      local.set 0
      loop  ;; label = @2
        local.get 1
        local.get 5
        i32.load8_u offset=3
        i32.store8
        local.get 1
        local.get 5
        i32.load16_u offset=2
        i32.store8 offset=1
        local.get 1
        local.get 5
        i32.load
        i32.const 8
        i32.shr_u
        i32.store8 offset=2
        local.get 1
        local.get 5
        i32.load
        i32.store8 offset=3
        local.get 5
        i32.const 4
        i32.add
        local.set 5
        local.get 1
        i32.const 4
        i32.add
        local.set 1
        local.get 0
        i32.const -1
        i32.add
        local.tee 0
        br_if 0 (;@2;)
      end
    end
    block  ;; label = @1
      local.get 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 4
      i32.const 2
      i32.shl
      i32.add
      i32.load
      local.set 5
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 3
            i32.const -1
            i32.add
            br_table 2 (;@2;) 1 (;@3;) 0 (;@4;) 2 (;@2;)
          end
          local.get 1
          local.get 5
          i32.const 8
          i32.shr_u
          i32.store8 offset=2
        end
        local.get 1
        local.get 5
        i32.const 16
        i32.shr_u
        i32.store8 offset=1
      end
      local.get 1
      local.get 5
      i32.const 24
      i32.shr_u
      i32.store8
    end)
  (func (;3;) (type 1) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 64
    i32.sub
    local.set 3
    i32.const 0
    local.set 4
    loop  ;; label = @1
      local.get 3
      local.get 4
      i32.add
      local.get 1
      local.get 4
      i32.add
      i32.load align=1
      local.tee 5
      i32.const 24
      i32.shl
      local.get 5
      i32.const 65280
      i32.and
      i32.const 8
      i32.shl
      i32.or
      local.get 5
      i32.const 8
      i32.shr_u
      i32.const 65280
      i32.and
      local.get 5
      i32.const 24
      i32.shr_u
      i32.or
      i32.or
      i32.store
      local.get 4
      i32.const 4
      i32.add
      local.tee 4
      i32.const 64
      i32.ne
      br_if 0 (;@1;)
    end
    i32.const 0
    local.set 6
    i32.const 1
    local.set 7
    local.get 0
    i32.load offset=28
    local.tee 8
    local.set 9
    local.get 0
    i32.load offset=24
    local.tee 10
    local.set 11
    local.get 0
    i32.load offset=20
    local.tee 12
    local.set 13
    local.get 0
    i32.load offset=16
    local.tee 14
    local.set 5
    local.get 0
    i32.load offset=12
    local.tee 15
    local.set 1
    local.get 0
    i32.load offset=8
    local.tee 16
    local.set 17
    local.get 0
    i32.load offset=4
    local.tee 18
    local.set 19
    local.get 0
    i32.load
    local.tee 20
    local.set 4
    loop  ;; label = @1
      local.get 17
      local.get 19
      local.get 4
      i32.xor
      i32.and
      local.get 19
      local.get 4
      i32.and
      i32.xor
      local.get 4
      i32.const 30
      i32.rotl
      local.get 4
      i32.const 19
      i32.rotl
      i32.xor
      local.get 4
      i32.const 10
      i32.rotl
      i32.xor
      i32.add
      local.get 9
      local.get 5
      i32.const 26
      i32.rotl
      local.get 5
      i32.const 21
      i32.rotl
      i32.xor
      local.get 5
      i32.const 7
      i32.rotl
      i32.xor
      i32.add
      local.get 11
      local.get 13
      i32.xor
      local.get 5
      i32.and
      local.get 11
      i32.xor
      i32.add
      local.get 2
      local.get 6
      i32.add
      local.tee 21
      i32.load
      i32.add
      local.get 3
      local.get 6
      i32.add
      local.tee 22
      i32.load
      i32.add
      local.tee 23
      i32.add
      local.tee 9
      i32.const 30
      i32.rotl
      local.get 9
      i32.const 19
      i32.rotl
      i32.xor
      local.get 9
      i32.const 10
      i32.rotl
      i32.xor
      local.get 9
      local.get 4
      i32.xor
      local.get 19
      i32.and
      local.get 9
      local.get 4
      i32.and
      i32.xor
      i32.add
      local.get 21
      i32.const 4
      i32.add
      i32.load
      local.get 11
      i32.add
      local.get 22
      i32.const 4
      i32.add
      i32.load
      i32.add
      local.get 23
      local.get 1
      i32.add
      local.tee 1
      local.get 13
      local.get 5
      i32.xor
      i32.and
      local.get 13
      i32.xor
      i32.add
      local.get 1
      i32.const 26
      i32.rotl
      local.get 1
      i32.const 21
      i32.rotl
      i32.xor
      local.get 1
      i32.const 7
      i32.rotl
      i32.xor
      i32.add
      local.tee 23
      i32.add
      local.tee 11
      i32.const 30
      i32.rotl
      local.get 11
      i32.const 19
      i32.rotl
      i32.xor
      local.get 11
      i32.const 10
      i32.rotl
      i32.xor
      local.get 11
      local.get 9
      i32.xor
      local.get 4
      i32.and
      local.get 11
      local.get 9
      i32.and
      i32.xor
      i32.add
      local.get 21
      i32.const 8
      i32.add
      i32.load
      local.get 13
      i32.add
      local.get 22
      i32.const 8
      i32.add
      i32.load
      i32.add
      local.get 23
      local.get 17
      i32.add
      local.tee 17
      local.get 1
      local.get 5
      i32.xor
      i32.and
      local.get 5
      i32.xor
      i32.add
      local.get 17
      i32.const 26
      i32.rotl
      local.get 17
      i32.const 21
      i32.rotl
      i32.xor
      local.get 17
      i32.const 7
      i32.rotl
      i32.xor
      i32.add
      local.tee 23
      i32.add
      local.tee 13
      i32.const 30
      i32.rotl
      local.get 13
      i32.const 19
      i32.rotl
      i32.xor
      local.get 13
      i32.const 10
      i32.rotl
      i32.xor
      local.get 13
      local.get 11
      i32.xor
      local.get 9
      i32.and
      local.get 13
      local.get 11
      i32.and
      i32.xor
      i32.add
      local.get 21
      i32.const 12
      i32.add
      i32.load
      local.get 5
      i32.add
      local.get 22
      i32.const 12
      i32.add
      i32.load
      i32.add
      local.get 23
      local.get 19
      i32.add
      local.tee 19
      local.get 17
      local.get 1
      i32.xor
      i32.and
      local.get 1
      i32.xor
      i32.add
      local.get 19
      i32.const 26
      i32.rotl
      local.get 19
      i32.const 21
      i32.rotl
      i32.xor
      local.get 19
      i32.const 7
      i32.rotl
      i32.xor
      i32.add
      local.tee 23
      i32.add
      local.tee 5
      i32.const 30
      i32.rotl
      local.get 5
      i32.const 19
      i32.rotl
      i32.xor
      local.get 5
      i32.const 10
      i32.rotl
      i32.xor
      local.get 5
      local.get 13
      i32.xor
      local.get 11
      i32.and
      local.get 5
      local.get 13
      i32.and
      i32.xor
      i32.add
      local.get 21
      i32.const 16
      i32.add
      i32.load
      local.get 1
      i32.add
      local.get 22
      i32.const 16
      i32.add
      i32.load
      i32.add
      local.get 23
      local.get 4
      i32.add
      local.tee 4
      local.get 19
      local.get 17
      i32.xor
      i32.and
      local.get 17
      i32.xor
      i32.add
      local.get 4
      i32.const 26
      i32.rotl
      local.get 4
      i32.const 21
      i32.rotl
      i32.xor
      local.get 4
      i32.const 7
      i32.rotl
      i32.xor
      i32.add
      local.tee 23
      i32.add
      local.tee 1
      i32.const 30
      i32.rotl
      local.get 1
      i32.const 19
      i32.rotl
      i32.xor
      local.get 1
      i32.const 10
      i32.rotl
      i32.xor
      local.get 1
      local.get 5
      i32.xor
      local.get 13
      i32.and
      local.get 1
      local.get 5
      i32.and
      i32.xor
      i32.add
      local.get 22
      i32.const 20
      i32.add
      i32.load
      local.get 21
      i32.const 20
      i32.add
      i32.load
      i32.add
      local.get 17
      i32.add
      local.get 23
      local.get 9
      i32.add
      local.tee 9
      local.get 4
      local.get 19
      i32.xor
      i32.and
      local.get 19
      i32.xor
      i32.add
      local.get 9
      i32.const 26
      i32.rotl
      local.get 9
      i32.const 21
      i32.rotl
      i32.xor
      local.get 9
      i32.const 7
      i32.rotl
      i32.xor
      i32.add
      local.tee 23
      i32.add
      local.tee 17
      i32.const 30
      i32.rotl
      local.get 17
      i32.const 19
      i32.rotl
      i32.xor
      local.get 17
      i32.const 10
      i32.rotl
      i32.xor
      local.get 17
      local.get 1
      i32.xor
      local.get 5
      i32.and
      local.get 17
      local.get 1
      i32.and
      i32.xor
      i32.add
      local.get 22
      i32.const 24
      i32.add
      i32.load
      local.get 21
      i32.const 24
      i32.add
      i32.load
      i32.add
      local.get 19
      i32.add
      local.get 23
      local.get 11
      i32.add
      local.tee 11
      local.get 9
      local.get 4
      i32.xor
      i32.and
      local.get 4
      i32.xor
      i32.add
      local.get 11
      i32.const 26
      i32.rotl
      local.get 11
      i32.const 21
      i32.rotl
      i32.xor
      local.get 11
      i32.const 7
      i32.rotl
      i32.xor
      i32.add
      local.tee 23
      i32.add
      local.tee 19
      i32.const 30
      i32.rotl
      local.get 19
      i32.const 19
      i32.rotl
      i32.xor
      local.get 19
      i32.const 10
      i32.rotl
      i32.xor
      local.get 19
      local.get 17
      i32.xor
      local.get 1
      i32.and
      local.get 19
      local.get 17
      i32.and
      i32.xor
      i32.add
      local.get 22
      i32.const 28
      i32.add
      i32.load
      local.get 21
      i32.const 28
      i32.add
      i32.load
      i32.add
      local.get 4
      i32.add
      local.get 23
      local.get 13
      i32.add
      local.tee 13
      local.get 11
      local.get 9
      i32.xor
      i32.and
      local.get 9
      i32.xor
      i32.add
      local.get 13
      i32.const 26
      i32.rotl
      local.get 13
      i32.const 21
      i32.rotl
      i32.xor
      local.get 13
      i32.const 7
      i32.rotl
      i32.xor
      i32.add
      local.tee 21
      i32.add
      local.set 4
      local.get 21
      local.get 5
      i32.add
      local.set 5
      local.get 6
      i32.const 32
      i32.add
      local.set 6
      local.get 7
      i32.const 1
      i32.and
      local.set 21
      i32.const 0
      local.set 7
      local.get 21
      br_if 0 (;@1;)
    end
    i32.const 0
    local.set 24
    local.get 3
    i32.load offset=32
    local.set 21
    local.get 3
    i32.load offset=28
    local.set 22
    local.get 3
    i32.load offset=24
    local.set 6
    local.get 3
    i32.load offset=20
    local.set 7
    local.get 3
    i32.load offset=52
    local.set 23
    local.get 3
    i32.load offset=16
    local.set 25
    local.get 3
    i32.load offset=48
    local.set 26
    local.get 3
    i32.load offset=12
    local.set 27
    local.get 3
    i32.load offset=44
    local.set 28
    local.get 3
    i32.load offset=8
    local.set 29
    local.get 3
    i32.load offset=40
    local.set 30
    local.get 3
    i32.load offset=60
    local.set 31
    local.get 3
    i32.load offset=4
    local.set 32
    local.get 3
    i32.load offset=36
    local.set 33
    local.get 3
    i32.load offset=56
    local.set 34
    local.get 3
    i32.load
    local.set 3
    loop  ;; label = @1
      local.get 17
      local.get 19
      local.get 4
      i32.xor
      i32.and
      local.get 19
      local.get 4
      i32.and
      i32.xor
      local.get 4
      i32.const 30
      i32.rotl
      local.get 4
      i32.const 19
      i32.rotl
      i32.xor
      local.get 4
      i32.const 10
      i32.rotl
      i32.xor
      i32.add
      local.get 9
      local.get 5
      i32.const 26
      i32.rotl
      local.get 5
      i32.const 21
      i32.rotl
      i32.xor
      local.get 5
      i32.const 7
      i32.rotl
      i32.xor
      i32.add
      local.get 11
      local.get 13
      i32.xor
      local.get 5
      i32.and
      local.get 11
      i32.xor
      i32.add
      local.get 2
      i32.const 64
      i32.add
      local.tee 35
      i32.load
      i32.add
      local.get 34
      i32.const 15
      i32.rotl
      local.get 34
      i32.const 13
      i32.rotl
      i32.xor
      local.get 34
      i32.const 10
      i32.shr_u
      i32.xor
      local.get 33
      i32.add
      local.get 3
      i32.add
      local.get 32
      i32.const 25
      i32.rotl
      local.get 32
      i32.const 14
      i32.rotl
      i32.xor
      local.get 32
      i32.const 3
      i32.shr_u
      i32.xor
      i32.add
      local.tee 3
      i32.add
      local.tee 36
      i32.add
      local.tee 9
      i32.const 30
      i32.rotl
      local.get 9
      i32.const 19
      i32.rotl
      i32.xor
      local.get 9
      i32.const 10
      i32.rotl
      i32.xor
      local.get 9
      local.get 4
      i32.xor
      local.get 19
      i32.and
      local.get 9
      local.get 4
      i32.and
      i32.xor
      i32.add
      local.get 2
      i32.const 68
      i32.add
      i32.load
      local.get 11
      i32.add
      local.get 36
      local.get 1
      i32.add
      local.tee 11
      local.get 13
      local.get 5
      i32.xor
      i32.and
      local.get 13
      i32.xor
      i32.add
      local.get 11
      i32.const 26
      i32.rotl
      local.get 11
      i32.const 21
      i32.rotl
      i32.xor
      local.get 11
      i32.const 7
      i32.rotl
      i32.xor
      i32.add
      local.get 30
      local.get 32
      i32.add
      local.get 31
      i32.const 15
      i32.rotl
      local.get 31
      i32.const 13
      i32.rotl
      i32.xor
      local.get 31
      i32.const 10
      i32.shr_u
      i32.xor
      i32.add
      local.get 29
      i32.const 25
      i32.rotl
      local.get 29
      i32.const 14
      i32.rotl
      i32.xor
      local.get 29
      i32.const 3
      i32.shr_u
      i32.xor
      i32.add
      local.tee 32
      i32.add
      local.tee 36
      i32.add
      local.tee 1
      i32.const 30
      i32.rotl
      local.get 1
      i32.const 19
      i32.rotl
      i32.xor
      local.get 1
      i32.const 10
      i32.rotl
      i32.xor
      local.get 1
      local.get 9
      i32.xor
      local.get 4
      i32.and
      local.get 1
      local.get 9
      i32.and
      i32.xor
      i32.add
      local.get 2
      i32.const 72
      i32.add
      i32.load
      local.get 13
      i32.add
      local.get 3
      i32.const 15
      i32.rotl
      local.get 3
      i32.const 13
      i32.rotl
      i32.xor
      local.get 3
      i32.const 10
      i32.shr_u
      i32.xor
      local.get 29
      i32.add
      local.get 28
      i32.add
      local.get 27
      i32.const 25
      i32.rotl
      local.get 27
      i32.const 14
      i32.rotl
      i32.xor
      local.get 27
      i32.const 3
      i32.shr_u
      i32.xor
      i32.add
      local.tee 29
      i32.add
      local.get 36
      local.get 17
      i32.add
      local.tee 13
      local.get 11
      local.get 5
      i32.xor
      i32.and
      local.get 5
      i32.xor
      i32.add
      local.get 13
      i32.const 26
      i32.rotl
      local.get 13
      i32.const 21
      i32.rotl
      i32.xor
      local.get 13
      i32.const 7
      i32.rotl
      i32.xor
      i32.add
      local.tee 36
      i32.add
      local.tee 17
      i32.const 30
      i32.rotl
      local.get 17
      i32.const 19
      i32.rotl
      i32.xor
      local.get 17
      i32.const 10
      i32.rotl
      i32.xor
      local.get 17
      local.get 1
      i32.xor
      local.get 9
      i32.and
      local.get 17
      local.get 1
      i32.and
      i32.xor
      i32.add
      local.get 2
      i32.const 76
      i32.add
      i32.load
      local.get 5
      i32.add
      local.get 26
      local.get 27
      i32.add
      local.get 32
      i32.const 15
      i32.rotl
      local.get 32
      i32.const 13
      i32.rotl
      i32.xor
      local.get 32
      i32.const 10
      i32.shr_u
      i32.xor
      i32.add
      local.get 25
      i32.const 25
      i32.rotl
      local.get 25
      i32.const 14
      i32.rotl
      i32.xor
      local.get 25
      i32.const 3
      i32.shr_u
      i32.xor
      i32.add
      local.tee 27
      i32.add
      local.get 36
      local.get 19
      i32.add
      local.tee 36
      local.get 13
      local.get 11
      i32.xor
      i32.and
      local.get 11
      i32.xor
      i32.add
      local.get 36
      i32.const 26
      i32.rotl
      local.get 36
      i32.const 21
      i32.rotl
      i32.xor
      local.get 36
      i32.const 7
      i32.rotl
      i32.xor
      i32.add
      local.tee 19
      i32.add
      local.tee 5
      i32.const 30
      i32.rotl
      local.get 5
      i32.const 19
      i32.rotl
      i32.xor
      local.get 5
      i32.const 10
      i32.rotl
      i32.xor
      local.get 5
      local.get 17
      i32.xor
      local.get 1
      i32.and
      local.get 5
      local.get 17
      i32.and
      i32.xor
      i32.add
      local.get 2
      i32.const 80
      i32.add
      i32.load
      local.get 11
      i32.add
      local.get 23
      local.get 25
      i32.add
      local.get 29
      i32.const 15
      i32.rotl
      local.get 29
      i32.const 13
      i32.rotl
      i32.xor
      local.get 29
      i32.const 10
      i32.shr_u
      i32.xor
      i32.add
      local.get 7
      i32.const 25
      i32.rotl
      local.get 7
      i32.const 14
      i32.rotl
      i32.xor
      local.get 7
      i32.const 3
      i32.shr_u
      i32.xor
      i32.add
      local.tee 25
      i32.add
      local.get 19
      local.get 4
      i32.add
      local.tee 11
      local.get 36
      local.get 13
      i32.xor
      i32.and
      local.get 13
      i32.xor
      i32.add
      local.get 11
      i32.const 26
      i32.rotl
      local.get 11
      i32.const 21
      i32.rotl
      i32.xor
      local.get 11
      i32.const 7
      i32.rotl
      i32.xor
      i32.add
      local.tee 19
      i32.add
      local.tee 4
      i32.const 30
      i32.rotl
      local.get 4
      i32.const 19
      i32.rotl
      i32.xor
      local.get 4
      i32.const 10
      i32.rotl
      i32.xor
      local.get 4
      local.get 5
      i32.xor
      local.get 17
      i32.and
      local.get 4
      local.get 5
      i32.and
      i32.xor
      i32.add
      local.get 2
      i32.const 84
      i32.add
      i32.load
      local.get 13
      i32.add
      local.get 7
      local.get 34
      i32.add
      local.get 27
      i32.const 15
      i32.rotl
      local.get 27
      i32.const 13
      i32.rotl
      i32.xor
      local.get 27
      i32.const 10
      i32.shr_u
      i32.xor
      i32.add
      local.get 6
      i32.const 25
      i32.rotl
      local.get 6
      i32.const 14
      i32.rotl
      i32.xor
      local.get 6
      i32.const 3
      i32.shr_u
      i32.xor
      i32.add
      local.tee 7
      i32.add
      local.get 19
      local.get 9
      i32.add
      local.tee 9
      local.get 11
      local.get 36
      i32.xor
      i32.and
      local.get 36
      i32.xor
      i32.add
      local.get 9
      i32.const 26
      i32.rotl
      local.get 9
      i32.const 21
      i32.rotl
      i32.xor
      local.get 9
      i32.const 7
      i32.rotl
      i32.xor
      i32.add
      local.tee 13
      i32.add
      local.tee 19
      i32.const 30
      i32.rotl
      local.get 19
      i32.const 19
      i32.rotl
      i32.xor
      local.get 19
      i32.const 10
      i32.rotl
      i32.xor
      local.get 19
      local.get 4
      i32.xor
      local.get 5
      i32.and
      local.get 19
      local.get 4
      i32.and
      i32.xor
      i32.add
      local.get 36
      local.get 2
      i32.const 88
      i32.add
      i32.load
      i32.add
      local.get 6
      local.get 31
      i32.add
      local.get 25
      i32.const 15
      i32.rotl
      local.get 25
      i32.const 13
      i32.rotl
      i32.xor
      local.get 25
      i32.const 10
      i32.shr_u
      i32.xor
      i32.add
      local.get 22
      i32.const 25
      i32.rotl
      local.get 22
      i32.const 14
      i32.rotl
      i32.xor
      local.get 22
      i32.const 3
      i32.shr_u
      i32.xor
      i32.add
      local.tee 6
      i32.add
      local.get 13
      local.get 1
      i32.add
      local.tee 13
      local.get 9
      local.get 11
      i32.xor
      i32.and
      local.get 11
      i32.xor
      i32.add
      local.get 13
      i32.const 26
      i32.rotl
      local.get 13
      i32.const 21
      i32.rotl
      i32.xor
      local.get 13
      i32.const 7
      i32.rotl
      i32.xor
      i32.add
      local.tee 36
      i32.add
      local.tee 1
      i32.const 30
      i32.rotl
      local.get 1
      i32.const 19
      i32.rotl
      i32.xor
      local.get 1
      i32.const 10
      i32.rotl
      i32.xor
      local.get 1
      local.get 19
      i32.xor
      local.get 4
      i32.and
      local.get 1
      local.get 19
      i32.and
      i32.xor
      i32.add
      local.get 22
      local.get 3
      i32.add
      local.get 7
      i32.const 15
      i32.rotl
      local.get 7
      i32.const 13
      i32.rotl
      i32.xor
      local.get 7
      i32.const 10
      i32.shr_u
      i32.xor
      i32.add
      local.get 21
      i32.const 25
      i32.rotl
      local.get 21
      i32.const 14
      i32.rotl
      i32.xor
      local.get 21
      i32.const 3
      i32.shr_u
      i32.xor
      i32.add
      local.tee 22
      local.get 2
      i32.const 92
      i32.add
      i32.load
      i32.add
      local.get 11
      i32.add
      local.get 36
      local.get 17
      i32.add
      local.tee 11
      local.get 13
      local.get 9
      i32.xor
      i32.and
      local.get 9
      i32.xor
      i32.add
      local.get 11
      i32.const 26
      i32.rotl
      local.get 11
      i32.const 21
      i32.rotl
      i32.xor
      local.get 11
      i32.const 7
      i32.rotl
      i32.xor
      i32.add
      local.tee 36
      i32.add
      local.tee 17
      i32.const 30
      i32.rotl
      local.get 17
      i32.const 19
      i32.rotl
      i32.xor
      local.get 17
      i32.const 10
      i32.rotl
      i32.xor
      local.get 17
      local.get 1
      i32.xor
      local.get 19
      i32.and
      local.get 17
      local.get 1
      i32.and
      i32.xor
      i32.add
      local.get 32
      local.get 33
      i32.const 25
      i32.rotl
      local.get 33
      i32.const 14
      i32.rotl
      i32.xor
      local.get 33
      i32.const 3
      i32.shr_u
      i32.xor
      i32.add
      local.get 21
      i32.add
      local.get 6
      i32.const 15
      i32.rotl
      local.get 6
      i32.const 13
      i32.rotl
      i32.xor
      local.get 6
      i32.const 10
      i32.shr_u
      i32.xor
      i32.add
      local.tee 21
      local.get 2
      i32.const 96
      i32.add
      i32.load
      i32.add
      local.get 9
      i32.add
      local.get 36
      local.get 5
      i32.add
      local.tee 9
      local.get 11
      local.get 13
      i32.xor
      i32.and
      local.get 13
      i32.xor
      i32.add
      local.get 9
      i32.const 26
      i32.rotl
      local.get 9
      i32.const 21
      i32.rotl
      i32.xor
      local.get 9
      i32.const 7
      i32.rotl
      i32.xor
      i32.add
      local.tee 36
      i32.add
      local.tee 5
      i32.const 30
      i32.rotl
      local.get 5
      i32.const 19
      i32.rotl
      i32.xor
      local.get 5
      i32.const 10
      i32.rotl
      i32.xor
      local.get 5
      local.get 17
      i32.xor
      local.get 1
      i32.and
      local.get 5
      local.get 17
      i32.and
      i32.xor
      i32.add
      local.get 30
      i32.const 25
      i32.rotl
      local.get 30
      i32.const 14
      i32.rotl
      i32.xor
      local.get 30
      i32.const 3
      i32.shr_u
      i32.xor
      local.get 33
      i32.add
      local.get 29
      i32.add
      local.get 22
      i32.const 15
      i32.rotl
      local.get 22
      i32.const 13
      i32.rotl
      i32.xor
      local.get 22
      i32.const 10
      i32.shr_u
      i32.xor
      i32.add
      local.tee 33
      local.get 2
      i32.const 100
      i32.add
      i32.load
      i32.add
      local.get 13
      i32.add
      local.get 36
      local.get 4
      i32.add
      local.tee 37
      local.get 9
      local.get 11
      i32.xor
      i32.and
      local.get 11
      i32.xor
      i32.add
      local.get 37
      i32.const 26
      i32.rotl
      local.get 37
      i32.const 21
      i32.rotl
      i32.xor
      local.get 37
      i32.const 7
      i32.rotl
      i32.xor
      i32.add
      local.tee 13
      i32.add
      local.tee 4
      i32.const 30
      i32.rotl
      local.get 4
      i32.const 19
      i32.rotl
      i32.xor
      local.get 4
      i32.const 10
      i32.rotl
      i32.xor
      local.get 4
      local.get 5
      i32.xor
      local.get 17
      i32.and
      local.get 4
      local.get 5
      i32.and
      i32.xor
      i32.add
      local.get 28
      i32.const 25
      i32.rotl
      local.get 28
      i32.const 14
      i32.rotl
      i32.xor
      local.get 28
      i32.const 3
      i32.shr_u
      i32.xor
      local.get 30
      i32.add
      local.get 27
      i32.add
      local.get 21
      i32.const 15
      i32.rotl
      local.get 21
      i32.const 13
      i32.rotl
      i32.xor
      local.get 21
      i32.const 10
      i32.shr_u
      i32.xor
      i32.add
      local.tee 30
      local.get 2
      i32.const 104
      i32.add
      i32.load
      i32.add
      local.get 11
      i32.add
      local.get 13
      local.get 19
      i32.add
      local.tee 19
      local.get 37
      local.get 9
      i32.xor
      i32.and
      local.get 9
      i32.xor
      i32.add
      local.get 19
      i32.const 26
      i32.rotl
      local.get 19
      i32.const 21
      i32.rotl
      i32.xor
      local.get 19
      i32.const 7
      i32.rotl
      i32.xor
      i32.add
      local.tee 11
      i32.add
      local.tee 13
      i32.const 30
      i32.rotl
      local.get 13
      i32.const 19
      i32.rotl
      i32.xor
      local.get 13
      i32.const 10
      i32.rotl
      i32.xor
      local.get 13
      local.get 4
      i32.xor
      local.get 5
      i32.and
      local.get 13
      local.get 4
      i32.and
      i32.xor
      i32.add
      local.get 26
      i32.const 25
      i32.rotl
      local.get 26
      i32.const 14
      i32.rotl
      i32.xor
      local.get 26
      i32.const 3
      i32.shr_u
      i32.xor
      local.get 28
      i32.add
      local.get 25
      i32.add
      local.get 33
      i32.const 15
      i32.rotl
      local.get 33
      i32.const 13
      i32.rotl
      i32.xor
      local.get 33
      i32.const 10
      i32.shr_u
      i32.xor
      i32.add
      local.tee 28
      local.get 2
      i32.const 108
      i32.add
      i32.load
      i32.add
      local.get 9
      i32.add
      local.get 11
      local.get 1
      i32.add
      local.tee 11
      local.get 19
      local.get 37
      i32.xor
      i32.and
      local.get 37
      i32.xor
      i32.add
      local.get 11
      i32.const 26
      i32.rotl
      local.get 11
      i32.const 21
      i32.rotl
      i32.xor
      local.get 11
      i32.const 7
      i32.rotl
      i32.xor
      i32.add
      local.tee 1
      i32.add
      local.tee 36
      i32.const 30
      i32.rotl
      local.get 36
      i32.const 19
      i32.rotl
      i32.xor
      local.get 36
      i32.const 10
      i32.rotl
      i32.xor
      local.get 36
      local.get 13
      i32.xor
      local.get 4
      i32.and
      local.get 36
      local.get 13
      i32.and
      i32.xor
      i32.add
      local.get 23
      i32.const 25
      i32.rotl
      local.get 23
      i32.const 14
      i32.rotl
      i32.xor
      local.get 23
      i32.const 3
      i32.shr_u
      i32.xor
      local.get 26
      i32.add
      local.get 7
      i32.add
      local.get 30
      i32.const 15
      i32.rotl
      local.get 30
      i32.const 13
      i32.rotl
      i32.xor
      local.get 30
      i32.const 10
      i32.shr_u
      i32.xor
      i32.add
      local.tee 26
      local.get 2
      i32.const 112
      i32.add
      i32.load
      i32.add
      local.get 37
      i32.add
      local.get 1
      local.get 17
      i32.add
      local.tee 37
      local.get 11
      local.get 19
      i32.xor
      i32.and
      local.get 19
      i32.xor
      i32.add
      local.get 37
      i32.const 26
      i32.rotl
      local.get 37
      i32.const 21
      i32.rotl
      i32.xor
      local.get 37
      i32.const 7
      i32.rotl
      i32.xor
      i32.add
      local.tee 17
      i32.add
      local.tee 1
      i32.const 30
      i32.rotl
      local.get 1
      i32.const 19
      i32.rotl
      i32.xor
      local.get 1
      i32.const 10
      i32.rotl
      i32.xor
      local.get 1
      local.get 36
      i32.xor
      local.get 13
      i32.and
      local.get 1
      local.get 36
      i32.and
      i32.xor
      i32.add
      local.get 23
      local.get 34
      i32.const 25
      i32.rotl
      local.get 34
      i32.const 14
      i32.rotl
      i32.xor
      local.get 34
      i32.const 3
      i32.shr_u
      i32.xor
      i32.add
      local.get 6
      i32.add
      local.get 28
      i32.const 15
      i32.rotl
      local.get 28
      i32.const 13
      i32.rotl
      i32.xor
      local.get 28
      i32.const 10
      i32.shr_u
      i32.xor
      i32.add
      local.tee 23
      local.get 2
      i32.const 116
      i32.add
      i32.load
      i32.add
      local.get 19
      i32.add
      local.get 17
      local.get 5
      i32.add
      local.tee 9
      local.get 37
      local.get 11
      i32.xor
      i32.and
      local.get 11
      i32.xor
      i32.add
      local.get 9
      i32.const 26
      i32.rotl
      local.get 9
      i32.const 21
      i32.rotl
      i32.xor
      local.get 9
      i32.const 7
      i32.rotl
      i32.xor
      i32.add
      local.tee 5
      i32.add
      local.tee 17
      i32.const 30
      i32.rotl
      local.get 17
      i32.const 19
      i32.rotl
      i32.xor
      local.get 17
      i32.const 10
      i32.rotl
      i32.xor
      local.get 17
      local.get 1
      i32.xor
      local.get 36
      i32.and
      local.get 17
      local.get 1
      i32.and
      i32.xor
      i32.add
      local.get 31
      i32.const 25
      i32.rotl
      local.get 31
      i32.const 14
      i32.rotl
      i32.xor
      local.get 31
      i32.const 3
      i32.shr_u
      i32.xor
      local.get 34
      i32.add
      local.get 22
      i32.add
      local.get 26
      i32.const 15
      i32.rotl
      local.get 26
      i32.const 13
      i32.rotl
      i32.xor
      local.get 26
      i32.const 10
      i32.shr_u
      i32.xor
      i32.add
      local.tee 34
      local.get 2
      i32.const 120
      i32.add
      i32.load
      i32.add
      local.get 11
      i32.add
      local.get 5
      local.get 4
      i32.add
      local.tee 11
      local.get 9
      local.get 37
      i32.xor
      i32.and
      local.get 37
      i32.xor
      i32.add
      local.get 11
      i32.const 26
      i32.rotl
      local.get 11
      i32.const 21
      i32.rotl
      i32.xor
      local.get 11
      i32.const 7
      i32.rotl
      i32.xor
      i32.add
      local.tee 4
      i32.add
      local.tee 19
      i32.const 30
      i32.rotl
      local.get 19
      i32.const 19
      i32.rotl
      i32.xor
      local.get 19
      i32.const 10
      i32.rotl
      i32.xor
      local.get 19
      local.get 17
      i32.xor
      local.get 1
      i32.and
      local.get 19
      local.get 17
      i32.and
      i32.xor
      i32.add
      local.get 3
      i32.const 25
      i32.rotl
      local.get 3
      i32.const 14
      i32.rotl
      i32.xor
      local.get 3
      i32.const 3
      i32.shr_u
      i32.xor
      local.get 31
      i32.add
      local.get 21
      i32.add
      local.get 23
      i32.const 15
      i32.rotl
      local.get 23
      i32.const 13
      i32.rotl
      i32.xor
      local.get 23
      i32.const 10
      i32.shr_u
      i32.xor
      i32.add
      local.tee 31
      local.get 2
      i32.const 124
      i32.add
      i32.load
      i32.add
      local.get 37
      i32.add
      local.get 4
      local.get 13
      i32.add
      local.tee 13
      local.get 11
      local.get 9
      i32.xor
      i32.and
      local.get 9
      i32.xor
      i32.add
      local.get 13
      i32.const 26
      i32.rotl
      local.get 13
      i32.const 21
      i32.rotl
      i32.xor
      local.get 13
      i32.const 7
      i32.rotl
      i32.xor
      i32.add
      local.tee 5
      i32.add
      local.set 4
      local.get 5
      local.get 36
      i32.add
      local.set 5
      local.get 35
      local.set 2
      local.get 24
      i32.const 16
      i32.add
      local.tee 24
      i32.const 48
      i32.lt_u
      br_if 0 (;@1;)
    end
    local.get 0
    local.get 9
    local.get 8
    i32.add
    i32.store offset=28
    local.get 0
    local.get 11
    local.get 10
    i32.add
    i32.store offset=24
    local.get 0
    local.get 13
    local.get 12
    i32.add
    i32.store offset=20
    local.get 0
    local.get 5
    local.get 14
    i32.add
    i32.store offset=16
    local.get 0
    local.get 1
    local.get 15
    i32.add
    i32.store offset=12
    local.get 0
    local.get 17
    local.get 16
    i32.add
    i32.store offset=8
    local.get 0
    local.get 19
    local.get 18
    i32.add
    i32.store offset=4
    local.get 0
    local.get 4
    local.get 20
    i32.add
    i32.store)
  (func (;4;) (type 1) (param i32 i32 i32)
    (local i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load offset=104
          local.tee 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          i32.const 40
          i32.add
          local.tee 4
          local.get 3
          i32.add
          local.set 5
          local.get 1
          i32.const 64
          local.get 3
          i32.sub
          local.tee 3
          i32.lt_u
          br_if 1 (;@2;)
          local.get 5
          local.get 2
          local.get 3
          call 14
          drop
          local.get 0
          local.get 4
          i32.const 1056
          call 3
          local.get 0
          local.get 0
          i64.load offset=32
          i64.const 1
          i64.add
          i64.store offset=32
          local.get 1
          local.get 3
          i32.sub
          local.set 1
          local.get 2
          local.get 3
          i32.add
          local.set 2
        end
        block  ;; label = @3
          local.get 1
          i32.const 64
          i32.lt_u
          br_if 0 (;@3;)
          loop  ;; label = @4
            local.get 0
            local.get 2
            i32.const 1056
            call 3
            local.get 0
            local.get 0
            i64.load offset=32
            i64.const 1
            i64.add
            i64.store offset=32
            local.get 2
            i32.const 64
            i32.add
            local.set 2
            local.get 1
            i32.const -64
            i32.add
            local.tee 1
            i32.const 63
            i32.gt_u
            br_if 0 (;@4;)
          end
        end
        local.get 0
        i32.const 40
        i32.add
        local.get 2
        local.get 1
        call 14
        drop
        br 1 (;@1;)
      end
      local.get 5
      local.get 2
      local.get 1
      call 14
      drop
      local.get 0
      i32.load offset=104
      local.get 1
      i32.add
      local.set 1
    end
    local.get 0
    local.get 1
    i32.store offset=104)
  (func (;5;) (type 1) (param i32 i32 i32)
    (local i32 i32 i32 i32 i64)
    block  ;; label = @1
      local.get 1
      i32.const 33
      i32.lt_u
      br_if 0 (;@1;)
      loop  ;; label = @2
        br 0 (;@2;)
      end
    end
    block  ;; label = @1
      local.get 0
      i32.load offset=104
      local.tee 3
      i32.const 64
      i32.lt_u
      br_if 0 (;@1;)
      loop  ;; label = @2
        br 0 (;@2;)
      end
    end
    local.get 0
    i32.const 40
    i32.add
    local.tee 4
    local.get 3
    i32.add
    i32.const 128
    i32.store8
    local.get 3
    i32.const 1
    i32.add
    local.set 5
    block  ;; label = @1
      local.get 3
      i32.const 56
      i32.lt_u
      br_if 0 (;@1;)
      local.get 4
      local.get 5
      i32.add
      local.set 6
      i32.const 0
      local.set 5
      local.get 6
      i32.const 0
      i32.const 63
      local.get 3
      i32.sub
      call 15
      drop
      local.get 0
      local.get 4
      i32.const 1056
      call 3
    end
    local.get 4
    local.get 5
    i32.add
    i32.const 0
    i32.const 56
    local.get 5
    i32.sub
    call 15
    drop
    local.get 0
    local.get 0
    i32.load offset=104
    i32.const 3
    i32.shl
    local.tee 3
    i32.store8 offset=103
    local.get 0
    local.get 0
    i64.load offset=32
    local.tee 7
    i64.const 23
    i64.shr_u
    i64.store8 offset=99
    local.get 0
    local.get 7
    i64.const 31
    i64.shr_u
    i64.store8 offset=98
    local.get 0
    local.get 7
    i64.const 39
    i64.shr_u
    i64.store8 offset=97
    local.get 0
    local.get 7
    i64.const 47
    i64.shr_u
    i64.store8 offset=96
    local.get 0
    local.get 7
    i64.const 9
    i64.shl
    local.get 3
    i64.extend_i32_u
    i64.or
    local.tee 7
    i64.const 8
    i64.shr_u
    i64.store8 offset=102
    local.get 0
    local.get 7
    i64.const 16
    i64.shr_u
    i64.store8 offset=101
    local.get 0
    local.get 7
    i64.const 24
    i64.shr_u
    i64.store8 offset=100
    local.get 0
    local.get 4
    i32.const 1056
    call 3
    local.get 1
    local.get 2
    local.get 0
    call 2
    local.get 0
    i32.const 0
    i32.store offset=104
    local.get 0
    i64.const 0
    i64.store offset=32
    local.get 0
    i32.const 24
    i32.add
    i32.const 0
    i64.load offset=1048
    i64.store
    local.get 0
    i32.const 16
    i32.add
    i32.const 0
    i64.load offset=1040
    i64.store
    local.get 0
    i32.const 8
    i32.add
    i32.const 0
    i64.load offset=1032
    i64.store
    local.get 0
    i32.const 0
    i64.load offset=1024
    i64.store)
  (func (;6;) (type 0))
  (func (;7;) (type 2) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 112
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i32.const 24
      i32.add
      local.set 3
      local.get 2
      i32.const 16
      i32.add
      local.set 4
      local.get 2
      i32.const 8
      i32.add
      local.set 5
      i32.const 0
      local.set 6
      loop  ;; label = @2
        local.get 1
        local.set 7
        block  ;; label = @3
          local.get 1
          i32.eqz
          br_if 0 (;@3;)
          loop  ;; label = @4
            i32.const 0
            i64.const 0
            i64.store offset=1416
            i32.const 0
            i64.const 0
            i64.store offset=1408
            i32.const 0
            i64.const 0
            i64.store offset=1400
            i32.const 0
            i64.const 0
            i64.store offset=1392
            local.get 3
            i32.const 0
            i64.load offset=1048
            i64.store
            local.get 4
            i32.const 0
            i64.load offset=1040
            i64.store
            local.get 5
            i32.const 0
            i64.load offset=1032
            i64.store
            local.get 2
            i32.const 0
            i64.load offset=1024
            i64.store
            local.get 2
            i32.const 0
            i32.store offset=104
            local.get 2
            i64.const 0
            i64.store offset=32
            local.get 2
            i32.const 56
            i32.const 1312
            call 4
            local.get 2
            i32.const 32
            i32.const 1392
            call 5
            local.get 7
            i32.const -1
            i32.add
            local.tee 7
            br_if 0 (;@4;)
          end
        end
        local.get 6
        i32.const 1
        i32.add
        local.tee 6
        local.get 0
        i32.ne
        br_if 0 (;@2;)
      end
    end
    local.get 2
    i32.const 112
    i32.add
    global.set 0)
  (func (;8;) (type 3) (result i32)
    i32.const 562
    i32.const 1
    call 7
    i32.const 0)
  (func (;9;) (type 4) (param i32 i32) (result i32)
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
  (func (;11;) (type 3) (result i32)
    i32.const 0
    i32.const 0
    call 9)
  (func (;12;) (type 0))
  (func (;13;) (type 0))
  (func (;14;) (type 5) (param i32 i32 i32) (result i32)
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
  (func (;15;) (type 5) (param i32 i32 i32) (result i32)
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
  (global (;0;) (mut i32) (i32.const 17424))
  (global (;1;) i32 (i32.const 0))
  (global (;2;) i32 (i32.const 1424))
  (global (;3;) i32 (i32.const 17424))
  (export "memory" (memory 0))
  (export "_initialize" (func 1))
  (export "_run" (func 11))
  (export "__data_end" (global 2))
  (export "__heap_base" (global 3))
  (data (;0;) (i32.const 1024) "g\e6\09j\85\aeg\bbr\f3n<:\f5O\a5\7fR\0eQ\8ch\05\9b\ab\d9\83\1f\19\cd\e0[\98/\8aB\91D7q\cf\fb\c0\b5\a5\db\b5\e9[\c2V9\f1\11\f1Y\a4\82?\92\d5^\1c\ab\98\aa\07\d8\01[\83\12\be\851$\c3}\0cUt]\ber\fe\b1\de\80\a7\06\dc\9bt\f1\9b\c1\c1i\9b\e4\86G\be\ef\c6\9d\c1\0f\cc\a1\0c$o,\e9-\aa\84tJ\dc\a9\b0\5c\da\88\f9vRQ>\98m\c61\a8\c8'\03\b0\c7\7fY\bf\f3\0b\e0\c6G\91\a7\d5Qc\ca\06g))\14\85\0a\b7'8!\1b.\fcm,M\13\0d8STs\0ae\bb\0ajv.\c9\c2\81\85,r\92\a1\e8\bf\a2Kf\1a\a8p\8bK\c2\a3Ql\c7\19\e8\92\d1$\06\99\d6\855\0e\f4p\a0j\10\16\c1\a4\19\08l7\1eLwH'\b5\bc\b04\b3\0c\1c9J\aa\d8NO\ca\9c[\f3o.h\ee\82\8ftoc\a5x\14x\c8\84\08\02\c7\8c\fa\ff\be\90\eblP\a4\f7\a3\f9\be\f2xq\c6")
  (data (;1;) (i32.const 1312) "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"))

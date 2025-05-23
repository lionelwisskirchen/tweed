(module
  (type (;0;) (func))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (result i32)))
  (type (;3;) (func (param i32 i32) (result i32)))
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
  (func (;2;) (type 0))
  (func (;3;) (type 1) (param i32)
    block  ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      loop  ;; label = @2
        call 4
        local.get 0
        i32.const -1
        i32.add
        local.tee 0
        br_if 0 (;@2;)
      end
    end)
  (func (;4;) (type 0)
    (local i32 i32 i32 i32 i64)
    i32.const 0
    local.set 0
    loop  ;; label = @1
      local.get 0
      local.set 1
      i32.const 0
      local.set 2
      i32.const 0
      local.set 3
      loop  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.const 3
          i32.gt_u
          br_if 0 (;@3;)
          local.get 1
          i32.const 1280
          i32.add
          i32.load8_s
          i32.const 128
          i32.add
          local.get 1
          i32.const 1408
          i32.add
          i32.load8_s
          i32.mul
          local.get 3
          i32.add
          local.set 3
        end
        local.get 1
        i32.const 32
        i32.add
        local.set 1
        local.get 2
        i32.const 1
        i32.add
        local.tee 2
        i32.const 4
        i32.ne
        br_if 0 (;@2;)
      end
      local.get 0
      i32.const 1552
      i32.add
      i64.const 1
      i32.const 31
      local.get 0
      i32.const 2
      i32.shl
      local.tee 1
      i32.const 1152
      i32.add
      i32.load
      i32.sub
      i64.extend_i32_s
      local.tee 4
      i64.const -1
      i64.add
      i64.shl
      local.get 1
      i32.const 1024
      i32.add
      i64.load32_s
      local.get 3
      i64.extend_i32_s
      i64.mul
      i64.add
      local.get 4
      i64.shr_s
      i32.wrap_i64
      local.tee 1
      i32.const -128
      local.get 1
      i32.const -128
      i32.gt_s
      select
      local.tee 1
      i32.const 127
      local.get 1
      i32.const 127
      i32.lt_s
      select
      i32.store8
      local.get 0
      i32.const 1
      i32.add
      local.tee 0
      i32.const 32
      i32.ne
      br_if 0 (;@1;)
    end)
  (func (;5;) (type 2) (result i32)
    i32.const 1639
    call 3
    i32.const 0)
  (func (;6;) (type 3) (param i32 i32) (result i32)
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
  (memory (;0;) 1)
  (global (;0;) (mut i32) (i32.const 17584))
  (global (;1;) i32 (i32.const 0))
  (global (;2;) i32 (i32.const 1584))
  (global (;3;) i32 (i32.const 17584))
  (export "memory" (memory 0))
  (export "_initialize" (func 1))
  (export "_run" (func 8))
  (export "__data_end" (global 2))
  (export "__heap_base" (global 3))
  (data (;0;) (i32.const 1024) "\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\b6J\b7D\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff\f8\ff\ff\ff<9>D8\22\22*?9\1c\1a$\18\07\19\1f\00\1d#\01\0b\01\1e\1b\12\0c\13\09\08\fe\fd;=<?4\1e\1e,?9\18\16\1d\17\f7\1b#\0c\1c\22\fe\12\0a\1c\19\1a\18\0d\06\0f\fc\fb19=;*%&->5\13\11\22\19\f7 '\08\1d#\00\19\11\1a\18\1b\18\08\09\17\f8\fdA<67)\1b\1b&6.\12\16&\1e\00\1d%\0a\1e#\04$\1b!\1f\16\11\09\13#\fe\01\cf\c5+\ba\e5/\01\5c\cd).\cd\d6,74\c5?D\c4\b78<0L6\bf\d2?\a9=K\83\ef\9e\ee\e3=\e0*\b3\cf'\c9\f5+\eeG\fd\7f'\ba\bd\f0\c17\d4%&\e0\e9%\1f-R4\c7\ca2\ad:?\a6\b2\a0\06\f9I\f3\14\c9<=\d6\c64(+()\d6\cc(\ce*\19\d5\e16\0e>\be\d2\d1.e^\c2\c2\e3\ce%\1e)\cf\d32\c9.&\f5\da*\0d@\aa\e0\bb"))

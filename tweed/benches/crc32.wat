(module
  (type (;0;) (func))
  (type (;1;) (func (result i32)))
  (type (;2;) (func (param i32 i32) (result i32)))
  (type (;3;) (func (param i32)))
  (func (;0;) (type 0))
  (func (;1;) (type 0)
    block  ;; label = @1
      global.get 1
      i32.const 2048
      i32.add
      i32.load
      i32.eqz
      br_if 0 (;@1;)
      unreachable
    end
    global.get 1
    i32.const 2048
    i32.add
    i32.const 1
    i32.store
    call 0)
  (func (;2;) (type 1) (result i32)
    (local i32 i32)
    i32.const 1024
    local.set 0
    i32.const -1
    local.set 1
    loop  ;; label = @1
      call 7
      local.get 1
      i32.xor
      i32.const 255
      i32.and
      i32.const 2
      i32.shl
      i32.const 1024
      i32.add
      i32.load
      local.get 1
      i32.const 8
      i32.shr_u
      i32.xor
      local.set 1
      local.get 0
      i32.const -1
      i32.add
      local.tee 0
      br_if 0 (;@1;)
    end
    local.get 1
    i32.const -1
    i32.xor)
  (func (;3;) (type 0))
  (func (;4;) (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      i32.const 0
      return
    end
    i32.const 0
    local.set 2
    loop  ;; label = @1
      local.get 1
      local.set 3
      block  ;; label = @2
        local.get 1
        i32.eqz
        br_if 0 (;@2;)
        loop  ;; label = @3
          i32.const 0
          call 8
          call 2
          local.set 4
          local.get 3
          i32.const -1
          i32.add
          local.tee 3
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
    local.get 4
    i32.const 32767
    i32.and)
  (func (;5;) (type 1) (result i32)
    i32.const 170
    i32.const 1
    call 4)
  (func (;6;) (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    call 9
    call 3
    call 11
    local.get 2
    call 5
    i32.store offset=12
    call 12
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    i32.const 0)
  (func (;7;) (type 1) (result i32)
    (local i32)
    i32.const 0
    i32.const 0
    i32.load offset=2052
    i32.const 1103515245
    i32.mul
    i32.const 12345
    i32.add
    i32.const 2147483647
    i32.and
    local.tee 0
    i32.store offset=2052
    local.get 0
    i32.const 16
    i32.shr_u)
  (func (;8;) (type 3) (param i32)
    i32.const 0
    local.get 0
    i32.store offset=2052)
  (func (;9;) (type 0))
  (func (;10;) (type 1) (result i32)
    i32.const 0
    i32.const 0
    call 6)
  (func (;11;) (type 0))
  (func (;12;) (type 0))
  (memory (;0;) 1)
  (global (;0;) (mut i32) (i32.const 18064))
  (global (;1;) i32 (i32.const 0))
  (global (;2;) i32 (i32.const 2056))
  (global (;3;) i32 (i32.const 18064))
  (export "memory" (memory 0))
  (export "_initialize" (func 1))
  (export "_run" (func 10))
  (export "__data_end" (global 2))
  (export "__heap_base" (global 3))
  (data (;0;) (i32.const 1024) "\00\00\00\00\960\07w,a\0e\ee\baQ\09\99\19\c4m\07\8f\f4jp5\a5c\e9\a3\95d\9e2\88\db\0e\a4\b8\dcy\1e\e9\d5\e0\88\d9\d2\97+L\b6\09\bd|\b1~\07-\b8\e7\91\1d\bf\90d\10\b7\1d\f2 \b0jHq\b9\f3\deA\be\84}\d4\da\1a\eb\e4\ddmQ\b5\d4\f4\c7\85\d3\83V\98l\13\c0\a8kdz\f9b\fd\ec\c9e\8aO\5c\01\14\d9l\06cc=\0f\fa\f5\0d\08\8d\c8 n;^\10iL\e4A`\d5rqg\a2\d1\e4\03<G\d4\04K\fd\85\0d\d2k\b5\0a\a5\fa\a8\b55l\98\b2B\d6\c9\bb\db@\f9\bc\ac\e3l\d82u\5c\dfE\cf\0d\d6\dcY=\d1\ab\ac0\d9&:\00\deQ\80Q\d7\c8\16a\d0\bf\b5\f4\b4!#\c4\b3V\99\95\ba\cf\0f\a5\bd\b8\9e\b8\02(\08\88\05_\b2\d9\0c\c6$\e9\0b\b1\87|o/\11LhX\ab\1da\c1=-f\b6\90A\dcv\06q\db\01\bc \d2\98*\10\d5\ef\89\85\b1q\1f\b5\b6\06\a5\e4\bf\9f3\d4\b8\e8\a2\c9\07x4\f9\00\0f\8e\a8\09\96\18\98\0e\e1\bb\0dj\7f-=m\08\97ld\91\01\5cc\e6\f4Qkkbal\1c\d80e\85N\00b\f2\ed\95\06l{\a5\01\1b\c1\f4\08\82W\c4\0f\f5\c6\d9\b0eP\e9\b7\12\ea\b8\be\8b|\88\b9\fc\df\1d\ddbI-\da\15\f3|\d3\8ceL\d4\fbXa\b2M\ceQ\b5:t\00\bc\a3\e20\bb\d4A\a5\dfJ\d7\95\d8=m\c4\d1\a4\fb\f4\d6\d3j\e9iC\fc\d9n4F\88g\ad\d0\b8`\das-\04D\e5\1d\033_L\0a\aa\c9|\0d\dd<q\05P\aaA\02'\10\10\0b\be\86 \0c\c9%\b5hW\b3\85o \09\d4f\b9\9f\e4a\ce\0e\f9\de^\98\c9\d9)\22\98\d0\b0\b4\a8\d7\c7\17=\b3Y\81\0d\b4.;\5c\bd\b7\adl\ba\c0 \83\b8\ed\b6\b3\bf\9a\0c\e2\b6\03\9a\d2\b1t9G\d5\ea\afw\d2\9d\15&\db\04\83\16\dcs\12\0bc\e3\84;d\94>jm\0d\a8Zjz\0b\cf\0e\e4\9d\ff\09\93'\ae\00\0a\b1\9e\07}D\93\0f\f0\d2\a3\08\87h\f2\01\1e\fe\c2\06i]Wb\f7\cbge\80q6l\19\e7\06knv\1b\d4\fe\e0+\d3\89Zz\da\10\ccJ\ddgo\df\b9\f9\f9\ef\be\8eC\be\b7\17\d5\8e\b0`\e8\a3\d6\d6~\93\d1\a1\c4\c2\d88R\f2\dfO\f1g\bb\d1gW\bc\a6\dd\06\b5?K6\b2H\da+\0d\d8L\1b\0a\af\f6J\036`z\04A\c3\ef`\dfU\dfg\a8\ef\8en1y\beiF\8c\b3a\cb\1a\83f\bc\a0\d2o%6\e2hR\95w\0c\cc\03G\0b\bb\b9\16\02\22/&\05U\be;\ba\c5(\0b\bd\b2\92Z\b4+\04j\b3\5c\a7\ff\d7\c21\cf\d0\b5\8b\9e\d9,\1d\ae\de[\b0\c2d\9b&\f2c\ec\9c\a3ju\0a\93m\02\a9\06\09\9c?6\0e\eb\85g\07r\13W\00\05\82J\bf\95\14z\b8\e2\ae+\b1{8\1b\b6\0c\9b\8e\d2\92\0d\be\d5\e5\b7\ef\dc|!\df\db\0b\d4\d2\d3\86B\e2\d4\f1\f8\b3\ddhn\83\da\1f\cd\16\be\81[&\b9\f6\e1w\b0owG\b7\18\e6Z\08\88pj\0f\ff\ca;\06f\5c\0b\01\11\ff\9ee\8fi\aeb\f8\d3\ffkaE\cfl\16x\e2\0a\a0\ee\d2\0d\d7T\83\04N\c2\b3\039a&g\a7\f7\16`\d0MGiI\dbwn>Jj\d1\ae\dcZ\d6\d9f\0b\df@\f0;\d87S\ae\bc\a9\c5\9e\bb\de\7f\cf\b2G\e9\ff\b50\1c\f2\bd\bd\8a\c2\ba\ca0\93\b3S\a6\a3\b4$\056\d0\ba\93\06\d7\cd)W\deT\bfg\d9#.zf\b3\b8Ja\c4\02\1bh]\94+o*7\be\0b\b4\a1\8e\0c\c3\1b\df\05Z\8d\ef\02-"))

(module
  (type (;0;) (func))
  (type (;1;) (func (param i32 i32)))
  (type (;2;) (func (result i32)))
  (type (;3;) (func (param i32 i32) (result i32)))
  (func (;0;) (type 0))
  (func (;1;) (type 0)
    block  ;; label = @1
      global.get 1
      i32.const 1040
      i32.add
      i32.load
      i32.eqz
      br_if 0 (;@1;)
      unreachable
    end
    global.get 1
    i32.const 1040
    i32.add
    i32.const 1
    i32.store
    call 0)
  (func (;2;) (type 0))
  (func (;3;) (type 1) (param i32 i32)
    (local i32 i32 i32 i32)
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
            i32.const 0
            i32.const 3
            i32.store offset=1024
            i32.const 0
            i32.const 5
            i32.store offset=1028
            i32.const 0
            i32.const 0
            i32.store offset=1044
            block  ;; label = @5
              i32.const 0
              i32.load offset=1024
              i32.const 3
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1052
              i32.const 0
              i32.load offset=1056
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1048
              local.tee 4
              i32.const 0
              i32.load offset=1052
              local.tee 5
              i32.ge_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1024
              i32.const -3
              i32.add
              i32.store offset=1024
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 4
              local.get 5
              i32.sub
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1024
              i32.const 3
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1056
              i32.const 0
              i32.load offset=1052
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1048
              local.tee 4
              i32.const 0
              i32.load offset=1056
              local.tee 5
              i32.ge_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1024
              i32.const -3
              i32.add
              i32.store offset=1024
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 4
              local.get 5
              i32.sub
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1024
              i32.const 3
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1048
              i32.const 0
              i32.load offset=1056
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1052
              local.tee 4
              i32.const 0
              i32.load offset=1048
              local.tee 5
              i32.ge_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1024
              i32.const -3
              i32.add
              i32.store offset=1024
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 4
              local.get 5
              i32.sub
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1024
              i32.const 3
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1056
              i32.const 0
              i32.load offset=1048
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1052
              local.tee 4
              i32.const 0
              i32.load offset=1056
              local.tee 5
              i32.ge_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1024
              i32.const -3
              i32.add
              i32.store offset=1024
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 4
              local.get 5
              i32.sub
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1024
              i32.const 3
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1048
              i32.const 0
              i32.load offset=1052
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1056
              local.tee 4
              i32.const 0
              i32.load offset=1048
              local.tee 5
              i32.ge_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1024
              i32.const -3
              i32.add
              i32.store offset=1024
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 4
              local.get 5
              i32.sub
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1024
              i32.const 3
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1052
              i32.const 0
              i32.load offset=1048
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1056
              local.tee 4
              i32.const 0
              i32.load offset=1052
              local.tee 5
              i32.ge_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1024
              i32.const -3
              i32.add
              i32.store offset=1024
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 4
              local.get 5
              i32.sub
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 4
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              local.set 4
              i32.const 0
              i32.load offset=1076
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 4
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              local.set 4
              i32.const 0
              i32.load offset=1076
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 4
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              local.set 4
              i32.const 0
              i32.load offset=1080
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 4
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              local.set 4
              i32.const 0
              i32.load offset=1080
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 4
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              local.set 4
              i32.const 0
              i32.load offset=1084
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 4
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              local.set 4
              i32.const 0
              i32.load offset=1084
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 4
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              local.set 4
              i32.const 0
              i32.load offset=1072
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 4
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              local.set 4
              i32.const 0
              i32.load offset=1072
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 4
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              local.set 4
              i32.const 0
              i32.load offset=1080
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 4
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              local.set 4
              i32.const 0
              i32.load offset=1080
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 4
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              local.set 4
              i32.const 0
              i32.load offset=1084
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 4
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              local.set 4
              i32.const 0
              i32.load offset=1084
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 4
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              local.set 4
              i32.const 0
              i32.load offset=1072
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 4
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              local.set 4
              i32.const 0
              i32.load offset=1072
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 4
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              local.set 4
              i32.const 0
              i32.load offset=1076
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 4
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              local.set 4
              i32.const 0
              i32.load offset=1076
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 4
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              local.set 4
              i32.const 0
              i32.load offset=1084
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 4
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              local.set 4
              i32.const 0
              i32.load offset=1084
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 4
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              local.set 4
              i32.const 0
              i32.load offset=1072
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 4
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              local.set 4
              i32.const 0
              i32.load offset=1072
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 4
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              local.set 4
              i32.const 0
              i32.load offset=1076
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 4
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              local.set 4
              i32.const 0
              i32.load offset=1076
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 4
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              local.set 4
              i32.const 0
              i32.load offset=1080
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 4
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              local.set 4
              i32.const 0
              i32.load offset=1080
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              local.set 4
              i32.const 0
              i32.load offset=1076
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1084
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              local.set 4
              i32.const 0
              i32.load offset=1076
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1080
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              local.set 4
              i32.const 0
              i32.load offset=1076
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1084
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              local.set 4
              i32.const 0
              i32.load offset=1076
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1080
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              local.set 4
              i32.const 0
              i32.load offset=1080
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1084
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              local.set 4
              i32.const 0
              i32.load offset=1080
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1076
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              local.set 4
              i32.const 0
              i32.load offset=1080
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1084
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              local.set 4
              i32.const 0
              i32.load offset=1080
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1076
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              local.set 4
              i32.const 0
              i32.load offset=1084
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1080
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              local.set 4
              i32.const 0
              i32.load offset=1084
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1076
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              local.set 4
              i32.const 0
              i32.load offset=1084
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1080
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              local.set 4
              i32.const 0
              i32.load offset=1084
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1076
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              local.set 4
              i32.const 0
              i32.load offset=1088
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1084
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              local.set 4
              i32.const 0
              i32.load offset=1088
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1080
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              local.set 4
              i32.const 0
              i32.load offset=1088
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1084
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              local.set 4
              i32.const 0
              i32.load offset=1088
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1076
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              local.set 4
              i32.const 0
              i32.load offset=1088
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1080
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              local.set 4
              i32.const 0
              i32.load offset=1088
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1076
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              local.set 4
              i32.const 0
              i32.load offset=1072
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1084
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              local.set 4
              i32.const 0
              i32.load offset=1072
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1080
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              local.set 4
              i32.const 0
              i32.load offset=1072
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1084
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              local.set 4
              i32.const 0
              i32.load offset=1072
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1080
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              local.set 4
              i32.const 0
              i32.load offset=1080
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1084
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              local.set 4
              i32.const 0
              i32.load offset=1080
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              local.set 4
              i32.const 0
              i32.load offset=1080
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1084
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              local.set 4
              i32.const 0
              i32.load offset=1080
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              local.set 4
              i32.const 0
              i32.load offset=1084
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1080
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              local.set 4
              i32.const 0
              i32.load offset=1084
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              local.set 4
              i32.const 0
              i32.load offset=1084
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1080
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              local.set 4
              i32.const 0
              i32.load offset=1084
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              local.set 4
              i32.const 0
              i32.load offset=1088
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1084
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              local.set 4
              i32.const 0
              i32.load offset=1088
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1080
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              local.set 4
              i32.const 0
              i32.load offset=1088
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1084
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              local.set 4
              i32.const 0
              i32.load offset=1088
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              local.set 4
              i32.const 0
              i32.load offset=1088
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1080
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              local.set 4
              i32.const 0
              i32.load offset=1088
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              local.set 4
              i32.const 0
              i32.load offset=1072
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1084
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              local.set 4
              i32.const 0
              i32.load offset=1072
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1076
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              local.set 4
              i32.const 0
              i32.load offset=1072
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1084
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              local.set 4
              i32.const 0
              i32.load offset=1072
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1076
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              local.set 4
              i32.const 0
              i32.load offset=1076
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1084
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              local.set 4
              i32.const 0
              i32.load offset=1076
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              local.set 4
              i32.const 0
              i32.load offset=1076
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1084
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              local.set 4
              i32.const 0
              i32.load offset=1076
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              local.set 4
              i32.const 0
              i32.load offset=1084
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1076
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              local.set 4
              i32.const 0
              i32.load offset=1084
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              local.set 4
              i32.const 0
              i32.load offset=1084
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1076
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              local.set 4
              i32.const 0
              i32.load offset=1084
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              local.set 4
              i32.const 0
              i32.load offset=1088
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1084
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              local.set 4
              i32.const 0
              i32.load offset=1088
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1076
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              local.set 4
              i32.const 0
              i32.load offset=1088
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1084
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              local.set 4
              i32.const 0
              i32.load offset=1088
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              local.set 4
              i32.const 0
              i32.load offset=1088
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1076
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              local.set 4
              i32.const 0
              i32.load offset=1088
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              local.set 4
              i32.const 0
              i32.load offset=1072
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1080
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              local.set 4
              i32.const 0
              i32.load offset=1072
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1076
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              local.set 4
              i32.const 0
              i32.load offset=1072
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1080
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              local.set 4
              i32.const 0
              i32.load offset=1072
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1076
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              local.set 4
              i32.const 0
              i32.load offset=1076
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1080
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              local.set 4
              i32.const 0
              i32.load offset=1076
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              local.set 4
              i32.const 0
              i32.load offset=1076
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1080
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              local.set 4
              i32.const 0
              i32.load offset=1076
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              local.set 4
              i32.const 0
              i32.load offset=1080
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1076
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              local.set 4
              i32.const 0
              i32.load offset=1080
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              local.set 4
              i32.const 0
              i32.load offset=1080
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1076
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1088
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              local.set 4
              i32.const 0
              i32.load offset=1080
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              local.set 4
              i32.const 0
              i32.load offset=1088
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1080
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              local.set 4
              i32.const 0
              i32.load offset=1088
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1076
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              local.set 4
              i32.const 0
              i32.load offset=1088
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1080
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              local.set 4
              i32.const 0
              i32.load offset=1088
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              local.set 4
              i32.const 0
              i32.load offset=1088
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1076
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              local.set 4
              i32.const 0
              i32.load offset=1088
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              local.set 4
              i32.const 0
              i32.load offset=1072
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1084
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              local.set 4
              i32.const 0
              i32.load offset=1072
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1080
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              local.set 4
              i32.const 0
              i32.load offset=1072
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1084
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              local.set 4
              i32.const 0
              i32.load offset=1072
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1076
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              local.set 4
              i32.const 0
              i32.load offset=1072
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1080
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1072
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              local.set 4
              i32.const 0
              i32.load offset=1072
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1076
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              local.set 4
              i32.const 0
              i32.load offset=1076
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1084
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              local.set 4
              i32.const 0
              i32.load offset=1076
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1080
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              local.set 4
              i32.const 0
              i32.load offset=1076
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1084
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              local.set 4
              i32.const 0
              i32.load offset=1076
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              local.set 4
              i32.const 0
              i32.load offset=1076
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1080
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1076
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              local.set 4
              i32.const 0
              i32.load offset=1076
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              local.set 4
              i32.const 0
              i32.load offset=1080
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1084
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              local.set 4
              i32.const 0
              i32.load offset=1080
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1076
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              local.set 4
              i32.const 0
              i32.load offset=1080
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1084
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              local.set 4
              i32.const 0
              i32.load offset=1080
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              local.set 4
              i32.const 0
              i32.load offset=1080
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1076
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1084
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1080
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              local.set 4
              i32.const 0
              i32.load offset=1080
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              local.set 4
              i32.const 0
              i32.load offset=1084
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1080
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              local.set 4
              i32.const 0
              i32.load offset=1084
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1076
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              local.set 4
              i32.const 0
              i32.load offset=1084
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1080
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              local.set 4
              i32.const 0
              i32.load offset=1084
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1072
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              local.set 4
              i32.const 0
              i32.load offset=1084
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1076
              i32.store offset=1072
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1028
              i32.const 5
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1080
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1084
              i32.const 0
              i32.load offset=1076
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1088
              local.set 4
              i32.const 0
              i32.load offset=1084
              local.tee 5
              local.get 4
              i32.le_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1028
              i32.const -4
              i32.add
              i32.store offset=1028
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1104
              i32.add
              local.get 4
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1108
              i32.add
              local.get 5
              i32.store
              i32.const 0
              i32.load offset=1044
              i32.const 2
              i32.shl
              i32.const 1112
              i32.add
              local.get 5
              local.get 4
              i32.add
              i32.store
              i32.const 0
              i32.const 0
              i32.load offset=1044
              i32.const 3
              i32.add
              i32.store offset=1044
            end
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
  (func (;4;) (type 2) (result i32)
    i32.const 1232
    i32.const 1
    call 3
    i32.const 0)
  (func (;5;) (type 3) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    call 6
    call 2
    call 8
    local.get 2
    call 4
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
  (table (;0;) 1 1 funcref)
  (memory (;0;) 1)
  (global (;0;) (mut i32) (i32.const 17136))
  (global (;1;) i32 (i32.const 0))
  (global (;2;) i32 (i32.const 1128))
  (global (;3;) i32 (i32.const 17136))
  (export "memory" (memory 0))
  (export "_initialize" (func 1))
  (export "_run" (func 7))
  (export "__data_end" (global 2))
  (export "__heap_base" (global 3))
  (data (;0;) (i32.const 1024) "\03\00\00\00\05\00\00\00"))

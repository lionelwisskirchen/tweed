(module
  (type (;0;) (func))
  (type (;1;) (func (param i32 i32 i32 i32 i32) (result i32)))
  (type (;2;) (func (param i32 i32) (result i32)))
  (type (;3;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;4;) (func (result i32)))
  (type (;5;) (func (param i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;6;) (func (param i32 i32 i32) (result i32)))
  (type (;7;) (func (param i32) (result i32)))
  (func (;0;) (type 0))
  (func (;1;) (type 0)
    block  ;; label = @1
      global.get 1
      i32.const 1152
      i32.add
      i32.load
      i32.eqz
      br_if 0 (;@1;)
      unreachable
    end
    global.get 1
    i32.const 1152
    i32.add
    i32.const 1
    i32.store
    call 0)
  (func (;2;) (type 1) (param i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64)
    global.get 0
    i32.const 2432
    i32.sub
    local.tee 5
    global.set 0
    i32.const 0
    local.set 6
    local.get 5
    i32.const 0
    i32.store offset=2428
    local.get 5
    i32.const 0
    i32.store offset=2416
    local.get 5
    local.get 4
    i32.store offset=2424
    local.get 5
    local.get 3
    i32.store offset=2420
    block  ;; label = @1
      local.get 0
      i32.load align=1
      i32.const 694763304
      i32.ne
      br_if 0 (;@1;)
      local.get 5
      i32.const 1
      i32.store offset=2428
      local.get 0
      i32.const 4
      i32.add
      local.set 0
    end
    local.get 0
    call 18
    local.set 7
    local.get 5
    i32.const 1
    i32.store offset=1612
    local.get 5
    local.get 7
    i32.store offset=16
    local.get 5
    local.get 0
    i32.store offset=12
    i32.const 1
    local.set 8
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 7
            i32.const 1
            i32.lt_s
            br_if 0 (;@4;)
            local.get 7
            i32.const -3
            i32.add
            local.set 9
            local.get 7
            i32.const -1
            i32.add
            local.set 10
            local.get 5
            i32.const 1616
            i32.add
            local.set 11
            i32.const 0
            local.set 6
            i32.const 1
            local.set 8
            i32.const 1
            local.set 12
            i32.const 0
            local.set 13
            i32.const 0
            local.set 3
            loop  ;; label = @5
              local.get 0
              local.get 3
              i32.add
              local.tee 14
              local.get 7
              local.get 3
              i32.sub
              call 3
              local.set 15
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 14
                          i32.load8_u
                          local.tee 16
                          i32.const -40
                          i32.add
                          br_table 1 (;@10;) 4 (;@7;) 0 (;@11;)
                        end
                        block  ;; label = @11
                          local.get 16
                          i32.const 92
                          i32.eq
                          br_if 0 (;@11;)
                          local.get 16
                          i32.const 124
                          i32.ne
                          br_if 5 (;@6;)
                          block  ;; label = @12
                            local.get 6
                            i32.const 99
                            i32.le_s
                            br_if 0 (;@12;)
                            i32.const -8
                            local.set 16
                            br 11 (;@1;)
                          end
                          local.get 11
                          local.get 6
                          i32.const 3
                          i32.shl
                          i32.add
                          local.tee 12
                          local.get 14
                          i32.store offset=4
                          local.get 12
                          local.get 8
                          i32.const -1
                          i32.add
                          local.tee 14
                          local.get 13
                          local.get 5
                          i32.const 12
                          i32.add
                          local.get 14
                          i32.const 4
                          i32.shl
                          i32.add
                          i32.load offset=4
                          i32.const -1
                          i32.eq
                          select
                          i32.store
                          local.get 6
                          i32.const 1
                          i32.add
                          local.set 6
                          br 2 (;@9;)
                        end
                        i32.const -6
                        local.set 16
                        local.get 3
                        local.get 10
                        i32.ge_s
                        br_if 9 (;@1;)
                        block  ;; label = @11
                          local.get 14
                          i32.const 1
                          i32.add
                          i32.load8_u
                          local.tee 17
                          i32.const 120
                          i32.ne
                          br_if 0 (;@11;)
                          local.get 3
                          local.get 9
                          i32.ge_s
                          br_if 10 (;@1;)
                          local.get 14
                          i32.const 2
                          i32.add
                          i32.load8_u
                          call 15
                          i32.eqz
                          br_if 10 (;@1;)
                          local.get 14
                          i32.const 3
                          i32.add
                          i32.load8_u
                          call 15
                          br_if 5 (;@6;)
                          br 10 (;@1;)
                        end
                        i32.const 1024
                        local.get 17
                        i32.const 16
                        call 17
                        br_if 4 (;@6;)
                        br 9 (;@1;)
                      end
                      block  ;; label = @10
                        local.get 12
                        i32.const 99
                        i32.le_s
                        br_if 0 (;@10;)
                        i32.const -9
                        local.set 16
                        br 9 (;@1;)
                      end
                      local.get 5
                      i32.const 12
                      i32.add
                      local.get 12
                      i32.const 4
                      i32.shl
                      i32.add
                      local.tee 8
                      i32.const -1
                      i32.store offset=4
                      local.get 8
                      local.get 14
                      i32.const 1
                      i32.add
                      i32.store
                      local.get 12
                      i32.const 1
                      i32.add
                      local.set 8
                      local.get 13
                      i32.const 1
                      i32.add
                      local.set 13
                      local.get 4
                      i32.const 1
                      i32.ge_s
                      br_if 1 (;@8;)
                    end
                    local.get 8
                    local.set 12
                    br 2 (;@6;)
                  end
                  local.get 12
                  local.get 4
                  i32.gt_s
                  local.set 14
                  local.get 8
                  local.set 12
                  local.get 14
                  i32.eqz
                  br_if 1 (;@6;)
                  i32.const -7
                  local.set 16
                  br 6 (;@1;)
                end
                local.get 5
                i32.const 12
                i32.add
                local.get 12
                i32.const -1
                i32.add
                local.tee 16
                local.get 13
                local.get 5
                i32.const 12
                i32.add
                local.get 16
                i32.const 4
                i32.shl
                i32.add
                i32.load offset=4
                i32.const -1
                i32.eq
                select
                i32.const 4
                i32.shl
                i32.add
                local.tee 16
                local.get 14
                local.get 16
                i32.load
                i32.sub
                i32.store offset=4
                local.get 13
                i32.const 1
                i32.lt_s
                br_if 4 (;@2;)
                local.get 13
                i32.const -1
                i32.add
                local.set 13
                local.get 3
                i32.const 1
                i32.lt_s
                br_if 0 (;@6;)
                i32.const -1
                local.set 16
                local.get 14
                i32.const -1
                i32.add
                i32.load8_u
                i32.const 40
                i32.eq
                br_if 5 (;@1;)
              end
              local.get 15
              local.get 3
              i32.add
              local.tee 3
              local.get 7
              i32.lt_s
              br_if 0 (;@5;)
            end
            local.get 5
            local.get 6
            i32.store offset=2416
            local.get 5
            local.get 8
            i32.store offset=1612
            local.get 13
            br_if 2 (;@2;)
            block  ;; label = @5
              local.get 6
              i32.const 1
              i32.lt_s
              br_if 0 (;@5;)
              local.get 5
              i32.const 1624
              i32.add
              local.set 15
              local.get 5
              i32.const 1616
              i32.add
              local.set 12
              i32.const 0
              local.set 13
              loop  ;; label = @6
                block  ;; label = @7
                  local.get 13
                  local.tee 3
                  i32.const 1
                  i32.add
                  local.tee 13
                  local.get 6
                  i32.ge_s
                  br_if 0 (;@7;)
                  local.get 12
                  local.get 3
                  i32.const 3
                  i32.shl
                  i32.add
                  local.set 14
                  local.get 15
                  local.set 3
                  local.get 13
                  local.set 7
                  loop  ;; label = @8
                    block  ;; label = @9
                      local.get 14
                      i32.load
                      local.get 3
                      i32.load
                      i32.le_s
                      br_if 0 (;@9;)
                      local.get 14
                      i64.load align=4
                      local.set 18
                      local.get 14
                      local.get 3
                      i64.load align=4
                      i64.store align=4
                      local.get 3
                      local.get 18
                      i64.store align=4
                      local.get 5
                      i32.load offset=2416
                      local.set 6
                    end
                    local.get 3
                    i32.const 8
                    i32.add
                    local.set 3
                    local.get 7
                    i32.const 1
                    i32.add
                    local.tee 7
                    local.get 6
                    i32.lt_s
                    br_if 0 (;@8;)
                  end
                end
                local.get 15
                i32.const 8
                i32.add
                local.set 15
                local.get 13
                local.get 6
                i32.lt_s
                br_if 0 (;@6;)
              end
            end
            local.get 8
            i32.const 1
            i32.lt_s
            br_if 1 (;@3;)
          end
          i32.const 0
          local.get 6
          i32.sub
          local.set 16
          local.get 5
          i32.const 1616
          i32.add
          local.set 0
          i32.const 0
          local.set 12
          i32.const 0
          local.set 7
          loop  ;; label = @4
            local.get 5
            i32.const 12
            i32.add
            local.get 7
            i32.const 4
            i32.shl
            i32.add
            local.tee 13
            local.get 12
            i32.store offset=8
            local.get 13
            i32.const 0
            i32.store offset=12
            block  ;; label = @5
              local.get 12
              local.get 6
              i32.ge_s
              br_if 0 (;@5;)
              local.get 16
              local.get 12
              i32.add
              local.set 15
              local.get 0
              local.get 12
              i32.const 3
              i32.shl
              i32.add
              local.set 3
              i32.const 0
              local.set 14
              block  ;; label = @6
                loop  ;; label = @7
                  local.get 3
                  i32.load
                  local.get 7
                  i32.ne
                  br_if 1 (;@6;)
                  local.get 13
                  local.get 14
                  i32.const 1
                  i32.add
                  local.tee 14
                  i32.store offset=12
                  local.get 3
                  i32.const 8
                  i32.add
                  local.set 3
                  local.get 15
                  local.get 14
                  i32.add
                  br_if 0 (;@7;)
                end
                local.get 6
                local.set 12
                br 1 (;@5;)
              end
              local.get 12
              local.get 14
              i32.add
              local.set 12
            end
            local.get 7
            i32.const 1
            i32.add
            local.tee 7
            local.get 8
            i32.ne
            br_if 0 (;@4;)
          end
        end
        block  ;; label = @3
          local.get 2
          i32.const 0
          i32.ge_s
          br_if 0 (;@3;)
          i32.const -1
          local.set 16
          br 2 (;@1;)
        end
        local.get 5
        i32.load offset=12
        i32.load8_u
        i32.const 255
        i32.and
        i32.const 94
        i32.eq
        local.set 13
        local.get 2
        local.set 14
        i32.const 0
        local.set 3
        loop  ;; label = @3
          block  ;; label = @4
            local.get 1
            local.get 3
            i32.add
            local.get 14
            local.get 5
            i32.const 12
            i32.add
            i32.const 0
            call 4
            local.tee 16
            i32.const 0
            i32.lt_s
            br_if 0 (;@4;)
            local.get 16
            local.get 3
            i32.add
            local.set 16
            br 3 (;@1;)
          end
          local.get 13
          br_if 2 (;@1;)
          local.get 14
          i32.const -1
          i32.add
          local.set 14
          local.get 3
          local.get 2
          i32.lt_s
          local.set 7
          local.get 3
          i32.const 1
          i32.add
          local.set 3
          local.get 7
          br_if 0 (;@3;)
          br 2 (;@1;)
        end
      end
      i32.const -3
      local.set 16
    end
    local.get 5
    i32.const 2432
    i32.add
    global.set 0
    local.get 16)
  (func (;3;) (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32)
    i32.const 1
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load8_u
          i32.const -91
          i32.add
          br_table 0 (;@3;) 1 (;@2;) 2 (;@1;)
        end
        i32.const 0
        local.set 2
        block  ;; label = @3
          local.get 1
          i32.const 2
          i32.lt_s
          br_if 0 (;@3;)
          local.get 1
          i32.const -1
          i32.add
          local.set 3
          local.get 0
          i32.const 1
          i32.add
          local.set 4
          i32.const 0
          local.set 2
          loop  ;; label = @4
            i32.const 1
            local.set 0
            block  ;; label = @5
              block  ;; label = @6
                local.get 4
                local.get 2
                i32.add
                local.tee 5
                i32.load8_u
                i32.const -92
                i32.add
                br_table 0 (;@6;) 3 (;@3;) 1 (;@5;)
              end
              i32.const 4
              i32.const 2
              local.get 5
              i32.load8_u offset=1
              i32.const 120
              i32.eq
              select
              local.set 0
            end
            local.get 0
            local.get 2
            i32.add
            local.tee 2
            local.get 3
            i32.lt_s
            br_if 0 (;@4;)
          end
        end
        local.get 2
        i32.const 2
        i32.add
        i32.const 0
        local.get 2
        local.get 1
        i32.lt_s
        select
        return
      end
      i32.const 4
      i32.const 2
      local.get 0
      i32.load8_u offset=1
      i32.const 120
      i32.eq
      select
      local.set 2
    end
    local.get 2)
  (func (;4;) (type 3) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    local.get 2
    i32.const 1604
    i32.add
    local.set 4
    local.get 2
    local.get 3
    i32.const 4
    i32.shl
    i32.add
    local.set 5
    i32.const 0
    local.set 6
    loop  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 6
          br_if 0 (;@3;)
          local.get 5
          i32.load
          local.set 7
          br 1 (;@2;)
        end
        local.get 6
        local.get 5
        i32.load offset=8
        i32.add
        i32.const 3
        i32.shl
        local.get 4
        i32.add
        i32.const -4
        i32.add
        i32.load
        i32.const 1
        i32.add
        local.set 7
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 5
          i32.load offset=12
          local.tee 8
          br_if 0 (;@3;)
          local.get 5
          i32.load offset=4
          local.set 8
          br 1 (;@2;)
        end
        block  ;; label = @3
          local.get 6
          local.get 8
          i32.ne
          br_if 0 (;@3;)
          local.get 5
          i32.load
          local.get 5
          i32.load offset=4
          i32.add
          local.get 7
          i32.sub
          local.set 8
          br 1 (;@2;)
        end
        local.get 4
        local.get 6
        local.get 5
        i32.load offset=8
        i32.add
        i32.const 3
        i32.shl
        i32.add
        i32.load offset=4
        local.get 7
        i32.sub
        local.set 8
      end
      block  ;; label = @2
        local.get 7
        local.get 8
        local.get 0
        local.get 1
        local.get 2
        local.get 3
        call 8
        local.tee 8
        i32.const 0
        i32.gt_s
        br_if 0 (;@2;)
        local.get 6
        local.get 5
        i32.load offset=12
        i32.lt_s
        local.set 7
        local.get 6
        i32.const 1
        i32.add
        local.set 6
        local.get 7
        br_if 1 (;@1;)
      end
    end
    local.get 8)
  (func (;5;) (type 0))
  (func (;6;) (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      i32.const 0
      local.set 3
      loop  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          local.set 4
          loop  ;; label = @4
            i32.const 1088
            call 18
            local.set 5
            local.get 2
            i32.const 0
            i32.store offset=12
            i32.const -16
            local.set 6
            loop  ;; label = @5
              local.get 2
              local.get 6
              i32.const 1152
              i32.add
              i32.load
              i32.const 1088
              local.get 5
              local.get 2
              i32.const 4
              i32.add
              i32.const 1
              call 2
              local.get 2
              i32.load offset=12
              i32.add
              i32.store offset=12
              local.get 6
              i32.const 4
              i32.add
              local.tee 6
              br_if 0 (;@5;)
            end
            local.get 4
            i32.const 1
            i32.add
            local.tee 4
            local.get 1
            i32.ne
            br_if 0 (;@4;)
          end
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
    i32.load offset=12
    local.set 6
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 6)
  (func (;7;) (type 4) (result i32)
    i32.const 116
    i32.const 1
    call 6)
  (func (;8;) (type 5) (param i32 i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    i32.const 0
    local.set 6
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.const 1
              i32.lt_s
              br_if 0 (;@5;)
              local.get 3
              i32.const 0
              i32.lt_s
              br_if 0 (;@5;)
              local.get 3
              i32.const 1
              i32.add
              local.set 7
              local.get 1
              i32.const -2
              i32.add
              local.set 8
              i32.const 0
              local.set 9
              i32.const 0
              local.set 10
              i32.const 0
              local.set 11
              loop  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 0
                    local.get 10
                    i32.add
                    local.tee 6
                    i32.load8_u
                    local.tee 12
                    i32.const 40
                    i32.ne
                    br_if 0 (;@8;)
                    local.get 5
                    i32.const 4
                    i32.shl
                    local.get 4
                    i32.add
                    i32.const 20
                    i32.add
                    i32.load
                    i32.const 2
                    i32.add
                    local.set 13
                    br 1 (;@7;)
                  end
                  local.get 6
                  local.get 1
                  local.get 10
                  i32.sub
                  call 3
                  local.set 13
                  local.get 12
                  i32.const -42
                  i32.add
                  local.tee 14
                  i32.const 21
                  i32.gt_u
                  br_if 0 (;@7;)
                  i32.const 1
                  local.get 14
                  i32.shl
                  i32.const 2097155
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  i32.const -2
                  return
                end
                block  ;; label = @7
                  local.get 13
                  i32.const 1
                  i32.ge_s
                  br_if 0 (;@7;)
                  i32.const -5
                  return
                end
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 1
                            local.get 13
                            local.get 10
                            i32.add
                            local.tee 14
                            i32.le_s
                            br_if 0 (;@12;)
                            local.get 6
                            local.get 13
                            i32.add
                            i32.load8_u
                            i32.const -42
                            i32.add
                            local.tee 15
                            i32.const 21
                            i32.gt_u
                            br_if 0 (;@12;)
                            i32.const 1
                            local.get 15
                            i32.shl
                            i32.const 2097155
                            i32.and
                            i32.eqz
                            br_if 0 (;@12;)
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 0
                                local.get 14
                                i32.add
                                local.tee 15
                                i32.load8_u
                                local.tee 12
                                i32.const 63
                                i32.eq
                                br_if 0 (;@14;)
                                local.get 12
                                i32.const -42
                                i32.add
                                i32.const 2
                                i32.lt_u
                                br_if 1 (;@13;)
                                local.get 11
                                local.set 6
                                br 3 (;@11;)
                              end
                              local.get 6
                              local.get 13
                              local.get 2
                              local.get 11
                              i32.add
                              local.get 3
                              local.get 11
                              i32.sub
                              local.get 4
                              local.get 5
                              call 8
                              local.tee 6
                              i32.const 0
                              local.get 6
                              i32.const 0
                              i32.gt_s
                              select
                              local.get 11
                              i32.add
                              local.set 6
                              local.get 13
                              local.get 10
                              i32.add
                              i32.const 1
                              i32.add
                              local.set 14
                              br 2 (;@11;)
                            end
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 14
                                i32.const 1
                                i32.add
                                local.tee 16
                                local.get 1
                                i32.lt_s
                                br_if 0 (;@14;)
                                i32.const 1
                                local.set 9
                                br 1 (;@13;)
                              end
                              local.get 16
                              local.get 14
                              i32.const 2
                              i32.add
                              local.get 0
                              local.get 16
                              i32.add
                              i32.load8_u
                              i32.const 63
                              i32.ne
                              local.tee 9
                              select
                              local.set 16
                            end
                            local.get 1
                            local.get 16
                            i32.sub
                            local.set 17
                            local.get 0
                            local.get 16
                            i32.add
                            local.set 18
                            i32.const 43
                            local.set 19
                            local.get 11
                            local.set 10
                            local.get 11
                            local.set 0
                            i32.const -1
                            local.set 20
                            loop  ;; label = @13
                              block  ;; label = @14
                                local.get 6
                                local.get 13
                                local.get 2
                                local.get 10
                                i32.add
                                local.get 3
                                local.get 10
                                i32.sub
                                local.get 4
                                local.get 5
                                call 8
                                local.tee 12
                                i32.const -1
                                i32.gt_s
                                br_if 0 (;@14;)
                                local.get 15
                                i32.load8_u
                                i32.const 255
                                i32.and
                                i32.const 43
                                i32.eq
                                br_if 11 (;@3;)
                              end
                              local.get 12
                              i32.const 0
                              local.get 12
                              i32.const 0
                              i32.gt_s
                              select
                              local.get 10
                              i32.add
                              local.tee 10
                              local.set 14
                              block  ;; label = @14
                                local.get 1
                                local.get 16
                                i32.le_s
                                br_if 0 (;@14;)
                                local.get 0
                                local.get 18
                                local.get 17
                                local.get 2
                                local.get 10
                                i32.add
                                local.get 3
                                local.get 10
                                i32.sub
                                local.get 4
                                local.get 5
                                call 8
                                local.tee 20
                                local.get 10
                                i32.add
                                local.get 20
                                i32.const 0
                                i32.lt_s
                                select
                                local.set 14
                              end
                              block  ;; label = @14
                                local.get 12
                                i32.const 1
                                i32.lt_s
                                br_if 0 (;@14;)
                                local.get 14
                                local.set 0
                                local.get 9
                                local.get 14
                                local.get 11
                                i32.le_s
                                i32.or
                                br_if 1 (;@13;)
                              end
                            end
                            local.get 15
                            i32.load8_u
                            local.set 19
                            local.get 12
                            i32.const -1
                            i32.gt_s
                            br_if 8 (;@4;)
                            local.get 19
                            i32.const 255
                            i32.and
                            i32.const 42
                            i32.ne
                            br_if 8 (;@4;)
                            local.get 18
                            local.get 17
                            local.get 2
                            local.get 11
                            i32.add
                            local.get 3
                            local.get 11
                            i32.sub
                            local.get 4
                            local.get 5
                            call 8
                            local.tee 20
                            local.get 11
                            i32.add
                            local.get 14
                            local.get 20
                            i32.const 0
                            i32.gt_s
                            select
                            local.set 0
                            local.get 15
                            i32.load8_u
                            local.set 19
                            br 9 (;@3;)
                          end
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    local.get 12
                                    i32.const -36
                                    i32.add
                                    br_table 3 (;@13;) 4 (;@12;) 4 (;@12;) 4 (;@12;) 1 (;@15;) 0 (;@16;)
                                  end
                                  block  ;; label = @16
                                    local.get 12
                                    i32.const -91
                                    i32.add
                                    br_table 0 (;@16;) 4 (;@12;) 4 (;@12;) 2 (;@14;) 4 (;@12;)
                                  end
                                  i32.const 1
                                  local.set 13
                                  block  ;; label = @16
                                    local.get 8
                                    local.get 10
                                    i32.sub
                                    local.get 6
                                    i32.load8_u offset=1
                                    i32.const 94
                                    i32.eq
                                    local.tee 19
                                    i32.sub
                                    local.tee 20
                                    i32.const 0
                                    i32.lt_s
                                    br_if 0 (;@16;)
                                    local.get 6
                                    i32.const 1
                                    i32.add
                                    local.get 19
                                    i32.add
                                    local.tee 16
                                    i32.const 2
                                    i32.add
                                    local.set 21
                                    local.get 16
                                    i32.const 1
                                    i32.add
                                    local.set 18
                                    local.get 2
                                    local.get 11
                                    i32.add
                                    local.set 9
                                    i32.const 0
                                    local.set 6
                                    i32.const -1
                                    local.set 10
                                    block  ;; label = @17
                                      loop  ;; label = @18
                                        local.get 16
                                        local.get 6
                                        i32.add
                                        local.tee 15
                                        i32.load8_s
                                        local.tee 13
                                        i32.const 93
                                        i32.eq
                                        br_if 1 (;@17;)
                                        local.get 10
                                        i32.const 0
                                        i32.gt_s
                                        br_if 1 (;@17;)
                                        block  ;; label = @19
                                          block  ;; label = @20
                                            block  ;; label = @21
                                              local.get 13
                                              i32.const 45
                                              i32.ne
                                              br_if 0 (;@21;)
                                              i32.const 1
                                              local.set 12
                                              local.get 15
                                              local.get 9
                                              local.get 4
                                              call 9
                                              local.set 10
                                              br 1 (;@20;)
                                            end
                                            block  ;; label = @21
                                              local.get 18
                                              local.get 6
                                              i32.add
                                              i32.load8_u
                                              local.tee 17
                                              i32.const 45
                                              i32.ne
                                              br_if 0 (;@21;)
                                              local.get 21
                                              local.get 6
                                              i32.add
                                              i32.load8_u
                                              local.tee 10
                                              i32.eqz
                                              br_if 0 (;@21;)
                                              local.get 10
                                              i32.const 93
                                              i32.eq
                                              br_if 0 (;@21;)
                                              local.get 10
                                              i32.extend8_s
                                              local.set 15
                                              local.get 9
                                              i32.load8_s
                                              local.set 12
                                              block  ;; label = @22
                                                block  ;; label = @23
                                                  local.get 4
                                                  i32.load offset=2416
                                                  i32.eqz
                                                  br_if 0 (;@23;)
                                                  local.get 12
                                                  local.get 13
                                                  i32.ge_s
                                                  local.get 12
                                                  local.get 15
                                                  i32.le_s
                                                  i32.and
                                                  local.set 10
                                                  br 1 (;@22;)
                                                end
                                                i32.const 0
                                                local.set 10
                                                local.get 12
                                                call 16
                                                local.tee 12
                                                local.get 13
                                                call 16
                                                i32.lt_s
                                                br_if 0 (;@22;)
                                                local.get 12
                                                local.get 15
                                                call 16
                                                i32.le_s
                                                local.set 10
                                              end
                                              local.get 6
                                              i32.const 3
                                              i32.add
                                              local.set 6
                                              br 2 (;@19;)
                                            end
                                            i32.const 1
                                            local.set 12
                                            local.get 15
                                            local.get 9
                                            local.get 4
                                            call 9
                                            local.set 10
                                            local.get 13
                                            i32.const 92
                                            i32.ne
                                            br_if 0 (;@20;)
                                            i32.const 4
                                            i32.const 2
                                            local.get 17
                                            i32.const 120
                                            i32.eq
                                            select
                                            local.set 12
                                          end
                                          local.get 12
                                          local.get 6
                                          i32.add
                                          local.set 6
                                        end
                                        local.get 6
                                        local.get 20
                                        i32.le_s
                                        br_if 0 (;@18;)
                                      end
                                    end
                                    local.get 10
                                    i32.const 1
                                    i32.lt_s
                                    local.set 13
                                  end
                                  i32.const -1
                                  local.set 6
                                  i32.const 1
                                  i32.const -1
                                  local.get 13
                                  i32.const 1
                                  i32.and
                                  local.tee 13
                                  select
                                  i32.const -1
                                  local.get 19
                                  select
                                  local.tee 10
                                  local.get 10
                                  i32.const 1
                                  local.get 13
                                  select
                                  local.get 19
                                  select
                                  local.tee 13
                                  i32.const 1
                                  i32.lt_s
                                  br_if 10 (;@5;)
                                  local.get 13
                                  local.get 11
                                  i32.add
                                  local.set 6
                                  br 4 (;@11;)
                                end
                                block  ;; label = @15
                                  local.get 5
                                  i32.const 1
                                  i32.add
                                  local.tee 10
                                  local.get 4
                                  i32.load offset=1600
                                  i32.lt_s
                                  br_if 0 (;@15;)
                                  i32.const -4
                                  return
                                end
                                local.get 1
                                local.get 14
                                i32.sub
                                local.tee 16
                                i32.const 1
                                i32.lt_s
                                br_if 4 (;@10;)
                                local.get 3
                                local.get 11
                                i32.lt_s
                                br_if 12 (;@2;)
                                local.get 7
                                local.get 11
                                i32.sub
                                local.set 12
                                local.get 6
                                local.get 13
                                i32.add
                                local.set 20
                                local.get 2
                                local.get 11
                                i32.add
                                local.set 15
                                local.get 9
                                local.get 3
                                i32.add
                                local.tee 9
                                local.set 13
                                loop  ;; label = @15
                                  block  ;; label = @16
                                    local.get 15
                                    local.get 13
                                    local.get 4
                                    local.get 10
                                    call 4
                                    local.tee 6
                                    i32.const 0
                                    i32.lt_s
                                    br_if 0 (;@16;)
                                    local.get 20
                                    local.get 16
                                    local.get 15
                                    local.get 6
                                    i32.add
                                    local.get 9
                                    local.get 6
                                    i32.sub
                                    local.get 4
                                    local.get 10
                                    call 8
                                    i32.const -1
                                    i32.gt_s
                                    br_if 8 (;@8;)
                                  end
                                  local.get 13
                                  i32.const -1
                                  i32.add
                                  local.set 13
                                  local.get 12
                                  i32.const -1
                                  i32.add
                                  local.tee 12
                                  br_if 0 (;@15;)
                                  br 6 (;@9;)
                                end
                              end
                              i32.const 0
                              local.set 6
                              local.get 5
                              local.set 10
                              local.get 11
                              br_if 11 (;@2;)
                              br 6 (;@7;)
                            end
                            local.get 3
                            local.set 6
                            local.get 5
                            local.set 10
                            local.get 3
                            local.get 11
                            i32.ne
                            br_if 10 (;@2;)
                            br 5 (;@7;)
                          end
                          local.get 3
                          local.get 11
                          i32.le_s
                          br_if 9 (;@2;)
                          local.get 6
                          local.get 2
                          local.get 11
                          i32.add
                          local.get 4
                          call 9
                          local.tee 6
                          i32.const 1
                          i32.lt_s
                          br_if 6 (;@5;)
                          local.get 6
                          local.get 11
                          i32.add
                          local.set 6
                        end
                        local.get 5
                        local.set 10
                        br 3 (;@7;)
                      end
                      local.get 2
                      local.get 11
                      i32.add
                      local.get 3
                      local.get 11
                      i32.sub
                      local.get 4
                      local.get 10
                      call 4
                      local.set 6
                    end
                    local.get 6
                    i32.const 0
                    i32.lt_s
                    br_if 3 (;@5;)
                  end
                  block  ;; label = @8
                    local.get 4
                    i32.load offset=2408
                    local.tee 13
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 13
                    local.get 5
                    i32.const 3
                    i32.shl
                    i32.add
                    local.tee 13
                    local.get 6
                    i32.store offset=4
                    local.get 13
                    local.get 2
                    local.get 11
                    i32.add
                    i32.store
                  end
                  local.get 6
                  local.get 11
                  i32.add
                  local.set 6
                end
                local.get 14
                local.get 1
                i32.ge_s
                br_if 1 (;@5;)
                i32.const 0
                local.get 6
                i32.sub
                local.set 9
                local.get 10
                local.set 5
                local.get 14
                local.set 10
                local.get 6
                local.set 11
                local.get 6
                local.get 3
                i32.le_s
                br_if 0 (;@6;)
              end
            end
            local.get 6
            return
          end
          local.get 14
          local.set 0
        end
        local.get 19
        i32.const 255
        i32.and
        i32.const 43
        i32.ne
        br_if 1 (;@1;)
        local.get 0
        local.get 11
        i32.ne
        br_if 1 (;@1;)
      end
      i32.const -1
      return
    end
    i32.const -1
    local.get 0
    local.get 20
    i32.const 0
    i32.lt_s
    select
    local.get 0
    local.get 1
    local.get 16
    i32.gt_s
    select
    local.get 0
    local.get 0
    local.get 11
    i32.eq
    select)
  (func (;9;) (type 6) (param i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.load8_u
              local.tee 3
              i32.const 91
              i32.gt_s
              br_if 0 (;@5;)
              local.get 3
              i32.const 36
              i32.eq
              br_if 4 (;@1;)
              local.get 3
              i32.const 46
              i32.eq
              br_if 2 (;@3;)
              br 1 (;@4;)
            end
            block  ;; label = @5
              local.get 3
              i32.const 124
              i32.ne
              br_if 0 (;@5;)
              i32.const -4
              return
            end
            local.get 3
            i32.const 92
            i32.ne
            br_if 0 (;@4;)
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 0
                    i32.load8_u offset=1
                    local.tee 3
                    i32.const 114
                    i32.gt_s
                    br_if 0 (;@8;)
                    local.get 3
                    i32.const 83
                    i32.eq
                    br_if 1 (;@7;)
                    local.get 3
                    i32.const 100
                    i32.ne
                    br_if 3 (;@5;)
                    local.get 1
                    i32.load8_u
                    i32.const -48
                    i32.add
                    i32.const 10
                    i32.ge_u
                    br_if 7 (;@1;)
                    br 5 (;@3;)
                  end
                  local.get 3
                  i32.const 115
                  i32.eq
                  br_if 1 (;@6;)
                  local.get 3
                  i32.const 120
                  i32.ne
                  br_if 2 (;@5;)
                  local.get 0
                  i32.load8_u offset=2
                  call 16
                  local.set 3
                  local.get 0
                  i32.load8_u offset=3
                  call 16
                  local.tee 0
                  i32.const -48
                  i32.add
                  local.tee 2
                  local.get 0
                  i32.const -87
                  i32.add
                  local.get 2
                  i32.const 10
                  i32.lt_u
                  select
                  local.get 3
                  i32.const -48
                  i32.add
                  local.tee 0
                  local.get 3
                  i32.const 268435369
                  i32.add
                  local.get 0
                  i32.const 10
                  i32.lt_u
                  select
                  i32.const 4
                  i32.shl
                  i32.or
                  local.get 1
                  i32.load8_u
                  i32.ne
                  br_if 6 (;@1;)
                  br 4 (;@3;)
                end
                i32.const -1
                local.set 3
                local.get 1
                i32.load8_u
                local.tee 0
                i32.const 32
                i32.eq
                br_if 4 (;@2;)
                local.get 0
                i32.const -14
                i32.add
                i32.const -5
                i32.lt_u
                br_if 3 (;@3;)
                br 4 (;@2;)
              end
              local.get 1
              i32.load8_u
              local.tee 3
              i32.const 32
              i32.eq
              br_if 2 (;@3;)
              local.get 3
              i32.const -14
              i32.add
              i32.const -5
              i32.ge_u
              br_if 2 (;@3;)
              br 4 (;@1;)
            end
            local.get 3
            local.get 1
            i32.load8_u
            i32.ne
            br_if 3 (;@1;)
            br 1 (;@3;)
          end
          block  ;; label = @4
            local.get 2
            i32.load8_u offset=2416
            i32.const 1
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 3
            call 16
            local.get 1
            i32.load8_u
            call 16
            i32.ne
            br_if 3 (;@1;)
            br 1 (;@3;)
          end
          local.get 3
          local.get 1
          i32.load8_u
          i32.ne
          br_if 2 (;@1;)
        end
        i32.const 1
        local.set 3
      end
      local.get 3
      return
    end
    i32.const -1)
  (func (;10;) (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    call 11
    call 5
    call 13
    local.get 2
    call 7
    i32.store offset=12
    call 14
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    i32.const 0)
  (func (;11;) (type 0))
  (func (;12;) (type 4) (result i32)
    i32.const 0
    i32.const 0
    call 10)
  (func (;13;) (type 0))
  (func (;14;) (type 0))
  (func (;15;) (type 7) (param i32) (result i32)
    local.get 0
    i32.const -48
    i32.add
    i32.const 10
    i32.lt_u
    local.get 0
    i32.const 32
    i32.or
    i32.const -97
    i32.add
    i32.const 6
    i32.lt_u
    i32.or)
  (func (;16;) (type 7) (param i32) (result i32)
    local.get 0
    i32.const 32
    i32.or
    local.get 0
    local.get 0
    i32.const -65
    i32.add
    i32.const 26
    i32.lt_u
    select)
  (func (;17;) (type 6) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    local.get 2
    i32.const 0
    i32.ne
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.const 3
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.eqz
            br_if 0 (;@4;)
            block  ;; label = @5
              local.get 0
              i32.load8_u
              local.get 1
              i32.const 255
              i32.and
              i32.ne
              br_if 0 (;@5;)
              local.get 0
              local.set 4
              local.get 2
              local.set 5
              br 3 (;@2;)
            end
            local.get 2
            i32.const -1
            i32.add
            local.tee 5
            i32.const 0
            i32.ne
            local.set 3
            local.get 0
            i32.const 1
            i32.add
            local.tee 4
            i32.const 3
            i32.and
            i32.eqz
            br_if 1 (;@3;)
            local.get 5
            i32.eqz
            br_if 1 (;@3;)
            local.get 4
            i32.load8_u
            local.get 1
            i32.const 255
            i32.and
            i32.eq
            br_if 2 (;@2;)
            local.get 2
            i32.const -2
            i32.add
            local.tee 5
            i32.const 0
            i32.ne
            local.set 3
            local.get 0
            i32.const 2
            i32.add
            local.tee 4
            i32.const 3
            i32.and
            i32.eqz
            br_if 1 (;@3;)
            local.get 5
            i32.eqz
            br_if 1 (;@3;)
            local.get 4
            i32.load8_u
            local.get 1
            i32.const 255
            i32.and
            i32.eq
            br_if 2 (;@2;)
            local.get 2
            i32.const -3
            i32.add
            local.tee 5
            i32.const 0
            i32.ne
            local.set 3
            local.get 0
            i32.const 3
            i32.add
            local.tee 4
            i32.const 3
            i32.and
            i32.eqz
            br_if 1 (;@3;)
            local.get 5
            i32.eqz
            br_if 1 (;@3;)
            local.get 4
            i32.load8_u
            local.get 1
            i32.const 255
            i32.and
            i32.eq
            br_if 2 (;@2;)
            local.get 0
            i32.const 4
            i32.add
            local.set 4
            local.get 2
            i32.const -4
            i32.add
            local.tee 5
            i32.const 0
            i32.ne
            local.set 3
            br 1 (;@3;)
          end
          local.get 2
          local.set 5
          local.get 0
          local.set 4
        end
        local.get 3
        i32.eqz
        br_if 1 (;@1;)
        block  ;; label = @3
          local.get 4
          i32.load8_u
          local.get 1
          i32.const 255
          i32.and
          i32.eq
          br_if 0 (;@3;)
          local.get 5
          i32.const 4
          i32.lt_u
          br_if 0 (;@3;)
          local.get 1
          i32.const 255
          i32.and
          i32.const 16843009
          i32.mul
          local.set 0
          loop  ;; label = @4
            i32.const 16843008
            local.get 4
            i32.load
            local.get 0
            i32.xor
            local.tee 2
            i32.sub
            local.get 2
            i32.or
            i32.const -2139062144
            i32.and
            i32.const -2139062144
            i32.ne
            br_if 2 (;@2;)
            local.get 4
            i32.const 4
            i32.add
            local.set 4
            local.get 5
            i32.const -4
            i32.add
            local.tee 5
            i32.const 3
            i32.gt_u
            br_if 0 (;@4;)
          end
        end
        local.get 5
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 1
      i32.const 255
      i32.and
      local.set 2
      loop  ;; label = @2
        block  ;; label = @3
          local.get 4
          i32.load8_u
          local.get 2
          i32.ne
          br_if 0 (;@3;)
          local.get 4
          return
        end
        local.get 4
        i32.const 1
        i32.add
        local.set 4
        local.get 5
        i32.const -1
        i32.add
        local.tee 5
        br_if 0 (;@2;)
      end
    end
    i32.const 0)
  (func (;18;) (type 7) (param i32) (result i32)
    (local i32 i32 i32)
    local.get 0
    local.set 1
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 0
          i32.load8_u
          br_if 0 (;@3;)
          local.get 0
          local.get 0
          i32.sub
          return
        end
        local.get 0
        i32.const 1
        i32.add
        local.tee 1
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load8_u
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.const 2
        i32.add
        local.tee 1
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load8_u
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.const 3
        i32.add
        local.tee 1
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load8_u
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.const 4
        i32.add
        local.tee 1
        i32.const 3
        i32.and
        br_if 1 (;@1;)
      end
      local.get 1
      i32.const -4
      i32.add
      local.set 2
      local.get 1
      i32.const -5
      i32.add
      local.set 1
      loop  ;; label = @2
        local.get 1
        i32.const 4
        i32.add
        local.set 1
        i32.const 16843008
        local.get 2
        i32.const 4
        i32.add
        local.tee 2
        i32.load
        local.tee 3
        i32.sub
        local.get 3
        i32.or
        i32.const -2139062144
        i32.and
        i32.const -2139062144
        i32.eq
        br_if 0 (;@2;)
      end
      loop  ;; label = @2
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 2
        i32.load8_u
        local.set 3
        local.get 2
        i32.const 1
        i32.add
        local.set 2
        local.get 3
        br_if 0 (;@2;)
      end
    end
    local.get 1
    local.get 0
    i32.sub)
  (table (;0;) 1 1 funcref)
  (memory (;0;) 1)
  (global (;0;) (mut i32) (i32.const 17168))
  (global (;1;) i32 (i32.const 0))
  (global (;2;) i32 (i32.const 1156))
  (global (;3;) i32 (i32.const 17168))
  (export "memory" (memory 0))
  (export "_initialize" (func 1))
  (export "_run" (func 12))
  (export "__data_end" (global 2))
  (export "__heap_base" (global 3))
  (data (;0;) (i32.const 1024) "^$().[]*+?|\5cSsd\00(ab)+\00([ab^c][ab^c])+\00(b.+)+\00a[ab]*\00")
  (data (;1;) (i32.const 1088) "abbbababaabccababcacbcbcbabbabcbabcabcbbcbbac\00\00\00\10\04\00\00&\04\00\00-\04\00\00\16\04\00\00"))

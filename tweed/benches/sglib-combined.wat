(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (type (;1;) (func))
  (type (;2;) (func (param i32)))
  (type (;3;) (func (param i32) (result i32)))
  (type (;4;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;5;) (func (param i32 i32)))
  (type (;6;) (func (result i32)))
  (type (;7;) (func (param i32 i32 i32) (result i32)))
  (func (;0;) (type 1))
  (func (;1;) (type 1)
    block  ;; label = @1
      global.get 1
      i32.const 1424
      i32.add
      i32.load
      i32.eqz
      br_if 0 (;@1;)
      unreachable
    end
    global.get 1
    i32.const 1424
    i32.add
    i32.const 1
    i32.store
    call 0)
  (func (;2;) (type 2) (param i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.set 1
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      loop  ;; label = @2
        local.get 2
        local.tee 3
        i32.load offset=8
        local.tee 2
        br_if 0 (;@2;)
      end
      local.get 1
      i32.const 0
      i32.store offset=12
      i32.const 1
      local.set 4
      block  ;; label = @2
        loop  ;; label = @3
          local.get 1
          i32.const 12
          i32.add
          local.set 5
          local.get 4
          i32.const 2
          i32.lt_s
          local.set 6
          i32.const 1
          local.set 7
          block  ;; label = @4
            block  ;; label = @5
              loop  ;; label = @6
                local.get 3
                local.set 2
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 6
                      br_if 0 (;@9;)
                      i32.const 2
                      local.set 8
                      local.get 2
                      local.set 9
                      block  ;; label = @10
                        loop  ;; label = @11
                          local.get 9
                          i32.load offset=4
                          local.set 9
                          local.get 8
                          local.get 4
                          i32.ge_s
                          br_if 1 (;@10;)
                          local.get 8
                          i32.const 1
                          i32.add
                          local.set 8
                          local.get 9
                          br_if 0 (;@11;)
                        end
                      end
                      block  ;; label = @10
                        local.get 9
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 9
                        i32.load offset=4
                        local.set 8
                        local.get 9
                        i32.const 0
                        i32.store offset=4
                        local.get 6
                        br_if 2 (;@8;)
                        local.get 8
                        i32.eqz
                        br_if 2 (;@8;)
                        i32.const 2
                        local.set 3
                        local.get 8
                        local.set 9
                        loop  ;; label = @11
                          local.get 9
                          i32.load offset=4
                          local.set 9
                          local.get 3
                          local.get 4
                          i32.ge_s
                          br_if 4 (;@7;)
                          local.get 3
                          i32.const 1
                          i32.add
                          local.set 3
                          local.get 9
                          br_if 0 (;@11;)
                          br 4 (;@7;)
                        end
                      end
                      local.get 5
                      local.get 2
                      i32.store
                      local.get 7
                      i32.const 1
                      i32.and
                      i32.eqz
                      br_if 4 (;@5;)
                      local.get 1
                      i32.load offset=12
                      local.tee 9
                      i32.eqz
                      br_if 5 (;@4;)
                      i32.const 0
                      local.set 8
                      local.get 9
                      local.set 2
                      loop  ;; label = @10
                        local.get 2
                        local.tee 2
                        local.get 8
                        i32.store offset=8
                        local.get 2
                        local.set 8
                        local.get 2
                        i32.load offset=4
                        local.tee 2
                        br_if 0 (;@10;)
                        br 8 (;@2;)
                      end
                    end
                    local.get 2
                    i32.load offset=4
                    local.set 8
                    local.get 2
                    i32.const 0
                    i32.store offset=4
                  end
                  local.get 8
                  local.set 9
                end
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 9
                    br_if 0 (;@8;)
                    i32.const 0
                    local.set 3
                    br 1 (;@7;)
                  end
                  local.get 9
                  i32.load offset=4
                  local.set 3
                  local.get 9
                  i32.const 0
                  i32.store offset=4
                end
                block  ;; label = @7
                  local.get 8
                  i32.eqz
                  br_if 0 (;@7;)
                  loop  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 2
                        i32.load
                        local.get 8
                        i32.load
                        i32.ge_s
                        br_if 0 (;@10;)
                        local.get 5
                        local.get 2
                        i32.store
                        local.get 2
                        i32.const 4
                        i32.add
                        local.set 5
                        local.get 2
                        i32.load offset=4
                        local.set 2
                        br 1 (;@9;)
                      end
                      local.get 5
                      local.get 8
                      i32.store
                      local.get 8
                      i32.const 4
                      i32.add
                      local.set 5
                      local.get 8
                      i32.load offset=4
                      local.set 8
                    end
                    block  ;; label = @9
                      local.get 2
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 8
                      br_if 1 (;@8;)
                    end
                  end
                  local.get 2
                  local.get 8
                  local.get 2
                  select
                  local.set 2
                end
                local.get 5
                local.get 2
                i32.store
                block  ;; label = @7
                  local.get 2
                  i32.eqz
                  br_if 0 (;@7;)
                  loop  ;; label = @8
                    local.get 2
                    local.tee 8
                    i32.load offset=4
                    local.tee 2
                    br_if 0 (;@8;)
                  end
                  local.get 8
                  i32.const 4
                  i32.add
                  local.set 5
                end
                i32.const 0
                local.set 7
                local.get 3
                br_if 0 (;@6;)
              end
            end
            local.get 1
            i32.load offset=12
            local.set 3
            local.get 1
            i32.const 0
            i32.store offset=12
            local.get 4
            i32.const 1
            i32.shl
            local.set 4
            local.get 3
            br_if 1 (;@3;)
          end
        end
        i32.const 0
        local.set 9
      end
      local.get 0
      local.get 9
      i32.store
    end)
  (func (;3;) (type 3) (param i32) (result i32)
    (local i32 i32 i32)
    local.get 0
    i32.load offset=4
    local.set 1
    i32.const 0
    local.set 2
    local.get 0
    i32.const 0
    i32.store offset=4
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load offset=8
          local.tee 3
          i32.eqz
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 1
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            i32.load offset=12
            local.set 2
            loop  ;; label = @5
              block  ;; label = @6
                local.get 1
                local.get 2
                local.get 3
                call_indirect (type 0)
                br_if 0 (;@6;)
                local.get 0
                local.get 1
                i32.store
                br 4 (;@2;)
              end
              local.get 1
              i32.load offset=4
              local.tee 1
              br_if 0 (;@5;)
            end
          end
          local.get 0
          i32.const 0
          i32.store
          i32.const 0
          return
        end
        local.get 0
        local.get 1
        i32.store
        local.get 1
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 0
      local.get 1
      i32.load offset=4
      i32.store offset=4
      local.get 1
      local.set 2
    end
    local.get 2)
  (func (;4;) (type 4) (param i32 i32 i32 i32) (result i32)
    local.get 0
    local.get 3
    i32.store offset=28
    local.get 0
    local.get 2
    i32.store offset=24
    local.get 0
    local.get 1
    i32.store offset=16
    local.get 0
    i32.const 0
    i32.store offset=20
    local.get 0
    local.get 1
    i32.load
    i32.store offset=4
    local.get 0
    local.get 2
    i32.store offset=8
    local.get 0
    local.get 3
    i32.store offset=12
    block  ;; label = @1
      local.get 0
      call 3
      local.tee 3
      br_if 0 (;@1;)
      local.get 0
      call 5
      local.set 3
    end
    local.get 3)
  (func (;5;) (type 3) (param i32) (result i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 0
      call 3
      local.tee 1
      br_if 0 (;@1;)
      loop  ;; label = @2
        local.get 0
        local.get 0
        i32.load offset=20
        local.tee 1
        i32.const 1
        i32.add
        local.tee 2
        i32.store offset=20
        block  ;; label = @3
          local.get 1
          i32.const 18
          i32.le_s
          br_if 0 (;@3;)
          i32.const 0
          return
        end
        local.get 0
        i32.load offset=16
        local.get 2
        i32.const 2
        i32.shl
        i32.add
        i32.load
        local.set 1
        local.get 0
        local.get 0
        i64.load offset=24 align=4
        i64.store offset=8 align=4
        local.get 0
        local.get 1
        i32.store offset=4
        local.get 0
        call 3
        local.tee 1
        i32.eqz
        br_if 0 (;@2;)
      end
    end
    local.get 1)
  (func (;6;) (type 5) (param i32 i32)
    (local i32 i32 i32)
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 2
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.store
      local.get 1
      i32.const 1
      i32.store8 offset=4
      return
    end
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.load
          local.tee 3
          local.get 2
          i32.load
          local.tee 4
          i32.lt_s
          br_if 0 (;@3;)
          local.get 2
          local.get 1
          i32.le_u
          br_if 1 (;@2;)
          local.get 3
          local.get 4
          i32.ne
          br_if 1 (;@2;)
        end
        local.get 2
        i32.const 8
        i32.add
        local.get 1
        call 6
        local.get 2
        i32.load8_u offset=4
        br_if 1 (;@1;)
        local.get 0
        i32.load
        local.tee 2
        i32.load offset=8
        local.set 1
        block  ;; label = @3
          local.get 2
          i32.load offset=12
          local.tee 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          i32.load8_u offset=4
          i32.const 1
          i32.ne
          br_if 0 (;@3;)
          local.get 1
          i32.load8_u offset=4
          i32.const 1
          i32.ne
          br_if 2 (;@1;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.load offset=8
              local.tee 0
              i32.eqz
              br_if 0 (;@5;)
              local.get 0
              i32.load8_u offset=4
              i32.const 1
              i32.eq
              br_if 1 (;@4;)
            end
            local.get 1
            i32.load offset=12
            local.tee 0
            i32.eqz
            br_if 3 (;@1;)
            local.get 0
            i32.load8_u offset=4
            i32.const 1
            i32.ne
            br_if 3 (;@1;)
          end
          local.get 1
          i32.const 0
          i32.store8 offset=4
          local.get 3
          i32.const 0
          i32.store8 offset=4
          local.get 2
          i32.const 1
          i32.store8 offset=4
          return
        end
        local.get 1
        i32.load8_u offset=4
        i32.const 1
        i32.ne
        br_if 1 (;@1;)
        block  ;; label = @3
          local.get 1
          i32.load offset=8
          local.tee 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          i32.load8_u offset=4
          i32.const 1
          i32.ne
          br_if 0 (;@3;)
          local.get 2
          local.get 1
          i32.load offset=12
          i32.store offset=8
          local.get 2
          i32.const 1
          i32.store8 offset=4
          local.get 1
          local.get 2
          i32.store offset=12
          local.get 1
          local.get 3
          i32.store offset=8
          local.get 1
          i32.const 0
          i32.store8 offset=4
          local.get 0
          local.get 1
          i32.store
          return
        end
        local.get 1
        i32.load offset=12
        local.tee 3
        i32.eqz
        br_if 1 (;@1;)
        local.get 3
        i32.load8_u offset=4
        i32.const 1
        i32.ne
        br_if 1 (;@1;)
        local.get 3
        i32.load offset=8
        local.set 4
        local.get 2
        local.get 3
        i32.load offset=12
        i32.store offset=8
        local.get 1
        local.get 4
        i32.store offset=12
        local.get 3
        i32.const 0
        i32.store8 offset=4
        local.get 3
        local.get 1
        i32.store offset=8
        local.get 3
        local.get 2
        i32.store offset=12
        local.get 2
        i32.const 1
        i32.store8 offset=4
        local.get 0
        local.get 3
        i32.store
        return
      end
      local.get 2
      i32.const 12
      i32.add
      local.get 1
      call 6
      local.get 2
      i32.load8_u offset=4
      br_if 0 (;@1;)
      local.get 0
      i32.load
      local.tee 2
      i32.load offset=12
      local.set 1
      block  ;; label = @2
        local.get 2
        i32.load offset=8
        local.tee 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.load8_u offset=4
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        local.get 1
        i32.load8_u offset=4
        i32.const 1
        i32.ne
        br_if 1 (;@1;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.load offset=12
            local.tee 0
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            i32.load8_u offset=4
            i32.const 1
            i32.eq
            br_if 1 (;@3;)
          end
          local.get 1
          i32.load offset=8
          local.tee 0
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          i32.load8_u offset=4
          i32.const 1
          i32.ne
          br_if 2 (;@1;)
        end
        local.get 1
        i32.const 0
        i32.store8 offset=4
        local.get 3
        i32.const 0
        i32.store8 offset=4
        local.get 2
        i32.const 1
        i32.store8 offset=4
        return
      end
      local.get 1
      i32.load8_u offset=4
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 1
        i32.load offset=12
        local.tee 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.load8_u offset=4
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        local.get 2
        local.get 1
        i32.load offset=8
        i32.store offset=12
        local.get 2
        i32.const 1
        i32.store8 offset=4
        local.get 1
        local.get 2
        i32.store offset=8
        local.get 1
        local.get 3
        i32.store offset=12
        local.get 1
        i32.const 0
        i32.store8 offset=4
        local.get 0
        local.get 1
        i32.store
        return
      end
      local.get 1
      i32.load offset=8
      local.tee 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.load8_u offset=4
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=12
      local.set 4
      local.get 2
      local.get 3
      i32.load offset=8
      i32.store offset=12
      local.get 1
      local.get 4
      i32.store offset=8
      local.get 3
      i32.const 0
      i32.store8 offset=4
      local.get 3
      local.get 1
      i32.store offset=12
      local.get 3
      local.get 2
      i32.store offset=8
      local.get 2
      i32.const 1
      i32.store8 offset=4
      local.get 0
      local.get 3
      i32.store
    end)
  (func (;7;) (type 2) (param i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    local.get 0
    i32.const 0
    i32.store
    block  ;; label = @1
      local.get 0
      i32.load16_s offset=644
      local.tee 1
      i32.const 1
      i32.lt_s
      br_if 0 (;@1;)
      local.get 0
      i32.load
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=652
      local.set 2
      local.get 0
      i32.load offset=648
      local.set 3
      local.get 0
      i32.const 132
      i32.add
      local.set 4
      local.get 0
      i32.const 4
      i32.add
      local.set 5
      loop  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 5
            local.get 1
            i32.const 65535
            i32.and
            local.tee 6
            i32.const -1
            i32.add
            local.tee 7
            i32.add
            local.tee 8
            i32.load8_s
            local.tee 9
            i32.const 2
            i32.lt_s
            br_if 0 (;@4;)
            local.get 0
            local.get 1
            i32.const -1
            i32.add
            local.tee 1
            i32.store16 offset=644
            br 1 (;@3;)
          end
          local.get 4
          local.get 7
          i32.const 2
          i32.shl
          i32.add
          i32.load
          i32.const 12
          i32.const 8
          local.get 9
          select
          i32.add
          i32.load
          local.set 1
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 3
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 2
                  i32.eqz
                  br_if 1 (;@6;)
                  loop  ;; label = @8
                    local.get 1
                    i32.eqz
                    br_if 4 (;@4;)
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 3
                        local.get 1
                        local.get 2
                        call_indirect (type 0)
                        local.tee 7
                        i32.const 0
                        i32.ge_s
                        br_if 0 (;@10;)
                        i32.const 8
                        local.set 7
                        br 1 (;@9;)
                      end
                      local.get 7
                      i32.eqz
                      br_if 4 (;@5;)
                      i32.const 12
                      local.set 7
                    end
                    local.get 1
                    local.get 7
                    i32.add
                    i32.load
                    local.set 1
                    br 0 (;@8;)
                  end
                end
                local.get 1
                i32.eqz
                br_if 2 (;@4;)
                br 1 (;@5;)
              end
              local.get 1
              i32.eqz
              br_if 1 (;@4;)
              local.get 3
              i32.load
              local.set 9
              loop  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 9
                    local.get 1
                    i32.load
                    local.tee 7
                    i32.ge_s
                    br_if 0 (;@8;)
                    i32.const 8
                    local.set 7
                    br 1 (;@7;)
                  end
                  local.get 9
                  local.get 7
                  i32.eq
                  br_if 2 (;@5;)
                  i32.const 12
                  local.set 7
                end
                local.get 1
                local.get 7
                i32.add
                i32.load
                local.tee 1
                br_if 0 (;@6;)
                br 2 (;@4;)
              end
            end
            local.get 5
            local.get 6
            i32.add
            i32.const 0
            i32.store8
            local.get 4
            local.get 6
            i32.const 2
            i32.shl
            i32.add
            local.get 1
            i32.store
            local.get 0
            local.get 0
            i32.load16_u offset=644
            i32.const 1
            i32.add
            i32.store16 offset=644
          end
          local.get 8
          local.get 8
          i32.load8_u
          i32.const 1
          i32.add
          i32.store8
          local.get 0
          i32.load16_u offset=644
          local.set 1
        end
        local.get 1
        i32.extend16_s
        i32.const 1
        i32.lt_s
        br_if 1 (;@1;)
        block  ;; label = @3
          local.get 0
          i32.load16_u offset=646
          local.get 5
          local.get 1
          i32.const 65535
          i32.and
          i32.const -1
          i32.add
          local.tee 7
          i32.add
          i32.load8_s
          i32.const 65535
          i32.and
          i32.ne
          br_if 0 (;@3;)
          local.get 0
          local.get 4
          local.get 7
          i32.const 2
          i32.shl
          i32.add
          i32.load
          i32.store
        end
        local.get 0
        i32.load
        i32.eqz
        br_if 0 (;@2;)
      end
    end)
  (func (;8;) (type 0) (param i32 i32) (result i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      loop  ;; label = @2
        br 0 (;@2;)
      end
    end
    local.get 0
    i64.const 0
    i64.store offset=648 align=4
    local.get 0
    i32.const 1
    i32.store16 offset=646
    block  ;; label = @1
      local.get 1
      br_if 0 (;@1;)
      local.get 0
      i32.const 0
      i32.store
      local.get 0
      i32.const 0
      i32.store16 offset=644
      i32.const 0
      return
    end
    local.get 0
    local.get 1
    i32.store offset=132
    local.get 0
    i32.const 0
    i32.store8 offset=4
    local.get 0
    i32.const 1
    i32.store16 offset=644
    local.get 0
    call 7
    local.get 0
    i32.load)
  (func (;9;) (type 1))
  (func (;10;) (type 0) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 1120
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
            i32.const 9712
            i32.const 1024
            i32.const 400
            call 19
            local.set 5
            local.get 2
            i32.const 0
            i32.store
            local.get 2
            i32.const 100
            i32.store offset=672
            i32.const 1
            local.set 6
            loop  ;; label = @5
              block  ;; label = @6
                local.get 2
                i32.const 672
                i32.add
                local.get 6
                i32.const -1
                i32.add
                local.tee 6
                i32.const 2
                i32.shl
                local.tee 7
                i32.add
                i32.load
                local.tee 8
                local.get 2
                local.get 7
                i32.add
                i32.load
                local.tee 9
                i32.sub
                local.tee 7
                i32.const 3
                i32.lt_s
                br_if 0 (;@6;)
                loop  ;; label = @7
                  block  ;; label = @8
                    local.get 9
                    i32.const 1
                    i32.add
                    local.tee 7
                    local.get 8
                    i32.const -1
                    i32.add
                    local.tee 10
                    i32.ge_s
                    br_if 0 (;@8;)
                    local.get 9
                    i32.const 2
                    i32.shl
                    local.get 5
                    i32.add
                    local.set 11
                    loop  ;; label = @9
                      local.get 7
                      local.get 10
                      local.get 7
                      local.get 10
                      i32.gt_s
                      select
                      local.set 12
                      local.get 7
                      i32.const -1
                      i32.add
                      local.set 13
                      local.get 7
                      i32.const 2
                      i32.shl
                      local.get 5
                      i32.add
                      local.set 7
                      local.get 11
                      i32.load
                      local.set 14
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            loop  ;; label = @13
                              block  ;; label = @14
                                local.get 7
                                i32.load
                                local.tee 15
                                local.get 14
                                i32.le_s
                                br_if 0 (;@14;)
                                local.get 13
                                i32.const 1
                                i32.add
                                local.tee 16
                                local.get 10
                                i32.gt_s
                                br_if 2 (;@12;)
                                local.get 10
                                i32.const 2
                                i32.shl
                                local.get 5
                                i32.add
                                local.set 12
                                loop  ;; label = @15
                                  local.get 12
                                  i32.load
                                  local.tee 17
                                  local.get 14
                                  i32.lt_s
                                  br_if 5 (;@10;)
                                  local.get 12
                                  i32.const -4
                                  i32.add
                                  local.set 12
                                  local.get 16
                                  local.get 10
                                  i32.lt_s
                                  local.set 17
                                  local.get 10
                                  i32.const -1
                                  i32.add
                                  local.set 10
                                  local.get 17
                                  br_if 0 (;@15;)
                                end
                                local.get 13
                                local.set 7
                                br 3 (;@11;)
                              end
                              local.get 7
                              i32.const 4
                              i32.add
                              local.set 7
                              local.get 12
                              local.get 13
                              i32.const 1
                              i32.add
                              local.tee 13
                              i32.ne
                              br_if 0 (;@13;)
                            end
                          end
                          local.get 10
                          local.set 7
                        end
                        local.get 7
                        i32.const 2
                        i32.shl
                        local.get 5
                        i32.add
                        local.tee 13
                        i32.load
                        local.set 10
                        local.get 13
                        local.get 14
                        i32.store
                        local.get 11
                        local.get 10
                        i32.store
                        local.get 7
                        local.set 10
                        br 2 (;@8;)
                      end
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 16
                          local.get 10
                          i32.lt_s
                          br_if 0 (;@11;)
                          local.get 16
                          local.set 7
                          br 1 (;@10;)
                        end
                        local.get 7
                        local.get 17
                        i32.store
                        local.get 12
                        local.get 15
                        i32.store
                        local.get 13
                        i32.const 2
                        i32.add
                        local.set 7
                        block  ;; label = @11
                          local.get 13
                          i32.const 3
                          i32.add
                          local.get 10
                          i32.ge_s
                          br_if 0 (;@11;)
                          local.get 10
                          i32.const -1
                          i32.add
                          local.set 10
                          br 1 (;@10;)
                        end
                        local.get 7
                        local.get 16
                        local.get 7
                        local.get 10
                        i32.lt_s
                        select
                        local.set 7
                      end
                      local.get 7
                      local.get 10
                      i32.lt_s
                      br_if 0 (;@9;)
                    end
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 7
                        local.get 9
                        i32.sub
                        local.tee 13
                        i32.const 2
                        i32.lt_s
                        br_if 0 (;@10;)
                        local.get 8
                        local.get 10
                        i32.sub
                        local.tee 14
                        i32.const 2
                        i32.lt_s
                        br_if 1 (;@9;)
                        block  ;; label = @11
                          local.get 13
                          local.get 14
                          i32.const -1
                          i32.add
                          i32.ge_u
                          br_if 0 (;@11;)
                          local.get 2
                          i32.const 672
                          i32.add
                          local.get 6
                          i32.const 2
                          i32.shl
                          local.tee 13
                          i32.add
                          local.get 8
                          i32.store
                          local.get 2
                          local.get 13
                          i32.add
                          local.get 10
                          i32.const 1
                          i32.add
                          i32.store
                          local.get 6
                          i32.const 1
                          i32.add
                          local.set 6
                          br 2 (;@9;)
                        end
                        local.get 2
                        i32.const 672
                        i32.add
                        local.get 6
                        i32.const 2
                        i32.shl
                        local.tee 13
                        i32.add
                        local.get 7
                        i32.store
                        local.get 2
                        local.get 13
                        i32.add
                        local.get 9
                        i32.store
                        local.get 10
                        i32.const 1
                        i32.add
                        local.set 9
                        local.get 6
                        i32.const 1
                        i32.add
                        local.set 6
                        br 2 (;@8;)
                      end
                      local.get 10
                      i32.const 1
                      i32.add
                      local.set 9
                      br 1 (;@8;)
                    end
                    local.get 7
                    local.set 8
                  end
                  local.get 8
                  local.get 9
                  i32.sub
                  local.tee 7
                  i32.const 2
                  i32.gt_s
                  br_if 0 (;@7;)
                end
              end
              block  ;; label = @6
                local.get 7
                i32.const 2
                i32.ne
                br_if 0 (;@6;)
                local.get 9
                i32.const 2
                i32.shl
                local.get 5
                i32.add
                local.tee 7
                i32.load
                local.tee 13
                local.get 8
                i32.const 2
                i32.shl
                i32.const 9708
                i32.add
                local.tee 10
                i32.load
                local.tee 14
                i32.le_s
                br_if 0 (;@6;)
                local.get 7
                local.get 14
                i32.store
                local.get 10
                local.get 13
                i32.store
              end
              local.get 6
              i32.const 0
              i32.gt_s
              br_if 0 (;@5;)
            end
            i32.const 1520
            i32.const 8192
            call 13
            i32.const 0
            i32.const 0
            i32.store offset=1428
            i32.const -400
            local.set 13
            loop  ;; label = @5
              local.get 13
              i32.const 1424
              i32.add
              i32.load
              local.set 10
              i32.const 12
              call 14
              local.tee 7
              local.get 10
              i32.store
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    i32.const 0
                    i32.load offset=1428
                    local.tee 10
                    br_if 0 (;@8;)
                    local.get 7
                    i32.const 0
                    i32.store offset=8
                    i32.const 0
                    local.get 7
                    i32.store offset=1428
                    local.get 7
                    local.set 10
                    i32.const 0
                    local.set 7
                    br 1 (;@7;)
                  end
                  local.get 7
                  local.get 10
                  i32.load offset=8
                  i32.store offset=8
                  local.get 10
                  local.get 7
                  i32.store offset=8
                  local.get 7
                  local.get 10
                  i32.store offset=4
                  local.get 7
                  i32.load offset=8
                  local.tee 10
                  i32.eqz
                  br_if 1 (;@6;)
                end
                local.get 10
                local.get 7
                i32.store offset=4
              end
              local.get 13
              i32.const 4
              i32.add
              local.tee 13
              br_if 0 (;@5;)
            end
            i32.const 1428
            call 2
            local.get 2
            i32.const 0
            i32.store offset=1116
            block  ;; label = @5
              i32.const 0
              i32.load offset=1428
              local.tee 13
              i32.eqz
              br_if 0 (;@5;)
              loop  ;; label = @6
                local.get 13
                local.tee 7
                i32.load offset=8
                local.tee 13
                br_if 0 (;@6;)
              end
              loop  ;; label = @6
                local.get 2
                local.get 2
                i32.load offset=1116
                i32.const 1
                i32.add
                i32.store offset=1116
                local.get 7
                i32.load offset=4
                local.tee 7
                br_if 0 (;@6;)
              end
            end
            i32.const 0
            local.set 10
            i32.const 1440
            i32.const 0
            i32.const 80
            call 20
            local.set 12
            loop  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 10
                  i32.const 2
                  i32.shl
                  i32.const 1024
                  i32.add
                  i32.load
                  local.tee 13
                  i32.const 20
                  i32.rem_u
                  i32.const 2
                  i32.shl
                  local.get 12
                  i32.add
                  local.tee 14
                  i32.load
                  local.tee 7
                  i32.eqz
                  br_if 0 (;@7;)
                  loop  ;; label = @8
                    local.get 7
                    i32.load
                    local.get 13
                    i32.eq
                    br_if 2 (;@6;)
                    local.get 7
                    i32.load offset=4
                    local.tee 7
                    br_if 0 (;@8;)
                  end
                end
                i32.const 8
                call 14
                local.tee 7
                local.get 14
                i32.load
                i32.store offset=4
                local.get 7
                local.get 13
                i32.store
                local.get 14
                local.get 7
                i32.store
              end
              local.get 10
              i32.const 1
              i32.add
              local.tee 10
              i32.const 100
              i32.ne
              br_if 0 (;@5;)
            end
            block  ;; label = @5
              local.get 2
              i32.const 1084
              i32.add
              local.get 12
              i32.const 0
              i32.const 0
              call 4
              i32.eqz
              br_if 0 (;@5;)
              loop  ;; label = @6
                local.get 2
                local.get 2
                i32.load offset=1116
                i32.const 1
                i32.add
                i32.store offset=1116
                local.get 2
                i32.const 1084
                i32.add
                call 5
                br_if 0 (;@6;)
              end
            end
            i32.const 0
            local.set 7
            i32.const -400
            local.set 13
            block  ;; label = @5
              loop  ;; label = @6
                local.get 2
                i32.const 672
                i32.add
                local.get 7
                i32.const 2
                i32.shl
                i32.add
                local.get 13
                i32.const 1424
                i32.add
                i32.load
                i32.store
                local.get 7
                i32.const 1
                i32.add
                i32.const 101
                i32.rem_s
                local.tee 7
                i32.eqz
                br_if 1 (;@5;)
                local.get 13
                i32.const 4
                i32.add
                local.tee 13
                br_if 0 (;@6;)
              end
              i32.const 0
              local.set 13
              loop  ;; label = @6
                local.get 2
                local.get 2
                i32.const 672
                i32.add
                local.get 13
                i32.const 2
                i32.shl
                i32.add
                i32.load
                local.get 2
                i32.load offset=1116
                i32.add
                i32.store offset=1116
                local.get 13
                i32.const 1
                i32.add
                i32.const 101
                i32.rem_s
                local.tee 13
                local.get 7
                i32.ne
                br_if 0 (;@6;)
              end
              i32.const 0
              local.set 17
              loop  ;; label = @6
                local.get 2
                i32.const 672
                i32.add
                local.get 17
                i32.const 2
                i32.shl
                local.tee 7
                i32.add
                local.get 7
                i32.const 1024
                i32.add
                i32.load
                i32.store
                block  ;; label = @7
                  local.get 17
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 17
                  local.set 7
                  loop  ;; label = @8
                    local.get 2
                    i32.const 672
                    i32.add
                    local.get 7
                    i32.const 1
                    i32.shr_u
                    local.tee 13
                    i32.const 2
                    i32.shl
                    i32.add
                    local.tee 10
                    i32.load
                    local.tee 14
                    local.get 2
                    i32.const 672
                    i32.add
                    local.get 7
                    i32.const 2
                    i32.shl
                    i32.add
                    local.tee 12
                    i32.load
                    local.tee 15
                    i32.ge_s
                    br_if 1 (;@7;)
                    local.get 10
                    local.get 15
                    i32.store
                    local.get 12
                    local.get 14
                    i32.store
                    local.get 7
                    i32.const 1
                    i32.gt_u
                    local.set 10
                    local.get 13
                    local.set 7
                    local.get 10
                    br_if 0 (;@8;)
                  end
                end
                local.get 17
                i32.const 1
                i32.add
                local.tee 17
                i32.const 100
                i32.ne
                br_if 0 (;@6;)
              end
              i32.const 100
              local.set 14
              loop  ;; label = @6
                local.get 2
                local.get 2
                i32.load offset=672
                local.get 2
                i32.load offset=1116
                i32.add
                i32.store offset=1116
                local.get 2
                local.get 2
                i32.const 672
                i32.add
                local.get 14
                local.tee 7
                i32.const -1
                i32.add
                local.tee 14
                i32.const 2
                i32.shl
                i32.add
                i32.load
                local.tee 12
                i32.store offset=672
                block  ;; label = @7
                  local.get 7
                  i32.const 3
                  i32.lt_s
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 13
                  i32.const 1
                  local.set 7
                  i32.const 2
                  local.set 10
                  loop  ;; label = @8
                    local.get 7
                    local.get 13
                    local.get 12
                    local.get 2
                    i32.const 672
                    i32.add
                    local.get 7
                    i32.const 2
                    i32.shl
                    i32.add
                    i32.load
                    i32.lt_s
                    select
                    local.set 7
                    block  ;; label = @9
                      local.get 10
                      local.get 14
                      i32.ge_s
                      br_if 0 (;@9;)
                      local.get 10
                      local.get 7
                      local.get 2
                      i32.const 672
                      i32.add
                      local.get 7
                      i32.const 2
                      i32.shl
                      i32.add
                      i32.load
                      local.get 2
                      i32.const 672
                      i32.add
                      local.get 10
                      i32.const 2
                      i32.shl
                      i32.add
                      i32.load
                      i32.lt_s
                      select
                      local.set 7
                    end
                    local.get 7
                    local.get 13
                    i32.eq
                    br_if 1 (;@7;)
                    local.get 2
                    i32.const 672
                    i32.add
                    local.get 13
                    i32.const 2
                    i32.shl
                    i32.add
                    local.get 2
                    i32.const 672
                    i32.add
                    local.get 7
                    i32.const 2
                    i32.shl
                    i32.add
                    local.tee 13
                    i32.load
                    i32.store
                    local.get 13
                    local.get 12
                    i32.store
                    local.get 7
                    i32.const 1
                    i32.shl
                    local.tee 15
                    i32.const 2
                    i32.add
                    local.set 10
                    local.get 7
                    local.set 13
                    local.get 15
                    i32.const 1
                    i32.or
                    local.tee 7
                    local.get 14
                    i32.lt_s
                    br_if 0 (;@8;)
                  end
                end
                local.get 14
                br_if 0 (;@6;)
              end
              i32.const 0
              local.set 12
              local.get 2
              i32.const 0
              i32.store offset=668
              i32.const 0
              local.set 14
              loop  ;; label = @6
                local.get 14
                i32.const 2
                i32.shl
                i32.const 1024
                i32.add
                i32.load
                local.set 10
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 12
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 12
                    local.set 7
                    loop  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 10
                          local.get 7
                          i32.load
                          local.tee 13
                          i32.ge_s
                          br_if 0 (;@11;)
                          i32.const 8
                          local.set 13
                          br 1 (;@10;)
                        end
                        local.get 10
                        local.get 13
                        i32.eq
                        br_if 3 (;@7;)
                        i32.const 12
                        local.set 13
                      end
                      local.get 7
                      local.get 13
                      i32.add
                      i32.load
                      local.tee 7
                      br_if 0 (;@9;)
                    end
                  end
                  i32.const 16
                  call 14
                  local.tee 7
                  local.get 10
                  i32.store
                  local.get 7
                  i64.const 0
                  i64.store offset=8 align=4
                  local.get 2
                  i32.const 668
                  i32.add
                  local.get 7
                  call 6
                  local.get 2
                  i32.load offset=668
                  local.tee 12
                  i32.const 0
                  i32.store8 offset=4
                end
                local.get 14
                i32.const 1
                i32.add
                local.tee 14
                i32.const 100
                i32.ne
                br_if 0 (;@6;)
              end
              block  ;; label = @6
                local.get 2
                local.get 12
                call 8
                local.tee 7
                i32.eqz
                br_if 0 (;@6;)
                loop  ;; label = @7
                  local.get 2
                  local.get 7
                  i32.load
                  local.get 2
                  i32.load offset=1116
                  i32.add
                  i32.store offset=1116
                  local.get 2
                  call 7
                  local.get 2
                  i32.load
                  local.tee 7
                  br_if 0 (;@7;)
                end
              end
              local.get 4
              i32.const 1
              i32.add
              local.tee 4
              local.get 1
              i32.ne
              br_if 1 (;@4;)
              br 2 (;@3;)
            end
          end
          loop  ;; label = @4
            br 0 (;@4;)
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
    i32.load offset=1116
    local.set 7
    local.get 2
    i32.const 1120
    i32.add
    global.set 0
    local.get 7)
  (func (;11;) (type 6) (result i32)
    i32.const 31
    i32.const 1
    call 10)
  (func (;12;) (type 0) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    call 15
    call 9
    call 17
    local.get 2
    call 11
    i32.store offset=12
    call 18
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    i32.const 0)
  (func (;13;) (type 5) (param i32 i32)
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
    i32.store offset=10112
    i32.const 0
    local.get 0
    local.get 1
    i32.add
    i32.store offset=10116
    i32.const 0
    i32.const 0
    i32.store offset=10120)
  (func (;14;) (type 3) (param i32) (result i32)
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
    i32.load offset=10120
    local.get 0
    i32.add
    i32.const 4
    i32.const 0
    i32.load offset=10112
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
    i32.store offset=10120
    block  ;; label = @1
      local.get 0
      local.get 3
      i32.add
      local.tee 0
      i32.const 0
      i32.load offset=10116
      i32.gt_u
      br_if 0 (;@1;)
      i32.const 0
      local.get 0
      i32.store offset=10112
      local.get 2
      local.set 1
    end
    local.get 1)
  (func (;15;) (type 1))
  (func (;16;) (type 6) (result i32)
    i32.const 0
    i32.const 0
    call 12)
  (func (;17;) (type 1))
  (func (;18;) (type 1))
  (func (;19;) (type 7) (param i32 i32 i32) (result i32)
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
  (func (;20;) (type 7) (param i32 i32 i32) (result i32)
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
  (global (;0;) (mut i32) (i32.const 26128))
  (global (;1;) i32 (i32.const 0))
  (global (;2;) i32 (i32.const 10124))
  (global (;3;) i32 (i32.const 26128))
  (export "memory" (memory 0))
  (export "_initialize" (func 1))
  (export "_run" (func 16))
  (export "__data_end" (global 2))
  (export "__heap_base" (global 3))
  (data (;0;) (i32.const 1024) "\0e\00\00\00B\00\00\00\0c\00\00\00)\00\00\00V\00\00\00E\00\00\00\13\00\00\00M\00\00\00D\00\00\00&\00\00\00\1a\00\00\00*\00\00\00%\00\00\00\17\00\00\00\11\00\00\00\1d\00\00\007\00\00\00\0d\00\00\00Z\00\00\00\5c\00\00\00L\00\00\00c\00\00\00\0a\00\00\006\00\00\009\00\00\00S\00\00\00(\00\00\00,\00\00\00K\00\00\00!\00\00\00\18\00\00\00\1c\00\00\00P\00\00\00\12\00\00\00N\00\00\00 \00\00\00]\00\00\00Y\00\00\004\00\00\00\0b\00\00\00\15\00\00\00`\00\00\002\00\00\00\0f\00\00\000\00\00\00?\00\00\00W\00\00\00\14\00\00\00\08\00\00\00U\00\00\00+\00\00\00\10\00\00\00^\00\00\00X\00\00\005\00\00\00T\00\00\00J\00\00\00[\00\00\00C\00\00\00$\00\00\00_\00\00\00=\00\00\00@\00\00\00\05\00\00\00\1e\00\00\00R\00\00\00H\00\00\00.\00\00\00;\00\00\00\09\00\00\00\07\00\00\00\03\00\00\00'\00\00\00\1f\00\00\00\04\00\00\00I\00\00\00F\00\00\00<\00\00\00:\00\00\00Q\00\00\008\00\00\003\00\00\00-\00\00\00\01\00\00\00\06\00\00\001\00\00\00\1b\00\00\00/\00\00\00\22\00\00\00#\00\00\00>\00\00\00a\00\00\00\02\00\00\00O\00\00\00b\00\00\00\19\00\00\00\16\00\00\00A\00\00\00G\00\00\00\00\00\00\00"))

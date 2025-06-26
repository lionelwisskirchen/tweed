(module
  (type (;0;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;1;) (func))
  (type (;2;) (func (result i32)))
  (type (;3;) (func (param i32 i32) (result i32)))
  (type (;4;) (func (param i32 i32)))
  (type (;5;) (func (param i32) (result i32)))
  (type (;6;) (func (param i32 i32 i32) (result i32)))
  (func (;0;) (type 1))
  (func (;1;) (type 1)
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
  (func (;2;) (type 2) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            i32.load8_u offset=1828
            local.tee 0
            br_if 0 (;@4;)
            block  ;; label = @5
              i32.const 0
              i32.load16_u offset=1830
              br_if 0 (;@5;)
              i32.const 1
              return
            end
            block  ;; label = @5
              i32.const 0
              i32.load16_u offset=2644
              i32.eqz
              br_if 0 (;@5;)
              block  ;; label = @6
                i32.const 0
                i32.load16_u offset=2646
                local.tee 0
                br_if 0 (;@6;)
                i32.const 1536
                local.set 0
                block  ;; label = @7
                  loop  ;; label = @8
                    block  ;; label = @9
                      call 3
                      i32.const 255
                      i32.ne
                      br_if 0 (;@9;)
                      br 2 (;@7;)
                    end
                    local.get 0
                    i32.const -1
                    i32.add
                    local.tee 0
                    i32.const 65535
                    i32.and
                    br_if 0 (;@8;)
                    br 6 (;@2;)
                  end
                end
                block  ;; label = @7
                  loop  ;; label = @8
                    call 3
                    local.tee 1
                    i32.const 255
                    i32.ne
                    br_if 1 (;@7;)
                    local.get 0
                    i32.const -1
                    i32.add
                    local.tee 0
                    i32.const 65535
                    i32.and
                    i32.eqz
                    br_if 6 (;@2;)
                    br 0 (;@8;)
                  end
                end
                i32.const 0
                i32.load16_u offset=3952
                local.tee 0
                i32.const 208
                i32.add
                local.get 1
                i32.ne
                br_if 4 (;@2;)
                i32.const 0
                i32.const 8
                i32.store8 offset=3954
                i32.const 0
                i32.const 0
                i32.load16_u offset=2644
                i32.store16 offset=2646
                i32.const 0
                local.get 0
                i32.const 1
                i32.add
                i32.const 7
                i32.and
                i32.store16 offset=3952
                i32.const 0
                i32.const 0
                i32.store16 offset=3124
                i32.const 0
                i32.const 0
                i32.store offset=3120 align=2
                i32.const 8
                i32.const 1
                call 4
                drop
                i32.const 8
                i32.const 1
                call 4
                drop
                i32.const 0
                i32.load16_u offset=2646
                local.set 0
              end
              i32.const 0
              local.get 0
              i32.const -1
              i32.add
              i32.store16 offset=2646
            end
            block  ;; label = @5
              i32.const 0
              i32.load8_u offset=2648
              i32.eqz
              br_if 0 (;@5;)
              i32.const 0
              local.set 2
              loop  ;; label = @6
                i32.const 2672
                i32.const 2800
                local.get 2
                i32.const 2652
                i32.add
                i32.load8_u
                local.tee 0
                i32.const 2658
                i32.add
                i32.load8_u
                select
                local.set 3
                i32.const 0
                local.set 1
                block  ;; label = @7
                  i32.const 2928
                  i32.const 3008
                  local.get 0
                  i32.const 2661
                  i32.add
                  i32.load8_u
                  local.tee 4
                  select
                  i32.const 3088
                  i32.const 3104
                  local.get 4
                  select
                  call 5
                  local.tee 4
                  i32.const 15
                  i32.and
                  local.tee 5
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 5
                  i32.const 1
                  call 4
                  local.set 1
                end
                local.get 0
                i32.const 1
                i32.shl
                i32.const 3120
                i32.add
                local.tee 5
                local.get 5
                i32.load16_u
                local.get 1
                local.get 4
                call 6
                i32.add
                local.tee 1
                i32.store16
                i32.const 0
                local.get 3
                i32.load16_u
                local.get 1
                i32.mul
                i32.store16 offset=3136
                i32.const 3440
                i32.const 3696
                local.get 0
                i32.const 3264
                i32.add
                i32.load8_u
                local.tee 0
                select
                local.set 4
                i32.const 3268
                i32.const 3348
                local.get 0
                select
                local.set 5
                block  ;; label = @7
                  block  ;; label = @8
                    i32.const 0
                    i32.load8_u offset=1840
                    i32.eqz
                    br_if 0 (;@8;)
                    i32.const 1
                    local.set 1
                    block  ;; label = @9
                      loop  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 5
                            local.get 4
                            call 5
                            local.tee 0
                            i32.const 15
                            i32.and
                            local.tee 6
                            i32.eqz
                            br_if 0 (;@12;)
                            local.get 6
                            i32.const 1
                            call 4
                            drop
                            block  ;; label = @13
                              local.get 0
                              i32.const 16
                              i32.ge_u
                              br_if 0 (;@13;)
                              local.get 1
                              local.set 0
                              br 2 (;@11;)
                            end
                            local.get 0
                            i32.const 4
                            i32.shr_u
                            local.get 1
                            i32.add
                            local.tee 0
                            i32.const 255
                            i32.and
                            i32.const 63
                            i32.le_u
                            br_if 1 (;@11;)
                            br 9 (;@3;)
                          end
                          local.get 0
                          i32.const 240
                          i32.ne
                          br_if 2 (;@9;)
                          local.get 1
                          i32.const 255
                          i32.and
                          i32.const 48
                          i32.gt_u
                          br_if 8 (;@3;)
                          local.get 1
                          i32.const 15
                          i32.add
                          local.set 0
                        end
                        local.get 0
                        i32.const 1
                        i32.add
                        local.set 1
                        local.get 0
                        i32.const 255
                        i32.and
                        i32.const 63
                        i32.lt_u
                        br_if 0 (;@10;)
                      end
                    end
                    i32.const 0
                    i32.load16_s offset=3136
                    i32.const 64
                    i32.add
                    i32.const 7
                    i32.shr_u
                    i32.const 128
                    i32.add
                    local.tee 0
                    i32.extend16_s
                    i32.const 15
                    i32.shr_s
                    i32.const -1
                    i32.xor
                    local.get 0
                    local.get 0
                    i32.const 65535
                    i32.and
                    i32.const 255
                    i32.gt_u
                    select
                    local.set 0
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      block  ;; label = @18
                                        block  ;; label = @19
                                          i32.const 0
                                          i32.load offset=1848
                                          br_table 0 (;@19;) 1 (;@18;) 3 (;@16;) 2 (;@17;) 4 (;@15;) 0 (;@19;)
                                        end
                                        i32.const 0
                                        local.get 0
                                        i32.store8 offset=1872
                                        br 11 (;@7;)
                                      end
                                      block  ;; label = @18
                                        block  ;; label = @19
                                          local.get 2
                                          i32.const 255
                                          i32.and
                                          br_table 5 (;@14;) 0 (;@19;) 1 (;@18;) 12 (;@7;)
                                        end
                                        i32.const 0
                                        i32.const -1
                                        i32.const 0
                                        i32.const 0
                                        i32.load8_u offset=2128
                                        local.get 0
                                        i32.const 255
                                        i32.and
                                        local.tee 0
                                        i32.const 88
                                        i32.mul
                                        i32.const 8
                                        i32.shr_u
                                        i32.sub
                                        local.tee 1
                                        i32.const -45
                                        i32.gt_s
                                        select
                                        local.get 1
                                        i32.const 44
                                        i32.add
                                        local.tee 1
                                        local.get 1
                                        i32.const 255
                                        i32.gt_u
                                        select
                                        i32.store8 offset=2128
                                        i32.const 0
                                        local.get 0
                                        i32.const 198
                                        i32.mul
                                        i32.const 8
                                        i32.shr_u
                                        local.get 0
                                        i32.add
                                        i32.const 0
                                        i32.load8_u offset=2384
                                        i32.add
                                        i32.const -227
                                        i32.add
                                        local.tee 0
                                        i32.const -1
                                        i32.xor
                                        i32.const 15
                                        i32.shr_u
                                        local.get 0
                                        local.get 0
                                        i32.const 255
                                        i32.gt_u
                                        select
                                        i32.store8 offset=2384
                                        br 11 (;@7;)
                                      end
                                      i32.const 0
                                      i32.const -1
                                      i32.const 0
                                      i32.const 0
                                      i32.load8_u offset=2128
                                      local.get 0
                                      i32.const 255
                                      i32.and
                                      local.tee 0
                                      i32.const 183
                                      i32.mul
                                      i32.const 8
                                      i32.shr_u
                                      i32.sub
                                      local.tee 1
                                      i32.const -92
                                      i32.gt_s
                                      select
                                      local.get 1
                                      i32.const 91
                                      i32.add
                                      local.tee 1
                                      local.get 1
                                      i32.const 255
                                      i32.gt_u
                                      select
                                      i32.store8 offset=2128
                                      i32.const 0
                                      local.get 0
                                      i32.const 103
                                      i32.mul
                                      i32.const 8
                                      i32.shr_u
                                      local.get 0
                                      i32.add
                                      i32.const 0
                                      i32.load8_u offset=1872
                                      i32.add
                                      i32.const -179
                                      i32.add
                                      local.tee 0
                                      i32.const -1
                                      i32.xor
                                      i32.const 15
                                      i32.shr_u
                                      local.get 0
                                      local.get 0
                                      i32.const 255
                                      i32.gt_u
                                      select
                                      i32.store8 offset=1872
                                      br 10 (;@7;)
                                    end
                                    block  ;; label = @17
                                      block  ;; label = @18
                                        local.get 2
                                        i32.const 255
                                        i32.and
                                        br_table 4 (;@14;) 6 (;@12;) 0 (;@18;) 1 (;@17;) 11 (;@7;)
                                      end
                                      i32.const 0
                                      i32.const 0
                                      i32.load8_u offset=2128
                                      local.get 0
                                      i32.const 255
                                      i32.and
                                      local.tee 0
                                      i32.const 88
                                      i32.mul
                                      i32.const 8
                                      i32.shr_u
                                      i32.const -44
                                      i32.add
                                      local.tee 4
                                      i32.sub
                                      local.tee 1
                                      i32.const -1
                                      i32.xor
                                      i32.const 15
                                      i32.shr_u
                                      local.get 1
                                      local.get 1
                                      i32.const 255
                                      i32.gt_u
                                      select
                                      i32.store8 offset=2128
                                      i32.const 0
                                      i32.const 0
                                      i32.load8_u offset=2256
                                      local.get 4
                                      i32.sub
                                      local.tee 1
                                      i32.const -1
                                      i32.xor
                                      i32.const 15
                                      i32.shr_u
                                      local.get 1
                                      local.get 1
                                      i32.const 255
                                      i32.gt_u
                                      select
                                      i32.store8 offset=2256
                                      i32.const 0
                                      local.get 0
                                      i32.const 198
                                      i32.mul
                                      i32.const 8
                                      i32.shr_u
                                      local.get 0
                                      i32.add
                                      i32.const -227
                                      i32.add
                                      local.tee 1
                                      i32.const 0
                                      i32.load8_u offset=2384
                                      i32.add
                                      local.tee 0
                                      i32.const -1
                                      i32.xor
                                      i32.const 15
                                      i32.shr_u
                                      local.get 0
                                      local.get 0
                                      i32.const 255
                                      i32.gt_u
                                      select
                                      i32.store8 offset=2384
                                      i32.const 0
                                      local.get 1
                                      i32.const 0
                                      i32.load8_u offset=2512
                                      i32.add
                                      local.tee 0
                                      i32.const -1
                                      i32.xor
                                      i32.const 15
                                      i32.shr_u
                                      local.get 0
                                      local.get 0
                                      i32.const 255
                                      i32.gt_u
                                      select
                                      i32.store8 offset=2512
                                      br 10 (;@7;)
                                    end
                                    i32.const 0
                                    i32.const 0
                                    i32.load8_u offset=2128
                                    local.get 0
                                    i32.const 255
                                    i32.and
                                    local.tee 0
                                    i32.const 183
                                    i32.mul
                                    i32.const 8
                                    i32.shr_u
                                    i32.const -91
                                    i32.add
                                    local.tee 4
                                    i32.sub
                                    local.tee 1
                                    i32.const -1
                                    i32.xor
                                    i32.const 15
                                    i32.shr_u
                                    local.get 1
                                    local.get 1
                                    i32.const 255
                                    i32.gt_u
                                    select
                                    i32.store8 offset=2128
                                    i32.const 0
                                    i32.const 0
                                    i32.load8_u offset=2256
                                    local.get 4
                                    i32.sub
                                    local.tee 1
                                    i32.const -1
                                    i32.xor
                                    i32.const 15
                                    i32.shr_u
                                    local.get 1
                                    local.get 1
                                    i32.const 255
                                    i32.gt_u
                                    select
                                    i32.store8 offset=2256
                                    i32.const 0
                                    local.get 0
                                    i32.const 103
                                    i32.mul
                                    i32.const 8
                                    i32.shr_u
                                    local.get 0
                                    i32.add
                                    i32.const -179
                                    i32.add
                                    local.tee 1
                                    i32.const 0
                                    i32.load8_u offset=1872
                                    i32.add
                                    local.tee 0
                                    i32.const -1
                                    i32.xor
                                    i32.const 15
                                    i32.shr_u
                                    local.get 0
                                    local.get 0
                                    i32.const 255
                                    i32.gt_u
                                    select
                                    i32.store8 offset=1872
                                    i32.const 0
                                    local.get 1
                                    i32.const 0
                                    i32.load8_u offset=2000
                                    i32.add
                                    local.tee 0
                                    i32.const -1
                                    i32.xor
                                    i32.const 15
                                    i32.shr_u
                                    local.get 0
                                    local.get 0
                                    i32.const 255
                                    i32.gt_u
                                    select
                                    i32.store8 offset=2000
                                    br 9 (;@7;)
                                  end
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      local.get 2
                                      i32.const 255
                                      i32.and
                                      br_table 3 (;@14;) 4 (;@13;) 0 (;@17;) 1 (;@16;) 10 (;@7;)
                                    end
                                    i32.const 0
                                    i32.const 0
                                    i32.load8_u offset=2128
                                    local.get 0
                                    i32.const 255
                                    i32.and
                                    local.tee 0
                                    i32.const 88
                                    i32.mul
                                    i32.const 8
                                    i32.shr_u
                                    i32.const -44
                                    i32.add
                                    local.tee 4
                                    i32.sub
                                    local.tee 1
                                    i32.const -1
                                    i32.xor
                                    i32.const 15
                                    i32.shr_u
                                    local.get 1
                                    local.get 1
                                    i32.const 255
                                    i32.gt_u
                                    select
                                    i32.store8 offset=2128
                                    i32.const 0
                                    i32.const 0
                                    i32.load8_u offset=2192
                                    local.get 4
                                    i32.sub
                                    local.tee 1
                                    i32.const -1
                                    i32.xor
                                    i32.const 15
                                    i32.shr_u
                                    local.get 1
                                    local.get 1
                                    i32.const 255
                                    i32.gt_u
                                    select
                                    i32.store8 offset=2192
                                    i32.const 0
                                    local.get 0
                                    i32.const 198
                                    i32.mul
                                    i32.const 8
                                    i32.shr_u
                                    local.get 0
                                    i32.add
                                    i32.const -227
                                    i32.add
                                    local.tee 1
                                    i32.const 0
                                    i32.load8_u offset=2384
                                    i32.add
                                    local.tee 0
                                    i32.const -1
                                    i32.xor
                                    i32.const 15
                                    i32.shr_u
                                    local.get 0
                                    local.get 0
                                    i32.const 255
                                    i32.gt_u
                                    select
                                    i32.store8 offset=2384
                                    i32.const 0
                                    local.get 1
                                    i32.const 0
                                    i32.load8_u offset=2448
                                    i32.add
                                    local.tee 0
                                    i32.const -1
                                    i32.xor
                                    i32.const 15
                                    i32.shr_u
                                    local.get 0
                                    local.get 0
                                    i32.const 255
                                    i32.gt_u
                                    select
                                    i32.store8 offset=2448
                                    br 9 (;@7;)
                                  end
                                  i32.const 0
                                  i32.const 0
                                  i32.load8_u offset=2128
                                  local.get 0
                                  i32.const 255
                                  i32.and
                                  local.tee 0
                                  i32.const 183
                                  i32.mul
                                  i32.const 8
                                  i32.shr_u
                                  i32.const -91
                                  i32.add
                                  local.tee 4
                                  i32.sub
                                  local.tee 1
                                  i32.const -1
                                  i32.xor
                                  i32.const 15
                                  i32.shr_u
                                  local.get 1
                                  local.get 1
                                  i32.const 255
                                  i32.gt_u
                                  select
                                  i32.store8 offset=2128
                                  i32.const 0
                                  i32.const 0
                                  i32.load8_u offset=2192
                                  local.get 4
                                  i32.sub
                                  local.tee 1
                                  i32.const -1
                                  i32.xor
                                  i32.const 15
                                  i32.shr_u
                                  local.get 1
                                  local.get 1
                                  i32.const 255
                                  i32.gt_u
                                  select
                                  i32.store8 offset=2192
                                  i32.const 0
                                  local.get 0
                                  i32.const 103
                                  i32.mul
                                  i32.const 8
                                  i32.shr_u
                                  local.get 0
                                  i32.add
                                  i32.const -179
                                  i32.add
                                  local.tee 1
                                  i32.const 0
                                  i32.load8_u offset=1872
                                  i32.add
                                  local.tee 0
                                  i32.const -1
                                  i32.xor
                                  i32.const 15
                                  i32.shr_u
                                  local.get 0
                                  local.get 0
                                  i32.const 255
                                  i32.gt_u
                                  select
                                  i32.store8 offset=1872
                                  i32.const 0
                                  local.get 1
                                  i32.const 0
                                  i32.load8_u offset=1936
                                  i32.add
                                  local.tee 0
                                  i32.const -1
                                  i32.xor
                                  i32.const 15
                                  i32.shr_u
                                  local.get 0
                                  local.get 0
                                  i32.const 255
                                  i32.gt_u
                                  select
                                  i32.store8 offset=1936
                                  br 8 (;@7;)
                                end
                                local.get 2
                                i32.const 255
                                i32.and
                                br_table 0 (;@14;) 1 (;@13;) 2 (;@12;) 3 (;@11;) 4 (;@10;) 5 (;@9;) 7 (;@7;)
                              end
                              i32.const 0
                              local.get 0
                              i32.store8 offset=2128
                              i32.const 0
                              local.get 0
                              i32.store8 offset=1872
                              i32.const 0
                              local.get 0
                              i32.store8 offset=2384
                              br 6 (;@7;)
                            end
                            i32.const 0
                            local.get 0
                            i32.store8 offset=2192
                            i32.const 0
                            local.get 0
                            i32.store8 offset=1936
                            i32.const 0
                            local.get 0
                            i32.store8 offset=2448
                            br 5 (;@7;)
                          end
                          i32.const 0
                          local.get 0
                          i32.store8 offset=2256
                          i32.const 0
                          local.get 0
                          i32.store8 offset=2000
                          i32.const 0
                          local.get 0
                          i32.store8 offset=2512
                          br 4 (;@7;)
                        end
                        i32.const 0
                        local.get 0
                        i32.store8 offset=2320
                        i32.const 0
                        local.get 0
                        i32.store8 offset=2064
                        i32.const 0
                        local.get 0
                        i32.store8 offset=2576
                        br 3 (;@7;)
                      end
                      i32.const 0
                      i32.const 0
                      i32.load8_u offset=2128
                      local.get 0
                      i32.const 255
                      i32.and
                      local.tee 1
                      i32.const 88
                      i32.mul
                      i32.const 8
                      i32.shr_u
                      i32.const -44
                      i32.add
                      local.tee 0
                      i32.sub
                      local.tee 4
                      i32.const -1
                      i32.xor
                      i32.const 15
                      i32.shr_u
                      local.get 4
                      local.get 4
                      i32.const 255
                      i32.gt_u
                      select
                      i32.store8 offset=2128
                      i32.const 0
                      i32.const 0
                      i32.load8_u offset=2192
                      local.get 0
                      i32.sub
                      local.tee 4
                      i32.const -1
                      i32.xor
                      i32.const 15
                      i32.shr_u
                      local.get 4
                      local.get 4
                      i32.const 255
                      i32.gt_u
                      select
                      i32.store8 offset=2192
                      i32.const 0
                      i32.const 0
                      i32.load8_u offset=2256
                      local.get 0
                      i32.sub
                      local.tee 4
                      i32.const -1
                      i32.xor
                      i32.const 15
                      i32.shr_u
                      local.get 4
                      local.get 4
                      i32.const 255
                      i32.gt_u
                      select
                      i32.store8 offset=2256
                      i32.const 0
                      i32.const 0
                      i32.load8_u offset=2320
                      local.get 0
                      i32.sub
                      local.tee 0
                      i32.const -1
                      i32.xor
                      i32.const 15
                      i32.shr_u
                      local.get 0
                      local.get 0
                      i32.const 255
                      i32.gt_u
                      select
                      i32.store8 offset=2320
                      i32.const 0
                      local.get 1
                      i32.const 198
                      i32.mul
                      i32.const 8
                      i32.shr_u
                      local.get 1
                      i32.add
                      i32.const -227
                      i32.add
                      local.tee 0
                      i32.const 0
                      i32.load8_u offset=2384
                      i32.add
                      local.tee 1
                      i32.const -1
                      i32.xor
                      i32.const 15
                      i32.shr_u
                      local.get 1
                      local.get 1
                      i32.const 255
                      i32.gt_u
                      select
                      i32.store8 offset=2384
                      i32.const 0
                      local.get 0
                      i32.const 0
                      i32.load8_u offset=2448
                      i32.add
                      local.tee 1
                      i32.const -1
                      i32.xor
                      i32.const 15
                      i32.shr_u
                      local.get 1
                      local.get 1
                      i32.const 255
                      i32.gt_u
                      select
                      i32.store8 offset=2448
                      i32.const 0
                      local.get 0
                      i32.const 0
                      i32.load8_u offset=2512
                      i32.add
                      local.tee 1
                      i32.const -1
                      i32.xor
                      i32.const 15
                      i32.shr_u
                      local.get 1
                      local.get 1
                      i32.const 255
                      i32.gt_u
                      select
                      i32.store8 offset=2512
                      i32.const 0
                      local.get 0
                      i32.const 0
                      i32.load8_u offset=2576
                      i32.add
                      local.tee 0
                      i32.const -1
                      i32.xor
                      i32.const 15
                      i32.shr_u
                      local.get 0
                      local.get 0
                      i32.const 255
                      i32.gt_u
                      select
                      i32.store8 offset=2576
                      br 2 (;@7;)
                    end
                    i32.const 0
                    i32.const 0
                    i32.load8_u offset=2128
                    local.get 0
                    i32.const 255
                    i32.and
                    local.tee 1
                    i32.const 183
                    i32.mul
                    i32.const 8
                    i32.shr_u
                    i32.const -91
                    i32.add
                    local.tee 0
                    i32.sub
                    local.tee 4
                    i32.const -1
                    i32.xor
                    i32.const 15
                    i32.shr_u
                    local.get 4
                    local.get 4
                    i32.const 255
                    i32.gt_u
                    select
                    i32.store8 offset=2128
                    i32.const 0
                    i32.const 0
                    i32.load8_u offset=2192
                    local.get 0
                    i32.sub
                    local.tee 4
                    i32.const -1
                    i32.xor
                    i32.const 15
                    i32.shr_u
                    local.get 4
                    local.get 4
                    i32.const 255
                    i32.gt_u
                    select
                    i32.store8 offset=2192
                    i32.const 0
                    i32.const 0
                    i32.load8_u offset=2256
                    local.get 0
                    i32.sub
                    local.tee 4
                    i32.const -1
                    i32.xor
                    i32.const 15
                    i32.shr_u
                    local.get 4
                    local.get 4
                    i32.const 255
                    i32.gt_u
                    select
                    i32.store8 offset=2256
                    i32.const 0
                    i32.const 0
                    i32.load8_u offset=2320
                    local.get 0
                    i32.sub
                    local.tee 0
                    i32.const -1
                    i32.xor
                    i32.const 15
                    i32.shr_u
                    local.get 0
                    local.get 0
                    i32.const 255
                    i32.gt_u
                    select
                    i32.store8 offset=2320
                    i32.const 0
                    local.get 1
                    i32.const 103
                    i32.mul
                    i32.const 8
                    i32.shr_u
                    local.get 1
                    i32.add
                    i32.const -179
                    i32.add
                    local.tee 0
                    i32.const 0
                    i32.load8_u offset=1872
                    i32.add
                    local.tee 1
                    i32.const -1
                    i32.xor
                    i32.const 15
                    i32.shr_u
                    local.get 1
                    local.get 1
                    i32.const 255
                    i32.gt_u
                    select
                    i32.store8 offset=1872
                    i32.const 0
                    local.get 0
                    i32.const 0
                    i32.load8_u offset=1936
                    i32.add
                    local.tee 1
                    i32.const -1
                    i32.xor
                    i32.const 15
                    i32.shr_u
                    local.get 1
                    local.get 1
                    i32.const 255
                    i32.gt_u
                    select
                    i32.store8 offset=1936
                    i32.const 0
                    local.get 0
                    i32.const 0
                    i32.load8_u offset=2000
                    i32.add
                    local.tee 1
                    i32.const -1
                    i32.xor
                    i32.const 15
                    i32.shr_u
                    local.get 1
                    local.get 1
                    i32.const 255
                    i32.gt_u
                    select
                    i32.store8 offset=2000
                    i32.const 0
                    local.get 0
                    i32.const 0
                    i32.load8_u offset=2064
                    i32.add
                    local.tee 0
                    i32.const -1
                    i32.xor
                    i32.const 15
                    i32.shr_u
                    local.get 0
                    local.get 0
                    i32.const 255
                    i32.gt_u
                    select
                    i32.store8 offset=2064
                    br 1 (;@7;)
                  end
                  i32.const 1
                  local.set 0
                  block  ;; label = @8
                    block  ;; label = @9
                      loop  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 5
                            local.get 4
                            call 5
                            local.tee 1
                            i32.const 15
                            i32.and
                            local.tee 6
                            i32.eqz
                            br_if 0 (;@12;)
                            local.get 6
                            i32.const 1
                            call 4
                            local.set 7
                            block  ;; label = @13
                              local.get 1
                              i32.const 16
                              i32.lt_u
                              br_if 0 (;@13;)
                              local.get 1
                              i32.const 4
                              i32.shr_u
                              local.tee 1
                              local.get 0
                              i32.add
                              i32.const 255
                              i32.and
                              i32.const 63
                              i32.gt_u
                              br_if 10 (;@3;)
                              local.get 0
                              i32.const 255
                              i32.and
                              local.set 0
                              loop  ;; label = @14
                                local.get 0
                                i32.const 1088
                                i32.add
                                i32.load8_s
                                i32.const 1
                                i32.shl
                                i32.const 3136
                                i32.add
                                i32.const 0
                                i32.store16
                                local.get 0
                                i32.const 1
                                i32.add
                                local.set 0
                                local.get 1
                                i32.const -1
                                i32.add
                                local.tee 1
                                i32.const 65535
                                i32.and
                                br_if 0 (;@14;)
                              end
                            end
                            local.get 0
                            i32.const 255
                            i32.and
                            local.tee 1
                            i32.const 1088
                            i32.add
                            i32.load8_s
                            i32.const 1
                            i32.shl
                            i32.const 3136
                            i32.add
                            local.get 3
                            local.get 1
                            i32.const 1
                            i32.shl
                            i32.add
                            i32.load16_u
                            local.get 7
                            local.get 6
                            call 6
                            i32.mul
                            i32.store16
                            br 1 (;@11;)
                          end
                          local.get 1
                          i32.const 240
                          i32.ne
                          br_if 2 (;@9;)
                          local.get 0
                          i32.const 255
                          i32.and
                          local.tee 0
                          i32.const 48
                          i32.gt_u
                          br_if 8 (;@3;)
                          local.get 0
                          i32.const -1
                          i32.add
                          local.set 0
                          i32.const 16
                          local.set 1
                          loop  ;; label = @12
                            local.get 0
                            i32.const 1089
                            i32.add
                            i32.load8_s
                            i32.const 1
                            i32.shl
                            i32.const 3136
                            i32.add
                            i32.const 0
                            i32.store16
                            local.get 0
                            i32.const 1
                            i32.add
                            local.set 0
                            local.get 1
                            i32.const -1
                            i32.add
                            local.tee 1
                            i32.const 65535
                            i32.and
                            br_if 0 (;@12;)
                          end
                        end
                        local.get 0
                        i32.const 1
                        i32.add
                        local.tee 0
                        i32.const 255
                        i32.and
                        i32.const 64
                        i32.lt_u
                        br_if 0 (;@10;)
                        br 2 (;@8;)
                      end
                    end
                    local.get 0
                    i32.const 255
                    i32.and
                    local.set 0
                    loop  ;; label = @9
                      local.get 0
                      i32.const 1088
                      i32.add
                      i32.load8_s
                      i32.const 1
                      i32.shl
                      i32.const 3136
                      i32.add
                      i32.const 0
                      i32.store16
                      local.get 0
                      i32.const 1
                      i32.add
                      local.tee 0
                      i32.const 255
                      i32.and
                      i32.const 64
                      i32.ne
                      br_if 0 (;@9;)
                    end
                  end
                  i32.const 3136
                  local.set 0
                  i32.const 0
                  local.set 4
                  loop  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 0
                        i32.load16_u offset=4
                        local.tee 1
                        local.get 0
                        i32.load16_u offset=2
                        local.tee 5
                        i32.or
                        local.get 0
                        i32.load16_u offset=6
                        local.tee 6
                        i32.or
                        local.get 0
                        i32.load16_u offset=8
                        local.tee 7
                        i32.or
                        local.get 0
                        i32.load16_u offset=10
                        local.tee 3
                        i32.or
                        local.get 0
                        i32.load16_u offset=12
                        local.tee 8
                        i32.or
                        local.get 0
                        i32.load16_u offset=14
                        local.tee 9
                        i32.or
                        i32.const 65535
                        i32.and
                        br_if 0 (;@10;)
                        local.get 0
                        i32.load16_u
                        local.tee 1
                        local.set 5
                        local.get 1
                        local.set 6
                        local.get 1
                        local.set 7
                        local.get 1
                        local.set 3
                        local.get 1
                        local.set 8
                        local.get 1
                        local.set 10
                        br 1 (;@9;)
                      end
                      local.get 0
                      local.get 0
                      i32.load16_u
                      local.tee 11
                      local.get 7
                      i32.add
                      local.tee 12
                      local.get 8
                      local.get 1
                      i32.add
                      local.tee 13
                      i32.add
                      local.tee 10
                      local.get 9
                      local.get 5
                      i32.add
                      local.tee 14
                      local.get 3
                      local.get 6
                      i32.add
                      local.tee 15
                      i32.add
                      local.tee 16
                      i32.add
                      i32.store16
                      local.get 10
                      local.get 16
                      i32.sub
                      local.set 10
                      local.get 11
                      local.get 7
                      i32.sub
                      local.tee 7
                      local.get 1
                      local.get 8
                      i32.sub
                      i32.extend16_s
                      i32.const 362
                      i32.mul
                      i32.const 128
                      i32.add
                      i32.const 8
                      i32.shr_u
                      local.get 13
                      i32.sub
                      local.tee 11
                      i32.add
                      local.tee 1
                      local.get 5
                      local.get 9
                      i32.sub
                      local.tee 5
                      i32.extend16_s
                      i32.const 277
                      i32.mul
                      i32.const 128
                      i32.add
                      i32.const 8
                      i32.shr_u
                      local.get 16
                      local.get 3
                      local.get 6
                      i32.sub
                      local.tee 9
                      local.get 5
                      i32.sub
                      i32.extend16_s
                      i32.const 196
                      i32.mul
                      i32.const 128
                      i32.add
                      i32.const 8
                      i32.shr_u
                      local.tee 17
                      i32.add
                      i32.sub
                      local.tee 5
                      i32.sub
                      local.set 8
                      local.get 1
                      local.get 5
                      i32.add
                      local.set 1
                      local.get 7
                      local.get 11
                      i32.sub
                      local.tee 7
                      local.get 14
                      local.get 15
                      i32.sub
                      i32.extend16_s
                      i32.const 362
                      i32.mul
                      i32.const 128
                      i32.add
                      i32.const 8
                      i32.shr_u
                      local.get 5
                      i32.sub
                      local.tee 6
                      i32.sub
                      local.set 3
                      local.get 7
                      local.get 6
                      i32.add
                      local.set 5
                      local.get 12
                      local.get 13
                      i32.sub
                      local.tee 13
                      local.get 6
                      local.get 17
                      local.get 9
                      i32.extend16_s
                      i32.const 669
                      i32.mul
                      i32.const 128
                      i32.add
                      i32.const 8
                      i32.shr_u
                      i32.sub
                      i32.add
                      local.tee 6
                      i32.add
                      local.set 7
                      local.get 13
                      local.get 6
                      i32.sub
                      local.set 6
                    end
                    local.get 0
                    local.get 10
                    i32.store16 offset=14
                    local.get 0
                    local.get 8
                    i32.store16 offset=12
                    local.get 0
                    local.get 3
                    i32.store16 offset=10
                    local.get 0
                    local.get 7
                    i32.store16 offset=8
                    local.get 0
                    local.get 6
                    i32.store16 offset=6
                    local.get 0
                    local.get 5
                    i32.store16 offset=4
                    local.get 0
                    local.get 1
                    i32.store16 offset=2
                    local.get 0
                    i32.const 16
                    i32.add
                    local.set 0
                    local.get 4
                    i32.const 255
                    i32.and
                    local.set 1
                    local.get 4
                    i32.const 1
                    i32.add
                    local.set 4
                    local.get 1
                    i32.const 7
                    i32.lt_u
                    br_if 0 (;@8;)
                  end
                  i32.const 3136
                  local.set 0
                  i32.const 0
                  local.set 4
                  loop  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 0
                        i32.load16_u offset=32
                        local.tee 1
                        local.get 0
                        i32.load16_u offset=16
                        local.tee 5
                        i32.or
                        local.get 0
                        i32.load16_u offset=48
                        local.tee 6
                        i32.or
                        local.get 0
                        i32.load16_u offset=64
                        local.tee 7
                        i32.or
                        local.get 0
                        i32.load16_u offset=80
                        local.tee 3
                        i32.or
                        local.get 0
                        i32.load16_u offset=96
                        local.tee 8
                        i32.or
                        local.get 0
                        i32.load16_u offset=112
                        local.tee 9
                        i32.or
                        i32.const 65535
                        i32.and
                        br_if 0 (;@10;)
                        local.get 0
                        local.get 0
                        i32.load16_s
                        i32.const 64
                        i32.add
                        i32.const 7
                        i32.shr_u
                        i32.const 128
                        i32.add
                        local.tee 1
                        i32.extend16_s
                        i32.const -1
                        i32.xor
                        i32.const 15
                        i32.shr_u
                        local.get 1
                        local.get 1
                        i32.const 65535
                        i32.and
                        i32.const 255
                        i32.gt_u
                        select
                        i32.const 255
                        i32.and
                        local.tee 1
                        i32.store16 offset=32
                        local.get 0
                        local.get 1
                        i32.store16 offset=16
                        local.get 0
                        local.get 1
                        i32.store16
                        local.get 1
                        local.set 5
                        local.get 1
                        local.set 6
                        local.get 1
                        local.set 7
                        local.get 1
                        local.set 3
                        br 1 (;@9;)
                      end
                      local.get 0
                      local.get 9
                      local.get 5
                      i32.add
                      local.tee 16
                      local.get 3
                      local.get 6
                      i32.add
                      local.tee 11
                      i32.add
                      local.tee 12
                      i32.extend16_s
                      local.tee 14
                      local.get 0
                      i32.load16_u
                      local.tee 15
                      local.get 7
                      i32.add
                      local.tee 17
                      local.get 8
                      local.get 1
                      i32.add
                      local.tee 10
                      i32.add
                      i32.extend16_s
                      local.tee 18
                      i32.add
                      i32.const 64
                      i32.add
                      i32.const 7
                      i32.shr_u
                      i32.const 128
                      i32.add
                      local.tee 13
                      i32.extend16_s
                      i32.const -1
                      i32.xor
                      i32.const 15
                      i32.shr_u
                      local.get 13
                      local.get 13
                      i32.const 65535
                      i32.and
                      i32.const 255
                      i32.gt_u
                      select
                      i32.const 255
                      i32.and
                      i32.store16
                      local.get 0
                      local.get 5
                      local.get 9
                      i32.sub
                      local.tee 5
                      i32.extend16_s
                      i32.const 277
                      i32.mul
                      i32.const 128
                      i32.add
                      i32.const 8
                      i32.shr_u
                      local.get 12
                      local.get 3
                      local.get 6
                      i32.sub
                      local.tee 9
                      local.get 5
                      i32.sub
                      i32.extend16_s
                      i32.const 196
                      i32.mul
                      i32.const 128
                      i32.add
                      i32.const 8
                      i32.shr_u
                      local.tee 5
                      i32.add
                      i32.sub
                      local.tee 6
                      i32.extend16_s
                      local.tee 13
                      local.get 15
                      local.get 7
                      i32.sub
                      local.tee 7
                      local.get 1
                      local.get 8
                      i32.sub
                      i32.extend16_s
                      i32.const 362
                      i32.mul
                      i32.const 128
                      i32.add
                      i32.const 8
                      i32.shr_u
                      local.get 10
                      i32.sub
                      local.tee 3
                      i32.add
                      i32.extend16_s
                      local.tee 8
                      i32.add
                      i32.const 64
                      i32.add
                      i32.const 7
                      i32.shr_u
                      i32.const 128
                      i32.add
                      local.tee 1
                      i32.extend16_s
                      i32.const -1
                      i32.xor
                      i32.const 15
                      i32.shr_u
                      local.get 1
                      local.get 1
                      i32.const 65535
                      i32.and
                      i32.const 255
                      i32.gt_u
                      select
                      i32.const 255
                      i32.and
                      i32.store16 offset=16
                      local.get 0
                      local.get 16
                      local.get 11
                      i32.sub
                      i32.extend16_s
                      i32.const 362
                      i32.mul
                      i32.const 128
                      i32.add
                      i32.const 8
                      i32.shr_u
                      local.get 6
                      i32.sub
                      local.tee 16
                      i32.extend16_s
                      local.tee 6
                      local.get 7
                      local.get 3
                      i32.sub
                      i32.extend16_s
                      local.tee 11
                      i32.add
                      i32.const 64
                      i32.add
                      i32.const 7
                      i32.shr_u
                      i32.const 128
                      i32.add
                      local.tee 1
                      i32.extend16_s
                      i32.const -1
                      i32.xor
                      i32.const 15
                      i32.shr_u
                      local.get 1
                      local.get 1
                      i32.const 65535
                      i32.and
                      i32.const 255
                      i32.gt_u
                      select
                      i32.const 255
                      i32.and
                      i32.store16 offset=32
                      local.get 18
                      local.get 14
                      i32.sub
                      i32.const 64
                      i32.add
                      i32.const 7
                      i32.shr_u
                      i32.const 128
                      i32.add
                      local.tee 1
                      i32.extend16_s
                      i32.const -1
                      i32.xor
                      i32.const 15
                      i32.shr_u
                      local.get 1
                      local.get 1
                      i32.const 65535
                      i32.and
                      i32.const 255
                      i32.gt_u
                      select
                      i32.const 255
                      i32.and
                      local.set 3
                      local.get 8
                      local.get 13
                      i32.sub
                      i32.const 64
                      i32.add
                      i32.const 7
                      i32.shr_u
                      i32.const 128
                      i32.add
                      local.tee 1
                      i32.extend16_s
                      i32.const -1
                      i32.xor
                      i32.const 15
                      i32.shr_u
                      local.get 1
                      local.get 1
                      i32.const 65535
                      i32.and
                      i32.const 255
                      i32.gt_u
                      select
                      i32.const 255
                      i32.and
                      local.set 7
                      local.get 11
                      local.get 6
                      i32.sub
                      i32.const 64
                      i32.add
                      i32.const 7
                      i32.shr_u
                      i32.const 128
                      i32.add
                      local.tee 1
                      i32.extend16_s
                      i32.const -1
                      i32.xor
                      i32.const 15
                      i32.shr_u
                      local.get 1
                      local.get 1
                      i32.const 65535
                      i32.and
                      i32.const 255
                      i32.gt_u
                      select
                      i32.const 255
                      i32.and
                      local.set 6
                      local.get 5
                      local.get 9
                      i32.extend16_s
                      i32.const 669
                      i32.mul
                      i32.const 128
                      i32.add
                      i32.const 8
                      i32.shr_u
                      i32.sub
                      local.get 16
                      i32.add
                      i32.extend16_s
                      local.tee 8
                      local.get 17
                      local.get 10
                      i32.sub
                      i32.extend16_s
                      local.tee 9
                      i32.add
                      i32.const 64
                      i32.add
                      i32.const 7
                      i32.shr_u
                      i32.const 128
                      i32.add
                      local.tee 1
                      i32.extend16_s
                      i32.const -1
                      i32.xor
                      i32.const 15
                      i32.shr_u
                      local.get 1
                      local.get 1
                      i32.const 65535
                      i32.and
                      i32.const 255
                      i32.gt_u
                      select
                      i32.const 255
                      i32.and
                      local.set 5
                      local.get 9
                      local.get 8
                      i32.sub
                      i32.const 64
                      i32.add
                      i32.const 7
                      i32.shr_u
                      i32.const 128
                      i32.add
                      local.tee 1
                      i32.extend16_s
                      i32.const -1
                      i32.xor
                      i32.const 15
                      i32.shr_u
                      local.get 1
                      local.get 1
                      i32.const 65535
                      i32.and
                      i32.const 255
                      i32.gt_u
                      select
                      i32.const 255
                      i32.and
                      local.set 1
                    end
                    local.get 0
                    local.get 3
                    i32.store16 offset=112
                    local.get 0
                    local.get 7
                    i32.store16 offset=96
                    local.get 0
                    local.get 6
                    i32.store16 offset=80
                    local.get 0
                    local.get 5
                    i32.store16 offset=64
                    local.get 0
                    local.get 1
                    i32.store16 offset=48
                    local.get 0
                    i32.const 2
                    i32.add
                    local.set 0
                    local.get 4
                    i32.const 255
                    i32.and
                    local.set 1
                    local.get 4
                    i32.const 1
                    i32.add
                    local.set 4
                    local.get 1
                    i32.const 7
                    i32.lt_u
                    br_if 0 (;@8;)
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            i32.const 0
                            i32.load offset=1848
                            br_table 0 (;@12;) 1 (;@11;) 3 (;@9;) 2 (;@10;) 4 (;@8;) 0 (;@12;)
                          end
                          i32.const 3136
                          local.set 1
                          i32.const 64
                          local.set 5
                          i32.const 0
                          local.set 0
                          loop  ;; label = @12
                            local.get 0
                            i32.const 1872
                            i32.add
                            local.get 1
                            i32.load8_u
                            local.tee 4
                            i32.store8
                            local.get 0
                            i32.const 2128
                            i32.add
                            local.get 4
                            i32.store8
                            local.get 0
                            i32.const 2384
                            i32.add
                            local.get 4
                            i32.store8
                            local.get 0
                            i32.const 1
                            i32.add
                            local.set 0
                            local.get 1
                            i32.const 2
                            i32.add
                            local.set 1
                            local.get 5
                            i32.const -1
                            i32.add
                            local.tee 5
                            i32.const 255
                            i32.and
                            br_if 0 (;@12;)
                            br 5 (;@7;)
                          end
                        end
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 2
                              i32.const 255
                              i32.and
                              br_table 0 (;@13;) 1 (;@12;) 2 (;@11;) 6 (;@7;)
                            end
                            i32.const 3136
                            local.set 1
                            i32.const 64
                            local.set 5
                            i32.const 0
                            local.set 0
                            loop  ;; label = @13
                              local.get 0
                              i32.const 1872
                              i32.add
                              local.get 1
                              i32.load8_u
                              local.tee 4
                              i32.store8
                              local.get 0
                              i32.const 2128
                              i32.add
                              local.get 4
                              i32.store8
                              local.get 0
                              i32.const 2384
                              i32.add
                              local.get 4
                              i32.store8
                              local.get 0
                              i32.const 1
                              i32.add
                              local.set 0
                              local.get 1
                              i32.const 2
                              i32.add
                              local.set 1
                              local.get 5
                              i32.const -1
                              i32.add
                              local.tee 5
                              i32.const 255
                              i32.and
                              br_if 0 (;@13;)
                              br 6 (;@7;)
                            end
                          end
                          i32.const 3136
                          local.set 1
                          i32.const 64
                          local.set 6
                          i32.const 0
                          local.set 0
                          loop  ;; label = @12
                            local.get 0
                            i32.const 2128
                            i32.add
                            local.tee 4
                            local.get 4
                            i32.load8_u
                            local.get 1
                            i32.load8_u
                            local.tee 4
                            i32.const 88
                            i32.mul
                            i32.const 8
                            i32.shr_u
                            i32.sub
                            i32.const 44
                            i32.add
                            local.tee 5
                            i32.const -1
                            i32.xor
                            i32.const 15
                            i32.shr_u
                            local.get 5
                            local.get 5
                            i32.const 255
                            i32.gt_u
                            select
                            i32.store8
                            local.get 0
                            i32.const 2384
                            i32.add
                            local.tee 5
                            local.get 4
                            local.get 4
                            i32.const 198
                            i32.mul
                            i32.const 8
                            i32.shr_u
                            i32.add
                            local.get 5
                            i32.load8_u
                            i32.add
                            i32.const -227
                            i32.add
                            local.tee 4
                            i32.const -1
                            i32.xor
                            i32.const 15
                            i32.shr_u
                            local.get 4
                            local.get 4
                            i32.const 255
                            i32.gt_u
                            select
                            i32.store8
                            local.get 0
                            i32.const 1
                            i32.add
                            local.set 0
                            local.get 1
                            i32.const 2
                            i32.add
                            local.set 1
                            local.get 6
                            i32.const -1
                            i32.add
                            local.tee 6
                            i32.const 255
                            i32.and
                            br_if 0 (;@12;)
                            br 5 (;@7;)
                          end
                        end
                        i32.const 3136
                        local.set 1
                        i32.const 64
                        local.set 5
                        i32.const 0
                        local.set 0
                        loop  ;; label = @11
                          local.get 0
                          i32.const 2128
                          i32.add
                          local.tee 4
                          i32.const -1
                          i32.const 0
                          local.get 4
                          i32.load8_u
                          local.get 1
                          i32.load8_u
                          local.tee 4
                          i32.const 183
                          i32.mul
                          i32.const 8
                          i32.shr_u
                          i32.sub
                          local.tee 6
                          i32.const -92
                          i32.gt_s
                          select
                          local.get 6
                          i32.const 91
                          i32.add
                          local.tee 6
                          local.get 6
                          i32.const 255
                          i32.gt_u
                          select
                          i32.store8
                          local.get 0
                          i32.const 1872
                          i32.add
                          local.tee 6
                          local.get 4
                          local.get 6
                          i32.load8_u
                          i32.add
                          local.get 4
                          i32.const 103
                          i32.mul
                          i32.const 8
                          i32.shr_u
                          i32.add
                          i32.const -179
                          i32.add
                          local.tee 4
                          i32.const -1
                          i32.xor
                          i32.const 15
                          i32.shr_u
                          local.get 4
                          local.get 4
                          i32.const 255
                          i32.gt_u
                          select
                          i32.store8
                          local.get 0
                          i32.const 1
                          i32.add
                          local.set 0
                          local.get 1
                          i32.const 2
                          i32.add
                          local.set 1
                          local.get 5
                          i32.const -1
                          i32.add
                          local.tee 5
                          i32.const 255
                          i32.and
                          br_if 0 (;@11;)
                          br 4 (;@7;)
                        end
                      end
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 2
                              i32.const 255
                              i32.and
                              br_table 0 (;@13;) 1 (;@12;) 2 (;@11;) 3 (;@10;) 6 (;@7;)
                            end
                            i32.const 3136
                            local.set 1
                            i32.const 64
                            local.set 5
                            i32.const 0
                            local.set 0
                            loop  ;; label = @13
                              local.get 0
                              i32.const 1872
                              i32.add
                              local.get 1
                              i32.load8_u
                              local.tee 4
                              i32.store8
                              local.get 0
                              i32.const 2128
                              i32.add
                              local.get 4
                              i32.store8
                              local.get 0
                              i32.const 2384
                              i32.add
                              local.get 4
                              i32.store8
                              local.get 0
                              i32.const 1
                              i32.add
                              local.set 0
                              local.get 1
                              i32.const 2
                              i32.add
                              local.set 1
                              local.get 5
                              i32.const -1
                              i32.add
                              local.tee 5
                              i32.const 255
                              i32.and
                              br_if 0 (;@13;)
                              br 6 (;@7;)
                            end
                          end
                          i32.const 3136
                          local.set 1
                          i32.const 64
                          local.set 5
                          i32.const 128
                          local.set 0
                          loop  ;; label = @12
                            local.get 0
                            i32.const 1872
                            i32.add
                            local.get 1
                            i32.load8_u
                            local.tee 4
                            i32.store8
                            local.get 0
                            i32.const 2128
                            i32.add
                            local.get 4
                            i32.store8
                            local.get 0
                            i32.const 2384
                            i32.add
                            local.get 4
                            i32.store8
                            local.get 0
                            i32.const 1
                            i32.add
                            local.set 0
                            local.get 1
                            i32.const 2
                            i32.add
                            local.set 1
                            local.get 5
                            i32.const -1
                            i32.add
                            local.tee 5
                            i32.const 255
                            i32.and
                            br_if 0 (;@12;)
                            br 5 (;@7;)
                          end
                        end
                        i32.const 0
                        i32.const 0
                        call 7
                        i32.const 32
                        i32.const 128
                        call 7
                        br 3 (;@7;)
                      end
                      i32.const 0
                      i32.const 0
                      call 8
                      i32.const 32
                      i32.const 128
                      call 8
                      br 2 (;@7;)
                    end
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 2
                            i32.const 255
                            i32.and
                            br_table 0 (;@12;) 1 (;@11;) 2 (;@10;) 3 (;@9;) 5 (;@7;)
                          end
                          i32.const 3136
                          local.set 1
                          i32.const 64
                          local.set 5
                          i32.const 0
                          local.set 0
                          loop  ;; label = @12
                            local.get 0
                            i32.const 1872
                            i32.add
                            local.get 1
                            i32.load8_u
                            local.tee 4
                            i32.store8
                            local.get 0
                            i32.const 2128
                            i32.add
                            local.get 4
                            i32.store8
                            local.get 0
                            i32.const 2384
                            i32.add
                            local.get 4
                            i32.store8
                            local.get 0
                            i32.const 1
                            i32.add
                            local.set 0
                            local.get 1
                            i32.const 2
                            i32.add
                            local.set 1
                            local.get 5
                            i32.const -1
                            i32.add
                            local.tee 5
                            i32.const 255
                            i32.and
                            br_if 0 (;@12;)
                            br 5 (;@7;)
                          end
                        end
                        i32.const 3136
                        local.set 1
                        i32.const 64
                        local.set 0
                        i32.const 64
                        local.set 5
                        loop  ;; label = @11
                          local.get 0
                          i32.const 1872
                          i32.add
                          local.get 1
                          i32.load8_u
                          local.tee 4
                          i32.store8
                          local.get 0
                          i32.const 2128
                          i32.add
                          local.get 4
                          i32.store8
                          local.get 0
                          i32.const 2384
                          i32.add
                          local.get 4
                          i32.store8
                          local.get 0
                          i32.const 1
                          i32.add
                          local.set 0
                          local.get 1
                          i32.const 2
                          i32.add
                          local.set 1
                          local.get 5
                          i32.const -1
                          i32.add
                          local.tee 5
                          i32.const 255
                          i32.and
                          br_if 0 (;@11;)
                          br 4 (;@7;)
                        end
                      end
                      i32.const 0
                      i32.const 0
                      call 9
                      i32.const 4
                      i32.const 64
                      call 9
                      br 2 (;@7;)
                    end
                    i32.const 0
                    i32.const 0
                    call 10
                    i32.const 4
                    i32.const 64
                    call 10
                    br 1 (;@7;)
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 2
                              i32.const 255
                              i32.and
                              br_table 0 (;@13;) 1 (;@12;) 2 (;@11;) 3 (;@10;) 4 (;@9;) 5 (;@8;) 6 (;@7;)
                            end
                            i32.const 3136
                            local.set 1
                            i32.const 64
                            local.set 5
                            i32.const 0
                            local.set 0
                            loop  ;; label = @13
                              local.get 0
                              i32.const 1872
                              i32.add
                              local.get 1
                              i32.load8_u
                              local.tee 4
                              i32.store8
                              local.get 0
                              i32.const 2128
                              i32.add
                              local.get 4
                              i32.store8
                              local.get 0
                              i32.const 2384
                              i32.add
                              local.get 4
                              i32.store8
                              local.get 0
                              i32.const 1
                              i32.add
                              local.set 0
                              local.get 1
                              i32.const 2
                              i32.add
                              local.set 1
                              local.get 5
                              i32.const -1
                              i32.add
                              local.tee 5
                              i32.const 255
                              i32.and
                              br_if 0 (;@13;)
                              br 6 (;@7;)
                            end
                          end
                          i32.const 3136
                          local.set 1
                          i32.const 64
                          local.set 0
                          i32.const 64
                          local.set 5
                          loop  ;; label = @12
                            local.get 0
                            i32.const 1872
                            i32.add
                            local.get 1
                            i32.load8_u
                            local.tee 4
                            i32.store8
                            local.get 0
                            i32.const 2128
                            i32.add
                            local.get 4
                            i32.store8
                            local.get 0
                            i32.const 2384
                            i32.add
                            local.get 4
                            i32.store8
                            local.get 0
                            i32.const 1
                            i32.add
                            local.set 0
                            local.get 1
                            i32.const 2
                            i32.add
                            local.set 1
                            local.get 5
                            i32.const -1
                            i32.add
                            local.tee 5
                            i32.const 255
                            i32.and
                            br_if 0 (;@12;)
                            br 5 (;@7;)
                          end
                        end
                        i32.const 3136
                        local.set 1
                        i32.const 64
                        local.set 5
                        i32.const 128
                        local.set 0
                        loop  ;; label = @11
                          local.get 0
                          i32.const 1872
                          i32.add
                          local.get 1
                          i32.load8_u
                          local.tee 4
                          i32.store8
                          local.get 0
                          i32.const 2128
                          i32.add
                          local.get 4
                          i32.store8
                          local.get 0
                          i32.const 2384
                          i32.add
                          local.get 4
                          i32.store8
                          local.get 0
                          i32.const 1
                          i32.add
                          local.set 0
                          local.get 1
                          i32.const 2
                          i32.add
                          local.set 1
                          local.get 5
                          i32.const -1
                          i32.add
                          local.tee 5
                          i32.const 255
                          i32.and
                          br_if 0 (;@11;)
                          br 4 (;@7;)
                        end
                      end
                      i32.const 3136
                      local.set 1
                      i32.const 64
                      local.set 5
                      i32.const 192
                      local.set 0
                      loop  ;; label = @10
                        local.get 0
                        i32.const 1872
                        i32.add
                        local.get 1
                        i32.load8_u
                        local.tee 4
                        i32.store8
                        local.get 0
                        i32.const 2128
                        i32.add
                        local.get 4
                        i32.store8
                        local.get 0
                        i32.const 2384
                        i32.add
                        local.get 4
                        i32.store8
                        local.get 0
                        i32.const 1
                        i32.add
                        local.set 0
                        local.get 1
                        i32.const 2
                        i32.add
                        local.set 1
                        local.get 5
                        i32.const -1
                        i32.add
                        local.tee 5
                        i32.const 255
                        i32.and
                        br_if 0 (;@10;)
                        br 3 (;@7;)
                      end
                    end
                    i32.const 0
                    i32.const 0
                    call 11
                    i32.const 4
                    i32.const 64
                    call 11
                    i32.const 32
                    i32.const 128
                    call 11
                    i32.const 36
                    i32.const 192
                    call 11
                    br 1 (;@7;)
                  end
                  i32.const 0
                  i32.const 0
                  call 12
                  i32.const 4
                  i32.const 64
                  call 12
                  i32.const 32
                  i32.const 128
                  call 12
                  i32.const 36
                  i32.const 192
                  call 12
                end
                local.get 2
                i32.const 1
                i32.add
                local.tee 2
                i32.const 0
                i32.load8_u offset=2648
                i32.lt_u
                br_if 0 (;@6;)
              end
            end
            i32.const 0
            i32.load8_u offset=1828
            local.tee 0
            br_if 0 (;@4;)
            i32.const 0
            i32.const 0
            i32.load16_u offset=1830
            i32.const -1
            i32.add
            i32.store16 offset=1830
            i32.const 0
            local.set 0
          end
          local.get 0
          return
        end
        i32.const 28
        local.set 0
        br 1 (;@1;)
      end
      i32.const 29
      local.set 0
    end
    i32.const 0
    i32.load8_u offset=1828
    local.tee 1
    local.get 0
    local.get 1
    select)
  (func (;3;) (type 2) (result i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        i32.const 0
        i32.load8_u offset=3955
        local.tee 0
        br_if 0 (;@2;)
        i32.const 0
        i32.const 4
        i32.store8 offset=4224
        block  ;; label = @3
          i32.const 3972
          i32.const 252
          i32.const 3955
          i32.const 0
          i32.load offset=1836
          i32.const 0
          i32.load offset=1832
          call_indirect (type 0)
          local.tee 0
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          local.get 0
          i32.store8 offset=1828
        end
        i32.const 0
        i32.load8_u offset=3955
        local.tee 0
        br_if 0 (;@2;)
        i32.const 0
        i32.const 0
        i32.load8_u offset=3956
        local.tee 0
        i32.const -1
        i32.xor
        i32.store8 offset=3956
        i32.const -39
        i32.const -1
        local.get 0
        i32.const 255
        i32.eq
        select
        local.set 0
        br 1 (;@1;)
      end
      i32.const 0
      local.get 0
      i32.const -1
      i32.add
      i32.store8 offset=3955
      i32.const 0
      i32.const 0
      i32.load8_u offset=4224
      local.tee 0
      i32.const 1
      i32.add
      i32.store8 offset=4224
      local.get 0
      i32.const 3968
      i32.add
      i32.load8_u
      local.set 0
    end
    local.get 0
    i32.const 255
    i32.and)
  (func (;4;) (type 3) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    i32.const 0
    i32.load8_u offset=3954
    local.set 2
    i32.const 0
    i32.load16_u offset=4226
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.const 9
        i32.ge_u
        br_if 0 (;@2;)
        local.get 3
        local.set 4
        local.get 0
        local.set 5
        br 1 (;@1;)
      end
      i32.const 0
      local.get 3
      local.get 2
      i32.shl
      i32.store16 offset=4226
      local.get 1
      call 17
      i32.const 0
      i32.load16_u offset=4226
      i32.or
      i32.const 8
      i32.const 0
      i32.load8_u offset=3954
      local.tee 2
      i32.sub
      i32.shl
      local.tee 4
      i32.const 8
      i32.shr_u
      i32.const 255
      i32.and
      local.get 3
      i32.const 65280
      i32.and
      i32.or
      local.set 3
      local.get 0
      i32.const -8
      i32.add
      local.set 5
    end
    local.get 4
    i32.const 65535
    i32.and
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        local.get 5
        i32.const 255
        i32.and
        local.tee 6
        i32.ge_u
        br_if 0 (;@2;)
        i32.const 0
        local.get 4
        local.get 2
        i32.shl
        i32.store16 offset=4226
        local.get 1
        call 17
        i32.const 0
        i32.load16_u offset=4226
        i32.or
        local.get 6
        i32.const 0
        i32.load8_u offset=3954
        local.tee 2
        i32.sub
        i32.shl
        local.set 1
        local.get 2
        local.get 5
        i32.sub
        i32.const 8
        i32.add
        local.set 2
        br 1 (;@1;)
      end
      local.get 4
      local.get 6
      i32.shl
      local.set 1
      local.get 2
      local.get 5
      i32.sub
      local.set 2
    end
    i32.const 0
    local.get 2
    i32.store8 offset=3954
    i32.const 0
    local.get 1
    i32.store16 offset=4226
    local.get 3
    i32.const 16
    local.get 0
    i32.sub
    i32.shr_u)
  (func (;5;) (type 3) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    local.get 0
    i32.const 32
    i32.add
    local.set 2
    i32.const 0
    local.set 3
    call 16
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        loop  ;; label = @3
          block  ;; label = @4
            local.get 4
            i32.const 65535
            i32.and
            local.get 2
            local.get 3
            i32.const 255
            i32.and
            local.tee 5
            i32.const 1
            i32.shl
            i32.add
            i32.load16_u
            local.tee 6
            i32.gt_u
            br_if 0 (;@4;)
            local.get 6
            i32.const 65535
            i32.ne
            br_if 2 (;@2;)
          end
          local.get 4
          i32.const 1
          i32.shl
          call 16
          i32.or
          local.set 4
          local.get 3
          i32.const 1
          i32.add
          local.tee 3
          i32.const 255
          i32.and
          i32.const 16
          i32.ne
          br_if 0 (;@3;)
        end
        i32.const 0
        local.set 4
        br 1 (;@1;)
      end
      local.get 1
      local.get 0
      local.get 5
      i32.add
      i32.const 64
      i32.add
      i32.load8_u
      local.get 4
      i32.add
      local.get 0
      local.get 5
      i32.const 1
      i32.shl
      i32.add
      i32.load8_u
      i32.sub
      i32.const 255
      i32.and
      i32.add
      i32.load8_u
      local.set 4
    end
    local.get 4
    i32.const 255
    i32.and)
  (func (;6;) (type 3) (param i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      local.get 1
      i32.const -1
      i32.add
      i32.const 255
      i32.and
      local.tee 2
      i32.const 14
      i32.gt_u
      br_if 0 (;@1;)
      local.get 2
      i32.const 1
      i32.shl
      i32.const 1152
      i32.add
      i32.load16_u
      local.get 0
      i32.le_u
      br_if 0 (;@1;)
      local.get 1
      i32.extend8_s
      i32.const 1
      i32.shl
      i32.const 1180
      i32.add
      i32.load16_u
      local.get 0
      i32.add
      local.set 0
    end
    local.get 0
    i32.extend16_s)
  (func (;7;) (type 4) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    local.get 1
    i32.const 2384
    i32.add
    local.set 2
    local.get 1
    i32.const 2128
    i32.add
    local.set 1
    local.get 0
    i32.const 1
    i32.shl
    i32.const 3136
    i32.add
    local.set 3
    i32.const 0
    local.set 4
    loop  ;; label = @1
      i32.const 0
      local.set 0
      loop  ;; label = @2
        local.get 1
        local.get 1
        i32.load8_u
        local.get 3
        i32.load8_u
        local.tee 5
        i32.const 88
        i32.mul
        i32.const 8
        i32.shr_u
        i32.const -44
        i32.add
        local.tee 6
        i32.sub
        local.tee 7
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 7
        local.get 7
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 1
        i32.const 8
        i32.add
        local.tee 7
        local.get 7
        i32.load8_u
        local.get 6
        i32.sub
        local.tee 7
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 7
        local.get 7
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 2
        local.get 5
        local.get 5
        i32.const 198
        i32.mul
        i32.const 8
        i32.shr_u
        i32.add
        i32.const -227
        i32.add
        local.tee 7
        local.get 2
        i32.load8_u
        i32.add
        local.tee 5
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 5
        local.get 5
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 2
        i32.const 8
        i32.add
        local.tee 5
        local.get 7
        local.get 5
        i32.load8_u
        i32.add
        local.tee 5
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 5
        local.get 5
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 2
        i32.const 1
        i32.add
        local.set 2
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 3
        i32.const 2
        i32.add
        local.set 3
        local.get 0
        i32.const 255
        i32.and
        local.set 5
        local.get 0
        i32.const 1
        i32.add
        local.set 0
        local.get 5
        i32.const 7
        i32.lt_u
        br_if 0 (;@2;)
      end
      local.get 2
      i32.const 8
      i32.add
      local.set 2
      local.get 1
      i32.const 8
      i32.add
      local.set 1
      local.get 4
      i32.const 255
      i32.and
      local.set 0
      local.get 4
      i32.const 1
      i32.add
      local.set 4
      local.get 0
      i32.const 3
      i32.lt_u
      br_if 0 (;@1;)
    end)
  (func (;8;) (type 4) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    local.get 1
    i32.const 2128
    i32.add
    local.set 2
    local.get 1
    i32.const 1872
    i32.add
    local.set 1
    local.get 0
    i32.const 1
    i32.shl
    i32.const 3136
    i32.add
    local.set 3
    i32.const 0
    local.set 4
    loop  ;; label = @1
      i32.const 0
      local.set 0
      loop  ;; label = @2
        local.get 2
        local.get 2
        i32.load8_u
        local.get 3
        i32.load8_u
        local.tee 5
        i32.const 183
        i32.mul
        i32.const 8
        i32.shr_u
        i32.const -91
        i32.add
        local.tee 6
        i32.sub
        local.tee 7
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 7
        local.get 7
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 2
        i32.const 8
        i32.add
        local.tee 7
        local.get 7
        i32.load8_u
        local.get 6
        i32.sub
        local.tee 7
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 7
        local.get 7
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 1
        local.get 5
        local.get 5
        i32.const 103
        i32.mul
        i32.const 8
        i32.shr_u
        i32.add
        i32.const -179
        i32.add
        local.tee 7
        local.get 1
        i32.load8_u
        i32.add
        local.tee 5
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 5
        local.get 5
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 1
        i32.const 8
        i32.add
        local.tee 5
        local.get 7
        local.get 5
        i32.load8_u
        i32.add
        local.tee 5
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 5
        local.get 5
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 2
        i32.const 1
        i32.add
        local.set 2
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 3
        i32.const 2
        i32.add
        local.set 3
        local.get 0
        i32.const 255
        i32.and
        local.set 5
        local.get 0
        i32.const 1
        i32.add
        local.set 0
        local.get 5
        i32.const 7
        i32.lt_u
        br_if 0 (;@2;)
      end
      local.get 2
      i32.const 8
      i32.add
      local.set 2
      local.get 1
      i32.const 8
      i32.add
      local.set 1
      local.get 4
      i32.const 255
      i32.and
      local.set 0
      local.get 4
      i32.const 1
      i32.add
      local.set 4
      local.get 0
      i32.const 3
      i32.lt_u
      br_if 0 (;@1;)
    end)
  (func (;9;) (type 4) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    local.get 1
    i32.const 2384
    i32.add
    local.set 2
    local.get 1
    i32.const 2128
    i32.add
    local.set 1
    local.get 0
    i32.const 1
    i32.shl
    i32.const 3136
    i32.add
    local.set 0
    i32.const 0
    local.set 3
    loop  ;; label = @1
      i32.const 0
      local.set 4
      loop  ;; label = @2
        local.get 1
        local.get 1
        i32.load8_u
        local.get 0
        i32.load8_u
        local.tee 5
        i32.const 88
        i32.mul
        i32.const 8
        i32.shr_u
        i32.const -44
        i32.add
        local.tee 6
        i32.sub
        local.tee 7
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 7
        local.get 7
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 1
        i32.const 1
        i32.add
        local.tee 7
        local.get 7
        i32.load8_u
        local.get 6
        i32.sub
        local.tee 7
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 7
        local.get 7
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 2
        local.get 5
        local.get 5
        i32.const 198
        i32.mul
        i32.const 8
        i32.shr_u
        i32.add
        i32.const -227
        i32.add
        local.tee 7
        local.get 2
        i32.load8_u
        i32.add
        local.tee 5
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 5
        local.get 5
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 2
        i32.const 1
        i32.add
        local.tee 5
        local.get 7
        local.get 5
        i32.load8_u
        i32.add
        local.tee 5
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 5
        local.get 5
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 2
        i32.const 2
        i32.add
        local.set 2
        local.get 1
        i32.const 2
        i32.add
        local.set 1
        local.get 0
        i32.const 2
        i32.add
        local.set 0
        local.get 4
        i32.const 255
        i32.and
        local.set 5
        local.get 4
        i32.const 1
        i32.add
        local.set 4
        local.get 5
        i32.const 3
        i32.lt_u
        br_if 0 (;@2;)
      end
      local.get 0
      i32.const 8
      i32.add
      local.set 0
      local.get 3
      i32.const 255
      i32.and
      local.set 4
      local.get 3
      i32.const 1
      i32.add
      local.set 3
      local.get 4
      i32.const 7
      i32.lt_u
      br_if 0 (;@1;)
    end)
  (func (;10;) (type 4) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    local.get 1
    i32.const 2128
    i32.add
    local.set 2
    local.get 1
    i32.const 1872
    i32.add
    local.set 1
    local.get 0
    i32.const 1
    i32.shl
    i32.const 3136
    i32.add
    local.set 0
    i32.const 0
    local.set 3
    loop  ;; label = @1
      i32.const 0
      local.set 4
      loop  ;; label = @2
        local.get 2
        local.get 2
        i32.load8_u
        local.get 0
        i32.load8_u
        local.tee 5
        i32.const 183
        i32.mul
        i32.const 8
        i32.shr_u
        i32.const -91
        i32.add
        local.tee 6
        i32.sub
        local.tee 7
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 7
        local.get 7
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 2
        i32.const 1
        i32.add
        local.tee 7
        local.get 7
        i32.load8_u
        local.get 6
        i32.sub
        local.tee 7
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 7
        local.get 7
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 1
        local.get 5
        local.get 5
        i32.const 103
        i32.mul
        i32.const 8
        i32.shr_u
        i32.add
        i32.const -179
        i32.add
        local.tee 7
        local.get 1
        i32.load8_u
        i32.add
        local.tee 5
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 5
        local.get 5
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 1
        i32.const 1
        i32.add
        local.tee 5
        local.get 7
        local.get 5
        i32.load8_u
        i32.add
        local.tee 5
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 5
        local.get 5
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 2
        i32.const 2
        i32.add
        local.set 2
        local.get 1
        i32.const 2
        i32.add
        local.set 1
        local.get 0
        i32.const 2
        i32.add
        local.set 0
        local.get 4
        i32.const 255
        i32.and
        local.set 5
        local.get 4
        i32.const 1
        i32.add
        local.set 4
        local.get 5
        i32.const 3
        i32.lt_u
        br_if 0 (;@2;)
      end
      local.get 0
      i32.const 8
      i32.add
      local.set 0
      local.get 3
      i32.const 255
      i32.and
      local.set 4
      local.get 3
      i32.const 1
      i32.add
      local.set 3
      local.get 4
      i32.const 7
      i32.lt_u
      br_if 0 (;@1;)
    end)
  (func (;11;) (type 4) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    local.get 1
    i32.const 2384
    i32.add
    local.set 2
    local.get 1
    i32.const 2128
    i32.add
    local.set 1
    local.get 0
    i32.const 1
    i32.shl
    i32.const 3136
    i32.add
    local.set 3
    i32.const 0
    local.set 4
    loop  ;; label = @1
      i32.const 0
      local.set 5
      loop  ;; label = @2
        local.get 1
        local.get 1
        i32.load8_u
        local.get 3
        i32.load8_u
        local.tee 6
        i32.const 88
        i32.mul
        i32.const 8
        i32.shr_u
        i32.const -44
        i32.add
        local.tee 0
        i32.sub
        local.tee 7
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 7
        local.get 7
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 1
        i32.const 1
        i32.add
        local.tee 7
        local.get 7
        i32.load8_u
        local.get 0
        i32.sub
        local.tee 7
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 7
        local.get 7
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 1
        i32.const 8
        i32.add
        local.tee 7
        local.get 7
        i32.load8_u
        local.get 0
        i32.sub
        local.tee 7
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 7
        local.get 7
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 1
        i32.const 9
        i32.add
        local.tee 7
        local.get 7
        i32.load8_u
        local.get 0
        i32.sub
        local.tee 0
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 0
        local.get 0
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 2
        local.get 6
        local.get 6
        i32.const 198
        i32.mul
        i32.const 8
        i32.shr_u
        i32.add
        i32.const -227
        i32.add
        local.tee 0
        local.get 2
        i32.load8_u
        i32.add
        local.tee 6
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 6
        local.get 6
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 2
        i32.const 1
        i32.add
        local.tee 6
        local.get 0
        local.get 6
        i32.load8_u
        i32.add
        local.tee 6
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 6
        local.get 6
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 2
        i32.const 8
        i32.add
        local.tee 6
        local.get 0
        local.get 6
        i32.load8_u
        i32.add
        local.tee 6
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 6
        local.get 6
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 2
        i32.const 9
        i32.add
        local.tee 6
        local.get 0
        local.get 6
        i32.load8_u
        i32.add
        local.tee 0
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 0
        local.get 0
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 2
        i32.const 2
        i32.add
        local.set 2
        local.get 1
        i32.const 2
        i32.add
        local.set 1
        local.get 3
        i32.const 2
        i32.add
        local.set 3
        local.get 5
        i32.const 255
        i32.and
        local.set 0
        local.get 5
        i32.const 1
        i32.add
        local.set 5
        local.get 0
        i32.const 3
        i32.lt_u
        br_if 0 (;@2;)
      end
      local.get 2
      i32.const 8
      i32.add
      local.set 2
      local.get 1
      i32.const 8
      i32.add
      local.set 1
      local.get 3
      i32.const 8
      i32.add
      local.set 3
      local.get 4
      i32.const 255
      i32.and
      local.set 0
      local.get 4
      i32.const 1
      i32.add
      local.set 4
      local.get 0
      i32.const 3
      i32.lt_u
      br_if 0 (;@1;)
    end)
  (func (;12;) (type 4) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    local.get 1
    i32.const 2128
    i32.add
    local.set 2
    local.get 1
    i32.const 1872
    i32.add
    local.set 1
    local.get 0
    i32.const 1
    i32.shl
    i32.const 3136
    i32.add
    local.set 3
    i32.const 0
    local.set 4
    loop  ;; label = @1
      i32.const 0
      local.set 5
      loop  ;; label = @2
        local.get 2
        local.get 2
        i32.load8_u
        local.get 3
        i32.load8_u
        local.tee 6
        i32.const 183
        i32.mul
        i32.const 8
        i32.shr_u
        i32.const -91
        i32.add
        local.tee 0
        i32.sub
        local.tee 7
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 7
        local.get 7
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 2
        i32.const 1
        i32.add
        local.tee 7
        local.get 7
        i32.load8_u
        local.get 0
        i32.sub
        local.tee 7
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 7
        local.get 7
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 2
        i32.const 8
        i32.add
        local.tee 7
        local.get 7
        i32.load8_u
        local.get 0
        i32.sub
        local.tee 7
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 7
        local.get 7
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 2
        i32.const 9
        i32.add
        local.tee 7
        local.get 7
        i32.load8_u
        local.get 0
        i32.sub
        local.tee 0
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 0
        local.get 0
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 1
        local.get 6
        local.get 6
        i32.const 103
        i32.mul
        i32.const 8
        i32.shr_u
        i32.add
        i32.const -179
        i32.add
        local.tee 0
        local.get 1
        i32.load8_u
        i32.add
        local.tee 6
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 6
        local.get 6
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 1
        i32.const 1
        i32.add
        local.tee 6
        local.get 0
        local.get 6
        i32.load8_u
        i32.add
        local.tee 6
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 6
        local.get 6
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 1
        i32.const 8
        i32.add
        local.tee 6
        local.get 0
        local.get 6
        i32.load8_u
        i32.add
        local.tee 6
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 6
        local.get 6
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 1
        i32.const 9
        i32.add
        local.tee 6
        local.get 0
        local.get 6
        i32.load8_u
        i32.add
        local.tee 0
        i32.const -1
        i32.xor
        i32.const 15
        i32.shr_u
        local.get 0
        local.get 0
        i32.const 255
        i32.gt_u
        select
        i32.store8
        local.get 2
        i32.const 2
        i32.add
        local.set 2
        local.get 1
        i32.const 2
        i32.add
        local.set 1
        local.get 3
        i32.const 2
        i32.add
        local.set 3
        local.get 5
        i32.const 255
        i32.and
        local.set 0
        local.get 5
        i32.const 1
        i32.add
        local.set 5
        local.get 0
        i32.const 3
        i32.lt_u
        br_if 0 (;@2;)
      end
      local.get 2
      i32.const 8
      i32.add
      local.set 2
      local.get 1
      i32.const 8
      i32.add
      local.set 1
      local.get 3
      i32.const 8
      i32.add
      local.set 3
      local.get 4
      i32.const 255
      i32.and
      local.set 0
      local.get 4
      i32.const 1
      i32.add
      local.set 4
      local.get 0
      i32.const 3
      i32.lt_u
      br_if 0 (;@1;)
    end)
  (func (;13;) (type 0) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 4
    global.set 0
    local.get 0
    i32.const 0
    i32.const 44
    call 28
    local.set 5
    i32.const 0
    local.get 2
    i32.store offset=1836
    i32.const 0
    local.get 1
    i32.store offset=1832
    i32.const 0
    local.get 3
    i32.store8 offset=1840
    i32.const 0
    i32.const 8
    i32.store8 offset=3954
    i32.const 0
    i32.const 0
    i32.store8 offset=1828
    i32.const 0
    i32.const 0
    i32.store16 offset=1842
    i32.const 0
    i32.const 0
    i32.store16 offset=1844
    i32.const 0
    i32.const 0
    i32.store8 offset=1846
    i32.const 0
    i32.const 0
    i32.store16 offset=2644
    i32.const 0
    i32.const 0
    i32.store8 offset=4228
    i32.const 0
    i32.const 0
    i32.store8 offset=4229
    i32.const 0
    i32.const 0
    i32.store8 offset=4230
    i32.const 0
    i32.const 0
    i32.store8 offset=3956
    i32.const 0
    i32.const 0
    i32.store8 offset=4224
    i32.const 0
    i32.const 0
    i32.store8 offset=3955
    i32.const 0
    i32.const 0
    i32.store16 offset=4226
    i32.const 8
    i32.const 0
    call 4
    drop
    i32.const 8
    i32.const 0
    call 4
    drop
    block  ;; label = @1
      i32.const 0
      i32.load8_u offset=1828
      local.tee 0
      br_if 0 (;@1;)
      i32.const 8
      i32.const 0
      call 4
      local.set 2
      i32.const 8
      i32.const 0
      call 4
      local.set 0
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.const 255
            i32.and
            i32.const 255
            i32.ne
            br_if 0 (;@4;)
            local.get 0
            i32.const 255
            i32.and
            i32.const 216
            i32.eq
            br_if 1 (;@3;)
          end
          i32.const 4095
          local.set 1
          block  ;; label = @4
            loop  ;; label = @5
              local.get 0
              i32.const 255
              i32.and
              local.set 2
              i32.const 8
              i32.const 0
              call 4
              local.set 0
              block  ;; label = @6
                local.get 2
                i32.const 255
                i32.ne
                br_if 0 (;@6;)
                i32.const 19
                local.set 2
                local.get 0
                i32.const 255
                i32.and
                i32.const -216
                i32.add
                br_table 2 (;@4;) 4 (;@2;) 0 (;@6;)
              end
              local.get 1
              i32.const -1
              i32.add
              local.tee 1
              i32.const 65535
              i32.and
              br_if 0 (;@5;)
            end
            i32.const 19
            local.set 2
            br 2 (;@2;)
          end
          i32.const 0
          i32.load16_u offset=4226
          i32.const 65280
          i32.lt_u
          br_if 1 (;@2;)
        end
        local.get 4
        i32.const 14
        i32.add
        call 14
        local.tee 2
        br_if 0 (;@2;)
        i32.const 37
        local.set 2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 4
                  i32.load8_u offset=14
                  local.tee 0
                  i32.const -192
                  i32.add
                  br_table 0 (;@7;) 2 (;@5;) 5 (;@2;) 1 (;@6;)
                end
                i32.const 8
                local.set 2
                i32.const 16
                i32.const 0
                call 4
                local.set 0
                block  ;; label = @7
                  i32.const 8
                  i32.const 0
                  call 4
                  i32.const 8
                  i32.eq
                  br_if 0 (;@7;)
                  i32.const 7
                  local.set 2
                  br 5 (;@2;)
                end
                i32.const 0
                i32.const 16
                i32.const 0
                call 4
                local.tee 1
                i32.store16 offset=1844
                local.get 1
                i32.const -16385
                i32.add
                i32.const 65535
                i32.and
                i32.const 49152
                i32.lt_u
                br_if 4 (;@2;)
                i32.const 0
                i32.const 16
                i32.const 0
                call 4
                local.tee 2
                i32.store16 offset=1842
                block  ;; label = @7
                  local.get 2
                  i32.const -16385
                  i32.add
                  i32.const 65535
                  i32.and
                  i32.const 49152
                  i32.ge_u
                  br_if 0 (;@7;)
                  i32.const 9
                  local.set 2
                  br 5 (;@2;)
                end
                i32.const 0
                i32.const 8
                i32.const 0
                call 4
                local.tee 1
                i32.store8 offset=1846
                block  ;; label = @7
                  local.get 1
                  i32.const 255
                  i32.and
                  local.tee 2
                  i32.const 3
                  i32.le_u
                  br_if 0 (;@7;)
                  i32.const 10
                  local.set 2
                  br 5 (;@2;)
                end
                block  ;; label = @7
                  local.get 0
                  local.get 2
                  i32.const 3
                  i32.mul
                  i32.const 8
                  i32.add
                  i32.eq
                  br_if 0 (;@7;)
                  i32.const 11
                  local.set 2
                  br 5 (;@2;)
                end
                i32.const 0
                local.set 2
                local.get 1
                i32.const 255
                i32.and
                i32.eqz
                br_if 2 (;@4;)
                i32.const 0
                local.set 0
                block  ;; label = @7
                  loop  ;; label = @8
                    local.get 0
                    i32.const 4231
                    i32.add
                    i32.const 8
                    i32.const 0
                    call 4
                    i32.store8
                    local.get 0
                    i32.const 4234
                    i32.add
                    i32.const 4
                    i32.const 0
                    call 4
                    i32.store8
                    local.get 0
                    i32.const 4237
                    i32.add
                    i32.const 4
                    i32.const 0
                    call 4
                    i32.store8
                    local.get 0
                    i32.const 2658
                    i32.add
                    i32.const 8
                    i32.const 0
                    call 4
                    local.tee 2
                    i32.store8
                    local.get 2
                    i32.const 254
                    i32.and
                    br_if 1 (;@7;)
                    local.get 0
                    i32.const 1
                    i32.add
                    local.tee 0
                    i32.const 0
                    i32.load8_u offset=1846
                    local.tee 2
                    i32.ge_u
                    br_if 4 (;@4;)
                    br 0 (;@8;)
                  end
                end
                i32.const 36
                local.set 2
                br 4 (;@2;)
              end
              local.get 0
              i32.const 201
              i32.eq
              br_if 2 (;@3;)
            end
            i32.const 20
            local.set 2
            br 2 (;@2;)
          end
          i32.const 0
          i32.load8_u offset=1828
          local.tee 0
          br_if 2 (;@1;)
          i32.const 26
          local.set 0
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 2
                  i32.const -1
                  i32.add
                  br_table 0 (;@7;) 6 (;@1;) 1 (;@6;) 6 (;@1;)
                end
                i32.const 27
                local.set 0
                i32.const 0
                i32.load8_u offset=4234
                i32.const 1
                i32.ne
                br_if 5 (;@1;)
                i32.const 0
                i32.load8_u offset=4237
                i32.const 255
                i32.and
                i32.const 1
                i32.ne
                br_if 5 (;@1;)
                i32.const 0
                i32.const 1
                i32.store8 offset=2648
                i32.const 0
                i32.const 0
                i32.store offset=1848
                i32.const 0
                i32.const 0
                i32.store8 offset=2652
                i32.const 7
                local.set 0
                i32.const 3
                local.set 2
                br 1 (;@5;)
              end
              i32.const 27
              local.set 0
              i32.const 0
              i32.load8_u offset=4235
              i32.const 1
              i32.ne
              br_if 4 (;@1;)
              i32.const 0
              i32.load8_u offset=4238
              i32.const 255
              i32.and
              i32.const 1
              i32.ne
              br_if 4 (;@1;)
              i32.const 0
              i32.load8_u offset=4236
              i32.const 255
              i32.and
              i32.const 1
              i32.ne
              br_if 4 (;@1;)
              i32.const 0
              i32.load8_u offset=4239
              i32.const 255
              i32.and
              i32.const 1
              i32.ne
              br_if 4 (;@1;)
              i32.const 0
              i32.load8_u offset=4237
              local.set 2
              block  ;; label = @6
                i32.const 0
                i32.load8_u offset=4234
                local.tee 3
                i32.const 1
                i32.ne
                local.tee 1
                br_if 0 (;@6;)
                local.get 2
                i32.const 255
                i32.and
                i32.const 1
                i32.ne
                br_if 0 (;@6;)
                i32.const 3
                local.set 2
                i32.const 0
                i32.const 3
                i32.store8 offset=2648
                i32.const 0
                i32.const 1
                i32.store offset=1848
                i32.const 0
                i32.const 2
                i32.store8 offset=2654
                i32.const 0
                i32.const 256
                i32.store16 offset=2652
                i32.const 7
                local.set 0
                br 1 (;@5;)
              end
              block  ;; label = @6
                local.get 1
                br_if 0 (;@6;)
                local.get 2
                i32.const 255
                i32.and
                i32.const 2
                i32.ne
                br_if 0 (;@6;)
                i32.const 4
                local.set 2
                i32.const 0
                i32.const 4
                i32.store8 offset=2648
                i32.const 3
                local.set 6
                i32.const 0
                i32.const 3
                i32.store offset=1848
                i32.const 0
                i32.const 33619968
                i32.store offset=2652
                i32.const 7
                local.set 7
                i32.const 15
                local.set 0
                i32.const 16
                local.set 3
                i32.const 8
                local.set 1
                br 2 (;@4;)
              end
              block  ;; label = @6
                local.get 3
                i32.const 2
                i32.ne
                local.tee 1
                br_if 0 (;@6;)
                local.get 2
                i32.const 255
                i32.and
                i32.const 1
                i32.ne
                br_if 0 (;@6;)
                i32.const 4
                local.set 6
                i32.const 0
                i32.const 4
                i32.store8 offset=2648
                i32.const 0
                i32.const 2
                i32.store offset=1848
                i32.const 0
                i32.const 33619968
                i32.store offset=2652
                i32.const 15
                local.set 7
                i32.const 7
                local.set 0
                i32.const 3
                local.set 2
                i32.const 8
                local.set 3
                i32.const 16
                local.set 1
                br 2 (;@4;)
              end
              local.get 1
              br_if 4 (;@1;)
              local.get 2
              i32.const 255
              i32.and
              i32.const 2
              i32.ne
              br_if 4 (;@1;)
              i32.const 0
              i32.const 6
              i32.store8 offset=2648
              i32.const 4
              local.set 2
              i32.const 0
              i32.const 4
              i32.store offset=1848
              i32.const 0
              i32.const 513
              i32.store16 offset=2656
              i32.const 0
              i32.const 0
              i32.store offset=2652
              i32.const 15
              local.set 0
              i32.const 16
              local.set 1
              i32.const 16
              local.set 3
              i32.const 4
              local.set 6
              i32.const 15
              local.set 7
              br 1 (;@4;)
            end
            i32.const 8
            local.set 1
            i32.const 8
            local.set 3
            i32.const 3
            local.set 6
            i32.const 7
            local.set 7
          end
          i32.const 0
          local.get 3
          i32.store8 offset=1857
          i32.const 0
          local.get 1
          i32.store8 offset=1856
          i32.const 0
          local.get 7
          i32.const 0
          i32.load16_u offset=1842
          i32.add
          local.get 6
          i32.shr_u
          local.tee 1
          i32.store16 offset=1852
          i32.const 0
          local.get 0
          i32.const 0
          i32.load16_u offset=1844
          i32.add
          local.get 2
          i32.shr_u
          local.tee 0
          i32.store16 offset=1854
          i32.const 0
          local.get 0
          local.get 1
          i32.mul
          i32.store16 offset=1830
          block  ;; label = @4
            local.get 4
            i32.const 15
            i32.add
            call 14
            local.tee 2
            br_if 0 (;@4;)
            i32.const 18
            local.set 2
            block  ;; label = @5
              block  ;; label = @6
                local.get 4
                i32.load8_u offset=15
                i32.const -217
                i32.add
                br_table 1 (;@5;) 0 (;@6;) 2 (;@4;)
              end
              i32.const 16
              i32.const 0
              call 4
              local.set 0
              i32.const 0
              i32.const 8
              i32.const 0
              call 4
              local.tee 1
              i32.store8 offset=4228
              i32.const 14
              local.set 2
              local.get 1
              i32.const 255
              i32.and
              local.tee 3
              i32.const 1
              i32.shl
              local.get 0
              i32.const -6
              i32.add
              i32.const 65535
              i32.and
              i32.ne
              br_if 1 (;@4;)
              local.get 3
              i32.const -4
              i32.add
              i32.const -3
              i32.lt_u
              br_if 1 (;@4;)
              local.get 0
              i32.const -3
              i32.add
              local.set 6
              block  ;; label = @6
                local.get 1
                i32.const 255
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                i32.const 15
                local.set 2
                i32.const 0
                local.set 7
                loop  ;; label = @7
                  i32.const 8
                  i32.const 0
                  call 4
                  local.set 1
                  i32.const 8
                  i32.const 0
                  call 4
                  local.set 8
                  i32.const 0
                  i32.load8_u offset=1846
                  local.tee 3
                  i32.eqz
                  br_if 3 (;@4;)
                  local.get 6
                  i32.const -2
                  i32.add
                  local.set 6
                  i32.const 0
                  local.set 0
                  local.get 1
                  i32.const 255
                  i32.and
                  local.set 1
                  block  ;; label = @8
                    loop  ;; label = @9
                      local.get 0
                      i32.const 4231
                      i32.add
                      i32.load8_u
                      local.get 1
                      i32.eq
                      br_if 1 (;@8;)
                      local.get 3
                      local.get 0
                      i32.const 1
                      i32.add
                      local.tee 0
                      i32.ne
                      br_if 0 (;@9;)
                      br 5 (;@4;)
                    end
                  end
                  local.get 7
                  i32.const 4240
                  i32.add
                  local.get 0
                  i32.store8
                  local.get 0
                  i32.const 3264
                  i32.add
                  local.get 8
                  i32.const 15
                  i32.and
                  i32.store8
                  local.get 0
                  i32.const 2661
                  i32.add
                  local.get 8
                  i32.const 240
                  i32.and
                  i32.const 4
                  i32.shr_u
                  i32.store8
                  local.get 7
                  i32.const 1
                  i32.add
                  local.tee 7
                  i32.const 0
                  i32.load8_u offset=4228
                  i32.lt_u
                  br_if 0 (;@7;)
                end
              end
              i32.const 0
              i32.const 8
              i32.const 0
              call 4
              i32.store8 offset=2640
              i32.const 0
              i32.const 8
              i32.const 0
              call 4
              i32.store8 offset=2641
              i32.const 0
              i32.const 4
              i32.const 0
              call 4
              i32.store8 offset=2642
              i32.const 0
              i32.const 4
              i32.const 0
              call 4
              i32.store8 offset=2643
              block  ;; label = @6
                local.get 6
                i32.const -3
                i32.add
                local.tee 0
                i32.const 65535
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                loop  ;; label = @7
                  i32.const 8
                  i32.const 0
                  call 4
                  drop
                  local.get 0
                  i32.const -1
                  i32.add
                  local.tee 0
                  i32.const 65535
                  i32.and
                  br_if 0 (;@7;)
                end
              end
              block  ;; label = @6
                i32.const 0
                i32.load8_u offset=4228
                local.tee 6
                i32.eqz
                br_if 0 (;@6;)
                i32.const 0
                local.set 0
                i32.const 0
                i32.load8_u offset=4229
                local.set 1
                loop  ;; label = @7
                  i32.const 24
                  local.set 2
                  local.get 1
                  local.get 0
                  i32.const 4240
                  i32.add
                  i32.load8_u
                  local.tee 3
                  i32.const 2661
                  i32.add
                  i32.load8_u
                  i32.shr_u
                  i32.const 1
                  i32.and
                  i32.eqz
                  br_if 3 (;@4;)
                  local.get 1
                  local.get 3
                  i32.const 3264
                  i32.add
                  i32.load8_u
                  i32.const 2
                  i32.add
                  i32.const 255
                  i32.and
                  i32.shr_u
                  i32.const 1
                  i32.and
                  i32.eqz
                  br_if 3 (;@4;)
                  local.get 6
                  local.get 0
                  i32.const 1
                  i32.add
                  local.tee 0
                  i32.ne
                  br_if 0 (;@7;)
                end
                i32.const 0
                local.set 0
                i32.const 0
                i32.load8_u offset=4230
                local.set 2
                block  ;; label = @7
                  loop  ;; label = @8
                    i32.const 2
                    i32.const 1
                    local.get 0
                    i32.const 4240
                    i32.add
                    i32.load8_u
                    i32.const 2658
                    i32.add
                    i32.load8_u
                    select
                    local.get 2
                    i32.and
                    i32.eqz
                    br_if 1 (;@7;)
                    local.get 6
                    local.get 0
                    i32.const 1
                    i32.add
                    local.tee 0
                    i32.eq
                    br_if 2 (;@6;)
                    br 0 (;@8;)
                  end
                end
                i32.const 23
                local.set 2
                br 2 (;@4;)
              end
              i32.const 0
              i32.const 0
              i32.store16 offset=3124
              i32.const 0
              i32.const 0
              i32.store offset=3120 align=2
              block  ;; label = @6
                i32.const 0
                i32.load16_u offset=2644
                local.tee 0
                i32.eqz
                br_if 0 (;@6;)
                i32.const 0
                local.get 0
                i32.store16 offset=2646
                i32.const 0
                i32.const 0
                i32.store16 offset=3952
              end
              call 15
              i32.const 0
              i32.load8_u offset=1828
              local.tee 0
              br_if 4 (;@1;)
              i32.const 0
              local.set 0
              local.get 5
              i32.const 0
              i32.load16_u offset=1842
              i32.store
              local.get 5
              i32.const 2384
              i32.store offset=40
              local.get 5
              i32.const 2128
              i32.store offset=36
              local.get 5
              i32.const 1872
              i32.store offset=32
              local.get 5
              i32.const 0
              i32.load16_u offset=1844
              i32.store offset=4
              local.get 5
              i32.const 0
              i32.load8_u offset=1846
              i32.store offset=8
              local.get 5
              i32.const 0
              i32.load16_u offset=1852
              i32.store offset=12
              local.get 5
              i32.const 0
              i32.load16_u offset=1854
              i32.store offset=16
              local.get 5
              i32.const 0
              i32.load8_u offset=1856
              i32.store offset=24
              local.get 5
              i32.const 0
              i32.load8_u offset=1857
              i32.store offset=28
              local.get 5
              i32.const 0
              i32.load offset=1848
              i32.store offset=20
              br 4 (;@1;)
            end
            i32.const 18
            local.set 2
          end
          i32.const 0
          i32.load8_u offset=1828
          local.tee 0
          local.get 2
          local.get 0
          select
          local.set 0
          br 2 (;@1;)
        end
        i32.const 17
        local.set 2
      end
      i32.const 0
      i32.load8_u offset=1828
      local.tee 0
      local.get 2
      local.get 0
      select
      local.set 0
    end
    local.get 4
    i32.const 16
    i32.add
    global.set 0
    local.get 0)
  (func (;14;) (type 5) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    loop (result i32)  ;; label = @1
      i32.const 8
      i32.const 0
      call 4
      i32.const 255
      i32.and
      i32.const 255
      i32.ne
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        loop  ;; label = @11
                          i32.const 8
                          i32.const 0
                          call 4
                          local.tee 2
                          i32.const 255
                          i32.and
                          local.tee 3
                          i32.const -192
                          i32.add
                          br_table 2 (;@9;) 2 (;@9;) 2 (;@9;) 2 (;@9;) 3 (;@8;) 2 (;@9;) 2 (;@9;) 2 (;@9;) 4 (;@7;) 2 (;@9;) 2 (;@9;) 2 (;@9;) 5 (;@6;) 2 (;@9;) 2 (;@9;) 2 (;@9;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 4 (;@7;) 2 (;@9;) 2 (;@9;) 2 (;@9;) 9 (;@2;) 7 (;@4;) 8 (;@3;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 7 (;@4;) 0 (;@11;) 1 (;@10;)
                        end
                      end
                      local.get 3
                      br_table 8 (;@1;) 2 (;@7;) 5 (;@4;)
                    end
                    local.get 0
                    local.get 2
                    i32.store8
                    i32.const 0
                    local.set 2
                    br 3 (;@5;)
                  end
                  i32.const 16
                  i32.const 0
                  call 4
                  local.tee 2
                  i32.const 3
                  i32.lt_u
                  br_if 6 (;@1;)
                  local.get 2
                  i32.const -2
                  i32.add
                  local.set 4
                  loop  ;; label = @8
                    i32.const 8
                    i32.const 0
                    call 4
                    local.tee 3
                    i32.const 14
                    i32.and
                    br_if 7 (;@1;)
                    local.get 3
                    i32.const 240
                    i32.and
                    i32.const 16
                    i32.gt_u
                    br_if 7 (;@1;)
                    i32.const 0
                    local.set 2
                    i32.const 0
                    i32.const 0
                    i32.load8_u offset=4229
                    i32.const 1
                    local.get 3
                    i32.const 3
                    i32.shr_u
                    i32.const 2
                    i32.and
                    local.get 3
                    i32.const 1
                    i32.and
                    i32.or
                    local.tee 5
                    i32.shl
                    i32.or
                    i32.store8 offset=4229
                    local.get 5
                    i32.const 2
                    i32.shl
                    local.tee 3
                    i32.const 1228
                    i32.add
                    i32.load
                    local.set 6
                    local.get 3
                    i32.const 1212
                    i32.add
                    i32.load
                    local.set 7
                    i32.const 0
                    local.set 3
                    loop  ;; label = @9
                      local.get 1
                      local.get 2
                      i32.add
                      i32.const 8
                      i32.const 0
                      call 4
                      local.tee 8
                      i32.store8
                      local.get 8
                      i32.const 255
                      i32.and
                      local.get 3
                      i32.add
                      local.set 3
                      local.get 2
                      i32.const 1
                      i32.add
                      local.tee 2
                      i32.const 16
                      i32.ne
                      br_if 0 (;@9;)
                    end
                    local.get 3
                    i32.const 65535
                    i32.and
                    local.tee 8
                    i32.const 12
                    i32.const 255
                    local.get 5
                    i32.const 2
                    i32.lt_u
                    select
                    i32.gt_u
                    br_if 7 (;@1;)
                    block  ;; label = @9
                      local.get 8
                      i32.eqz
                      br_if 0 (;@9;)
                      i32.const 0
                      local.set 2
                      loop  ;; label = @10
                        local.get 6
                        local.get 2
                        i32.const 255
                        i32.and
                        i32.add
                        i32.const 8
                        i32.const 0
                        call 4
                        i32.store8
                        local.get 8
                        local.get 2
                        i32.const 1
                        i32.add
                        local.tee 2
                        i32.const 255
                        i32.and
                        i32.gt_u
                        br_if 0 (;@10;)
                      end
                    end
                    local.get 4
                    i32.const 65535
                    i32.and
                    local.get 3
                    i32.const 17
                    i32.add
                    local.tee 9
                    i32.const 65535
                    i32.and
                    i32.lt_u
                    br_if 7 (;@1;)
                    i32.const 0
                    local.set 8
                    i32.const 0
                    local.set 2
                    i32.const 0
                    local.set 10
                    i32.const 0
                    local.set 3
                    loop  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 1
                          local.get 2
                          i32.add
                          i32.load8_u
                          local.tee 6
                          br_if 0 (;@11;)
                          i32.const 65535
                          local.set 5
                          i32.const 0
                          local.set 11
                          i32.const 0
                          local.set 12
                          br 1 (;@10;)
                        end
                        local.get 3
                        local.get 6
                        i32.add
                        local.tee 13
                        i32.const -1
                        i32.add
                        local.set 5
                        local.get 3
                        local.set 11
                        local.get 10
                        local.set 12
                        local.get 6
                        local.get 10
                        i32.add
                        local.set 10
                        local.get 13
                        local.set 3
                      end
                      local.get 7
                      local.get 8
                      i32.add
                      local.tee 6
                      local.get 11
                      i32.store16
                      local.get 6
                      i32.const 32
                      i32.add
                      local.get 5
                      i32.store16
                      local.get 7
                      local.get 2
                      i32.add
                      i32.const 64
                      i32.add
                      local.get 12
                      i32.store8
                      local.get 8
                      i32.const 2
                      i32.add
                      local.set 8
                      local.get 3
                      i32.const 1
                      i32.shl
                      local.set 3
                      local.get 2
                      i32.const 1
                      i32.add
                      local.tee 2
                      i32.const 16
                      i32.ne
                      br_if 0 (;@9;)
                    end
                    local.get 4
                    local.get 9
                    i32.sub
                    local.tee 4
                    i32.const 65535
                    i32.and
                    i32.eqz
                    br_if 7 (;@1;)
                    br 0 (;@8;)
                  end
                end
                i32.const 18
                local.set 2
                br 1 (;@5;)
              end
              i32.const 17
              local.set 2
            end
            local.get 1
            i32.const 16
            i32.add
            global.set 0
            local.get 2
            return
          end
          i32.const 16
          i32.const 0
          call 4
          local.tee 2
          i32.const 3
          i32.lt_u
          br_if 2 (;@1;)
          local.get 2
          i32.const -2
          i32.add
          local.set 2
          loop  ;; label = @4
            i32.const 8
            i32.const 0
            call 4
            drop
            local.get 2
            i32.const -1
            i32.add
            local.tee 2
            i32.const 65535
            i32.and
            br_if 0 (;@4;)
            br 3 (;@1;)
          end
        end
        i32.const 16
        i32.const 0
        call 4
        i32.const 4
        i32.ne
        br_if 1 (;@1;)
        i32.const 0
        i32.const 16
        i32.const 0
        call 4
        i32.store16 offset=2644
        br 1 (;@1;)
      end
      i32.const 16
      i32.const 0
      call 4
      local.tee 2
      i32.const 3
      i32.lt_u
      br_if 0 (;@1;)
      local.get 2
      i32.const -2
      i32.add
      local.set 5
      loop (result i32)  ;; label = @2
        i32.const 8
        i32.const 0
        call 4
        local.tee 6
        i32.const 15
        i32.and
        local.tee 2
        i32.const 1
        i32.gt_u
        br_if 1 (;@1;)
        i32.const 0
        local.set 3
        i32.const 0
        i32.const 0
        i32.load8_u offset=4230
        i32.const 2
        i32.const 1
        local.get 2
        select
        i32.or
        i32.store8 offset=4230
        i32.const 2672
        i32.const 2800
        local.get 2
        select
        local.set 2
        loop  ;; label = @3
          i32.const 8
          i32.const 0
          call 4
          local.set 8
          block  ;; label = @4
            local.get 6
            i32.const 255
            i32.and
            i32.const 16
            i32.lt_u
            local.tee 7
            br_if 0 (;@4;)
            i32.const 8
            i32.const 0
            call 4
            local.get 8
            i32.const 8
            i32.shl
            i32.add
            local.set 8
          end
          local.get 2
          local.get 3
          i32.add
          local.get 8
          i32.store16
          local.get 3
          i32.const 2
          i32.add
          local.tee 3
          i32.const 128
          i32.ne
          br_if 0 (;@3;)
        end
        i32.const -64
        local.set 3
        loop  ;; label = @3
          local.get 2
          local.get 3
          i32.const 1088
          i32.add
          i32.load8_u
          local.get 2
          i32.load16_s
          i32.mul
          i32.const 4
          i32.add
          i32.const 3
          i32.shr_u
          i32.store16
          local.get 2
          i32.const 2
          i32.add
          local.set 2
          local.get 3
          i32.const 1
          i32.add
          local.tee 3
          br_if 0 (;@3;)
        end
        i32.const 65
        i32.const 129
        local.get 7
        select
        local.tee 2
        local.get 5
        i32.const 65535
        i32.and
        i32.gt_u
        br_if 1 (;@1;)
        local.get 5
        local.get 2
        i32.sub
        local.tee 5
        i32.const 65535
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        br 0 (;@2;)
      end
    end)
  (func (;15;) (type 1)
    (local i32 i32 i32)
    i32.const 0
    i32.load8_u offset=3955
    local.set 0
    i32.const 0
    i32.load8_u offset=4224
    local.set 1
    i32.const 0
    i32.load16_u offset=4226
    local.set 2
    block  ;; label = @1
      i32.const 0
      i32.load8_u offset=3954
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const -1
      i32.add
      local.tee 1
      i32.const 255
      i32.and
      i32.const 3968
      i32.add
      local.get 2
      i32.store8
      local.get 0
      i32.const 1
      i32.add
      local.set 0
    end
    i32.const 0
    i32.const 8
    i32.store8 offset=3954
    i32.const 0
    local.get 1
    i32.const -1
    i32.add
    local.tee 1
    i32.store8 offset=4224
    i32.const 0
    local.get 0
    i32.const 1
    i32.add
    i32.store8 offset=3955
    local.get 1
    i32.const 255
    i32.and
    i32.const 3968
    i32.add
    local.get 2
    i32.const 8
    i32.shr_u
    i32.store8
    i32.const 8
    i32.const 1
    call 4
    drop
    i32.const 8
    i32.const 1
    call 4
    drop)
  (func (;16;) (type 2) (result i32)
    (local i32 i32 i32)
    i32.const 0
    i32.load16_u offset=4226
    local.tee 0
    local.set 1
    block  ;; label = @1
      i32.const 0
      i32.load8_u offset=3954
      local.tee 2
      br_if 0 (;@1;)
      i32.const 1
      call 17
      local.set 1
      i32.const 0
      i32.load8_u offset=3954
      i32.const 8
      i32.add
      local.set 2
      local.get 1
      i32.const 0
      i32.load16_u offset=4226
      i32.or
      local.set 1
    end
    i32.const 0
    local.get 1
    i32.const 1
    i32.shl
    i32.store16 offset=4226
    i32.const 0
    local.get 2
    i32.const -1
    i32.add
    i32.store8 offset=3954
    local.get 0
    i32.const 32768
    i32.and
    i32.const 15
    i32.shr_u)
  (func (;17;) (type 5) (param i32) (result i32)
    (local i32 i32 i32)
    call 3
    local.set 1
    block  ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const 255
      i32.ne
      br_if 0 (;@1;)
      call 3
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      i32.const 0
      i32.const 0
      i32.load8_u offset=3955
      i32.const 2
      i32.add
      i32.store8 offset=3955
      i32.const 0
      i32.const 0
      i32.load8_u offset=4224
      local.tee 2
      i32.const -2
      i32.add
      local.tee 3
      i32.store8 offset=4224
      local.get 2
      i32.const -1
      i32.add
      i32.const 255
      i32.and
      i32.const 3968
      i32.add
      local.get 0
      i32.store8
      local.get 3
      i32.const 255
      i32.and
      i32.const 3968
      i32.add
      i32.const 255
      i32.store8
    end
    local.get 1)
  (func (;18;) (type 0) (param i32 i32 i32 i32) (result i32)
    (local i32)
    local.get 0
    i32.const 0
    i32.load offset=4244
    local.tee 4
    i32.const 1248
    i32.add
    i32.const 570
    local.get 4
    i32.sub
    local.tee 4
    local.get 1
    local.get 4
    local.get 1
    i32.lt_u
    select
    local.tee 1
    call 27
    drop
    local.get 2
    local.get 1
    i32.store8
    i32.const 0
    i32.const 0
    i32.load offset=4244
    local.get 1
    i32.add
    i32.store offset=4244
    i32.const 0)
  (func (;19;) (type 1))
  (func (;20;) (type 4) (param i32 i32)
    (local i32 i32)
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
            i32.const 0
            i32.const 0
            i32.store offset=4244
            i32.const 4248
            i32.const 1
            i32.const 0
            i32.const 0
            call 13
            drop
            loop  ;; label = @5
              call 2
              i32.const 1
              i32.ne
              br_if 0 (;@5;)
            end
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
  (func (;21;) (type 2) (result i32)
    i32.const 5
    i32.const 1
    call 20
    i32.const 0)
  (func (;22;) (type 3) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    call 23
    call 19
    call 25
    local.get 2
    call 21
    i32.store offset=12
    call 26
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    i32.const 0)
  (func (;23;) (type 1))
  (func (;24;) (type 2) (result i32)
    i32.const 0
    i32.const 0
    call 22)
  (func (;25;) (type 1))
  (func (;26;) (type 1))
  (func (;27;) (type 6) (param i32 i32 i32) (result i32)
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
  (func (;28;) (type 6) (param i32 i32 i32) (result i32)
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
  (table (;0;) 2 2 funcref)
  (memory (;0;) 1)
  (global (;0;) (mut i32) (i32.const 20304))
  (global (;1;) i32 (i32.const 0))
  (global (;2;) i32 (i32.const 4292))
  (global (;3;) i32 (i32.const 20304))
  (export "memory" (memory 0))
  (export "_initialize" (func 1))
  (export "_run" (func 24))
  (export "__data_end" (global 2))
  (export "__heap_base" (global 3))
  (elem (;0;) (i32.const 1) func 18)
  (data (;0;) (i32.const 1024) "\80\b2\b2\a7\f6\a7\97\e8\e8\97\80\d1\db\d1\80e\b2\c5\c5\b2eE\8b\a7\b1\a7\8bE#`\83\97\97\83`#1[v\80v[1.QeeQ.*EOE*#66#\1c%\1c\13\13\0a\00\01\08\10\09\02\03\0a\11\18 \19\12\0b\04\05\0c\13\1a!(0)\22\1b\14\0d\06\07\0e\15\1c#*1892+$\1d\16\0f\17\1e%,3:;4-&\1f'.5<=6/7>?\01\00\02\00\04\00\08\00\10\00 \00@\00\80\00\00\01\00\02\00\04\00\08\00\10\00 \00@\ff\ff\fd\ff\f9\ff\f1\ff\e1\ff\c1\ff\81\ff\01\ff\01\fe\01\fc\01\f8\01\f0\01\e0\01\c0\01\80\c0\0b\00\00p\0b\00\00\14\0d\00\00\c4\0c\00\00 \0c\00\00\10\0c\00\00p\0e\00\00p\0d\00\00\00\00\00\00\ff\d8\ff\e0\00\10JFIF\00\01\01\01\00H\00H\00\00\ff\db\00C\00P7<F<2PFAFZUP_x\c8\82xnnx\f5\af\b9\91\c8\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\db\00C\01UZZxix\eb\82\82\eb\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\c0\00\11\08\00@\003\03\01\11\00\02\11\01\03\11\01\ff\c4\00\18\00\00\03\01\01\00\00\00\00\00\00\00\00\00\00\00\00\01\02\03\00\04\ff\c4\00&\10\00\02\02\01\03\03\04\03\01\00\00\00\00\00\00\00\01\02\00\11\03\12!1Aaq\04\22S\91\13#Qb\ff\c4\00\15\01\01\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\ff\c4\00\14\11\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\ff\da\00\0c\03\01\00\02\11\03\11\00?\00\c7\88\0b(Mf\e1\02\cfX\05E\88\00\a9\104\0e\afohT3d\03a\02jI\dbI\84YP\91P\a0=\a4\80|\88\18\d5H\05\09C\eb\ff\00+\f5\02x\d4\1c\acH\10\8e\b5U\a8\0c\a5/\dbR*\19S\f6\82\04\a1\0eE\f8\84\0d\ad~! \d00\04\93\5c\99C\e2\c6W \d4\d7} :\e1@\f6I\f1 \a3\8b\17\c5\09G&5\05\b5\1e u\02\95\c0\fa\81\cfR\0cl\0d\a0%\bd\dd\d1\f3*(C\b0\04\0e:\dc)\b2\13\f8j\11%S\22\a9}\e0.\a8\06\c1\da\a0)!M05*(\b9\17\85\06\15P\97\8c\d8\ef \e5\01\f9\d3R\a1\b5\0e\b0\14\9092(\e2`\d9\17\cc\a2\d9\b1\025X\1e`\0cX\95\06\a6 \f8\90;gA\b0\b3\02O\9cU@\8e\b5\fe\09Q\12\d7\01\fd;\11\99{\98U}K\bbd\00\ec\bd\04\05Eb6\b2;B\1b*\1c@t\b8\11\06\00\b8\1f\ff\d9"))

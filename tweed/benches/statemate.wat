(module
  (type (;0;) (func))
  (type (;1;) (func (param i32 i32)))
  (type (;2;) (func (result i32)))
  (type (;3;) (func (param i32 i32) (result i32)))
  (type (;4;) (func (param i32 i32 i32) (result i32)))
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
  (func (;2;) (type 0)
    (local i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          i32.const 0
          i32.load8_u offset=1044
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          i32.const 0
          i32.load offset=1104
          local.tee 0
          i32.store offset=1108
          br 1 (;@2;)
        end
        i32.const 0
        i32.load offset=1104
        local.set 0
        i32.const 0
        i32.load8_u offset=1046
        i32.eqz
        br_if 1 (;@1;)
      end
      i32.const 0
      local.get 0
      i32.store offset=1112
    end
    block  ;; label = @1
      i32.const 0
      i32.load offset=1116
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.eq
      br_if 0 (;@1;)
      i32.const 0
      i32.const 0
      i32.store offset=1116
      i32.const 0
      i32.const 0
      i32.store8 offset=1120
    end
    block  ;; label = @1
      i32.const 0
      i32.load offset=1124
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.eq
      br_if 0 (;@1;)
      i32.const 0
      i32.const 0
      i32.store offset=1124
      i32.const 0
      i32.const 0
      i32.store8 offset=1128
    end
    block  ;; label = @1
      i32.const 0
      i32.load offset=1132
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.eq
      br_if 0 (;@1;)
      i32.const 0
      i32.const 0
      i32.store offset=1132
      i32.const 0
      i32.const 0
      i32.store8 offset=1128
    end
    block  ;; label = @1
      i32.const 0
      i32.load offset=1136
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.eq
      br_if 0 (;@1;)
      i32.const 0
      i32.const 0
      i32.store offset=1136
    end
    block  ;; label = @1
      i32.const 0
      i32.load offset=1140
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.eq
      br_if 0 (;@1;)
      i32.const 0
      i32.const 0
      i32.store offset=1140
    end
    block  ;; label = @1
      block  ;; label = @2
        i32.const 0
        i32.load8_u offset=1040
        br_if 0 (;@2;)
        i32.const 0
        i32.load offset=1144
        i32.const 0
        i32.load offset=1148
        i32.eq
        br_if 1 (;@1;)
      end
      i32.const 0
      local.get 0
      i32.store offset=1152
    end)
  (func (;3;) (type 0)
    (local i32 i32 i32 i32)
    block  ;; label = @1
      i32.const 0
      i32.load8_u offset=1050
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              i32.const 0
              i32.load8_u offset=1159
              i32.const -1
              i32.add
              br_table 0 (;@5;) 1 (;@4;) 2 (;@3;) 3 (;@2;)
            end
            block  ;; label = @5
              i32.const 0
              i32.load8_u offset=1172
              local.tee 0
              i32.const 0
              i32.load8_u offset=1173
              local.tee 1
              i32.or
              i32.const 255
              i32.and
              br_if 0 (;@5;)
              i32.const 0
              i32.const 3
              i32.store8 offset=1159
              i32.const 0
              i32.const 0
              i32.store8 offset=1175
              i32.const 0
              i32.const 0
              i32.store8 offset=1174
              i32.const 0
              i32.const 0
              i32.store8 offset=1176
              i32.const 0
              i32.const 0
              i32.store8 offset=1157
              return
            end
            block  ;; label = @5
              i32.const 0
              i32.load8_u offset=1157
              i32.const 1
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load8_u offset=1177
              local.set 2
              block  ;; label = @6
                local.get 0
                i32.const 255
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                local.get 2
                i32.const 255
                i32.and
                br_if 0 (;@6;)
                i32.const 0
                i32.const 1
                i32.store8 offset=1176
                i32.const 0
                i32.const 1
                i32.store8 offset=1157
                i32.const 0
                i32.const 0
                i32.store8 offset=1174
                return
              end
              i32.const 0
              i32.load8_u offset=1178
              local.set 3
              block  ;; label = @6
                local.get 1
                i32.const 255
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                local.get 3
                i32.const 255
                i32.and
                br_if 0 (;@6;)
                i32.const 0
                i32.const 1
                i32.store8 offset=1175
                i32.const 0
                i32.const 1
                i32.store8 offset=1157
                i32.const 0
                i32.const 0
                i32.store8 offset=1174
                return
              end
              block  ;; label = @6
                local.get 0
                i32.const 255
                i32.and
                br_if 0 (;@6;)
                local.get 2
                i32.const 255
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                i32.const 0
                i32.const 1
                i32.store8 offset=1157
                i32.const 0
                i32.const 0
                i32.store8 offset=1176
                i32.const 0
                i32.const 0
                i32.store8 offset=1174
                return
              end
              local.get 1
              i32.const 255
              i32.and
              br_if 4 (;@1;)
              local.get 3
              i32.const 255
              i32.and
              i32.eqz
              br_if 4 (;@1;)
              i32.const 0
              i32.const 1
              i32.store8 offset=1157
              i32.const 0
              i32.const 0
              i32.store8 offset=1175
              i32.const 0
              i32.const 0
              i32.store8 offset=1174
              return
            end
            i32.const 0
            i32.const 0
            i32.store8 offset=1174
            return
          end
          block  ;; label = @4
            i32.const 0
            i32.load8_u offset=1179
            local.tee 0
            i32.const 0
            i32.load8_u offset=1180
            local.tee 1
            i32.or
            i32.const 255
            i32.and
            br_if 0 (;@4;)
            i32.const 0
            i32.const 3
            i32.store8 offset=1159
            i32.const 0
            i32.const 0
            i32.store8 offset=1175
            i32.const 0
            i32.const 0
            i32.store8 offset=1174
            i32.const 0
            i32.const 0
            i32.store8 offset=1176
            i32.const 0
            i32.const 0
            i32.store8 offset=1158
            return
          end
          block  ;; label = @4
            i32.const 0
            i32.load8_u offset=1158
            i32.const 1
            i32.ne
            br_if 0 (;@4;)
            i32.const 0
            i32.load8_u offset=1181
            local.set 2
            block  ;; label = @5
              local.get 0
              i32.const 255
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              local.get 2
              i32.const 255
              i32.and
              br_if 0 (;@5;)
              i32.const 0
              i32.const 1
              i32.store8 offset=1176
              i32.const 0
              i32.const 1
              i32.store8 offset=1158
              i32.const 0
              i32.const 0
              i32.store8 offset=1174
              return
            end
            i32.const 0
            i32.load8_u offset=1182
            local.set 3
            block  ;; label = @5
              local.get 1
              i32.const 255
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              local.get 3
              i32.const 255
              i32.and
              br_if 0 (;@5;)
              i32.const 0
              i32.const 1
              i32.store8 offset=1175
              i32.const 0
              i32.const 1
              i32.store8 offset=1158
              i32.const 0
              i32.const 0
              i32.store8 offset=1174
              return
            end
            block  ;; label = @5
              local.get 0
              i32.const 255
              i32.and
              br_if 0 (;@5;)
              local.get 2
              i32.const 255
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              i32.const 0
              i32.const 1
              i32.store8 offset=1158
              i32.const 0
              i32.const 0
              i32.store8 offset=1176
              i32.const 0
              i32.const 0
              i32.store8 offset=1174
              return
            end
            local.get 1
            i32.const 255
            i32.and
            br_if 3 (;@1;)
            local.get 3
            i32.const 255
            i32.and
            i32.eqz
            br_if 3 (;@1;)
            i32.const 0
            i32.const 1
            i32.store8 offset=1158
            i32.const 0
            i32.const 0
            i32.store8 offset=1175
            i32.const 0
            i32.const 0
            i32.store8 offset=1174
            return
          end
          i32.const 0
          i32.const 0
          i32.store8 offset=1174
          return
        end
        i32.const 0
        i32.load8_u offset=1179
        local.set 0
        block  ;; label = @3
          i32.const 0
          i32.load8_u offset=1183
          local.tee 2
          i32.eqz
          i32.const 0
          i32.load8_u offset=1180
          local.tee 3
          i32.const 255
          i32.and
          i32.const 0
          i32.ne
          i32.and
          local.tee 1
          i32.const 1
          i32.ne
          br_if 0 (;@3;)
          local.get 0
          i32.const 255
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          i32.const 1
          i32.store8 offset=1175
          i32.const 0
          i32.const 1
          i32.store8 offset=1176
          i32.const 0
          i32.const 2
          i32.store8 offset=1159
          i32.const 0
          i32.const 0
          i32.store8 offset=1174
          return
        end
        block  ;; label = @3
          local.get 1
          local.get 0
          i32.const 255
          i32.and
          i32.eqz
          i32.and
          i32.const 1
          i32.ne
          br_if 0 (;@3;)
          i32.const 0
          i32.const 1
          i32.store8 offset=1175
          i32.const 0
          i32.const 2
          i32.store8 offset=1159
          i32.const 0
          i32.const 0
          i32.store8 offset=1174
          return
        end
        block  ;; label = @3
          local.get 2
          br_if 0 (;@3;)
          local.get 3
          i32.const 255
          i32.and
          br_if 0 (;@3;)
          local.get 0
          i32.const 255
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          i32.const 1
          i32.store8 offset=1176
          i32.const 0
          i32.const 2
          i32.store8 offset=1159
          i32.const 0
          i32.const 0
          i32.store8 offset=1174
          return
        end
        i32.const 0
        i32.load8_u offset=1172
        local.set 1
        i32.const 0
        i32.load8_u offset=1173
        local.set 0
        block  ;; label = @3
          local.get 2
          br_if 0 (;@3;)
          local.get 0
          i32.const 255
          i32.and
          br_if 0 (;@3;)
          local.get 1
          i32.const 255
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          i32.const 1
          i32.store8 offset=1176
          i32.const 0
          i32.const 1
          i32.store8 offset=1159
          i32.const 0
          i32.const 0
          i32.store8 offset=1174
          return
        end
        block  ;; label = @3
          local.get 0
          i32.const 255
          i32.and
          local.tee 0
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          i32.const 255
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          i32.const 1
          i32.store8 offset=1176
          i32.const 0
          i32.const 1
          i32.store8 offset=1175
          i32.const 0
          i32.const 1
          i32.store8 offset=1159
          i32.const 0
          i32.const 0
          i32.store8 offset=1174
          return
        end
        local.get 0
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        local.get 2
        i32.or
        i32.const 255
        i32.and
        br_if 1 (;@1;)
        i32.const 0
        i32.const 1
        i32.store8 offset=1175
        i32.const 0
        i32.const 1
        i32.store8 offset=1159
        i32.const 0
        i32.const 0
        i32.store8 offset=1174
        return
      end
      i32.const 0
      i32.const 3
      i32.store8 offset=1159
      i32.const 0
      i32.const 0
      i32.store8 offset=1174
    end)
  (func (;4;) (type 0)
    (local i32 i32)
    block  ;; label = @1
      i32.const 0
      i32.load8_u offset=1053
      local.tee 0
      br_if 0 (;@1;)
      i32.const 0
      i32.load8_u offset=1055
      i32.const 255
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      i32.const 0
      i32.load8_u offset=1054
      i32.const 255
      i32.and
      br_if 0 (;@1;)
      i32.const 0
      i32.const 0
      i32.store8 offset=1046
      i32.const 0
      i32.const 0
      i32.store8 offset=1044
      return
    end
    block  ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        i32.const 0
        i32.load8_u offset=1050
        br_if 0 (;@2;)
        i32.const 0
        i32.const 3
        i32.store8 offset=1159
      end
      block  ;; label = @2
        i32.const 0
        i32.load8_u offset=1059
        br_if 0 (;@2;)
        i32.const 0
        i32.const 1
        i32.store8 offset=1171
        i32.const 0
        i32.const 0
        i32.store8 offset=1040
      end
      i32.const 0
      i32.const 1
      i32.store8 offset=1060
      i32.const 0
      i32.const 1
      i32.store8 offset=1051
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                i32.const 0
                i32.load8_u offset=1160
                i32.const -1
                i32.add
                br_table 0 (;@6;) 1 (;@5;) 2 (;@4;) 3 (;@3;)
              end
              i32.const 0
              i32.load offset=1184
              i32.const 59
              i32.ne
              br_if 3 (;@2;)
              i32.const 0
              i32.load offset=1188
              i32.const 59
              i32.eq
              br_if 3 (;@2;)
              i32.const 0
              i32.const 3
              i32.store8 offset=1160
              i32.const 0
              i32.const 3
              i32.store8 offset=1163
              i32.const 0
              i32.const 0
              i32.store8 offset=1174
              br 3 (;@2;)
            end
            block  ;; label = @5
              i32.const 0
              i32.load8_u offset=1192
              i32.eqz
              br_if 0 (;@5;)
              i32.const 0
              i32.load8_u offset=1193
              i32.const 255
              i32.and
              br_if 0 (;@5;)
              i32.const 0
              i32.load8_u offset=1194
              i32.const 255
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              i32.const 0
              i32.const 3
              i32.store8 offset=1160
              i32.const 0
              i32.const 3
              i32.store8 offset=1163
              i32.const 0
              i32.const 0
              i32.load offset=1104
              i32.store offset=1132
              i32.const 0
              i32.const 0
              i32.store8 offset=1128
              i32.const 0
              i32.const 0
              i32.store8 offset=1174
              br 3 (;@2;)
            end
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    i32.const 0
                    i32.load8_u offset=1156
                    i32.const -1
                    i32.add
                    br_table 0 (;@8;) 1 (;@7;) 2 (;@6;) 3 (;@5;)
                  end
                  i32.const 0
                  i32.load8_u offset=1195
                  br_if 5 (;@2;)
                  i32.const 0
                  i32.const 3
                  i32.store8 offset=1156
                  i32.const 0
                  i32.const 0
                  i32.store8 offset=1128
                  i32.const 0
                  i32.const 0
                  i32.store8 offset=1174
                  br 5 (;@2;)
                end
                i32.const 0
                i32.load8_u offset=1196
                br_if 4 (;@2;)
                i32.const 0
                i32.const 3
                i32.store8 offset=1156
                i32.const 0
                i32.const 0
                i32.store8 offset=1120
                i32.const 0
                i32.const 0
                i32.store8 offset=1174
                br 4 (;@2;)
              end
              block  ;; label = @6
                i32.const 0
                i32.load8_u offset=1196
                i32.eqz
                br_if 0 (;@6;)
                i32.const 0
                i32.const 1
                i32.store8 offset=1120
                i32.const 0
                i32.const 2
                i32.store8 offset=1156
                i32.const 0
                i32.const 0
                i32.store8 offset=1174
                br 4 (;@2;)
              end
              i32.const 0
              i32.load8_u offset=1195
              i32.eqz
              br_if 3 (;@2;)
              i32.const 0
              i32.const 1
              i32.store8 offset=1128
              i32.const 0
              i32.const 1
              i32.store8 offset=1156
              i32.const 0
              i32.const 0
              i32.store8 offset=1174
              br 3 (;@2;)
            end
            i32.const 0
            i32.const 3
            i32.store8 offset=1156
            i32.const 0
            i32.const 0
            i32.store8 offset=1174
            br 2 (;@2;)
          end
          block  ;; label = @4
            i32.const 0
            i32.load offset=1184
            i32.const 61
            i32.lt_s
            br_if 0 (;@4;)
            i32.const 0
            i32.load offset=1188
            i32.const 60
            i32.gt_s
            br_if 0 (;@4;)
            i32.const 0
            i32.load8_u offset=1197
            i32.const 255
            i32.and
            br_if 0 (;@4;)
            i32.const 0
            i32.load8_u offset=1198
            i32.const 255
            i32.and
            br_if 0 (;@4;)
            i32.const 0
            i32.const 1
            i32.store8 offset=1160
            i32.const 0
            i32.const 0
            i32.store8 offset=1128
            i32.const 0
            i32.const 0
            i32.store8 offset=1174
            i32.const 0
            i32.const 0
            i32.store8 offset=1120
            br 2 (;@2;)
          end
          block  ;; label = @4
            i32.const 0
            i32.load8_u offset=1192
            i32.const 0
            i32.ne
            i32.const 0
            i32.load8_u offset=1193
            i32.const 255
            i32.and
            i32.eqz
            i32.and
            local.tee 0
            i32.const 1
            i32.ne
            br_if 0 (;@4;)
            i32.const 0
            i32.load8_u offset=1199
            i32.const 255
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            i32.const 0
            i32.const 2
            i32.store8 offset=1160
            i32.const 0
            i32.const 3
            i32.store8 offset=1156
            i32.const 0
            i32.const 0
            i32.load offset=1104
            i32.store offset=1116
            i32.const 0
            i32.const 0
            i32.store8 offset=1120
            i32.const 0
            i32.const 0
            i32.store8 offset=1174
            br 2 (;@2;)
          end
          block  ;; label = @4
            local.get 0
            i32.const 0
            i32.load8_u offset=1194
            i32.const 255
            i32.and
            i32.const 0
            i32.ne
            i32.and
            i32.const 1
            i32.ne
            br_if 0 (;@4;)
            i32.const 0
            i32.const 2
            i32.store8 offset=1160
            i32.const 0
            i32.const 3
            i32.store8 offset=1156
            i32.const 0
            i32.const 0
            i32.load offset=1104
            i32.store offset=1124
            i32.const 0
            i32.const 0
            i32.store8 offset=1128
            i32.const 0
            i32.const 0
            i32.store8 offset=1174
            br 2 (;@2;)
          end
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  i32.const 0
                  i32.load8_u offset=1163
                  i32.const -1
                  i32.add
                  br_table 0 (;@7;) 1 (;@6;) 2 (;@5;) 3 (;@4;)
                end
                block  ;; label = @7
                  i32.const 0
                  i32.load offset=1200
                  i32.const 405
                  i32.lt_s
                  br_if 0 (;@7;)
                  i32.const 0
                  i32.const 3
                  i32.store8 offset=1163
                  i32.const 0
                  i32.const 0
                  i32.store8 offset=1120
                  i32.const 0
                  i32.const 0
                  i32.store8 offset=1174
                  br 5 (;@2;)
                end
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      i32.const 0
                      i32.load8_u offset=1166
                      i32.const -1
                      i32.add
                      br_table 0 (;@9;) 1 (;@8;) 2 (;@7;)
                    end
                    block  ;; label = @9
                      block  ;; label = @10
                        i32.const 0
                        i32.load8_u offset=1195
                        i32.eqz
                        br_if 0 (;@10;)
                        i32.const 0
                        i32.load8_u offset=1204
                        i32.const 255
                        i32.and
                        i32.eqz
                        br_if 1 (;@9;)
                      end
                      i32.const 0
                      i32.load8_u offset=1196
                      i32.eqz
                      br_if 7 (;@2;)
                      i32.const 0
                      i32.load8_u offset=1205
                      i32.const 255
                      i32.and
                      br_if 7 (;@2;)
                    end
                    i32.const 0
                    i32.const 3
                    i32.store8 offset=1163
                    i32.const 0
                    i32.const 0
                    i32.store8 offset=1120
                    i32.const 0
                    i32.const 0
                    i32.store8 offset=1174
                    i32.const 0
                    i32.const 0
                    i32.store8 offset=1166
                    br 6 (;@2;)
                  end
                  block  ;; label = @8
                    i32.const 0
                    i32.load8_u offset=1195
                    i32.eqz
                    br_if 0 (;@8;)
                    i32.const 0
                    i32.load8_u offset=1204
                    i32.const 255
                    i32.and
                    br_if 0 (;@8;)
                    i32.const 0
                    i32.const 1
                    i32.store8 offset=1166
                    i32.const 0
                    i32.const 0
                    i32.store8 offset=1174
                    br 6 (;@2;)
                  end
                  i32.const 0
                  i32.load8_u offset=1196
                  br_if 5 (;@2;)
                  i32.const 0
                  i32.load8_u offset=1205
                  i32.const 255
                  i32.and
                  i32.eqz
                  br_if 5 (;@2;)
                  i32.const 0
                  i32.const 3
                  i32.store8 offset=1163
                  i32.const 0
                  i32.const 0
                  i32.store8 offset=1120
                  i32.const 0
                  i32.const 0
                  i32.store8 offset=1174
                  i32.const 0
                  i32.const 0
                  i32.store8 offset=1166
                  br 5 (;@2;)
                end
                i32.const 0
                i32.const 2
                i32.store8 offset=1166
                i32.const 0
                i32.const 0
                i32.store8 offset=1174
                br 4 (;@2;)
              end
              block  ;; label = @6
                i32.const 0
                i32.load offset=1200
                i32.const 0
                i32.gt_s
                br_if 0 (;@6;)
                i32.const 0
                i32.const 3
                i32.store8 offset=1163
                i32.const 0
                i32.const 0
                i32.store8 offset=1128
                i32.const 0
                i32.const 0
                i32.store8 offset=1174
                br 4 (;@2;)
              end
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    i32.const 0
                    i32.load8_u offset=1167
                    i32.const -1
                    i32.add
                    br_table 0 (;@8;) 1 (;@7;) 2 (;@6;)
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        i32.const 0
                        i32.load8_u offset=1196
                        i32.eqz
                        br_if 0 (;@10;)
                        i32.const 0
                        i32.load8_u offset=1205
                        i32.const 255
                        i32.and
                        i32.eqz
                        br_if 1 (;@9;)
                      end
                      i32.const 0
                      i32.load8_u offset=1195
                      i32.eqz
                      br_if 1 (;@8;)
                      i32.const 0
                      i32.load8_u offset=1204
                      i32.const 255
                      i32.and
                      br_if 1 (;@8;)
                    end
                    i32.const 0
                    i32.const 3
                    i32.store8 offset=1163
                    i32.const 0
                    i32.const 0
                    i32.store8 offset=1128
                    i32.const 0
                    i32.const 0
                    i32.store8 offset=1174
                    br 6 (;@2;)
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        i32.const 0
                        i32.load8_u offset=1164
                        i32.const -1
                        i32.add
                        br_table 0 (;@10;) 1 (;@9;) 2 (;@8;)
                      end
                      i32.const 0
                      i32.const 0
                      i32.store8 offset=1063
                      i32.const 0
                      i32.load8_u offset=1062
                      i32.eqz
                      br_if 7 (;@2;)
                      i32.const 0
                      i32.const 1
                      i32.store8 offset=1128
                      i32.const 0
                      i32.const 2
                      i32.store8 offset=1164
                      i32.const 0
                      i32.const 1
                      i32.store8 offset=1057
                      i32.const 0
                      i32.const 0
                      i32.store8 offset=1174
                      i32.const 0
                      i32.const 0
                      i32.store8 offset=1206
                      i32.const 0
                      i32.const 0
                      i32.store8 offset=1120
                      br 7 (;@2;)
                    end
                    i32.const 0
                    i32.load8_u offset=1064
                    i32.eqz
                    br_if 6 (;@2;)
                    i32.const 0
                    i32.const 1
                    i32.store8 offset=1206
                    i32.const 0
                    i32.const 1
                    i32.store8 offset=1063
                    i32.const 0
                    i32.const 1
                    i32.store8 offset=1164
                    i32.const 0
                    i32.const 1
                    i32.store8 offset=1120
                    i32.const 0
                    i32.const 0
                    i32.load offset=1104
                    i32.store offset=1136
                    i32.const 0
                    i32.const 0
                    i32.store8 offset=1174
                    i32.const 0
                    i32.const 0
                    i32.store8 offset=1057
                    i32.const 0
                    i32.const 0
                    i32.store8 offset=1128
                    br 6 (;@2;)
                  end
                  i32.const 0
                  i32.const 2
                  i32.store8 offset=1164
                  i32.const 0
                  i32.const 1
                  i32.store8 offset=1057
                  i32.const 0
                  i32.const 0
                  i32.store8 offset=1174
                  br 5 (;@2;)
                end
                block  ;; label = @7
                  i32.const 0
                  i32.load8_u offset=1195
                  br_if 0 (;@7;)
                  i32.const 0
                  i32.load8_u offset=1204
                  i32.const 255
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  i32.const 0
                  i32.const 3
                  i32.store8 offset=1163
                  i32.const 0
                  i32.const 0
                  i32.store8 offset=1128
                  i32.const 0
                  i32.const 0
                  i32.store8 offset=1174
                  br 5 (;@2;)
                end
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      i32.const 0
                      i32.load8_u offset=1165
                      i32.const -1
                      i32.add
                      br_table 0 (;@9;) 1 (;@8;) 2 (;@7;)
                    end
                    i32.const 0
                    i32.const 0
                    i32.store8 offset=1063
                    i32.const 0
                    i32.load8_u offset=1062
                    i32.eqz
                    br_if 6 (;@2;)
                    i32.const 0
                    i32.const 2
                    i32.store8 offset=1165
                    i32.const 0
                    i32.const 1
                    i32.store8 offset=1057
                    i32.const 0
                    i32.const 1
                    i32.store8 offset=1128
                    i32.const 0
                    i32.const 0
                    i32.store8 offset=1207
                    i32.const 0
                    i32.const 0
                    i32.store8 offset=1174
                    i32.const 0
                    i32.const 0
                    i32.store8 offset=1120
                    br 6 (;@2;)
                  end
                  block  ;; label = @8
                    i32.const 0
                    i32.load8_u offset=1064
                    i32.eqz
                    br_if 0 (;@8;)
                    i32.const 0
                    i32.const 1
                    i32.store8 offset=1207
                    i32.const 0
                    i32.const 1
                    i32.store8 offset=1063
                    i32.const 0
                    i32.const 1
                    i32.store8 offset=1165
                    i32.const 0
                    i32.const 1
                    i32.store8 offset=1120
                    i32.const 0
                    i32.const 0
                    i32.load offset=1104
                    i32.store offset=1140
                    i32.const 0
                    i32.const 0
                    i32.store8 offset=1128
                    i32.const 0
                    i32.const 0
                    i32.store8 offset=1174
                    i32.const 0
                    i32.const 0
                    i32.store8 offset=1057
                    br 6 (;@2;)
                  end
                  i32.const 0
                  i32.load8_u offset=1196
                  i32.eqz
                  br_if 5 (;@2;)
                  i32.const 0
                  i32.load8_u offset=1205
                  i32.const 255
                  i32.and
                  br_if 5 (;@2;)
                  i32.const 0
                  i32.const 1
                  i32.store8 offset=1167
                  i32.const 0
                  i32.const 0
                  i32.store8 offset=1174
                  i32.const 0
                  i32.const 0
                  i32.store8 offset=1165
                  br 5 (;@2;)
                end
                i32.const 0
                i32.const 2
                i32.store8 offset=1165
                i32.const 0
                i32.const 1
                i32.store8 offset=1057
                i32.const 0
                i32.const 1
                i32.store8 offset=1128
                i32.const 0
                i32.const 0
                i32.store8 offset=1174
                br 4 (;@2;)
              end
              i32.const 0
              i32.const 2
              i32.store8 offset=1167
              i32.const 0
              i32.const 2
              i32.store8 offset=1165
              i32.const 0
              i32.const 1
              i32.store8 offset=1057
              i32.const 0
              i32.const 1
              i32.store8 offset=1128
              i32.const 0
              i32.const 0
              i32.store8 offset=1174
              br 3 (;@2;)
            end
            i32.const 0
            i32.load offset=1200
            local.set 0
            block  ;; label = @5
              i32.const 0
              i32.load8_u offset=1195
              i32.eqz
              br_if 0 (;@5;)
              i32.const 0
              i32.load8_u offset=1204
              i32.const 255
              i32.and
              br_if 0 (;@5;)
              local.get 0
              i32.const 1
              i32.lt_s
              br_if 0 (;@5;)
              i32.const 0
              i32.const 2
              i32.store8 offset=1163
              i32.const 0
              i32.const 2
              i32.store8 offset=1167
              i32.const 0
              i32.const 2
              i32.store8 offset=1165
              i32.const 0
              i32.const 1
              i32.store8 offset=1057
              i32.const 0
              i32.const 1
              i32.store8 offset=1128
              i32.const 0
              i32.const 0
              i32.store8 offset=1174
              br 3 (;@2;)
            end
            i32.const 0
            i32.load8_u offset=1196
            i32.eqz
            br_if 2 (;@2;)
            i32.const 0
            i32.load8_u offset=1205
            i32.const 255
            i32.and
            br_if 2 (;@2;)
            local.get 0
            i32.const 404
            i32.gt_s
            br_if 2 (;@2;)
            i32.const 0
            i32.const 1
            i32.store8 offset=1120
            i32.const 0
            i32.const 1
            i32.store8 offset=1163
            i32.const 0
            i32.const 2
            i32.store8 offset=1166
            i32.const 0
            i32.const 0
            i32.store8 offset=1174
            br 2 (;@2;)
          end
          i32.const 0
          i32.const 3
          i32.store8 offset=1163
          i32.const 0
          i32.const 0
          i32.store8 offset=1174
          br 1 (;@2;)
        end
        i32.const 0
        i32.const 2
        i32.store8 offset=1160
        i32.const 0
        i32.const 0
        i32.store8 offset=1174
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            i32.load8_u offset=1161
            i32.const 1
            i32.ne
            br_if 0 (;@4;)
            block  ;; label = @5
              i32.const 0
              i32.load8_u offset=1208
              local.tee 0
              i32.const 1
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1112
              local.tee 1
              i32.eqz
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1104
              local.get 1
              i32.sub
              i32.const 1
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load8_u offset=1194
              i32.const 0
              i32.load8_u offset=1199
              i32.const 255
              i32.and
              i32.or
              i32.eqz
              br_if 0 (;@5;)
              i32.const 0
              i32.const 1
              i32.store8 offset=1161
              i32.const 0
              i32.const 0
              i32.load offset=1184
              i32.const 1
              i32.add
              i32.store offset=1184
              i32.const 0
              i32.const 0
              i32.store8 offset=1174
              br 2 (;@3;)
            end
            block  ;; label = @5
              i32.const 0
              i32.load8_u offset=1162
              i32.const 1
              i32.ne
              br_if 0 (;@5;)
              local.get 0
              i32.const 1
              i32.ne
              br_if 3 (;@2;)
              i32.const 0
              i32.load offset=1108
              local.tee 0
              i32.eqz
              br_if 3 (;@2;)
              i32.const 0
              i32.load offset=1104
              local.get 0
              i32.sub
              i32.const 3
              i32.ne
              br_if 3 (;@2;)
              i32.const 0
              i32.load8_u offset=1194
              i32.const 255
              i32.and
              br_if 3 (;@2;)
              i32.const 0
              i32.load8_u offset=1199
              i32.const 255
              i32.and
              br_if 3 (;@2;)
              i32.const 0
              i32.load offset=1184
              local.tee 0
              i32.const 1
              i32.lt_s
              br_if 3 (;@2;)
              i32.const 0
              local.get 0
              i32.const -1
              i32.add
              i32.store offset=1184
              i32.const 0
              i32.const 0
              i32.store8 offset=1174
              br 2 (;@3;)
            end
            i32.const 0
            i32.const 0
            i32.store8 offset=1174
            br 1 (;@3;)
          end
          i32.const 0
          i32.const 1
          i32.store8 offset=1161
          i32.const 0
          i32.const 0
          i32.store offset=1184
          i32.const 0
          i32.const 0
          i32.store8 offset=1174
        end
        i32.const 0
        i32.const 1
        i32.store8 offset=1162
      end
      i32.const 0
      i32.const 0
      i32.load8_u offset=1044
      i32.store8 offset=1045
      i32.const 0
      i32.const 0
      i32.load8_u offset=1046
      i32.store8 offset=1047
    end)
  (func (;5;) (type 0)
    (local i32)
    block  ;; label = @1
      i32.const 0
      i32.load8_u offset=1056
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                i32.const 0
                i32.load8_u offset=1169
                i32.const -1
                i32.add
                br_table 0 (;@6;) 1 (;@5;) 2 (;@4;)
              end
              i32.const 0
              i32.load8_u offset=1209
              i32.eqz
              br_if 4 (;@1;)
              i32.const 0
              i32.load8_u offset=1210
              i32.const 255
              i32.and
              br_if 4 (;@1;)
              block  ;; label = @6
                i32.const 0
                i32.load8_u offset=1195
                i32.eqz
                br_if 0 (;@6;)
                i32.const 0
                i32.load8_u offset=1196
                i32.const 255
                i32.and
                br_if 5 (;@1;)
              end
              i32.const 0
              i32.const 1
              i32.store8 offset=1064
              i32.const 0
              i32.const 0
              i32.store8 offset=1174
              i32.const 2
              local.set 0
              br 3 (;@2;)
            end
            i32.const 0
            i32.const 0
            i32.store8 offset=1064
            i32.const 0
            i32.load8_u offset=1209
            br_if 3 (;@1;)
            i32.const 0
            i32.load8_u offset=1210
            i32.const 255
            i32.and
            i32.eqz
            br_if 3 (;@1;)
            i32.const 0
            i32.const 0
            i32.store8 offset=1174
            br 1 (;@3;)
          end
          i32.const 0
          i32.const 0
          i32.store8 offset=1174
        end
        i32.const 1
        local.set 0
      end
      i32.const 0
      local.get 0
      i32.store8 offset=1169
    end)
  (func (;6;) (type 0)
    (local i32)
    block  ;; label = @1
      i32.const 0
      i32.load8_u offset=1059
      local.tee 0
      br_if 0 (;@1;)
      i32.const 0
      i32.load8_u offset=1061
      i32.const 255
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      i32.const 0
      i32.load8_u offset=1060
      i32.const 255
      i32.and
      br_if 0 (;@1;)
      i32.const 0
      i32.const 0
      i32.store8 offset=1040
      return
    end
    block  ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            i32.load8_u offset=1171
            i32.const -1
            i32.add
            br_table 0 (;@4;) 1 (;@3;) 2 (;@2;)
          end
          i32.const 0
          i32.load offset=1212
          local.tee 0
          i32.const 1
          i32.lt_s
          br_if 2 (;@1;)
          local.get 0
          i32.const 0
          i32.load offset=1216
          i32.eq
          br_if 2 (;@1;)
          i32.const 0
          i32.const 2
          i32.store8 offset=1171
          i32.const 0
          i32.const 2
          i32.store offset=1224
          i32.const 0
          i32.const 3
          i32.store8 offset=1170
          i32.const 0
          i32.const 1
          i32.store8 offset=1040
          i32.const 0
          i32.const 0
          i32.store8 offset=1220
          i32.const 0
          i32.const 0
          i32.store8 offset=1174
          i32.const 0
          i32.const 0
          i32.store offset=1144
          return
        end
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              i32.const 0
              i32.load8_u offset=1199
              br_if 0 (;@5;)
              i32.const 0
              i32.load8_u offset=1228
              i32.const 255
              i32.and
              br_if 1 (;@4;)
            end
            i32.const 0
            i32.load8_u offset=1194
            br_if 1 (;@3;)
            i32.const 0
            i32.load8_u offset=1229
            i32.const 255
            i32.and
            i32.eqz
            br_if 1 (;@3;)
          end
          i32.const 0
          i32.const 1
          i32.store8 offset=1171
          i32.const 0
          i32.const 0
          i32.store8 offset=1174
          i32.const 0
          i32.const 0
          i32.store8 offset=1170
          return
        end
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              i32.const 0
              i32.load8_u offset=1170
              i32.const -1
              i32.add
              br_table 4 (;@1;) 0 (;@5;) 1 (;@4;) 2 (;@3;)
            end
            i32.const 0
            i32.load offset=1212
            i32.const 0
            i32.load offset=1224
            i32.const -2
            i32.add
            i32.le_s
            br_if 3 (;@1;)
            i32.const 0
            i32.const 1
            i32.store8 offset=1220
            i32.const 0
            i32.const 1
            i32.store8 offset=1170
            i32.const 0
            i32.const 0
            i32.store8 offset=1174
            return
          end
          i32.const 0
          i32.const 0
          i32.store8 offset=1040
          i32.const 0
          i32.load offset=1144
          i32.const 11
          i32.ne
          br_if 2 (;@1;)
          i32.const 0
          i32.load offset=1148
          i32.const 11
          i32.eq
          br_if 2 (;@1;)
          i32.const 0
          i32.const 2
          i32.store8 offset=1170
          i32.const 0
          i32.const 0
          i32.store8 offset=1174
          return
        end
        i32.const 0
        i32.const 2
        i32.store offset=1224
        i32.const 0
        i32.const 3
        i32.store8 offset=1170
        i32.const 0
        i32.const 1
        i32.store8 offset=1040
        i32.const 0
        i32.const 0
        i32.store offset=1144
        i32.const 0
        i32.const 0
        i32.store8 offset=1174
        return
      end
      i32.const 0
      i32.const 1
      i32.store8 offset=1171
      i32.const 0
      i32.const 0
      i32.store8 offset=1174
    end)
  (func (;7;) (type 0)
    (local i32 i32 i32 i32 i32 i32)
    i32.const 0
    i32.const 1
    i32.store offset=1104
    i32.const 0
    i32.const 0
    i32.store8 offset=1208
    loop  ;; label = @1
      i32.const 0
      i32.const 1
      i32.store8 offset=1174
      i32.const 0
      i32.const 0
      i32.load8_u offset=1208
      i32.const 1
      i32.add
      i32.store8 offset=1208
      i32.const 0
      local.set 0
      i32.const 2
      local.set 1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                i32.const 0
                i32.load8_u offset=1168
                i32.const -1
                i32.add
                br_table 0 (;@6;) 1 (;@5;) 2 (;@4;) 3 (;@3;)
              end
              i32.const 0
              local.set 0
              i32.const 0
              i32.load8_u offset=1230
              br_if 3 (;@2;)
              i32.const 2
              local.set 1
              i32.const 0
              i32.load8_u offset=1231
              i32.const 255
              i32.and
              br_if 2 (;@3;)
              br 3 (;@2;)
            end
            block  ;; label = @5
              i32.const 0
              i32.load8_u offset=1230
              i32.eqz
              br_if 0 (;@5;)
              i32.const 0
              i32.load8_u offset=1231
              i32.const 255
              i32.and
              br_if 0 (;@5;)
              i32.const 1
              local.set 1
              i32.const -100
              local.set 0
              br 2 (;@3;)
            end
            i32.const 0
            i32.load8_u offset=1236
            i32.eqz
            br_if 2 (;@2;)
            i32.const 0
            i32.load8_u offset=1237
            i32.const 255
            i32.and
            br_if 2 (;@2;)
            i32.const 3
            local.set 1
            i32.const 100
            local.set 0
            br 1 (;@3;)
          end
          i32.const 0
          local.set 0
          i32.const 0
          i32.load8_u offset=1236
          br_if 1 (;@2;)
          i32.const 2
          local.set 1
          i32.const 0
          i32.load8_u offset=1237
          i32.const 255
          i32.and
          i32.eqz
          br_if 1 (;@2;)
        end
        i32.const 0
        local.get 0
        i32.store offset=1232
        i32.const 0
        local.get 1
        i32.store8 offset=1168
        i32.const 0
        i32.const 0
        i32.store8 offset=1174
      end
      block  ;; label = @2
        i32.const 0
        i32.load8_u offset=1050
        br_if 0 (;@2;)
        i32.const 0
        i32.const 3
        i32.store8 offset=1159
      end
      i32.const 0
      i32.const 0
      i32.store8 offset=1051
      block  ;; label = @2
        i32.const 0
        i32.load8_u offset=1056
        br_if 0 (;@2;)
        i32.const 0
        i32.const 1
        i32.store8 offset=1169
      end
      block  ;; label = @2
        i32.const 0
        i32.load8_u offset=1059
        br_if 0 (;@2;)
        i32.const 0
        i32.const 1
        i32.store8 offset=1171
        i32.const 0
        i32.const 0
        i32.store8 offset=1040
      end
      block  ;; label = @2
        i32.const 0
        i32.load8_u offset=1053
        br_if 0 (;@2;)
        i32.const 0
        i32.const 256
        i32.store16 offset=1044
        i32.const 0
        i32.const 2
        i32.store8 offset=1160
        i32.const 0
        i32.const 1
        i32.store8 offset=1161
        i32.const 0
        i32.const 1
        i32.store8 offset=1162
        i32.const 0
        i32.const 0
        i32.store8 offset=1046
        i32.const 0
        i32.const 0
        i32.store offset=1184
      end
      i32.const 0
      i32.const 1
      i32.store8 offset=1060
      i32.const 0
      i32.const 1
      i32.store8 offset=1057
      i32.const 0
      i32.const 1
      i32.store8 offset=1051
      i32.const 0
      i32.const 1
      i32.store8 offset=1054
      i32.const 0
      i32.load8_u offset=1240
      local.set 0
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              i32.const 0
              i32.load8_u offset=1238
              local.tee 1
              i32.const 0
              i32.load8_u offset=1239
              i32.eq
              br_if 0 (;@5;)
              local.get 0
              i32.const 255
              i32.and
              br_if 0 (;@5;)
              i32.const 0
              local.get 1
              i32.store8 offset=1241
              br 1 (;@4;)
            end
            i32.const 0
            i32.load8_u offset=1242
            local.tee 1
            i32.const 0
            i32.load8_u offset=1243
            i32.eq
            br_if 0 (;@4;)
            local.get 0
            i32.const 255
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            i32.const 0
            local.get 1
            i32.store8 offset=1238
            br 1 (;@3;)
          end
          i32.const 0
          i32.load8_u offset=1244
          local.tee 1
          i32.const 0
          i32.load8_u offset=1245
          i32.eq
          br_if 0 (;@3;)
          local.get 0
          i32.const 255
          i32.and
          br_if 0 (;@3;)
          i32.const 0
          local.get 1
          i32.store8 offset=1246
          br 1 (;@2;)
        end
        i32.const 0
        i32.load8_u offset=1247
        local.tee 1
        i32.const 0
        i32.load8_u offset=1248
        i32.eq
        br_if 0 (;@2;)
        local.get 0
        i32.const 255
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        local.get 1
        i32.store8 offset=1244
      end
      i32.const 0
      i32.const 0
      i32.load8_u offset=1052
      i32.store8 offset=1050
      i32.const 0
      i32.const 0
      i32.load8_u offset=1055
      i32.store8 offset=1053
      i32.const 0
      i32.const 0
      i32.load8_u offset=1058
      i32.store8 offset=1056
      i32.const 0
      i32.const 0
      i32.load8_u offset=1061
      i32.store8 offset=1059
      i32.const 0
      i32.const 0
      i32.load8_u offset=1249
      i32.store8 offset=1179
      i32.const 0
      i32.const 0
      i32.load8_u offset=1246
      i32.store8 offset=1172
      i32.const 0
      i32.const 0
      i32.load8_u offset=1250
      i32.store8 offset=1180
      i32.const 0
      i32.const 0
      i32.load8_u offset=1241
      i32.store8 offset=1173
      call 3
      i32.const 0
      i32.const 0
      i32.load8_u offset=1199
      i32.store8 offset=1236
      i32.const 0
      i32.const 0
      i32.load8_u offset=1194
      i32.store8 offset=1230
      i32.const 0
      i32.const 0
      i32.load offset=1212
      i32.store offset=1252
      i32.const 0
      i32.const 0
      i32.load8_u offset=1209
      i32.store8 offset=1256
      i32.const 0
      i32.const 0
      i32.load offset=1200
      i32.store offset=1260
      i32.const 0
      i32.const 0
      i32.load8_u offset=1264
      i32.store8 offset=1265
      i32.const 0
      i32.const 0
      i32.load8_u offset=1179
      i32.store8 offset=1249
      i32.const 0
      i32.const 0
      i32.load8_u offset=1172
      i32.store8 offset=1246
      i32.const 0
      i32.const 0
      i32.load8_u offset=1180
      i32.store8 offset=1250
      i32.const 0
      i32.const 0
      i32.load8_u offset=1173
      i32.store8 offset=1241
      i32.const 0
      i32.const 0
      i32.load8_u offset=1183
      i32.store8 offset=1266
      i32.const 0
      i32.const 0
      i32.load8_u offset=1192
      i32.store8 offset=1267
      call 4
      i32.const 0
      i32.const 0
      i32.load8_u offset=1199
      i32.store8 offset=1236
      i32.const 0
      i32.const 0
      i32.load8_u offset=1194
      i32.store8 offset=1230
      i32.const 0
      i32.const 0
      i32.load offset=1212
      i32.store offset=1252
      i32.const 0
      i32.const 0
      i32.load8_u offset=1209
      i32.store8 offset=1256
      i32.const 0
      i32.const 0
      i32.load offset=1200
      i32.store offset=1260
      i32.const 0
      i32.const 0
      i32.load8_u offset=1264
      i32.store8 offset=1265
      i32.const 0
      i32.const 0
      i32.load8_u offset=1179
      i32.store8 offset=1249
      i32.const 0
      i32.const 0
      i32.load8_u offset=1172
      i32.store8 offset=1246
      i32.const 0
      i32.const 0
      i32.load8_u offset=1180
      i32.store8 offset=1250
      i32.const 0
      i32.const 0
      i32.load8_u offset=1173
      i32.store8 offset=1241
      i32.const 0
      i32.const 0
      i32.load8_u offset=1183
      i32.store8 offset=1266
      i32.const 0
      i32.const 0
      i32.load8_u offset=1192
      i32.store8 offset=1267
      call 5
      i32.const 0
      i32.const 0
      i32.load8_u offset=1199
      i32.store8 offset=1236
      i32.const 0
      i32.const 0
      i32.load8_u offset=1194
      i32.store8 offset=1230
      i32.const 0
      i32.const 0
      i32.load offset=1212
      i32.store offset=1252
      i32.const 0
      i32.const 0
      i32.load8_u offset=1209
      i32.store8 offset=1256
      i32.const 0
      i32.const 0
      i32.load offset=1200
      i32.store offset=1260
      i32.const 0
      i32.const 0
      i32.load8_u offset=1264
      i32.store8 offset=1265
      i32.const 0
      i32.const 0
      i32.load8_u offset=1179
      i32.store8 offset=1249
      i32.const 0
      i32.const 0
      i32.load8_u offset=1172
      i32.store8 offset=1246
      i32.const 0
      i32.const 0
      i32.load8_u offset=1180
      i32.store8 offset=1250
      i32.const 0
      i32.const 0
      i32.load8_u offset=1173
      i32.store8 offset=1241
      i32.const 0
      i32.const 0
      i32.load8_u offset=1183
      i32.store8 offset=1266
      i32.const 0
      i32.const 0
      i32.load8_u offset=1192
      i32.store8 offset=1267
      call 6
      i32.const 0
      i32.const 0
      i32.load offset=1212
      local.tee 0
      i32.store offset=1252
      i32.const 0
      i32.const 0
      i32.load8_u offset=1209
      local.tee 1
      i32.store8 offset=1256
      i32.const 0
      i32.const 0
      i32.load offset=1200
      i32.store offset=1260
      i32.const 0
      i32.const 0
      i32.load8_u offset=1264
      i32.store8 offset=1265
      i32.const 0
      i32.const 0
      i32.load8_u offset=1179
      local.tee 2
      i32.store8 offset=1249
      i32.const 0
      i32.const 0
      i32.load8_u offset=1172
      local.tee 3
      i32.store8 offset=1246
      i32.const 0
      i32.const 0
      i32.load8_u offset=1180
      local.tee 4
      i32.store8 offset=1250
      i32.const 0
      i32.const 0
      i32.load8_u offset=1173
      local.tee 5
      i32.store8 offset=1241
      i32.const 0
      i32.const 0
      i32.load8_u offset=1183
      i32.store8 offset=1266
      i32.const 0
      i32.const 0
      i32.load8_u offset=1050
      i32.store8 offset=1051
      i32.const 0
      local.get 0
      i32.store offset=1216
      i32.const 0
      local.get 0
      i32.store offset=1272
      i32.const 0
      local.get 5
      i32.store8 offset=1178
      i32.const 0
      local.get 4
      i32.store8 offset=1182
      i32.const 0
      local.get 3
      i32.store8 offset=1177
      i32.const 0
      local.get 2
      i32.store8 offset=1181
      i32.const 0
      i32.const 0
      i32.load8_u offset=1053
      i32.store8 offset=1054
      i32.const 0
      i32.const 0
      i32.load8_u offset=1056
      i32.store8 offset=1057
      i32.const 0
      i32.const 0
      i32.load8_u offset=1059
      i32.store8 offset=1060
      i32.const 0
      i32.const 0
      i32.load offset=1184
      i32.store offset=1188
      i32.const 0
      i32.const 0
      i32.load offset=1268
      i32.store offset=1232
      i32.const 0
      i32.const 0
      i32.load offset=1144
      i32.store offset=1148
      i32.const 0
      local.get 1
      i32.store8 offset=1210
      i32.const 0
      local.get 1
      i32.store8 offset=1276
      i32.const 0
      i32.const 0
      i32.load8_u offset=1220
      local.tee 0
      i32.store8 offset=1192
      i32.const 0
      local.get 0
      i32.store8 offset=1193
      i32.const 0
      i32.const 0
      i32.load8_u offset=1175
      local.tee 0
      i32.store8 offset=1195
      i32.const 0
      local.get 0
      i32.store8 offset=1204
      i32.const 0
      i32.const 0
      i32.load8_u offset=1176
      local.tee 0
      i32.store8 offset=1196
      i32.const 0
      local.get 0
      i32.store8 offset=1205
      i32.const 0
      i32.const 0
      i32.load8_u offset=1128
      local.tee 0
      i32.store8 offset=1194
      i32.const 0
      local.get 0
      i32.store8 offset=1229
      i32.const 0
      i32.const 0
      i32.load8_u offset=1120
      local.tee 0
      i32.store8 offset=1199
      i32.const 0
      local.get 0
      i32.store8 offset=1228
      i32.const 0
      i32.const 0
      i32.load8_u offset=1244
      i32.store8 offset=1245
      i32.const 0
      i32.const 0
      i32.load8_u offset=1238
      i32.store8 offset=1239
      i32.const 0
      i32.const 0
      i32.load8_u offset=1242
      i32.store8 offset=1243
      i32.const 0
      i32.const 0
      i32.load8_u offset=1247
      i32.store8 offset=1248
      i32.const 0
      i32.const 0
      i32.load8_u offset=1277
      local.tee 0
      i32.store8 offset=1267
      i32.const 0
      local.get 0
      i32.store8 offset=1278
      i32.const 0
      i32.const 0
      i32.load8_u offset=1279
      local.tee 0
      i32.store8 offset=1230
      i32.const 0
      local.get 0
      i32.store8 offset=1231
      i32.const 0
      i32.const 0
      i32.load8_u offset=1280
      local.tee 0
      i32.store8 offset=1236
      i32.const 0
      local.get 0
      i32.store8 offset=1237
      i32.const 0
      i32.load8_u offset=1174
      i32.eqz
      br_if 0 (;@1;)
    end)
  (func (;8;) (type 1) (param i32 i32)
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
            i32.const 1040
            i32.const 0
            i32.const 64
            call 16
            drop
            i32.const 0
            i32.const 0
            i32.store offset=1112
            i32.const 0
            i32.const 0
            i32.store offset=1152
            i32.const 0
            i32.const 0
            i32.store offset=1108
            i32.const 0
            i32.const 0
            i32.store8 offset=1156
            i32.const 0
            i32.const 0
            i32.store8 offset=1157
            i32.const 0
            i32.const 0
            i32.store8 offset=1158
            i32.const 0
            i32.const 0
            i32.store8 offset=1159
            i32.const 0
            i32.const 0
            i32.store8 offset=1160
            i32.const 0
            i32.const 0
            i32.store8 offset=1161
            i32.const 0
            i32.const 0
            i32.store8 offset=1162
            i32.const 0
            i32.const 0
            i32.store8 offset=1163
            i32.const 0
            i32.const 0
            i32.store8 offset=1164
            i32.const 0
            i32.const 0
            i32.store8 offset=1165
            i32.const 0
            i32.const 0
            i32.store8 offset=1166
            i32.const 0
            i32.const 0
            i32.store8 offset=1167
            i32.const 0
            i32.const 0
            i32.store8 offset=1168
            i32.const 0
            i32.const 0
            i32.store8 offset=1169
            i32.const 0
            i32.const 0
            i32.store8 offset=1170
            i32.const 0
            i32.const 0
            i32.store8 offset=1171
            call 2
            call 7
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
  (func (;9;) (type 2) (result i32)
    i32.const 3330
    i32.const 1
    call 8
    i32.const 0)
  (func (;10;) (type 0))
  (func (;11;) (type 3) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    call 12
    call 10
    call 14
    local.get 2
    call 9
    i32.store offset=12
    call 15
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    i32.const 0)
  (func (;12;) (type 0))
  (func (;13;) (type 2) (result i32)
    i32.const 0
    i32.const 0
    call 11)
  (func (;14;) (type 0))
  (func (;15;) (type 0))
  (func (;16;) (type 4) (param i32 i32 i32) (result i32)
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
  (global (;0;) (mut i32) (i32.const 17296))
  (global (;1;) i32 (i32.const 0))
  (global (;2;) i32 (i32.const 1281))
  (global (;3;) i32 (i32.const 17296))
  (export "memory" (memory 0))
  (export "_initialize" (func 1))
  (export "_run" (func 13))
  (export "__data_end" (global 2))
  (export "__heap_base" (global 3)))

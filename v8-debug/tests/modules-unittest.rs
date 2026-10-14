// 回归用例 (轮7 W7DBGR 复审 round1 固化)
// 对齐原版 gtest: v8/test/unittests/objects/modules-unittest.cc
//   覆盖 ModuleInstantiationFailures1 / SyntheticModule / CallSiteInfo 位置面所对应的
//   v8-debug 本层纯逻辑: source_text_module resolve_export/resolve_import 环检测、
//   star exports 歧义、prepare_instantiate、synthetic_module 全族、call_site_info 位置计算。
// 说明: 本层无 Isolate/Factory, 深度堆依赖以闭包注入; 此处仅锁定「本层可忠实表达」的语义,
//   防止后续填体回退。原版完整实例化/求值链仍归推迟台账 (readme/zh.md)。

use std::cell::Cell as StdCell;

use aok::{OK, Void};
use v8_debug::{
  Error,
  call_site_info::{
    CallSiteInfoView, K_IS_BUILTIN_MASK, K_IS_DEFERRED_BASELINE_FRAME_MASK, K_IS_STRICT_MASK,
    K_IS_WASM_MASK, K_RAW_DATA_FIELD_COUNT, ScriptView, construct_from_raw_data,
    expand_deferred_frames, get_line_number, get_source_position,
  },
  module::{
    Cell, ExportEntry, ExportsTable, JsPromiseView, K_ERRORED, K_EVALUATED, K_EVALUATING, K_LINKED,
    K_PRE_LINKING, ModuleBase, PromiseState, Slot,
  },
  source_text_module::{
    CellIndexKind, ModuleImportPhase, ResolveContext, ResolveSet, export_index, get_cell,
    get_cell_index_kind, import_index, load_variable, prepare_instantiate, resolve_export,
    resolve_export_using_star_exports, resolve_import, store_variable,
  },
  synthetic_module::{
    synthetic_module_evaluate, synthetic_module_finish_instantiate,
    synthetic_module_prepare_instantiate, synthetic_module_resolve_export,
    synthetic_module_set_export,
  },
};

// ── source_text_module: cell index 换算 (source-text-module.cc:136-146) ──
#[test]
fn test_cell_index_kind_and_indices() -> Void {
  assert_eq!(get_cell_index_kind(-5), CellIndexKind::Import);
  assert_eq!(get_cell_index_kind(5), CellIndexKind::Export);
  assert_eq!(get_cell_index_kind(0), CellIndexKind::Invalid);
  // ExportIndex(cell_index) = cell_index - 1; ImportIndex = -cell_index - 1
  assert_eq!(export_index(3), 2);
  assert_eq!(import_index(-3), 2);

  let exports = vec![Cell {
    value: Slot::Object(10),
  }];
  let imports = vec![Cell {
    value: Slot::Object(20),
  }];
  assert_eq!(
    get_cell(1, &exports, &imports).map(|c| c.value),
    Some(Slot::Object(10))
  );
  assert_eq!(
    get_cell(-1, &exports, &imports).map(|c| c.value),
    Some(Slot::Object(20))
  );
  assert!(get_cell(0, &exports, &imports).is_none());
  assert_eq!(load_variable(1, &exports, &imports), Some(Slot::Object(10)));
  OK
}

#[test]
fn test_store_variable_export_only() -> Void {
  let mut exports = vec![Cell {
    value: Slot::TheHole,
  }];
  assert!(store_variable(1, &mut exports, Slot::Object(42)));
  assert_eq!(exports[0].value, Slot::Object(42));
  // 越界导出 → false (原版 GetCell 会 DCHECK, 本层以 Option 兜底)
  assert!(!store_variable(9, &mut exports, Slot::Object(1)));
  OK
}

// ── resolve_export: 本地 Cell 命中即返回 (source-text-module.cc:218-222) ──
#[test]
fn test_resolve_export_local_cell() -> Void {
  let mut exports = ExportsTable::new(1);
  exports.insert(
    "x",
    ExportEntry::Cell(Cell {
      value: Slot::Object(7),
    }),
  );
  let mut rs: ResolveSet = ResolveSet::default();
  let mut indirect = |_import: Option<&str>,
                      _req: i32,
                      _e: &mut ExportsTable,
                      _r: &mut ResolveSet,
                      _must: bool,
                      _spec: &str|
   -> Result<Option<Cell>, Error> {
    panic!("indirect_resolve 不应在本地 Cell 命中时被调用")
  };
  let mut ctx = ResolveContext {
    module_hash: 1,
    module_specifier: "spec",
    must_resolve: true,
    resolve_set: &mut rs,
  };
  let got = resolve_export(&mut exports, "x", &mut ctx, &[], &mut indirect);
  assert!(matches!(
    got,
    Ok(Some(Cell {
      value: Slot::Object(7)
    }))
  ));
  OK
}

// ── resolve_export: 间接导出环检测 (source-text-module.cc:224-245) ──
#[test]
fn test_resolve_export_indirect_cycle_detection() -> Void {
  let mut exports = ExportsTable::new(1);
  exports.insert("y", ExportEntry::Indirect(0));
  // 预置 resolve_set: 当前 (module_hash=1) 已含 "y" → 再次解析即环
  let mut rs: ResolveSet = ResolveSet::default();
  rs.entry(1u32).or_default().insert("y".to_string());
  let zero_count = StdCell::new(0u32);
  let mut indirect = |_import: Option<&str>,
                      _req: i32,
                      _e: &mut ExportsTable,
                      _r: &mut ResolveSet,
                      _must: bool,
                      _spec: &str|
   -> Result<Option<Cell>, Error> {
    zero_count.set(zero_count.get() + 1);
    Ok(None)
  };

  // must_resolve=true → 抛 CyclicModuleDependency
  let mut ctx1 = ResolveContext {
    module_hash: 1,
    module_specifier: "spec",
    must_resolve: true,
    resolve_set: &mut rs,
  };
  let err = resolve_export(&mut exports, "y", &mut ctx1, &[], &mut indirect);
  match err {
    Err(Error::EvaluationFailed(msg)) => assert!(msg.contains("Cyclic module dependency"), "{msg}"),
    other => panic!("期望 Cyclic 错误, 实得 {other:?}"),
  }
  assert_eq!(
    zero_count.get(),
    0,
    "环命中时应早返回, 不进入 indirect_resolve"
  );

  // must_resolve=false → Ok(None), 无异常 (原版 return MaybeHandle)
  exports.insert("z", ExportEntry::Indirect(0));
  rs.entry(1u32).or_default().insert("z".to_string());
  let mut ctx2 = ResolveContext {
    module_hash: 1,
    module_specifier: "spec",
    must_resolve: false,
    resolve_set: &mut rs,
  };
  let none = resolve_export(&mut exports, "z", &mut ctx2, &[], &mut indirect);
  assert!(matches!(none, Ok(None)));
  OK
}

// ── resolve_export: 星导出环检测 (must-fix 1 回归测试) ──
#[test]
fn test_resolve_export_star_cycle_detection() -> Void {
  let mut exports = ExportsTable::new(0); // 空本地导出表 (the_hole)
  let mut rs: ResolveSet = ResolveSet::default();
  // 预置 resolve_set: 当前 (module_hash=1) 已在解析 "star_cycle"
  rs.entry(1u32).or_default().insert("star_cycle".to_string());
  let mut indirect = |_import: Option<&str>,
                      _req: i32,
                      _e: &mut ExportsTable,
                      _r: &mut ResolveSet,
                      _must: bool,
                      _spec: &str|
   -> Result<Option<Cell>, Error> {
    panic!("环命中时应早退, 不应调用 indirect_resolve")
  };
  let mut ctx = ResolveContext {
    module_hash: 1,
    module_specifier: "mod_cycle",
    must_resolve: true,
    resolve_set: &mut rs,
  };
  let res = resolve_export(&mut exports, "star_cycle", &mut ctx, &[0], &mut indirect);
  assert!(matches!(
    res,
    Err(Error::EvaluationFailed(msg)) if msg.contains("Cyclic module dependency")
  ));
  OK
}

// ── resolve_export: 间接导出解析成功回填 Cell (source-text-module.cc:253-278) ──
#[test]
fn test_resolve_export_indirect_success_backfills_cell() -> Void {
  let mut exports = ExportsTable::new(1);
  exports.insert("w", ExportEntry::Indirect(2));
  let mut rs: ResolveSet = ResolveSet::default();
  let mut indirect = |_import: Option<&str>,
                      _req: i32,
                      _e: &mut ExportsTable,
                      _r: &mut ResolveSet,
                      _must: bool,
                      _spec: &str|
   -> Result<Option<Cell>, Error> {
    Ok(Some(Cell {
      value: Slot::Object(99),
    }))
  };
  let mut ctx = ResolveContext {
    module_hash: 5,
    module_specifier: "spec",
    must_resolve: true,
    resolve_set: &mut rs,
  };
  let got = resolve_export(&mut exports, "w", &mut ctx, &[], &mut indirect);
  assert!(matches!(
    got,
    Ok(Some(Cell {
      value: Slot::Object(99)
    }))
  ));
  // 回填: 表中 "w" 由 Indirect 变为 Cell (原版 ObjectHashTable::Put cell)
  assert!(matches!(exports.lookup("w"), Some(ExportEntry::Cell(_))));
  OK
}

// ── resolve_export_using_star_exports: 唯一命中 / 歧义 / default 跳过 ──
#[test]
fn test_resolve_export_using_star_exports_unique() -> Void {
  let mut exports = ExportsTable::new(1);
  let mut rs: ResolveSet = ResolveSet::default();
  // 两个 star index 解析到相同 value → 唯一, 回填 Cell
  let mut indirect = |_import: Option<&str>,
                      _req: i32,
                      _e: &mut ExportsTable,
                      _r: &mut ResolveSet,
                      _must: bool,
                      _spec: &str|
   -> Result<Option<Cell>, Error> {
    Ok(Some(Cell {
      value: Slot::Object(1),
    }))
  };
  let mut ctx = ResolveContext {
    module_hash: 1,
    module_specifier: "spec",
    must_resolve: false,
    resolve_set: &mut rs,
  };
  let got = resolve_export_using_star_exports(&mut exports, "s", &mut ctx, &[0, 1], &mut indirect);
  assert!(matches!(
    got,
    Ok(Some(Cell {
      value: Slot::Object(1)
    }))
  ));
  assert!(matches!(exports.lookup("s"), Some(ExportEntry::Cell(_))));
  OK
}

#[test]
fn test_resolve_export_using_star_exports_ambiguous() -> Void {
  let mut exports = ExportsTable::new(1);
  let mut rs: ResolveSet = ResolveSet::default();
  // 两次调用返回不同 value → 歧义
  let counter = StdCell::new(0u32);
  let mut indirect = |_import: Option<&str>,
                      _req: i32,
                      _e: &mut ExportsTable,
                      _r: &mut ResolveSet,
                      _must: bool,
                      _spec: &str|
   -> Result<Option<Cell>, Error> {
    let n = counter.get();
    counter.set(n + 1);
    Ok(Some(Cell {
      value: Slot::Object(100 + n),
    }))
  };
  let mut ctx = ResolveContext {
    module_hash: 1,
    module_specifier: "spec",
    must_resolve: true,
    resolve_set: &mut rs,
  };
  let got =
    resolve_export_using_star_exports(&mut exports, "amb", &mut ctx, &[0, 1], &mut indirect);
  match got {
    Err(Error::EvaluationFailed(msg)) => assert!(msg.contains("Ambiguous export"), "{msg}"),
    other => panic!("期望 Ambiguous 错误, 实得 {other:?}"),
  }
  OK
}

#[test]
fn test_resolve_export_star_default_and_unresolvable() -> Void {
  let mut exports = ExportsTable::new(0);
  let mut rs: ResolveSet = ResolveSet::default();
  let mut indirect = |_i: Option<&str>,
                      _r: i32,
                      _e: &mut ExportsTable,
                      _s: &mut ResolveSet,
                      _m: bool,
                      _p: &str|
   -> Result<Option<Cell>, Error> { Ok(None) };
  // default 名不参与 star 解析 (source-text-module.cc:337)
  let mut ctx1 = ResolveContext {
    module_hash: 1,
    module_specifier: "spec",
    must_resolve: true,
    resolve_set: &mut rs,
  };
  let d =
    resolve_export_using_star_exports(&mut exports, "default", &mut ctx1, &[0], &mut indirect);
  assert!(matches!(d, Err(Error::EvaluationFailed(_))));
  // 未命中且 must_resolve=false → Ok(None)
  let mut ctx2 = ResolveContext {
    module_hash: 1,
    module_specifier: "spec",
    must_resolve: false,
    resolve_set: &mut rs,
  };
  let u = resolve_export_using_star_exports(&mut exports, "none", &mut ctx2, &[0], &mut indirect);
  assert!(matches!(u, Ok(None)));
  OK
}

// ── resolve_import: phase 分派 (source-text-module.cc:281-331) ──
#[test]
fn test_resolve_import_phase_dispatch() -> Void {
  let requested = vec![Slot::Object(0xAA), Slot::Object(0xBB)];
  let mut rs: ResolveSet = ResolveSet::default();

  // kSource → 直接建 Cell(value = requested_modules[idx])
  let mut ctx1 = ResolveContext {
    module_hash: 1,
    module_specifier: "spec",
    must_resolve: true,
    resolve_set: &mut rs,
  };
  let src = resolve_import(
    ModuleImportPhase::Source,
    Some("x"),
    1,
    &requested,
    &mut ctx1,
    &mut |_n: &str,
          _i: usize,
          _s: Option<&str>,
          _r: &mut ResolveSet,
          _m: bool,
          _p: &str|
     -> Result<Option<Cell>, Error> { panic!("kSource 不应走 resolve_module_export") },
    &mut |_i: usize, _ph: ModuleImportPhase| -> Result<Option<Cell>, Error> {
      panic!("kSource 不应走 resolve_namespace")
    },
  );
  assert!(matches!(
    src,
    Ok(Some(Cell {
      value: Slot::Object(0xBB)
    }))
  ));

  // kEvaluation + Some(name) → resolve_module_export
  let mut ctx2 = ResolveContext {
    module_hash: 1,
    module_specifier: "spec",
    must_resolve: true,
    resolve_set: &mut rs,
  };
  let named = resolve_import(
    ModuleImportPhase::Evaluation,
    Some("q"),
    0,
    &requested,
    &mut ctx2,
    &mut |n: &str,
          _i: usize,
          _s: Option<&str>,
          _r: &mut ResolveSet,
          _m: bool,
          _p: &str|
     -> Result<Option<Cell>, Error> {
      assert_eq!(n, "q");
      Ok(Some(Cell {
        value: Slot::Object(1),
      }))
    },
    &mut |_i: usize, _ph: ModuleImportPhase| -> Result<Option<Cell>, Error> {
      panic!("Some(name) 不应走 resolve_namespace")
    },
  );
  assert!(matches!(
    named,
    Ok(Some(Cell {
      value: Slot::Object(1)
    }))
  ));

  // kDefer + None → resolve_namespace
  let mut ctx3 = ResolveContext {
    module_hash: 1,
    module_specifier: "spec",
    must_resolve: true,
    resolve_set: &mut rs,
  };
  let ns = resolve_import(
    ModuleImportPhase::Defer,
    None,
    0,
    &requested,
    &mut ctx3,
    &mut |_n: &str,
          _i: usize,
          _s: Option<&str>,
          _r: &mut ResolveSet,
          _m: bool,
          _p: &str|
     -> Result<Option<Cell>, Error> { panic!("None 不应走 resolve_module_export") },
    &mut |_i: usize, ph: ModuleImportPhase| -> Result<Option<Cell>, Error> {
      assert_eq!(ph, ModuleImportPhase::Defer);
      Ok(Some(Cell {
        value: Slot::Object(2),
      }))
    },
  );
  assert!(matches!(
    ns,
    Ok(Some(Cell {
      value: Slot::Object(2)
    }))
  ));
  OK
}

// ── prepare_instantiate: 解析 requested_modules + 递归跳过 kSource ──
#[test]
fn test_prepare_instantiate_parses_and_recurse_skips_source() -> Void {
  let mut base = ModuleBase {
    status: K_PRE_LINKING,
    ..Default::default()
  };
  // 合法模块: 含 1 个本地导出 + 2 个 import 请求
  let mut exports_cells: Vec<Cell> = vec![Cell {
    value: Slot::TheHole,
  }];
  // requested_modules 须预分配至 module_requests 长度 (本层以索引赋值)
  let mut requested: Vec<Slot> = vec![Slot::TheHole, Slot::TheHole];
  let requests = vec![
    (ModuleImportPhase::Evaluation, "a".to_string(), 0usize),
    (ModuleImportPhase::Source, "b".to_string(), 0usize),
  ];
  let mut recused: Vec<u32> = Vec::new();
  let ok = prepare_instantiate(
    &mut base,
    &mut exports_cells,
    &mut requested,
    &requests,
    &mut |spec: &str, i: usize, _ph: ModuleImportPhase| -> Result<Slot, Error> {
      Ok(Slot::Object(if spec == "a" { 1 } else { 2 } + i as u32))
    },
    &mut |slot: &Slot| -> Result<bool, Error> {
      recused.push(match slot {
        Slot::Object(x) => *x,
        _ => 0,
      });
      Ok(true)
    },
  );
  assert!(ok.unwrap());
  // 两项均被解析写入
  assert!(matches!(requested[0], Slot::Object(_)));
  assert!(matches!(requested[1], Slot::Object(_)));
  // 递归只发生一次 (跳过 phase=kSource 的 index 1)
  assert_eq!(recused.len(), 1, "kSource 项应被跳过, 不递归");
  OK
}

// ── prepare_instantiate: 空 regular_exports (must-fix 2 回归测试) ──
#[test]
fn test_prepare_instantiate_empty_regular_exports() -> Void {
  let mut base = ModuleBase {
    status: K_PRE_LINKING,
    ..Default::default()
  };
  let mut exports_cells: Vec<Cell> = Vec::new(); // 0 本地导出 (纯 import 模块)
  let mut requested: Vec<Slot> = vec![Slot::TheHole];
  let requests = vec![(ModuleImportPhase::Evaluation, "dep".to_string(), 0usize)];
  let ok = prepare_instantiate(
    &mut base,
    &mut exports_cells,
    &mut requested,
    &requests,
    &mut |_spec, _i, _ph| Ok(Slot::Object(42)),
    &mut |_slot| Ok(true),
  );
  assert!(ok.unwrap());
  assert_eq!(requested[0], Slot::Object(42));
  OK
}

// ── synthetic_module 全族 (synthetic-module.cc) ──
#[test]
fn test_synthetic_module_prepare_finish_resolve_set() -> Void {
  let mut base = ModuleBase::default();
  assert!(synthetic_module_prepare_instantiate(&mut base, &["p", "q"]));
  // 每个名建初始 Cell(undefined)
  assert!(matches!(
    base.exports.lookup("p"),
    Some(ExportEntry::Cell(Cell {
      value: Slot::Undefined
    }))
  ));
  // finish → kLinked
  assert!(synthetic_module_finish_instantiate(&mut base));
  assert_eq!(base.status, K_LINKED);

  // set_export 命中 Cell → 更新值
  assert!(synthetic_module_set_export(&mut base.exports, "p", Slot::Object(5)).unwrap());
  // 未命中 → Err
  assert!(synthetic_module_set_export(&mut base.exports, "missing", Slot::Object(6)).is_err());

  // resolve_export: Cell → Some; 未命中 must_resolve 二态
  assert!(matches!(
    synthetic_module_resolve_export(&base.exports, "p", true, "spec"),
    Ok(Some(Cell {
      value: Slot::Object(5)
    }))
  ));
  assert!(matches!(
    synthetic_module_resolve_export(&base.exports, "nope", false, "spec"),
    Ok(None)
  ));
  assert!(synthetic_module_resolve_export(&base.exports, "nope", true, "spec").is_err());
  OK
}

// ── synthetic_module_evaluate 状态机 (must-fix 3 回归测试) ──
#[test]
fn test_synthetic_module_evaluate_state_machine() -> Void {
  let mut base = ModuleBase::default();
  // 成功流程: 状态流转 kEvaluating -> kEvaluated, 记录 top_level_capability
  let res = synthetic_module_evaluate(&mut base, |b| {
    assert_eq!(b.status, K_EVALUATING);
    Ok(Slot::Object(88))
  });
  assert_eq!(res, Some(Slot::Object(88)));
  assert_eq!(base.status, K_EVALUATED);
  assert_eq!(
    base.top_level_capability,
    Some(JsPromiseView {
      status: PromiseState::Fulfilled,
      result: Slot::Object(88),
      has_handler: false,
    })
  );

  // 失败流程: 状态流转 kEvaluating -> kErrored, 记录 base.exception
  let mut base2 = ModuleBase::default();
  let err_res = synthetic_module_evaluate(&mut base2, |b| {
    assert_eq!(b.status, K_EVALUATING);
    Err(Slot::Object(999))
  });
  assert_eq!(err_res, None);
  assert_eq!(base2.status, K_ERRORED);
  assert_eq!(base2.exception, Slot::Object(999));
  OK
}

// ── call_site_info: 位域访问器 + 字段序 (call-site-info-inl.h/cc:26) ──
#[test]
fn test_call_site_info_flags_and_construct() -> Void {
  let mut raw = vec![0i64; K_RAW_DATA_FIELD_COUNT];
  raw[0] = 0x1111; // receiver
  raw[1] = 7; // function
  raw[2] = 0x2222; // code
  raw[3] = 300; // offset
  raw[4] = (K_IS_WASM_MASK | K_IS_STRICT_MASK | K_IS_BUILTIN_MASK) as i64; // flags
  let view = construct_from_raw_data(&raw, 0, |code| Some(code as u32));
  assert_eq!(view.receiver_or_instance, 0x1111);
  assert_eq!(view.code_offset_or_source_position, 300);
  assert!(view.is_wasm());
  assert!(view.is_strict());
  assert!(view.is_builtin());
  // builtin → function 视作 builtin id
  assert_eq!(view.function_builtin_id, Some(7));
  assert_eq!(view.code_object, Some(0x2222));
  OK
}

#[test]
fn test_construct_from_raw_data_out_of_bounds() -> Void {
  let raw = vec![1i64, 2]; // 长度小于 K_RAW_DATA_FIELD_COUNT
  let view = construct_from_raw_data(&raw, 0, |_| None);
  assert_eq!(view, CallSiteInfoView::default());
  OK
}

#[test]
fn test_expand_deferred_frames_clears_flag() -> Void {
  let mut raw = vec![0i64; K_RAW_DATA_FIELD_COUNT];
  raw[2] = 0x5; // code slot
  raw[3] = 999; // pc offset
  raw[4] = K_IS_DEFERRED_BASELINE_FRAME_MASK as i64;
  expand_deferred_frames(&mut raw, |_code, _pc| Some((Some(0x9u32), 42i32)));
  // offset 被替换为 bytecode offset, deferred 位被清除
  assert_eq!(raw[3], 42);
  assert_eq!(raw[4] as u32 & K_IS_DEFERRED_BASELINE_FRAME_MASK, 0);
  let v = CallSiteInfoView {
    flags: raw[4] as u32,
    ..Default::default()
  };
  assert!(!v.is_deferred_baseline_frame());
  OK
}

// ── call_site_info: ScriptView 位置计算 (对应 modules-unittest.cc GetLineNumber/Column) ──
#[test]
fn test_script_position_line_column() -> Void {
  let script = ScriptView {
    source: Some("line0\nline1\nline2".to_string()),
    has_valid_source: true,
    ..Default::default()
  };
  // 行索引 0-based; "\n" 计数
  assert_eq!(script.get_line_number(0), 0);
  assert_eq!(script.get_line_number(6), 1); // "line0\n" 之后
  assert_eq!(script.get_line_number(12), 2);
  let pi = script.get_position_info(7);
  assert_eq!(pi.line, 1);
  assert_eq!(pi.column, 1); // 行首 index 6 → 偏移 1
  assert_eq!(script.get_column_number(8), 2);

  // CallSiteInfo 层 1-based 行号
  let info = CallSiteInfoView::default();
  assert_eq!(get_line_number(&info, Some(&script), 6), 2); // line(1)+1
  // wasm 行号恒 1
  let wasm_info = CallSiteInfoView {
    flags: K_IS_WASM_MASK,
    ..Default::default()
  };
  assert_eq!(get_line_number(&wasm_info, Some(&script), 6), 1);
  OK
}

#[test]
fn test_get_source_position_caches_flag() -> Void {
  let mut info = CallSiteInfoView {
    code_offset_or_source_position: 10,
    ..Default::default()
  };
  let pos = get_source_position(&mut info, |offset| offset * 2);
  assert_eq!(pos, 20);
  assert!(info.is_source_position_computed());
  assert_eq!(info.code_offset_or_source_position, 20);
  OK
}

#[test]
fn test_as_array_index_canonical() -> Void {
  use v8_debug::module::as_array_index;
  assert_eq!(as_array_index("0"), Some(0));
  assert_eq!(as_array_index("1"), Some(1));
  assert_eq!(as_array_index("123"), Some(123));
  assert_eq!(as_array_index("4294967294"), Some(0xffff_fffe));
  // Canonical numeric string rules: leading zeroes are not array indices
  assert_eq!(as_array_index("01"), None);
  assert_eq!(as_array_index("00"), None);
  assert_eq!(as_array_index("000"), None);
  // Exceeds kMaxArrayIndex (0xffff_fffe = 4294967294)
  assert_eq!(as_array_index("4294967295"), None);
  assert_eq!(as_array_index(""), None);
  assert_eq!(as_array_index("abc"), None);
  OK
}

#[test]
fn test_script_view_utf8_multibyte_safety() -> Void {
  let script = ScriptView {
    source: Some("你好\n世界\n🦀".to_string()),
    has_valid_source: true,
    ..Default::default()
  };
  // "你" is 3 bytes (0..3), "好" is 3 bytes (3..6), '\n' is at byte 6
  // Slicing at byte 1, 2, 4, 5 (mid-character) must NOT panic!
  assert_eq!(script.get_line_number(1), 0);
  assert_eq!(script.get_line_number(2), 0);
  assert_eq!(script.get_line_number(3), 0);
  assert_eq!(script.get_line_number(7), 1); // after first \n
  let p = script.get_position_info(2);
  assert_eq!(p.line, 0);
  assert_eq!(p.column, 2);
  OK
}

#[test]
fn test_exports_table_lookup_mut_in_place() -> Void {
  let mut exports = ExportsTable::new(1);
  exports.insert(
    "key",
    ExportEntry::Cell(Cell {
      value: Slot::Undefined,
    }),
  );
  if let Some(ExportEntry::Cell(cell)) = exports.lookup_mut("key") {
    cell.value = Slot::Smi(12345);
  }
  assert_eq!(exports.number_of_elements(), 1);
  assert!(matches!(
    exports.lookup("key"),
    Some(ExportEntry::Cell(Cell {
      value: Slot::Smi(12345)
    }))
  ));
  OK
}

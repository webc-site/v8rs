// 由 tool/src/torque/misc.js 从 v8/src/torque/torque-compiler.h 生成; 修订就地改, 复跑覆盖
// 编译器选项/结果/入口 (字段真转换)

use crate::{
  ast::Ast,
  context,
  error::{Error, Result},
  kythe_data::KytheConsumer,
  server_data::LanguageServerData,
  source_positions::{SourceFileMap, SourcePosition},
  torque_parser::parse_torque,
  utils::TorqueMessage,
};

/// 原版 TorqueCompilerOptions 1:1 (15 字段)
#[derive(Debug, Clone, Default)]
pub struct TorqueCompilerOptions {
  pub output_directory: String,
  pub v8_root: String,
  pub collect_language_server_data: bool,
  pub collect_kythe_data: bool,
  pub output_tsa: bool,
  /// dcheck(...) 仅 debug 构建生成; LS 支持可强制生成
  pub force_assert_statements: bool,
  /// Forge 32 位交叉编译模式
  pub force_32bit_output: bool,
  /// 输出附加 IR 注释
  pub annotate_ir: bool,
  /// 源路径含 v8-root 前缀时剥除
  pub strip_v8_root: bool,
  /// 生成 Torque DWARF 行信息
  pub torque_dwarf: bool,
  /// metagen layout JSON 路径; 空则关闭校验
  pub layout_json_path: String,
  /// @cppObjectLayoutDefinition 类的字段改由 layout JSON 提供 (过渡开关)
  pub use_cpp_layouts: bool,
  /// layout-positions.json 可选路径 (逐成员 C++ 源位置)
  pub layout_positions_path: String,
  /// 生成 #pragma kythe_inline_metadata
  pub kythe_inline_metadata: bool,
  pub kythe_default_corpus: String,
}

// 原版 CurrentCompilerOptions 上下文 → 显式传参 (登记偏差)

/// 原版 TorqueCompilerResult 1:1
#[derive(Debug, Default)]
pub struct TorqueCompilerResult {
  /// SourceId → 文件名映射, 错误时填充以便还原出错位置
  pub source_file_map: Option<SourceFileMap>,
  /// LS 数据, collect_language_server_data 开启时填充
  pub language_server_data: LanguageServerData,
  /// 编译期收集的错误消息
  pub messages: Vec<TorqueMessage>,
}

/// 原版 TorqueCompilationUnit 1:1
#[derive(Debug, Clone, Default)]
pub struct TorqueCompilationUnit {
  pub source_file_path: String,
  pub file_content: String,
}

/// 原版 CompileTorque(source, options) (torque-compiler.cc:144-171):
/// 上下文束 (SourceFileMap/CurrentAst/TorqueMessages) + parse 段诊断收集。
/// declare/生成段 (PredeclarationVisitor → ImplementationVisitor,
/// torque-compiler.cc:84-134) 未填体: parse 干净时提前收敛返回空消息结果
/// (登记 readme; 填体后由此接入全流程, 届时移除该收敛分支)
pub fn compile_torque(
  source: &str,
  options: TorqueCompilerOptions,
) -> Result<TorqueCompilerResult> {
  let mut result = TorqueCompilerResult::default();

  // 上下文束换入 (原版各 ContextualClass::Scope)
  let mut file_map = SourceFileMap::new(String::new());
  let source_id = file_map.add_source("<torque>".to_owned());
  let previous_map = context::replace_source_file_map(file_map);
  let file_scope = context::SourceFileScope::new(source_id);
  let previous_ast = context::replace_ast(Ast::new());
  let _previous_messages = context::take_messages();
  context::set_target_architecture(options.force_32bit_output);

  let origin = SourcePosition {
    source: source_id,
    ..SourcePosition::default()
  };
  let mut ast = Ast::new();
  let parse = parse_torque(source, &mut ast, origin);

  // 收束: Abort = 诊断已入列 (原版 catch TorqueAbortCompilation); 其余错误冒泡
  let parse = match parse {
    Ok(()) => None,
    Err(Error::Abort) => Some(Error::Abort),
    Err(e) => {
      context::replace_ast(previous_ast);
      drop(file_scope);
      context::replace_source_file_map(previous_map);
      return Err(e);
    }
  };
  let _ = parse;
  let _ast = context::replace_ast(previous_ast);
  result.messages = context::take_messages();
  drop(file_scope);
  // 编译用映射 (含 <torque> 登记) 随结果返回
  result.source_file_map = Some(context::replace_source_file_map(previous_map));
  Ok(result)
}

/// 原版 CompileTorque(files, options) 重载
pub fn compile_torque_files(
  files: &[String],
  options: TorqueCompilerOptions,
) -> Result<TorqueCompilerResult> {
  // TODO(填体): 逐文件读入 + SourceFileMap 登记 + compile_torque
  let _ = (files, options);
  Err(Error::Unimplemented)
}

/// 原版 CompileTorqueForKythe
pub fn compile_torque_for_kythe(
  units: Vec<TorqueCompilationUnit>,
  options: TorqueCompilerOptions,
  kythe_consumer: &mut dyn KytheConsumer,
) -> Result<TorqueCompilerResult> {
  // TODO(填体)
  let _ = (units, options, kythe_consumer);
  Err(Error::Unimplemented)
}

// ── 原版 src/torque/torque-compiler.cc ──
// 主驱动流程签名清单
// L27: std::optional<std::string> ReadFile(const std::string& path)
// L35: void ReadAndParseTorqueFile(const std::string& path)
// L54: void CompileCurrentAst(TorqueCompilerOptions options)
// L144: TorqueCompilerResult CompileTorque(const std::string& source, TorqueCompilerOptions options)
// L174: TorqueCompilerResult CompileTorque(const std::vector<std::string>& files, TorqueCompilerOptions options)
// L205: TorqueCompilerResult CompileTorqueForKythe( std::vector<TorqueCompilationUnit> units, TorqueCompilerOptions options, KytheConsumer* consumer)

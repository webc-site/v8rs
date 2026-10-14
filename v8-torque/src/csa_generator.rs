// 由 tool/src/torque/generators.js 从 v8/src/torque/csa-generator.h 生成; 修订就地改, 复跑覆盖
// CSA 后端生成器

use crate::{
  cfg::ControlFlowGraph,
  declarable::BuiltinKind,
  error::{Error, Result},
  instructions::{BlockId, DefinitionLocation, Instruction},
  source_positions::SourcePosition,
  torque_code_generator::TorqueCodeGenerator,
  types::{TypeId, TypeVector, VisitResult},
  utils::Stack,
};

// C++ CSAGenerator : TorqueCodeGenerator → 组合 (base 字段 + 自有字段)。
/// 原版 CSAGenerator::ARGUMENTS_VARIABLE_STRING
pub const ARGUMENTS_VARIABLE_STRING: &str = "arguments";

/// 原版 CSAGenerator 1:1
#[derive(Debug, Default)]
pub struct CsaGenerator {
  pub base: TorqueCodeGenerator,
  pub linkage: Option<BuiltinKind>,
}

impl CsaGenerator {
  /// 原版 EmitGraph
  pub fn emit_graph(
    &mut self,
    cfg: &ControlFlowGraph,
    parameters: Stack<String>,
  ) -> Result<Option<Stack<String>>> {
    // TODO(填体)
    let _ = (cfg, parameters);
    Err(Error::Unimplemented)
  }

  /// 原版 EmitCSAValue (静态)
  pub fn emit_csa_value(
    result: &VisitResult,
    values: &Stack<String>,
    out: &mut String,
  ) -> Result<()> {
    // TODO(填体)
    let _ = (result, values, out);
    Err(Error::Unimplemented)
  }

  /// 原版 EmitSourcePosition (override)
  pub fn emit_source_position(&mut self, pos: SourcePosition, always_emit: bool) -> Result<()> {
    // TODO(填体)
    let _ = (pos, always_emit);
    Err(Error::Unimplemented)
  }

  /// 原版 PreCallableExceptionPreparation
  pub fn pre_callable_exception_preparation(
    &mut self,
    catch_block: Option<BlockId>,
  ) -> Result<String> {
    // TODO(填体)
    let _ = catch_block;
    Err(Error::Unimplemented)
  }

  /// 原版 PostCallableExceptionPreparation
  pub fn post_callable_exception_preparation(
    &mut self,
    catch_name: &str,
    return_type: TypeId,
    catch_block: Option<BlockId>,
    stack: &mut Stack<String>,
    exception_object_definition: Option<DefinitionLocation>,
  ) -> Result<()> {
    // TODO(填体)
    let _ = (
      catch_name,
      return_type,
      catch_block,
      stack,
      exception_object_definition,
    );
    Err(Error::Unimplemented)
  }

  /// 原版 ProcessArgumentsCommon
  pub fn process_arguments_common(
    &mut self,
    parameter_types: &TypeVector,
    constexpr_arguments: Vec<String>,
    stack: &mut Stack<String>,
  ) -> Result<Vec<String>> {
    // TODO(填体)
    let _ = (parameter_types, constexpr_arguments, stack);
    Err(Error::Unimplemented)
  }

  /// 原版 EmitBlock
  pub fn emit_block(&mut self, cfg: &ControlFlowGraph, block: BlockId) -> Result<Stack<String>> {
    // TODO(填体)
    let _ = (cfg, block);
    Err(Error::Unimplemented)
  }

  /// 原版 EmitInstruction(T) × 22 (后端相关逐指令重载, 见 .cc 清单)
  pub fn emit_instruction(
    &mut self,
    cfg: &ControlFlowGraph,
    instruction: &Instruction,
    stack: &mut Stack<String>,
  ) -> Result<()> {
    // TODO(填体): 逐指令分支 (TORQUE_BACKEND_DEPENDENT_INSTRUCTION_LIST)
    let _ = (cfg, instruction, stack);
    Err(Error::Unimplemented)
  }
}

// ── 原版 src/torque/csa-generator.cc ──
// L18: std::optional<Stack<std::string>> CSAGenerator::EmitGraph( Stack<std::string> parameters)
// L70: Stack<std::string> CSAGenerator::EmitBlock(const Block* block)
// L92: void CSAGenerator::EmitSourcePosition(SourcePosition pos, bool always_emit)
// L103: void CSAGenerator::EmitInstruction( const PushUninitializedInstruction& instruction, Stack<std::string>* stack)
// L115: void CSAGenerator::EmitInstruction( const PushBuiltinPointerInstruction& instruction, Stack<std::string>* stack)
// L125: void CSAGenerator::EmitInstruction( const NamespaceConstantInstruction& instruction, Stack<std::string>* stack)
// L155: std::vector<std::string> CSAGenerator::ProcessArgumentsCommon( const TypeVector& parameter_types, std::vector<std::string> constexpr_arguments, Stack<std::string>* stack)
// L176: void CSAGenerator::EmitInstruction(const CallIntrinsicInstruction& instruction, Stack<std::string>* stack)
// L317: void CSAGenerator::EmitInstruction(const CallCsaMacroInstruction& instruction, Stack<std::string>* stack)
// L368: void CSAGenerator::EmitInstruction( const CallCsaMacroAndBranchInstruction& instruction, Stack<std::string>* stack)
// L490: void CSAGenerator::EmitInstruction(const MakeLazyNodeInstruction& instruction, Stack<std::string>* stack)
// L525: void CSAGenerator::EmitInstruction(const CallBuiltinInstruction& instruction, Stack<std::string>* stack)
// L623: void CSAGenerator::EmitInstruction( const CallBuiltinPointerInstruction& instruction, Stack<std::string>* stack)
// L659: std::string CSAGenerator::PreCallableExceptionPreparation( std::optional<Block*> catch_block)
// L672: void CSAGenerator::PostCallableExceptionPreparation( const std::string& catch_name, const Type* return_type, std::optional<Block*> catch_block, Stack<std::string>* stack, const std::optional<DefinitionLocation>& exception_object_definition)
// L713: void CSAGenerator::EmitInstruction(const CallRuntimeInstruction& instruction, Stack<std::string>* stack)
// L769: void CSAGenerator::EmitInstruction(const BranchInstruction& instruction, Stack<std::string>* stack)
// L802: void CSAGenerator::EmitInstruction( const ConstexprBranchInstruction& instruction, Stack<std::string>* stack)
// L831: void CSAGenerator::EmitInstruction(const GotoInstruction& instruction, Stack<std::string>* stack)
// L846: void CSAGenerator::EmitInstruction(const GotoExternalInstruction& instruction, Stack<std::string>* stack)
// L855: void CSAGenerator::EmitInstruction(const ReturnInstruction& instruction, Stack<std::string>* stack)
// L867: void CSAGenerator::EmitInstruction(const PrintErrorInstruction& instruction, Stack<std::string>* stack)
// L873: void CSAGenerator::EmitInstruction(const AbortInstruction& instruction, Stack<std::string>* stack)
// L899: void CSAGenerator::EmitInstruction(const UnsafeCastInstruction& instruction, Stack<std::string>* stack)
// L909: void CSAGenerator::EmitInstruction(const LoadReferenceInstruction& instruction, Stack<std::string>* stack)
// L931: void CSAGenerator::EmitInstruction(const StoreReferenceInstruction& instruction, Stack<std::string>* stack)
// L949: std::string GetBitFieldSpecialization(const Type* container, const BitField& field)
// L967: void CSAGenerator::EmitInstruction(const LoadBitFieldInstruction& instruction, Stack<std::string>* stack)
// L1008: void CSAGenerator::EmitInstruction(const StoreBitFieldInstruction& instruction, Stack<std::string>* stack)
// L1061: void CSAGenerator::EmitCSAValue(VisitResult result, const Stack<std::string>& values, std::ostream& out)

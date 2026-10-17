// 由 tool/src/heap/gc-tracer.h.js 从 v8/src/heap/gc-tracer.{h,cc} + src/init/heap-symbols.h 生成; 修订就地改, 复跑覆盖
// GC 事件追踪: ScopeId 全表 (TRACER_SCOPES+背景段) 与事件枚举

/// 追踪作用域 (对应 GCTracer::ScopeId; TRACER_SCOPES + TRACER_BACKGROUND_SCOPES 展开)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum ScopeId {
  /// McIncremental
  McIncremental = 0,
  /// McIncrementalEmbedderTracing
  McIncrementalEmbedderTracing = 1,
  /// McIncrementalExternalEpilogue
  McIncrementalExternalEpilogue = 2,
  /// McIncrementalExternalPrologue
  McIncrementalExternalPrologue = 3,
  /// McIncrementalLayoutChange
  McIncrementalLayoutChange = 4,
  /// McIncrementalStart
  McIncrementalStart = 5,
  /// McIncrementalSweeping
  McIncrementalSweeping = 6,
  /// MinorMsIncrementalStart
  MinorMsIncrementalStart = 7,
  /// HeapEmbedderTracingEpilogue
  HeapEmbedderTracingEpilogue = 8,
  /// HeapEnsureSweepingCompleted
  HeapEnsureSweepingCompleted = 9,
  /// HeapEpilogue
  HeapEpilogue = 10,
  /// HeapEpilogueSafepoint
  HeapEpilogueSafepoint = 11,
  /// HeapExternalEpilogue
  HeapExternalEpilogue = 12,
  /// HeapExternalNearHeapLimit
  HeapExternalNearHeapLimit = 13,
  /// HeapExternalPrologue
  HeapExternalPrologue = 14,
  /// HeapExternalSecondPassCallbacks
  HeapExternalSecondPassCallbacks = 15,
  /// HeapExternalWeakGlobalHandles
  HeapExternalWeakGlobalHandles = 16,
  /// HeapPrologue
  HeapPrologue = 17,
  /// HeapPrologueSafepoint
  HeapPrologueSafepoint = 18,
  /// MarkCompactor
  MarkCompactor = 19,
  /// McClear
  McClear = 20,
  /// McEpilogue
  McEpilogue = 21,
  /// McEvacuate
  McEvacuate = 22,
  /// McFinish
  McFinish = 23,
  /// McMark
  McMark = 24,
  /// McPrologue
  McPrologue = 25,
  /// McSweep
  McSweep = 26,
  /// McClearDependentCode
  McClearDependentCode = 27,
  /// McClearExternalStringTable
  McClearExternalStringTable = 28,
  /// McClearFlushableBytecode
  McClearFlushableBytecode = 29,
  /// McClearFlushedJsFunctions
  McClearFlushedJsFunctions = 30,
  /// McClearJoinJob
  McClearJoinJob = 31,
  /// McClearJsWeakReferences
  McClearJsWeakReferences = 32,
  /// McClearMaps
  McClearMaps = 33,
  /// McClearSharedStructTypeRegistry
  McClearSharedStructTypeRegistry = 34,
  /// McClearSlotsBuffer
  McClearSlotsBuffer = 35,
  /// McClearStringForwardingTable
  McClearStringForwardingTable = 36,
  /// McClearSweepCodePointerTable
  McClearSweepCodePointerTable = 37,
  /// McClearSweepExternalPointerTable
  McClearSweepExternalPointerTable = 38,
  /// McClearSweepJsDispatchTable
  McClearSweepJsDispatchTable = 39,
  /// McClearSweepTrustedPointerTable
  McClearSweepTrustedPointerTable = 40,
  /// McClearSweepWasmCodePointerTable
  McClearSweepWasmCodePointerTable = 41,
  /// McClearWeakCollections
  McClearWeakCollections = 42,
  /// McClearWeakGlobalHandles
  McClearWeakGlobalHandles = 43,
  /// McClearWeakLists
  McClearWeakLists = 44,
  /// McClearWeakReferencesJoinFilterJob
  McClearWeakReferencesJoinFilterJob = 45,
  /// McClearWeakReferencesNonTrivial
  McClearWeakReferencesNonTrivial = 46,
  /// McClearWeakReferencesTrivial
  McClearWeakReferencesTrivial = 47,
  /// McClearWeakReferencesTrusted
  McClearWeakReferencesTrusted = 48,
  /// McCompleteSweepArrayBuffers
  McCompleteSweepArrayBuffers = 49,
  /// McCompleteSweeping
  McCompleteSweeping = 50,
  /// McEvacuateCandidates
  McEvacuateCandidates = 51,
  /// McEvacuateCleanUp
  McEvacuateCleanUp = 52,
  /// McEvacuateCopy
  McEvacuateCopy = 53,
  /// McEvacuateCopyParallel
  McEvacuateCopyParallel = 54,
  /// McEvacuateEpilogue
  McEvacuateEpilogue = 55,
  /// McEvacuatePinPages
  McEvacuatePinPages = 56,
  /// McEvacuatePrologue
  McEvacuatePrologue = 57,
  /// McEvacuateRebalance
  McEvacuateRebalance = 58,
  /// McEvacuateUpdatePointers
  McEvacuateUpdatePointers = 59,
  /// McEvacuateUpdatePointersClientHeaps
  McEvacuateUpdatePointersClientHeaps = 60,
  /// McEvacuateUpdatePointersParallel
  McEvacuateUpdatePointersParallel = 61,
  /// McEvacuateUpdatePointersSlotsMain
  McEvacuateUpdatePointersSlotsMain = 62,
  /// McEvacuateUpdatePointersToNewRoots
  McEvacuateUpdatePointersToNewRoots = 63,
  /// McEvacuateUpdatePointersWeak
  McEvacuateUpdatePointersWeak = 64,
  /// McEvacuateUpdatePointersPointerTables
  McEvacuateUpdatePointersPointerTables = 65,
  /// McFinishSweepArrayBuffers
  McFinishSweepArrayBuffers = 66,
  /// McMarkClientHeaps
  McMarkClientHeaps = 67,
  /// McMarkEmbedderPrologue
  McMarkEmbedderPrologue = 68,
  /// McMarkEmbedderTracing
  McMarkEmbedderTracing = 69,
  /// McMarkFinishIncremental
  McMarkFinishIncremental = 70,
  /// McMarkFullClosureParallel
  McMarkFullClosureParallel = 71,
  /// McMarkFullClosureParallelJoin
  McMarkFullClosureParallelJoin = 72,
  /// McMarkFullClosureSerial
  McMarkFullClosureSerial = 73,
  /// McMarkRetainMaps
  McMarkRetainMaps = 74,
  /// McMarkRoots
  McMarkRoots = 75,
  /// McMarkFullClosure
  McMarkFullClosure = 76,
  /// McMarkWeakClosureEphemeronMarking
  McMarkWeakClosureEphemeronMarking = 77,
  /// McMarkWeakClosureEphemeronLinear
  McMarkWeakClosureEphemeronLinear = 78,
  /// McMarkVerify
  McMarkVerify = 79,
  /// McSweepCode
  McSweepCode = 80,
  /// McSweepCodeLo
  McSweepCodeLo = 81,
  /// McSweepLo
  McSweepLo = 82,
  /// McSweepMap
  McSweepMap = 83,
  /// McSweepNew
  McSweepNew = 84,
  /// McSweepNewLo
  McSweepNewLo = 85,
  /// McSweepOld
  McSweepOld = 86,
  /// McSweepShared
  McSweepShared = 87,
  /// McSweepSharedLo
  McSweepSharedLo = 88,
  /// McSweepTrusted
  McSweepTrusted = 89,
  /// McSweepTrustedLo
  McSweepTrustedLo = 90,
  /// McSweepStartJobs
  McSweepStartJobs = 91,
  /// McWeaknessHandling
  McWeaknessHandling = 92,
  /// MinorMarkSweeper
  MinorMarkSweeper = 93,
  /// MinorMs
  MinorMs = 94,
  /// MinorMsClear
  MinorMsClear = 95,
  /// MinorMsFinish
  MinorMsFinish = 96,
  /// MinorMsMark
  MinorMsMark = 97,
  /// MinorMsSweep
  MinorMsSweep = 98,
  /// MinorMsClearStringForwardingTable
  MinorMsClearStringForwardingTable = 99,
  /// MinorMsClearStringTable
  MinorMsClearStringTable = 100,
  /// MinorMsClearWeakGlobalHandles
  MinorMsClearWeakGlobalHandles = 101,
  /// MinorMsCompleteSweepArrayBuffers
  MinorMsCompleteSweepArrayBuffers = 102,
  /// MinorMsCompleteSweeping
  MinorMsCompleteSweeping = 103,
  /// MinorMsMarkFinishIncremental
  MinorMsMarkFinishIncremental = 104,
  /// MinorMsMarkParallel
  MinorMsMarkParallel = 105,
  /// MinorMsMarkIncrementalSeed
  MinorMsMarkIncrementalSeed = 106,
  /// MinorMsMarkSeed
  MinorMsMarkSeed = 107,
  /// MinorMsMarkTracedHandles
  MinorMsMarkTracedHandles = 108,
  /// MinorMsMarkConservativeStack
  MinorMsMarkConservativeStack = 109,
  /// MinorMsMarkClosureParallel
  MinorMsMarkClosureParallel = 110,
  /// MinorMsMarkClosure
  MinorMsMarkClosure = 111,
  /// MinorMsMarkEmbedderPrologue
  MinorMsMarkEmbedderPrologue = 112,
  /// MinorMsMarkEmbedderTracing
  MinorMsMarkEmbedderTracing = 113,
  /// MinorMsMarkVerify
  MinorMsMarkVerify = 114,
  /// MinorMsIncrementalStep
  MinorMsIncrementalStep = 115,
  /// MinorMsSweepNew
  MinorMsSweepNew = 116,
  /// MinorMsSweepNewLo
  MinorMsSweepNewLo = 117,
  /// MinorMsSweepUpdateStringTable
  MinorMsSweepUpdateStringTable = 118,
  /// MinorMsSweepStartJobs
  MinorMsSweepStartJobs = 119,
  /// MinorMsFinishSweepArrayBuffers
  MinorMsFinishSweepArrayBuffers = 120,
  /// MinorMsFinishEnsureCapacity
  MinorMsFinishEnsureCapacity = 121,
  /// Safepoint
  Safepoint = 122,
  /// Scavenger
  Scavenger = 123,
  /// ScavengerCompleteSweepArrayBuffers
  ScavengerCompleteSweepArrayBuffers = 124,
  /// ScavengerFreeRememberedSet
  ScavengerFreeRememberedSet = 125,
  /// ScavengerResizeNewSpace
  ScavengerResizeNewSpace = 126,
  /// ScavengerScavenge
  ScavengerScavenge = 127,
  /// ScavengerScavengeCollectOldToNewPages
  ScavengerScavengeCollectOldToNewPages = 128,
  /// ScavengerScavengeWeakGlobalHandlesIdentify
  ScavengerScavengeWeakGlobalHandlesIdentify = 129,
  /// ScavengerScavengeWeakGlobalHandlesProcess
  ScavengerScavengeWeakGlobalHandlesProcess = 130,
  /// ScavengerScavengeParallel
  ScavengerScavengeParallel = 131,
  /// ScavengerScavengeParallelPhase
  ScavengerScavengeParallelPhase = 132,
  /// ScavengerScavengePinObjectsConservative
  ScavengerScavengePinObjectsConservative = 133,
  /// ScavengerScavengePinObjectsPrecise
  ScavengerScavengePinObjectsPrecise = 134,
  /// ScavengerScavengeRoots
  ScavengerScavengeRoots = 135,
  /// ScavengerScavengeStackRoots
  ScavengerScavengeStackRoots = 136,
  /// ScavengerScavengeWeak
  ScavengerScavengeWeak = 137,
  /// ScavengerScavengeFinalize
  ScavengerScavengeFinalize = 138,
  /// ScavengerScavengeRestoreAndQuarantinePinned
  ScavengerScavengeRestoreAndQuarantinePinned = 139,
  /// ScavengerSweepArrayBuffers
  ScavengerSweepArrayBuffers = 140,
  /// ScavengerTracedHandlesComputeWeaknessParallel
  ScavengerTracedHandlesComputeWeaknessParallel = 141,
  /// ScavengerTracedHandlesResetParallel
  ScavengerTracedHandlesResetParallel = 142,
  /// TimeToGlobalSafepoint
  TimeToGlobalSafepoint = 143,
  /// TimeToSafepoint
  TimeToSafepoint = 144,
  /// Unpark
  Unpark = 145,
  /// YoungArrayBufferSweep
  YoungArrayBufferSweep = 146,
  /// FullArrayBufferSweep
  FullArrayBufferSweep = 147,
  /// ConservativeStackScanning
  ConservativeStackScanning = 148,
  /// BackgroundYoungArrayBufferSweep
  BackgroundYoungArrayBufferSweep = 149,
  /// BackgroundFullArrayBufferSweep
  BackgroundFullArrayBufferSweep = 150,
  /// BackgroundCollection
  BackgroundCollection = 151,
  /// BackgroundUnpark
  BackgroundUnpark = 152,
  /// BackgroundSafepoint
  BackgroundSafepoint = 153,
  /// McBackgroundEvacuateCopy
  McBackgroundEvacuateCopy = 154,
  /// McBackgroundEvacuateUpdatePointers
  McBackgroundEvacuateUpdatePointers = 155,
  /// McBackgroundMarking
  McBackgroundMarking = 156,
  /// McBackgroundSweeping
  McBackgroundSweeping = 157,
  /// McClearStringTable
  McClearStringTable = 158,
  /// MinorMsBackgroundMarking
  MinorMsBackgroundMarking = 159,
  /// MinorMsBackgroundSweeping
  MinorMsBackgroundSweeping = 160,
  /// MinorMsBackgroundMarkingClosure
  MinorMsBackgroundMarkingClosure = 161,
  /// ScavengerBackgroundScavengeParallel
  ScavengerBackgroundScavengeParallel = 162,
  /// ScavengerBackgroundQuarantinedPageSweeping
  ScavengerBackgroundQuarantinedPageSweeping = 163,
  /// ScavengerBackgroundTracedHandlesResetParallel
  ScavengerBackgroundTracedHandlesResetParallel = 164,
  /// ScavengerBackgroundTracedHandlesComputeWeaknessParallel
  ScavengerBackgroundTracedHandlesComputeWeaknessParallel = 165,
}

impl ScopeId {
  /// 对应 NUMBER_OF_SCOPES
  pub const NUMBER_OF_SCOPES: u16 = 166;

  /// FIRST_INCREMENTAL_SCOPE = McIncremental
  pub const FIRST_INCREMENTAL_SCOPE: Self = Self::McIncremental;
  /// LAST_INCREMENTAL_SCOPE = McIncrementalSweeping
  pub const LAST_INCREMENTAL_SCOPE: Self = Self::McIncrementalSweeping;
  /// FIRST_TOP_MC_SCOPE = McClear
  pub const FIRST_TOP_MC_SCOPE: Self = Self::McClear;
  /// LAST_TOP_MC_SCOPE = McSweep
  pub const LAST_TOP_MC_SCOPE: Self = Self::McSweep;
  /// FIRST_BACKGROUND_SCOPE
  pub const FIRST_BACKGROUND_SCOPE: Self = Self::BackgroundYoungArrayBufferSweep;
  /// LAST_BACKGROUND_SCOPE
  pub const LAST_BACKGROUND_SCOPE: Self =
    Self::ScavengerBackgroundTracedHandlesComputeWeaknessParallel;
}

/// 年代回收速度模式 (对应 enum YoungGenerationSpeedMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum YoungGenerationSpeedMode {
  /// kUpToAndIncludingAtomicPause
  UpToAndIncludingAtomicPause = 0,
  /// kOnlyAtomicPause
  OnlyAtomicPause = 1,
}

/// GC 事件类型 (对应 GCTracer::Event::Type)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EventType {
  /// Scavenger
  Scavenger = 0,
  /// MarkCompactor
  MarkCompactor = 1,
  /// IncrementalMarkCompactor
  IncrementalMarkCompactor = 2,
  /// MinorMarkSweeper
  MinorMarkSweeper = 3,
  /// IncrementalMinorMarkSweeper
  IncrementalMinorMarkSweeper = 4,
  /// Start
  Start = 5,
}

/// GC 周期状态 (对应 GCTracer::Event::State; 状态图见原版注释)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EventState {
  /// NotRunning
  NotRunning = 0,
  /// Marking
  Marking = 1,
  /// Atomic
  Atomic = 2,
  /// Sweeping
  Sweeping = 3,
}

/// 增量模式 (对应 GCTracer::Mode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Mode {
  /// None
  None = 0,
  /// Scavenger
  Scavenger = 1,
  /// Finalize
  Finalize = 2,
}

/// 标记方式 (对应 GCTracer::MarkingType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MarkingType {
  /// kAtomic
  Atomic = 0,
  /// kIncremental
  Incremental = 1,
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(ScopeId::NUMBER_OF_SCOPES == 166);
  assert!(ScopeId::FIRST_BACKGROUND_SCOPE as u16 == 149);
  assert!(ScopeId::LAST_BACKGROUND_SCOPE as u16 == ScopeId::NUMBER_OF_SCOPES - 1);
  assert!(ScopeId::McIncrementalSweeping as u16 > ScopeId::McIncremental as u16);
  assert!(YoungGenerationSpeedMode::UpToAndIncludingAtomicPause as u8 == 0);
  assert!(EventType::Scavenger as u8 == 0);
  assert!(EventType::Start as u8 == 5);
  assert!(EventState::Sweeping as u8 == 3);
  assert!(Mode::Finalize as u8 == 2);
  assert!(MarkingType::Incremental as u8 == 1);
};

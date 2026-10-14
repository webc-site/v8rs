# v8-trap-handler

Wasm OOB trap handler: out-of-bounds wasm access raises SIGSEGV/SIGBUS ->
the signal handler intercepts it and rewrites the ucontext IP to a landing pad.

The posix subset is complete; Windows and simulator paths are deferred.

## Source mapping

| v8/src/trap-handler/ | this crate | conversion script |
|---|---|---|
| trap-handler.h | src/lib.rs | tool/src/trap-handler/trap-handler.h.js |
| trap-handler-internal.h | src/internal.rs | tool/src/trap-handler/trap-handler-internal.h.js |
| handler-shared.cc | src/shared.rs | tool/src/trap-handler/handler-shared.cc.js |
| handler-inside.cc | src/inside.rs | tool/src/trap-handler/handler-inside.cc.js |
| handler-inside-posix.h + .cc | src/inside_posix.rs | tool/src/trap-handler/handler-inside-posix.js |
| handler-outside.cc | src/outside.rs | tool/src/trap-handler/handler-outside.cc.js |
| handler-outside-posix.cc | src/outside_posix.rs | tool/src/trap-handler/handler-outside-posix.cc.js |
| (error hub, no upstream file) | src/error.rs | hand-written |

Note: handler-outside.h listed in the task does not exist in this v8 checkout;
the 8th file of the posix path is handler-inside.cc (platform-independent
IsFaultAddressCovered/IsAccessedMemoryCovered).

## Deferred

- handler-inside-win.* / handler-outside-win.cc (Windows)
- handler-outside-simulator.cc / trap-handler-simulator.h (simulator)
- full native-unittest with JIT code injection (real-fault integration
  tests exist: tests/signal.rs covers landing-pad recovery and fallback
  restore+replay)
- sanitizer install-detection (handler-outside-posix.cc:65-83)
- linux-side synthetic-siginfo ucontext rewrite unit test (libc siginfo
  union fields are not writable; linux covered by real-fault path)

## Semantic deviations

- thread_in_wasm flag: removed upstream; nesting semantics carried by
  TrapHandlerGuard (TLS), translated 1:1
- malloc-failure paths: allocation failure aborts in Rust, no false return
- EnableTrapHandler twice: IMMEDIATE_CRASH -> Err(Error::EnableTwice)
- is_fault_address_covered / is_accessed_memory_covered exposed as pub
  queries (self-guarded to satisfy the lock contract)
- g_old_handler: plain global upstream (racy but deadlock-free) ->
  Locked::lock_outside spinlock; remove_trap_handler runs inside the
  signal handler and takes this lock, so the lock window blocks
  kOobSignal via pthread_sigmask (outside_posix::with_old_handler)
- ReleaseHandlerData negative index: upstream release build is negative
  subscript UB -> debug crash (matches TH_DCHECK) + release early return
- gRecoveredTrapCount hit counting: upstream load+store (two-step, racy)
  -> fetch_add (lock-free, async-signal-safe, no lost updates)
- kEnableSlowChecks: ValidateCodeObjects free-list consistency check
  landed as debug_assert (hooked in register/release); per-offset >= 0
  check dropped (u32 is always true)

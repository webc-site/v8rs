#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod code_space_access_cc;
mod error;
mod wasm_objects_cc;

pub use code_space_access_cc::{CODE_SPACE_WRITE_REASON, CodeSpaceWriteScope};
pub use error::{Error, Result};
pub use wasm_objects_cc::{
  BASES_AND_SIZES_LAYOUT, CODE_POINTER_WIDTH, CallTargetSource, DispatchEntry,
  DispatchNewOrExisting, DispatchSetWrite, DispatchTableGrow, DispatchTableModel, EntryWrite,
  FuncRefSlot, GROW_FAIL_PATHS, GROW_REPORT_ROUNDING, GROW_SUCCESS_RETURNS, GlobalStorageSel,
  GlobalValueRead, INL_ACCESSOR_MACROS, ImplicitArgSource, InitTableEntriesError,
  K_BYTES_PER_EXCEPTION_VALUES_ARRAY_ELEMENT, K_GB_PAGES, K_GB_PAGES_FORMULA,
  K_MIN_NUM_OFFSET_FROM_TAGGED_BUFFER, K_MIN_REF_OFFSET_FROM_TAGGED_BUFFER, K_NO_MAXIMUM,
  MEMORY_RESERVE_HEURISTIC, MUST_GROW_IN_PLACE, MemoryChangeResizabilityStep, MemoryGrowOutcome,
  MemoryGrowParams, MemorySetNewBufferPlan, SetFunctionEntryOp, SetOp, SetRawMemoryParams,
  SetWrapperParams, TABLE_GROW_FAIL_PATHS, TableCopyOp, TableGetOp, TableGrowOutcome, TableNewPlan,
  TablePlaceholder, TableSetOp, USES_ENTRY_STEP, USES_INITIAL_LENGTH, USES_RESERVED_SLOT_OFFSET,
  UpdateMultipleKind, UseEntry, WasmWrapperMap, WrapperHandle, array_getelement,
  array_settaggedelement, capifunction_iswasmcapifunction, capifunction_matchessignature,
  capifunction_new, capifunction_sig, custommap_allocateuninitialized, custommap_createjswrapper,
  custommap_getfieldvalue, decode_i32_exception_value, decode_i64_exception_value,
  dispatchtable_adduse, dispatchtable_clear, dispatchtable_grow, dispatchtable_grow_amount,
  dispatchtable_maybegrowuseslist, dispatchtable_setfornonwrapper, dispatchtable_setforwrapper,
  dispatchtable_uses_min_free_entries, dispatchtable_uses_new_capacity, dispatchtabledata_add,
  dispatchtabledata_remove, dispatchtabledata_wrappercodepointerfordebugging,
  dispatchtableforimports_clear, dispatchtableforimports_setfornonwrapper,
  dispatchtableforimports_setforwrapper, encode_i32_exception_value, encode_i64_exception_value,
  exceptionpackage_getencodedsize, exceptionpackage_getencodedsize_for_tag,
  exceptionpackage_getexceptiontag, exceptionpackage_getexceptionvalues, exceptionpackage_new,
  exceptionpackage_new_with_size, exceptiontag_new, exportedfunction_getwrapper,
  exportedfunction_iswasmexportedfunction, exportedfunction_new, exportedfunction_new_full,
  externalfunction_iswasmexternalfunction, function_sig_matches_table, globalobject_new,
  internalfunction_getorcreateexternal, internalfunction_try_get_external,
  memoryobject_changearraybufferresizability, memoryobject_fixupresizablearraybuffer,
  memoryobject_getarraybuffer, memoryobject_grow, memoryobject_grow_exceeds, memoryobject_max_size,
  memoryobject_must_grow_in_place, memoryobject_new_from_pages, memoryobject_new_reserved_pages,
  memoryobject_new_with_backing_store, memoryobject_refreshbuffer, memoryobject_setnewbuffer,
  memoryobject_updateinstances, memoryobject_useininstance, min_growth_pages,
  moduleobject_extractutf8stringfrommodulebytes, moduleobject_getfunctionnameornull,
  moduleobject_getmodulenameornull, moduleobject_getrawfunctionname, moduleobject_new,
  new_capacity_pages, struct_allocatedescriptoruninitialized, struct_gcsafetype,
  struct_getfieldvalue, suspendingobject_new, tableobject_entries_grow_delta, tableobject_fill,
  tableobject_get, tableobject_grow, tableobject_grow_exceeds, tableobject_grow_max_size,
  tableobject_jstowasmelement, tableobject_new, tableobject_set, tableobject_setfunctiontableentry,
  tableobject_setfunctiontableplaceholder, tableobject_updatedispatchtable,
  tableobject_updatedispatchtable_batch, tagobject_matchessignature, tagobject_new,
  trustedinstancedata_copytableentries, trustedinstancedata_getcalltarget,
  trustedinstancedata_getglobalstorage, trustedinstancedata_getglobalvalue,
  trustedinstancedata_getorcreatefuncref, trustedinstancedata_initdatasegmentarrays,
  trustedinstancedata_inittableentries, trustedinstancedata_new, trustedinstancedata_setrawmemory,
  trustedinstancedata_try_get_func_ref,
};

// v8-objects-js-core/src/lib.rs

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod error;
pub mod field_index;
pub mod js_arguments;
pub mod js_array;
pub mod js_object;
pub mod js_receiver;

pub use error::{Error, Result};
pub use field_index::{FieldEncoding, FieldIndex};
pub use js_arguments::{
  AliasedArgumentsEntry, ArgumentLocation, JSArgumentsObject, JSSloppyArgumentsObject,
  JSStrictArgumentsObject, K_THE_HOLE_SENTINEL, SloppyArgumentsElements,
  SloppyArgumentsElementsBuf, SloppyArgumentsElementsExt,
};
pub use js_array::{JSArray, JSArrayBuf, JSArrayExt};
pub use js_object::{
  JSObject, JSObjectBuf, JSObjectExt, K_DICTIONARY_ELEMENTS_KIND, K_SHARED_ARRAY_ELEMENTS_KIND,
};
pub use js_receiver::{JSReceiver, JSReceiverExt};

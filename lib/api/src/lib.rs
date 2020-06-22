//! Wasmer API
#![deny(intra_doc_link_resolution_failure)]

mod exports;
mod externals;
mod import_object;
mod instance;
mod memory;
mod memory_view;
mod module;
mod native;
mod ordered_resolver;
mod ptr;
mod store;
mod table;
mod tunables;
mod types;
mod utils;

pub use crate::exports::{ExportError, Exportable, Exports};
pub use crate::externals::{
    Extern, Function, Global, HostFunction, Memory, Table, WasmExternType, WasmTypeList,
};
pub use crate::import_object::{ImportObject, ImportObjectIterator, LikeNamespace};
pub use crate::instance::Instance;
pub use crate::memory_view::{Atomically, MemoryView};
pub use crate::module::Module;
pub use crate::native::NativeFunc;
pub use crate::ordered_resolver::OrderedResolver;
pub use crate::ptr::{Array, Item, WasmPtr};
pub use crate::store::{Store, StoreObject};
pub use crate::tunables::Tunables;
pub use crate::types::{
    ExportType, ExternRef, ExternType, FunctionType, GlobalType, HostInfo, HostRef, ImportType,
    MemoryType, Mutability, TableType, Val, ValType,
};
pub use crate::types::{Val as Value, ValType as Type};
pub use crate::utils::is_wasm;
pub use target_lexicon::{Architecture, CallingConvention, OperatingSystem, Triple, HOST};
pub use wasm_common::{
    Bytes, GlobalInit, LocalFunctionIndex, Pages, ValueType, WASM_MAX_PAGES, WASM_MIN_PAGES,
    WASM_PAGE_SIZE,
};
#[cfg(feature = "compiler")]
pub use wasmer_compiler::CompilerConfig;
pub use wasmer_compiler::{
    wasmparser, CpuFeature, Features, FunctionMiddleware, FunctionMiddlewareGenerator,
    MiddlewareReaderState, Target,
};
pub use wasmer_engine::{
    ChainableNamedResolver, DeserializeError, Engine, InstantiationError, LinkError, NamedResolver,
    NamedResolverChain, Resolver, RuntimeError, SerializeError,
};
pub use wasmer_runtime::{raise_user_trap, MemoryError};
#[cfg(feature = "wat")]
pub use wat::parse_bytes as wat2wasm;

// The compilers are mutually exclusive
#[cfg(any(
    all(feature = "llvm", any(feature = "cranelift", feature = "singlepass")),
    all(feature = "cranelift", feature = "singlepass")
))]
compile_error!(
    r#"The `singlepass`, `cranelift` and `llvm` features are mutually exclusive.
If you wish to use more than one compiler, you can simply import it from it's own crate. Eg.:

```
use wasmer::{Store, Engine};
use wasmer_compiler_singlepass::SinglepassConfig;

// TODO: update this, this is now out of date:
let engine = Engine::new(SinglepassConfig::default());
let store = Store::new_config(&engine);
```"#
);

#[cfg(feature = "singlepass")]
pub use wasmer_compiler_singlepass::SinglepassConfig;

#[cfg(feature = "cranelift")]
pub use wasmer_compiler_cranelift::CraneliftConfig;

#[cfg(feature = "llvm")]
pub use wasmer_compiler_llvm::LLVMConfig;

#[cfg(feature = "jit")]
pub use wasmer_engine_jit::JITEngine;

#[cfg(feature = "native")]
pub use wasmer_engine_native::NativeEngine;

/// Version number of this crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

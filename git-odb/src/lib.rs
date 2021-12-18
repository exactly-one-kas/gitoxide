#![deny(missing_docs, unsafe_code, rust_2018_idioms)]

//! Git stores all of its data as _Objects_, which are data along with a hash over all data. Thus it's an
//! object store indexed by the signature of data itself with inherent deduplication: the same data will have the same hash,
//! and thus occupy the same space within the store.
//!
//! There is only one all-round object store, also known as the [`Store`], as it supports ~~everything~~ most of what git has to offer.
//!
//! * loose object reading and writing
//! * access to packed objects
//! * multiple loose objects and pack locations as gathered from `alternates` files.
// TODO: actually remove the deprecated items and remove thos allow
#![allow(deprecated)]

use std::cell::RefCell;
use std::path::PathBuf;
use std::sync::Arc;

use git_features::threading::OwnShared;
use git_features::zlib::stream::deflate;
pub use git_pack as pack;

mod store;
pub use store::{compound, dynamic, linked, loose, RefreshMode};

pub mod alternate;

/// A way to access objects along with pre-configured thread-local caches for packed base objects as well as objects themselves.
///
/// By default, no cache will be used.
pub struct Cache<S> {
    /// The inner provider of trait implementations we use in conjunction with our caches.
    pub inner: S,
    // TODO: have single-threaded code-paths also for pack-creation (entries from counts) so that we can use OwnShared here
    //       instead of Arc. However, it's probably not that important as these aren't called often.
    new_pack_cache: Option<Arc<cache::NewPackCacheFn>>,
    new_object_cache: Option<Arc<cache::NewObjectCacheFn>>,
    pack_cache: Option<RefCell<Box<cache::PackCache>>>,
    object_cache: Option<RefCell<Box<cache::ObjectCache>>>,
}

///
pub mod cache;

///
/// It can optionally compress the content, similarly to what would happen when using a [`loose::Store`][crate::store::loose::Store].
///
pub struct Sink {
    compressor: Option<RefCell<deflate::Write<std::io::Sink>>>,
}

/// Create a new [`Sink`] with compression disabled.
pub fn sink() -> Sink {
    Sink { compressor: None }
}

///
pub mod sink;

///
pub mod find;

/// An object database equivalent to `/dev/null`, dropping all objects stored into it.
mod traits;

pub use traits::{Find, FindExt, Write};

/// A thread-local handle to access any object.
pub type Handle = Cache<dynamic::Handle<OwnShared<dynamic::Store>>>;
/// A thread-local handle to access any object, but thread-safe and independent of the actual type of `OwnShared` or feature toggles in `git-features`.
pub type HandleArc = Cache<dynamic::Handle<Arc<dynamic::Store>>>;

/// A thread-safe store for creation of handles.
pub type Store = dynamic::Store;

/// Create a new cached odb handle with support for additional options.
pub fn at_opts(objects_dir: impl Into<PathBuf>, slots: dynamic::init::Slots) -> std::io::Result<Handle> {
    let handle =
        OwnShared::new(dynamic::Store::at_opts(objects_dir, slots)?).to_handle(RefreshMode::AfterAllIndicesLoaded);
    Ok(Cache::from(handle))
}

/// Create a new cached odb handle.
pub fn at(objects_dir: impl Into<PathBuf>) -> std::io::Result<Handle> {
    at_opts(objects_dir, Default::default())
}

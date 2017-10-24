/// C++ type: <span style='color: green;'>```QPixmapCache::Key```</span>
#[repr(C)]
pub struct Key([u8; ::type_sizes::QT_GUI_PIXMAP_CACHE_KEY]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Key {
  unsafe fn new_uninitialized() -> Key {
    Key(::std::mem::uninitialized())
  }
}

impl Key {
  /// C++ method: <span style='color: green;'>```bool QPixmapCache::Key::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPixmapCache_Key_isValid(self as *const ::pixmap_cache::Key) }
  }

  /// C++ method: <span style='color: green;'>```QPixmapCache::Key::Key```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::pixmap_cache::Key```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPixmapCache::Key::Key()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::pixmap_cache::Key) -> ::pixmap_cache::Key```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPixmapCache::Key::Key(const QPixmapCache::Key& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::pixmap_cache::Key
    where Args: overloading::KeyNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPixmapCache::Key& QPixmapCache::Key::operator=(const QPixmapCache::Key& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, other: &'l1 ::pixmap_cache::Key) -> &'l0 mut ::pixmap_cache::Key {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QPixmapCache_Key_operator_assign(self as *mut ::pixmap_cache::Key,
                                                       other as *const ::pixmap_cache::Key)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QPixmapCache::Key::operator==(const QPixmapCache::Key& key) const```</span>
  ///
  ///
  pub fn op_eq(&self, key: &::pixmap_cache::Key) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QPixmapCache_Key_operator_eq(self as *const ::pixmap_cache::Key,
                                                   key as *const ::pixmap_cache::Key)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPixmapCache::Key::operator!=(const QPixmapCache::Key& key) const```</span>
  ///
  ///
  pub fn op_neq(&self, key: &::pixmap_cache::Key) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QPixmapCache_Key_operator_neq(self as *const ::pixmap_cache::Key,
                                                    key as *const ::pixmap_cache::Key)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPixmapCache::Key::swap(QPixmapCache::Key& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::pixmap_cache::Key) {
    unsafe {
      ::ffi::qt_gui_c_QPixmapCache_Key_swap(self as *mut ::pixmap_cache::Key,
                                            other as *mut ::pixmap_cache::Key)
    }
  }
}

impl Drop for ::pixmap_cache::Key {
  /// C++ method: <span style='color: green;'>```[destructor] void QPixmapCache::Key::~Key()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPixmapCache_Key_destructor(self as *mut ::pixmap_cache::Key) }
  }
}

/// C++ type: <span style='color: green;'>```QPixmapCache```</span>
#[repr(C)]
pub struct PixmapCache(u8);

impl PixmapCache {
  /// C++ method: <span style='color: green;'>```static int QPixmapCache::cacheLimit()```</span>
  ///
  ///
  pub fn cache_limit() -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QPixmapCache_cacheLimit() }
  }

  /// C++ method: <span style='color: green;'>```static void QPixmapCache::clear()```</span>
  ///
  ///
  pub fn clear() {
    unsafe { ::ffi::qt_gui_c_QPixmapCache_clear() }
  }

  /// C++ method: <span style='color: green;'>```QPixmapCache::find```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn find(&::qt_core::string::String) -> *mut ::pixmap::Pixmap```<br>
  /// C++ method: <span style='color: green;'>```static QPixmap* QPixmapCache::find(const QString& key)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn find((&::qt_core::string::String, &mut ::pixmap::Pixmap)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QPixmapCache::find(const QString& key, QPixmap& pixmap)```</span>
  ///
  ///
  pub fn find<Args>(args: Args) -> Args::ReturnType
    where Args: overloading::PixmapCacheFindArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPixmapCache::find```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn find_unsafe((&::pixmap_cache::Key, *mut ::pixmap::Pixmap)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QPixmapCache::find(const QPixmapCache::Key& key, QPixmap* pixmap)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn find_unsafe((&::qt_core::string::String, *mut ::pixmap::Pixmap)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QPixmapCache::find(const QString& key, QPixmap* pixmap)```</span>
  ///
  ///
  pub unsafe fn find_unsafe<Args>(args: Args) -> bool
    where Args: overloading::PixmapCacheFindUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPixmapCache::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&::pixmap::Pixmap) -> ::pixmap_cache::Key```<br>
  /// C++ method: <span style='color: green;'>```static QPixmapCache::Key QPixmapCache::insert(const QPixmap& pixmap)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert((&::qt_core::string::String, &::pixmap::Pixmap)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QPixmapCache::insert(const QString& key, const QPixmap& pixmap)```</span>
  ///
  ///
  pub fn insert<Args>(args: Args) -> Args::ReturnType
    where Args: overloading::PixmapCacheInsertArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPixmapCache::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&::pixmap_cache::Key) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QPixmapCache::remove(const QPixmapCache::Key& key)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&::qt_core::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QPixmapCache::remove(const QString& key)```</span>
  ///
  ///
  pub fn remove<Args>(args: Args) -> ()
    where Args: overloading::PixmapCacheRemoveArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static bool QPixmapCache::replace(const QPixmapCache::Key& key, const QPixmap& pixmap)```</span>
  ///
  ///
  pub fn replace(key: &::pixmap_cache::Key, pixmap: &::pixmap::Pixmap) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QPixmapCache_replace(key as *const ::pixmap_cache::Key,
                                           pixmap as *const ::pixmap::Pixmap)
    }
  }

  /// C++ method: <span style='color: green;'>```static void QPixmapCache::setCacheLimit(int arg1)```</span>
  ///
  ///
  pub fn set_cache_limit(arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QPixmapCache_setCacheLimit(arg1) }
  }
}

impl ::cpp_utils::CppDeletable for ::pixmap_cache::PixmapCache {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QPixmapCache_delete
  }
}

/// C++ method: <span style='color: green;'>```void swap(QPixmapCache::Key& value1, QPixmapCache::Key& value2)```</span>
///
///
pub fn swap(value1: &mut ::pixmap_cache::Key, value2: &mut ::pixmap_cache::Key) {
  unsafe {
    ::ffi::qt_gui_c_QPixmapCache_G_swap(value1 as *mut ::pixmap_cache::Key,
                                        value2 as *mut ::pixmap_cache::Key)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Key::new](../struct.Key.html#method.new) method.
  pub trait KeyNewArgs {
    fn exec(self) -> ::pixmap_cache::Key;
  }
  impl KeyNewArgs for () {
    fn exec(self) -> ::pixmap_cache::Key {

      {
        let mut object: ::pixmap_cache::Key =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixmapCache_Key_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> KeyNewArgs for &'a ::pixmap_cache::Key {
    fn exec(self) -> ::pixmap_cache::Key {
      let other = self;
      {
        let mut object: ::pixmap_cache::Key =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixmapCache_Key_constructor_other(other as *const ::pixmap_cache::Key, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PixmapCache::find](../struct.PixmapCache.html#method.find) method.
  pub trait PixmapCacheFindArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> PixmapCacheFindArgs for &'a ::qt_core::string::String {
    type ReturnType = *mut ::pixmap::Pixmap;
    fn exec(self) -> *mut ::pixmap::Pixmap {
      let key = self;
      unsafe { ::ffi::qt_gui_c_QPixmapCache_find_const_QString_ref(key as *const ::qt_core::string::String) }
    }
  }
  impl<'a> PixmapCacheFindArgs for (&'a ::qt_core::string::String, &'a mut ::pixmap::Pixmap) {
    type ReturnType = bool;
    fn exec(self) -> bool {
      let key = self.0;
      let pixmap = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPixmapCache_find_const_QString_ref_QPixmap_ref(key as *const ::qt_core::string::String,
                                                                        pixmap as *mut ::pixmap::Pixmap)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PixmapCache::find_unsafe](../struct.PixmapCache.html#method.find_unsafe) method.
  pub trait PixmapCacheFindUnsafeArgs {
    unsafe fn exec(self) -> bool;
  }
  impl<'a> PixmapCacheFindUnsafeArgs for (&'a ::pixmap_cache::Key, *mut ::pixmap::Pixmap) {
    unsafe fn exec(self) -> bool {
      let key = self.0;
      let pixmap = self.1;
      ::ffi::qt_gui_c_QPixmapCache_find_const_QPixmapCache_Key_ref_QPixmap_ptr(key as *const ::pixmap_cache::Key,
                                                                               pixmap)
    }
  }
  impl<'a> PixmapCacheFindUnsafeArgs for (&'a ::qt_core::string::String, *mut ::pixmap::Pixmap) {
    unsafe fn exec(self) -> bool {
      let key = self.0;
      let pixmap = self.1;
      ::ffi::qt_gui_c_QPixmapCache_find_const_QString_ref_QPixmap_ptr(key as *const ::qt_core::string::String, pixmap)
    }
  }
  /// This trait represents a set of arguments accepted by [PixmapCache::insert](../struct.PixmapCache.html#method.insert) method.
  pub trait PixmapCacheInsertArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> PixmapCacheInsertArgs for (&'a ::qt_core::string::String, &'a ::pixmap::Pixmap) {
    type ReturnType = bool;
    fn exec(self) -> bool {
      let key = self.0;
      let pixmap = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPixmapCache_insert(key as *const ::qt_core::string::String,
                                            pixmap as *const ::pixmap::Pixmap)
      }
    }
  }
  impl<'a> PixmapCacheInsertArgs for &'a ::pixmap::Pixmap {
    type ReturnType = ::pixmap_cache::Key;
    fn exec(self) -> ::pixmap_cache::Key {
      let pixmap = self;
      {
        let mut object: ::pixmap_cache::Key =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixmapCache_insert_to_output(pixmap as *const ::pixmap::Pixmap, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PixmapCache::remove](../struct.PixmapCache.html#method.remove) method.
  pub trait PixmapCacheRemoveArgs {
    fn exec(self) -> ();
  }
  impl<'a> PixmapCacheRemoveArgs for &'a ::pixmap_cache::Key {
    fn exec(self) -> () {
      let key = self;
      unsafe { ::ffi::qt_gui_c_QPixmapCache_remove_QPixmapCache_Key(key as *const ::pixmap_cache::Key) }
    }
  }
  impl<'a> PixmapCacheRemoveArgs for &'a ::qt_core::string::String {
    fn exec(self) -> () {
      let key = self;
      unsafe { ::ffi::qt_gui_c_QPixmapCache_remove_QString(key as *const ::qt_core::string::String) }
    }
  }
}

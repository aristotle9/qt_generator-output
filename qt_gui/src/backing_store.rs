/// C++ type: <span style='color: green;'>```QBackingStore```</span>
#[repr(C)]
pub struct BackingStore(u8);

impl BackingStore {
  /// C++ method: <span style='color: green;'>```void QBackingStore::beginPaint(const QRegion& arg1)```</span>
  ///
  ///
  pub fn begin_paint(&mut self, arg1: &::region::Region) {
    unsafe {
      ::ffi::qt_gui_c_QBackingStore_beginPaint(self as *mut ::backing_store::BackingStore,
                                               arg1 as *const ::region::Region)
    }
  }

  /// C++ method: <span style='color: green;'>```void QBackingStore::endPaint()```</span>
  ///
  ///
  pub fn end_paint(&mut self) {
    unsafe { ::ffi::qt_gui_c_QBackingStore_endPaint(self as *mut ::backing_store::BackingStore) }
  }

  /// C++ method: <span style='color: green;'>```void QBackingStore::flush(const QRegion& region)```</span>
  ///
  ///
  pub fn flush(&mut self, region: &::region::Region) {
    unsafe {
      ::ffi::qt_gui_c_QBackingStore_flush_region(self as *mut ::backing_store::BackingStore,
                                                 region as *const ::region::Region)
    }
  }

  /// C++ method: <span style='color: green;'>```QBackingStore::flush```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn flush_unsafe(&mut self, (&::region::Region, *mut ::window::Window)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QBackingStore::flush(const QRegion& region, QWindow* window = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn flush_unsafe(&mut self, (&::region::Region, *mut ::window::Window, &::qt_core::point::Point)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QBackingStore::flush(const QRegion& region, QWindow* window = ?, const QPoint& offset = ?)```</span>
  ///
  ///
  pub unsafe fn flush_unsafe<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::BackingStoreFlushUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QBackingStore::hasStaticContents() const```</span>
  ///
  ///
  pub fn has_static_contents(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QBackingStore_hasStaticContents(self as *const ::backing_store::BackingStore) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QBackingStore::QBackingStore(QWindow* window)```</span>
  ///
  ///
  pub unsafe fn new(window: *mut ::window::Window) -> ::cpp_utils::CppBox<::backing_store::BackingStore> {
    let ffi_result = ::ffi::qt_gui_c_QBackingStore_new(window);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QPaintDevice* QBackingStore::paintDevice()```</span>
  ///
  ///
  pub fn paint_device(&mut self) -> *mut ::paint_device::PaintDevice {
    unsafe { ::ffi::qt_gui_c_QBackingStore_paintDevice(self as *mut ::backing_store::BackingStore) }
  }

  /// C++ method: <span style='color: green;'>```void QBackingStore::resize(const QSize& size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_gui_c_QBackingStore_resize(self as *mut ::backing_store::BackingStore,
                                           size as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QBackingStore::scroll(const QRegion& area, int dx, int dy)```</span>
  ///
  ///
  pub fn scroll(&mut self, area: &::region::Region, dx: ::libc::c_int, dy: ::libc::c_int) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QBackingStore_scroll(self as *mut ::backing_store::BackingStore,
                                           area as *const ::region::Region,
                                           dx,
                                           dy)
    }
  }

  /// C++ method: <span style='color: green;'>```void QBackingStore::setStaticContents(const QRegion& region)```</span>
  ///
  ///
  pub fn set_static_contents(&mut self, region: &::region::Region) {
    unsafe {
      ::ffi::qt_gui_c_QBackingStore_setStaticContents(self as *mut ::backing_store::BackingStore,
                                                      region as *const ::region::Region)
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QBackingStore::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QBackingStore_size_to_output(self as *const ::backing_store::BackingStore, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRegion QBackingStore::staticContents() const```</span>
  ///
  ///
  pub fn static_contents(&self) -> ::cpp_utils::CppBox<::region::Region> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QBackingStore_staticContents_as_ptr(self as *const ::backing_store::BackingStore) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QWindow* QBackingStore::window() const```</span>
  ///
  ///
  pub fn window(&self) -> *mut ::window::Window {
    unsafe { ::ffi::qt_gui_c_QBackingStore_window(self as *const ::backing_store::BackingStore) }
  }
}

impl ::cpp_utils::CppDeletable for ::backing_store::BackingStore {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QBackingStore_delete
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [BackingStore::flush_unsafe](../struct.BackingStore.html#method.flush_unsafe) method.
  pub trait BackingStoreFlushUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::backing_store::BackingStore) -> ();
  }
  impl<'largs> BackingStoreFlushUnsafeArgs<'largs> for (&'largs ::region::Region, *mut ::window::Window) {
    unsafe fn exec(self, original_self: &'largs mut ::backing_store::BackingStore) -> () {
      let region = self.0;
      let window = self.1;
      ::ffi::qt_gui_c_QBackingStore_flush_region_window(original_self as *mut ::backing_store::BackingStore,
                                                        region as *const ::region::Region,
                                                        window)
    }
  }
  impl<'largs> BackingStoreFlushUnsafeArgs<'largs>
    for (&'largs ::region::Region, *mut ::window::Window, &'largs ::qt_core::point::Point) {
    unsafe fn exec(self, original_self: &'largs mut ::backing_store::BackingStore) -> () {
      let region = self.0;
      let window = self.1;
      let offset = self.2;
      ::ffi::qt_gui_c_QBackingStore_flush_region_window_offset(original_self as *mut ::backing_store::BackingStore,
                                                               region as *const ::region::Region,
                                                               window,
                                                               offset as *const ::qt_core::point::Point)
    }
  }
}

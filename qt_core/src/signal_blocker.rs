/// C++ type: <span style='color: green;'>```QSignalBlocker```</span>
#[repr(C)]
pub struct SignalBlocker([u8; ::type_sizes::QT_CORE_SIGNAL_BLOCKER_SIGNAL_BLOCKER]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for SignalBlocker {
  unsafe fn new_uninitialized() -> SignalBlocker {
    SignalBlocker(::std::mem::uninitialized())
  }
}

impl SignalBlocker {
  /// C++ method: <span style='color: green;'>```[constructor] void QSignalBlocker::QSignalBlocker(QObject& o)```</span>
  ///
  ///
  pub fn new(o: &mut ::object::Object) -> ::signal_blocker::SignalBlocker {
    {
      let mut object: ::signal_blocker::SignalBlocker =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSignalBlocker_constructor_QObject_ref(o as *mut ::object::Object, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QSignalBlocker::QSignalBlocker(QObject* o)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(o: *mut ::object::Object) -> ::signal_blocker::SignalBlocker {
    {
      let mut object: ::signal_blocker::SignalBlocker =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QSignalBlocker_constructor_QObject_ptr(o, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QSignalBlocker::reblock()```</span>
  ///
  ///
  pub fn reblock(&mut self) {
    unsafe { ::ffi::qt_core_c_QSignalBlocker_reblock(self as *mut ::signal_blocker::SignalBlocker) }
  }

  /// C++ method: <span style='color: green;'>```void QSignalBlocker::unblock()```</span>
  ///
  ///
  pub fn unblock(&mut self) {
    unsafe { ::ffi::qt_core_c_QSignalBlocker_unblock(self as *mut ::signal_blocker::SignalBlocker) }
  }
}

impl Drop for ::signal_blocker::SignalBlocker {
  /// C++ method: <span style='color: green;'>```[destructor] void QSignalBlocker::~QSignalBlocker()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QSignalBlocker_destructor(self as *mut ::signal_blocker::SignalBlocker) }
  }
}

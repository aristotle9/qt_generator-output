/// C++ type: <span style='color: green;'>```QDebugStateSaver```</span>
#[repr(C)]
pub struct DebugStateSaver([u8; ::type_sizes::QT_CORE_DEBUG_STATE_SAVER_DEBUG_STATE_SAVER]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for DebugStateSaver {
  unsafe fn new_uninitialized() -> DebugStateSaver {
    DebugStateSaver(::std::mem::uninitialized())
  }
}

impl DebugStateSaver {
  /// C++ method: <span style='color: green;'>```[constructor] void QDebugStateSaver::QDebugStateSaver(QDebug& dbg)```</span>
  ///
  ///
  pub fn new(dbg: &mut ::debug::Debug) -> ::debug_state_saver::DebugStateSaver {
    {
      let mut object: ::debug_state_saver::DebugStateSaver =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDebugStateSaver_constructor(dbg as *mut ::debug::Debug, &mut object);
      }
      object
    }
  }
}

impl Drop for ::debug_state_saver::DebugStateSaver {
  /// C++ method: <span style='color: green;'>```[destructor] void QDebugStateSaver::~QDebugStateSaver()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QDebugStateSaver_destructor(self as *mut ::debug_state_saver::DebugStateSaver) }
  }
}

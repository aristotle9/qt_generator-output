/// C++ type: <span style='color: green;'>```QTextStreamManipulator```</span>
#[repr(C)]
pub struct TextStreamManipulator([u8; ::type_sizes::QT_CORE_TEXT_STREAM_MANIPULATOR_TEXT_STREAM_MANIPULATOR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TextStreamManipulator {
  unsafe fn new_uninitialized() -> TextStreamManipulator {
    TextStreamManipulator(::std::mem::uninitialized())
  }
}

impl TextStreamManipulator {
  /// C++ method: <span style='color: green;'>```void QTextStreamManipulator::exec(QTextStream& s)```</span>
  ///
  ///
  pub fn exec(&mut self, s: &mut ::text_stream::TextStream) {
    unsafe {
      ::ffi::qt_core_c_QTextStreamManipulator_exec(self as *mut ::text_stream_manipulator::TextStreamManipulator,
                                                   s as *mut ::text_stream::TextStream)
    }
  }
}

impl Drop for ::text_stream_manipulator::TextStreamManipulator {
  /// C++ method: <span style='color: green;'>```[destructor] void QTextStreamManipulator::~QTextStreamManipulator()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_core_c_QTextStreamManipulator_destructor(self as *mut ::text_stream_manipulator::TextStreamManipulator)
    }
  }
}

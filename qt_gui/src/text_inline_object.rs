/// C++ type: <span style='color: green;'>```QTextInlineObject```</span>
#[repr(C)]
pub struct TextInlineObject([u8; ::type_sizes::QT_GUI_TEXT_INLINE_OBJECT_TEXT_INLINE_OBJECT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TextInlineObject {
  unsafe fn new_uninitialized() -> TextInlineObject {
    TextInlineObject(::std::mem::uninitialized())
  }
}

impl TextInlineObject {
  /// C++ method: <span style='color: green;'>```double QTextInlineObject::ascent() const```</span>
  ///
  ///
  pub fn ascent(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextInlineObject_ascent(self as *const ::text_inline_object::TextInlineObject) }
  }

  /// C++ method: <span style='color: green;'>```double QTextInlineObject::descent() const```</span>
  ///
  ///
  pub fn descent(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextInlineObject_descent(self as *const ::text_inline_object::TextInlineObject) }
  }

  /// C++ method: <span style='color: green;'>```QTextFormat QTextInlineObject::format() const```</span>
  ///
  ///
  pub fn format(&self) -> ::text_format::TextFormat {
    {
      let mut object: ::text_format::TextFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextInlineObject_format_to_output(self as *const ::text_inline_object::TextInlineObject,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextInlineObject::formatIndex() const```</span>
  ///
  ///
  pub fn format_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextInlineObject_formatIndex(self as *const ::text_inline_object::TextInlineObject) }
  }

  /// C++ method: <span style='color: green;'>```double QTextInlineObject::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextInlineObject_height(self as *const ::text_inline_object::TextInlineObject) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextInlineObject::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextInlineObject_isValid(self as *const ::text_inline_object::TextInlineObject) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTextInlineObject::QTextInlineObject()```</span>
  ///
  ///
  pub fn new() -> ::text_inline_object::TextInlineObject {
    {
      let mut object: ::text_inline_object::TextInlineObject =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextInlineObject_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF QTextInlineObject::rect() const```</span>
  ///
  ///
  pub fn rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextInlineObject_rect_to_output(self as *const ::text_inline_object::TextInlineObject,
                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextInlineObject::setAscent(double a)```</span>
  ///
  ///
  pub fn set_ascent(&mut self, a: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QTextInlineObject_setAscent(self as *mut ::text_inline_object::TextInlineObject, a) }
  }

  /// C++ method: <span style='color: green;'>```void QTextInlineObject::setDescent(double d)```</span>
  ///
  ///
  pub fn set_descent(&mut self, d: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QTextInlineObject_setDescent(self as *mut ::text_inline_object::TextInlineObject, d) }
  }

  /// C++ method: <span style='color: green;'>```void QTextInlineObject::setWidth(double w)```</span>
  ///
  ///
  pub fn set_width(&mut self, w: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QTextInlineObject_setWidth(self as *mut ::text_inline_object::TextInlineObject, w) }
  }

  /// C++ method: <span style='color: green;'>```int QTextInlineObject::textPosition() const```</span>
  ///
  ///
  pub fn text_position(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextInlineObject_textPosition(self as *const ::text_inline_object::TextInlineObject) }
  }

  /// C++ method: <span style='color: green;'>```double QTextInlineObject::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextInlineObject_width(self as *const ::text_inline_object::TextInlineObject) }
  }
}

impl Drop for ::text_inline_object::TextInlineObject {
  /// C++ method: <span style='color: green;'>```[destructor] void QTextInlineObject::~QTextInlineObject()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextInlineObject_destructor(self as *mut ::text_inline_object::TextInlineObject) }
  }
}

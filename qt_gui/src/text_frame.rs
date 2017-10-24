/// C++ type: <span style='color: green;'>```QTextFrame::iterator```</span>
#[repr(C)]
pub struct Iterator([u8; ::type_sizes::QT_GUI_TEXT_FRAME_ITERATOR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Iterator {
  unsafe fn new_uninitialized() -> Iterator {
    Iterator(::std::mem::uninitialized())
  }
}

impl Iterator {
  /// C++ method: <span style='color: green;'>```bool QTextFrame::iterator::atEnd() const```</span>
  ///
  ///
  pub fn at_end(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextFrame_iterator_atEnd(self as *const ::text_frame::Iterator) }
  }

  /// C++ method: <span style='color: green;'>```QTextBlock QTextFrame::iterator::currentBlock() const```</span>
  ///
  ///
  pub fn current_block(&self) -> ::text_block::TextBlock {
    {
      let mut object: ::text_block::TextBlock =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFrame_iterator_currentBlock_to_output(self as *const ::text_frame::Iterator, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextFrame* QTextFrame::iterator::currentFrame() const```</span>
  ///
  ///
  pub fn current_frame(&self) -> *mut ::text_frame::TextFrame {
    unsafe { ::ffi::qt_gui_c_QTextFrame_iterator_currentFrame(self as *const ::text_frame::Iterator) }
  }

  /// C++ method: <span style='color: green;'>```QTextFrame::iterator::iterator```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::text_frame::Iterator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextFrame::iterator::iterator()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::text_frame::Iterator) -> ::text_frame::Iterator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextFrame::iterator::iterator(const QTextFrame::iterator& o)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::text_frame::Iterator
    where Args: overloading::IteratorNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QTextFrame::iterator& QTextFrame::iterator::operator=(const QTextFrame::iterator& o)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, o: &'l1 ::text_frame::Iterator) -> &'l0 mut ::text_frame::Iterator {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QTextFrame_iterator_operator_assign(self as *mut ::text_frame::Iterator,
                                                          o as *const ::text_frame::Iterator)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextFrame::iterator& QTextFrame::iterator::operator--()```</span>
  ///
  ///
  pub fn op_dec<'l0>(&'l0 mut self) -> &'l0 mut ::text_frame::Iterator {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextFrame_iterator_operator_dec(self as *mut ::text_frame::Iterator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextFrame::iterator QTextFrame::iterator::operator--(int arg1)```</span>
  ///
  ///
  pub fn op_dec_postfix(&mut self, arg1: ::libc::c_int) -> ::text_frame::Iterator {
    {
      let mut object: ::text_frame::Iterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFrame_iterator_operator_dec_postfix_to_output(self as *mut ::text_frame::Iterator,
                                                                           arg1,
                                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextFrame::iterator::operator==(const QTextFrame::iterator& o) const```</span>
  ///
  ///
  pub fn op_eq(&self, o: &::text_frame::Iterator) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextFrame_iterator_operator_eq(self as *const ::text_frame::Iterator,
                                                      o as *const ::text_frame::Iterator)
    }
  }

  /// C++ method: <span style='color: green;'>```QTextFrame::iterator& QTextFrame::iterator::operator++()```</span>
  ///
  ///
  pub fn op_inc<'l0>(&'l0 mut self) -> &'l0 mut ::text_frame::Iterator {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextFrame_iterator_operator_inc(self as *mut ::text_frame::Iterator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextFrame::iterator QTextFrame::iterator::operator++(int arg1)```</span>
  ///
  ///
  pub fn op_inc_postfix(&mut self, arg1: ::libc::c_int) -> ::text_frame::Iterator {
    {
      let mut object: ::text_frame::Iterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFrame_iterator_operator_inc_postfix_to_output(self as *mut ::text_frame::Iterator,
                                                                           arg1,
                                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextFrame::iterator::operator!=(const QTextFrame::iterator& o) const```</span>
  ///
  ///
  pub fn op_neq(&self, o: &::text_frame::Iterator) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextFrame_iterator_operator_neq(self as *const ::text_frame::Iterator,
                                                       o as *const ::text_frame::Iterator)
    }
  }

  /// C++ method: <span style='color: green;'>```QTextFrame* QTextFrame::iterator::parentFrame() const```</span>
  ///
  ///
  pub fn parent_frame(&self) -> *mut ::text_frame::TextFrame {
    unsafe { ::ffi::qt_gui_c_QTextFrame_iterator_parentFrame(self as *const ::text_frame::Iterator) }
  }
}

impl Drop for ::text_frame::Iterator {
  /// C++ method: <span style='color: green;'>```[destructor] void QTextFrame::iterator::~QTextFrame::iterator()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextFrame_iterator_destructor(self as *mut ::text_frame::Iterator) }
  }
}

/// C++ type: <span style='color: green;'>```QTextFrame```</span>
#[repr(C)]
pub struct TextFrame(u8);

impl TextFrame {
  /// C++ method: <span style='color: green;'>```QTextFrame::iterator QTextFrame::begin() const```</span>
  ///
  ///
  pub fn begin(&self) -> ::text_frame::Iterator {
    {
      let mut object: ::text_frame::Iterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFrame_begin_to_output(self as *const ::text_frame::TextFrame, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextFrame*> QTextFrame::childFrames() const```</span>
  ///
  ///
  pub fn child_frames(&self) -> ::list::ListTextFrameMutPtr {
    {
      let mut object: ::list::ListTextFrameMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFrame_childFrames_to_output(self as *const ::text_frame::TextFrame, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextFrame::iterator QTextFrame::end() const```</span>
  ///
  ///
  pub fn end(&self) -> ::text_frame::Iterator {
    {
      let mut object: ::text_frame::Iterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFrame_end_to_output(self as *const ::text_frame::TextFrame, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextCursor QTextFrame::firstCursorPosition() const```</span>
  ///
  ///
  pub fn first_cursor_position(&self) -> ::cpp_utils::CppBox<::text_cursor::TextCursor> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTextFrame_firstCursorPosition_as_ptr(self as *const ::text_frame::TextFrame) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```int QTextFrame::firstPosition() const```</span>
  ///
  ///
  pub fn first_position(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextFrame_firstPosition(self as *const ::text_frame::TextFrame) }
  }

  /// C++ method: <span style='color: green;'>```QTextFrameFormat QTextFrame::frameFormat() const```</span>
  ///
  ///
  pub fn frame_format(&self) -> ::text_frame_format::TextFrameFormat {
    {
      let mut object: ::text_frame_format::TextFrameFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFrame_frameFormat_to_output(self as *const ::text_frame::TextFrame, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextCursor QTextFrame::lastCursorPosition() const```</span>
  ///
  ///
  pub fn last_cursor_position(&self) -> ::cpp_utils::CppBox<::text_cursor::TextCursor> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTextFrame_lastCursorPosition_as_ptr(self as *const ::text_frame::TextFrame) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```int QTextFrame::lastPosition() const```</span>
  ///
  ///
  pub fn last_position(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextFrame_lastPosition(self as *const ::text_frame::TextFrame) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QTextFrame::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QTextFrame_metaObject(self as *const ::text_frame::TextFrame) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTextFrame::QTextFrame(QTextDocument* doc)```</span>
  ///
  ///
  pub unsafe fn new(doc: *mut ::text_document::TextDocument) -> ::cpp_utils::CppBox<::text_frame::TextFrame> {
    let ffi_result = ::ffi::qt_gui_c_QTextFrame_new(doc);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QTextFrame* QTextFrame::parentFrame() const```</span>
  ///
  ///
  pub fn parent_frame(&self) -> *mut ::text_frame::TextFrame {
    unsafe { ::ffi::qt_gui_c_QTextFrame_parentFrame(self as *const ::text_frame::TextFrame) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QTextFrame::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QTextFrame_qt_metacall(self as *mut ::text_frame::TextFrame,
                                           arg1 as *const ::qt_core::meta_object::Call,
                                           arg2,
                                           arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QTextFrame::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QTextFrame_qt_metacast(self as *mut ::text_frame::TextFrame, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QTextFrame::setFrameFormat(const QTextFrameFormat& format)```</span>
  ///
  ///
  pub fn set_frame_format(&mut self, format: &::text_frame_format::TextFrameFormat) {
    unsafe {
      ::ffi::qt_gui_c_QTextFrame_setFrameFormat(self as *mut ::text_frame::TextFrame,
                                                format as *const ::text_frame_format::TextFrameFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTextFrame::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QTextFrame_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTextFrame::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QTextFrame_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::text_frame::TextFrame {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QTextFrame_delete
  }
}

impl ::cpp_utils::DynamicCast<::text_frame::TextFrame> for ::text_object::TextObject {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::text_frame::TextFrame> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTextFrame_G_dynamic_cast_QTextFrame_ptr(self as *mut ::text_object::TextObject) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::text_frame::TextFrame> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextFrame_G_dynamic_cast_QTextFrame_ptr(self as *const ::text_object::TextObject as *mut ::text_object::TextObject) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::text_frame::TextFrame {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTextFrame_G_static_cast_QObject_ptr(self as *mut ::text_frame::TextFrame) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextFrame_G_static_cast_QObject_ptr(self as *const ::text_frame::TextFrame as *mut ::text_frame::TextFrame) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::text_object::TextObject> for ::text_frame::TextFrame {
  fn static_cast_mut(&mut self) -> &mut ::text_object::TextObject {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTextFrame_G_static_cast_QTextObject_ptr(self as *mut ::text_frame::TextFrame) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::text_object::TextObject {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextFrame_G_static_cast_QTextObject_ptr(self as *const ::text_frame::TextFrame as *mut ::text_frame::TextFrame) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_frame::TextFrame> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_frame::TextFrame {
    let ffi_result =
      ::ffi::qt_gui_c_QTextFrame_G_static_cast_QTextFrame_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_frame::TextFrame {
    let ffi_result = ::ffi::qt_gui_c_QTextFrame_G_static_cast_QTextFrame_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_frame::TextFrame> for ::text_object::TextObject {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_frame::TextFrame {
    let ffi_result =
      ::ffi::qt_gui_c_QTextFrame_G_static_cast_QTextFrame_ptr_QTextObject(self as *mut ::text_object::TextObject);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_frame::TextFrame {
    let ffi_result = ::ffi::qt_gui_c_QTextFrame_G_static_cast_QTextFrame_ptr_QTextObject(self as *const ::text_object::TextObject as *mut ::text_object::TextObject);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::text_frame::TextFrame {
  type Target = ::text_object::TextObject;
  fn deref(&self) -> &::text_object::TextObject {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextFrame_G_static_cast_QTextObject_ptr(self as *const ::text_frame::TextFrame as *mut ::text_frame::TextFrame) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::text_frame::TextFrame {
  fn deref_mut(&mut self) -> &mut ::text_object::TextObject {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTextFrame_G_static_cast_QTextObject_ptr(self as *mut ::text_frame::TextFrame) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Iterator::new](../struct.Iterator.html#method.new) method.
  pub trait IteratorNewArgs {
    fn exec(self) -> ::text_frame::Iterator;
  }
  impl IteratorNewArgs for () {
    fn exec(self) -> ::text_frame::Iterator {

      {
        let mut object: ::text_frame::Iterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextFrame_iterator_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> IteratorNewArgs for &'a ::text_frame::Iterator {
    fn exec(self) -> ::text_frame::Iterator {
      let o = self;
      {
        let mut object: ::text_frame::Iterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextFrame_iterator_constructor_o(o as *const ::text_frame::Iterator, &mut object);
        }
        object
      }
    }
  }
}

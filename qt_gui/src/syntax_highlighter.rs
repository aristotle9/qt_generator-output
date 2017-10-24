/// C++ type: <span style='color: green;'>```QSyntaxHighlighter```</span>
#[repr(C)]
pub struct SyntaxHighlighter(u8);

impl SyntaxHighlighter {
  /// C++ method: <span style='color: green;'>```QTextDocument* QSyntaxHighlighter::document() const```</span>
  ///
  ///
  pub fn document(&self) -> *mut ::text_document::TextDocument {
    unsafe { ::ffi::qt_gui_c_QSyntaxHighlighter_document(self as *const ::syntax_highlighter::SyntaxHighlighter) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QSyntaxHighlighter::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QSyntaxHighlighter_metaObject(self as *const ::syntax_highlighter::SyntaxHighlighter) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QSyntaxHighlighter::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QSyntaxHighlighter_qt_metacall(self as *mut ::syntax_highlighter::SyntaxHighlighter,
                                                   arg1 as *const ::qt_core::meta_object::Call,
                                                   arg2,
                                                   arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QSyntaxHighlighter::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QSyntaxHighlighter_qt_metacast(self as *mut ::syntax_highlighter::SyntaxHighlighter, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QSyntaxHighlighter::rehighlight()```</span>
  ///
  ///
  pub fn rehighlight(&mut self) {
    unsafe { ::ffi::qt_gui_c_QSyntaxHighlighter_rehighlight(self as *mut ::syntax_highlighter::SyntaxHighlighter) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QSyntaxHighlighter::rehighlightBlock(const QTextBlock& block)```</span>
  ///
  ///
  pub fn rehighlight_block(&mut self, block: &::text_block::TextBlock) {
    unsafe {
      ::ffi::qt_gui_c_QSyntaxHighlighter_rehighlightBlock(self as *mut ::syntax_highlighter::SyntaxHighlighter,
                                                          block as *const ::text_block::TextBlock)
    }
  }

  /// C++ method: <span style='color: green;'>```void QSyntaxHighlighter::setDocument(QTextDocument* doc)```</span>
  ///
  ///
  pub unsafe fn set_document(&mut self, doc: *mut ::text_document::TextDocument) {
    ::ffi::qt_gui_c_QSyntaxHighlighter_setDocument(self as *mut ::syntax_highlighter::SyntaxHighlighter, doc)
  }

  /// C++ method: <span style='color: green;'>```static QString QSyntaxHighlighter::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QSyntaxHighlighter_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSyntaxHighlighter::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QSyntaxHighlighter_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::syntax_highlighter::SyntaxHighlighter {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QSyntaxHighlighter_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `SyntaxHighlighter`.
  pub struct Signals<'a>(&'a ::syntax_highlighter::SyntaxHighlighter);
  /// Represents a built-in Qt signal `QSyntaxHighlighter::objectNameChanged`.
  ///
  /// An object of this type can be created from `SyntaxHighlighter` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SyntaxHighlighter` object.
  pub struct ObjectNameChanged<'a>(&'a ::syntax_highlighter::SyntaxHighlighter);
  impl<'a> ::qt_core::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ObjectNameChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QSyntaxHighlighter::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `SyntaxHighlighter`.
  pub struct Slots<'a>(&'a ::syntax_highlighter::SyntaxHighlighter);
  /// Represents a built-in Qt slot `QSyntaxHighlighter::rehighlight`.
  ///
  /// An object of this type can be created from `SyntaxHighlighter` with `object.slots().rehighlight()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SyntaxHighlighter` object.
  pub struct Rehighlight<'a>(&'a ::syntax_highlighter::SyntaxHighlighter);
  impl<'a> ::qt_core::connection::Receiver for Rehighlight<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1rehighlight()\0"
    }
  }
  /// Represents a built-in Qt slot `QSyntaxHighlighter::rehighlightBlock`.
  ///
  /// An object of this type can be created from `SyntaxHighlighter` with `object.slots().rehighlight_block()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SyntaxHighlighter` object.
  pub struct RehighlightBlock<'a>(&'a ::syntax_highlighter::SyntaxHighlighter);
  impl<'a> ::qt_core::connection::Receiver for RehighlightBlock<'a> {
    type Arguments = (&'static ::text_block::TextBlock,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1rehighlightBlock(const QTextBlock&)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QSyntaxHighlighter::rehighlight`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rehighlight(&self) -> Rehighlight {
      Rehighlight(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSyntaxHighlighter::rehighlightBlock`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rehighlight_block(&self) -> RehighlightBlock {
      RehighlightBlock(self.0)
    }
  }
  impl ::syntax_highlighter::SyntaxHighlighter {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
    /// Provides access to built-in Qt slots of this type
    pub fn slots(&self) -> Slots {
      Slots(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::syntax_highlighter::SyntaxHighlighter {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QSyntaxHighlighter_G_static_cast_QObject_ptr(self as *mut ::syntax_highlighter::SyntaxHighlighter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QSyntaxHighlighter_G_static_cast_QObject_ptr(self as *const ::syntax_highlighter::SyntaxHighlighter as *mut ::syntax_highlighter::SyntaxHighlighter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::syntax_highlighter::SyntaxHighlighter> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::syntax_highlighter::SyntaxHighlighter {
    let ffi_result =
      ::ffi::qt_gui_c_QSyntaxHighlighter_G_static_cast_QSyntaxHighlighter_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::syntax_highlighter::SyntaxHighlighter {
    let ffi_result = ::ffi::qt_gui_c_QSyntaxHighlighter_G_static_cast_QSyntaxHighlighter_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::syntax_highlighter::SyntaxHighlighter {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QSyntaxHighlighter_G_static_cast_QObject_ptr(self as *const ::syntax_highlighter::SyntaxHighlighter as *mut ::syntax_highlighter::SyntaxHighlighter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::syntax_highlighter::SyntaxHighlighter {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QSyntaxHighlighter_G_static_cast_QObject_ptr(self as *mut ::syntax_highlighter::SyntaxHighlighter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

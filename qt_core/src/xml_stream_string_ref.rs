/// C++ type: <span style='color: green;'>```QXmlStreamStringRef```</span>
#[repr(C)]
pub struct XmlStreamStringRef([u8; ::type_sizes::QT_CORE_XML_STREAM_STRING_REF_XML_STREAM_STRING_REF]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for XmlStreamStringRef {
  unsafe fn new_uninitialized() -> XmlStreamStringRef {
    XmlStreamStringRef(::std::mem::uninitialized())
  }
}

impl XmlStreamStringRef {
  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamStringRef::operator QStringRef() const```</span>
  ///
  ///
  pub fn as_q_string_ref(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamStringRef_convert_to_QStringRef_to_output(self as *const ::xml_stream_string_ref::XmlStreamStringRef, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QXmlStreamStringRef::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QXmlStreamStringRef_clear(self as *mut ::xml_stream_string_ref::XmlStreamStringRef) }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamStringRef::QXmlStreamStringRef```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::xml_stream_string_ref::XmlStreamStringRef```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QXmlStreamStringRef::QXmlStreamStringRef()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::string::String) -> ::xml_stream_string_ref::XmlStreamStringRef```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QXmlStreamStringRef::QXmlStreamStringRef(const QString& aString)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::string_ref::StringRef) -> ::xml_stream_string_ref::XmlStreamStringRef```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QXmlStreamStringRef::QXmlStreamStringRef(const QStringRef& aString)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::xml_stream_string_ref::XmlStreamStringRef) -> ::xml_stream_string_ref::XmlStreamStringRef```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QXmlStreamStringRef::QXmlStreamStringRef(const QXmlStreamStringRef& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::xml_stream_string_ref::XmlStreamStringRef
    where Args: overloading::XmlStreamStringRefNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QXmlStreamStringRef& QXmlStreamStringRef::operator=(const QXmlStreamStringRef& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::xml_stream_string_ref::XmlStreamStringRef)
                             -> &'l0 mut ::xml_stream_string_ref::XmlStreamStringRef {
    let ffi_result = unsafe { ::ffi::qt_core_c_QXmlStreamStringRef_operator_assign(self as *mut ::xml_stream_string_ref::XmlStreamStringRef, other as *const ::xml_stream_string_ref::XmlStreamStringRef) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QXmlStreamStringRef::position() const```</span>
  ///
  ///
  pub fn position(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QXmlStreamStringRef_position(self as *const ::xml_stream_string_ref::XmlStreamStringRef) }
  }

  /// C++ method: <span style='color: green;'>```int QXmlStreamStringRef::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QXmlStreamStringRef_size(self as *const ::xml_stream_string_ref::XmlStreamStringRef) }
  }

  /// C++ method: <span style='color: green;'>```const QString* QXmlStreamStringRef::string() const```</span>
  ///
  ///
  pub fn string(&self) -> *const ::string::String {
    unsafe { ::ffi::qt_core_c_QXmlStreamStringRef_string(self as *const ::xml_stream_string_ref::XmlStreamStringRef) }
  }

  /// C++ method: <span style='color: green;'>```void QXmlStreamStringRef::swap(QXmlStreamStringRef& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::xml_stream_string_ref::XmlStreamStringRef) {
    unsafe {
      ::ffi::qt_core_c_QXmlStreamStringRef_swap(self as *mut ::xml_stream_string_ref::XmlStreamStringRef,
                                                other as *mut ::xml_stream_string_ref::XmlStreamStringRef)
    }
  }
}

impl Drop for ::xml_stream_string_ref::XmlStreamStringRef {
  /// C++ method: <span style='color: green;'>```[destructor] void QXmlStreamStringRef::~QXmlStreamStringRef()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QXmlStreamStringRef_destructor(self as *mut ::xml_stream_string_ref::XmlStreamStringRef) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [XmlStreamStringRef::new](../struct.XmlStreamStringRef.html#method.new) method.
  pub trait XmlStreamStringRefNewArgs {
    fn exec(self) -> ::xml_stream_string_ref::XmlStreamStringRef;
  }
  impl<'a> XmlStreamStringRefNewArgs for &'a ::string::String {
    fn exec(self) -> ::xml_stream_string_ref::XmlStreamStringRef {
      let a_string = self;
      {
        let mut object: ::xml_stream_string_ref::XmlStreamStringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QXmlStreamStringRef_constructor_QString(a_string as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> XmlStreamStringRefNewArgs for &'a ::string_ref::StringRef {
    fn exec(self) -> ::xml_stream_string_ref::XmlStreamStringRef {
      let a_string = self;
      {
        let mut object: ::xml_stream_string_ref::XmlStreamStringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QXmlStreamStringRef_constructor_QStringRef(a_string as *const ::string_ref::StringRef,
                                                                      &mut object);
        }
        object
      }
    }
  }
  impl<'a> XmlStreamStringRefNewArgs for &'a ::xml_stream_string_ref::XmlStreamStringRef {
    fn exec(self) -> ::xml_stream_string_ref::XmlStreamStringRef {
      let other = self;
      {
        let mut object: ::xml_stream_string_ref::XmlStreamStringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QXmlStreamStringRef_constructor_QXmlStreamStringRef(other as *const ::xml_stream_string_ref::XmlStreamStringRef, &mut object);
        }
        object
      }
    }
  }
  impl XmlStreamStringRefNewArgs for () {
    fn exec(self) -> ::xml_stream_string_ref::XmlStreamStringRef {

      {
        let mut object: ::xml_stream_string_ref::XmlStreamStringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QXmlStreamStringRef_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
}

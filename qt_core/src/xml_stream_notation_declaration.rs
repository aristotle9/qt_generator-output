/// C++ type: <span style='color: green;'>```QXmlStreamNotationDeclaration```</span>
#[repr(C)]
pub struct XmlStreamNotationDeclaration([u8; ::type_sizes::QT_CORE_XML_STREAM_NOTATION_DECLARATION_XML_STREAM_NOTATION_DECLARATION]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for XmlStreamNotationDeclaration {
  unsafe fn new_uninitialized() -> XmlStreamNotationDeclaration {
    XmlStreamNotationDeclaration(::std::mem::uninitialized())
  }
}

impl XmlStreamNotationDeclaration {
  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamNotationDeclaration::name() const```</span>
  ///
  ///
  pub fn name(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamNotationDeclaration_name_to_output(self as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamNotationDeclaration::QXmlStreamNotationDeclaration```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::xml_stream_notation_declaration::XmlStreamNotationDeclaration```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QXmlStreamNotationDeclaration::QXmlStreamNotationDeclaration()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::xml_stream_notation_declaration::XmlStreamNotationDeclaration) -> ::xml_stream_notation_declaration::XmlStreamNotationDeclaration```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QXmlStreamNotationDeclaration::QXmlStreamNotationDeclaration(const QXmlStreamNotationDeclaration& arg1)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::xml_stream_notation_declaration::XmlStreamNotationDeclaration
    where Args: overloading::XmlStreamNotationDeclarationNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QXmlStreamNotationDeclaration& QXmlStreamNotationDeclaration::operator=(const QXmlStreamNotationDeclaration& arg1)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             arg1: &'l1 ::xml_stream_notation_declaration::XmlStreamNotationDeclaration)
                             -> &'l0 mut ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QXmlStreamNotationDeclaration_operator_assign(self as *mut ::xml_stream_notation_declaration::XmlStreamNotationDeclaration, arg1 as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QXmlStreamNotationDeclaration::operator==(const QXmlStreamNotationDeclaration& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::xml_stream_notation_declaration::XmlStreamNotationDeclaration) -> bool {
    unsafe { ::ffi::qt_core_c_QXmlStreamNotationDeclaration_operator_eq(self as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration, other as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```bool QXmlStreamNotationDeclaration::operator!=(const QXmlStreamNotationDeclaration& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::xml_stream_notation_declaration::XmlStreamNotationDeclaration) -> bool {
    unsafe { ::ffi::qt_core_c_QXmlStreamNotationDeclaration_operator_neq(self as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration, other as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamNotationDeclaration::publicId() const```</span>
  ///
  ///
  pub fn public_id(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamNotationDeclaration_publicId_to_output(self as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamNotationDeclaration::systemId() const```</span>
  ///
  ///
  pub fn system_id(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamNotationDeclaration_systemId_to_output(self as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration, &mut object);
      }
      object
    }
  }
}

impl Drop for ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {
  /// C++ method: <span style='color: green;'>```[destructor] void QXmlStreamNotationDeclaration::~QXmlStreamNotationDeclaration()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QXmlStreamNotationDeclaration_destructor(self as *mut ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [XmlStreamNotationDeclaration::new](../struct.XmlStreamNotationDeclaration.html#method.new) method.
  pub trait XmlStreamNotationDeclarationNewArgs {
    fn exec(self) -> ::xml_stream_notation_declaration::XmlStreamNotationDeclaration;
  }
  impl<'a> XmlStreamNotationDeclarationNewArgs for &'a ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {
    fn exec(self) -> ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {
      let arg1 = self;
      {
        let mut object: ::xml_stream_notation_declaration::XmlStreamNotationDeclaration =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QXmlStreamNotationDeclaration_constructor_arg1(arg1 as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration, &mut object);
        }
        object
      }
    }
  }
  impl XmlStreamNotationDeclarationNewArgs for () {
    fn exec(self) -> ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {

      {
        let mut object: ::xml_stream_notation_declaration::XmlStreamNotationDeclaration =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QXmlStreamNotationDeclaration_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
}

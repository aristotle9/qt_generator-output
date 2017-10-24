/// C++ type: <span style='color: green;'>```QXmlStreamEntityDeclaration```</span>
#[repr(C)]
pub struct XmlStreamEntityDeclaration([u8; ::type_sizes::QT_CORE_XML_STREAM_ENTITY_DECLARATION_XML_STREAM_ENTITY_DECLARATION]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for XmlStreamEntityDeclaration {
  unsafe fn new_uninitialized() -> XmlStreamEntityDeclaration {
    XmlStreamEntityDeclaration(::std::mem::uninitialized())
  }
}

impl XmlStreamEntityDeclaration {
  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamEntityDeclaration::name() const```</span>
  ///
  ///
  pub fn name(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamEntityDeclaration_name_to_output(self as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamEntityDeclaration::QXmlStreamEntityDeclaration```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::xml_stream_entity_declaration::XmlStreamEntityDeclaration```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QXmlStreamEntityDeclaration::QXmlStreamEntityDeclaration()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::xml_stream_entity_declaration::XmlStreamEntityDeclaration) -> ::xml_stream_entity_declaration::XmlStreamEntityDeclaration```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QXmlStreamEntityDeclaration::QXmlStreamEntityDeclaration(const QXmlStreamEntityDeclaration& arg1)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::xml_stream_entity_declaration::XmlStreamEntityDeclaration
    where Args: overloading::XmlStreamEntityDeclarationNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamEntityDeclaration::notationName() const```</span>
  ///
  ///
  pub fn notation_name(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamEntityDeclaration_notationName_to_output(self as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamEntityDeclaration& QXmlStreamEntityDeclaration::operator=(const QXmlStreamEntityDeclaration& arg1)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             arg1: &'l1 ::xml_stream_entity_declaration::XmlStreamEntityDeclaration)
                             -> &'l0 mut ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QXmlStreamEntityDeclaration_operator_assign(self as *mut ::xml_stream_entity_declaration::XmlStreamEntityDeclaration, arg1 as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QXmlStreamEntityDeclaration::operator==(const QXmlStreamEntityDeclaration& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::xml_stream_entity_declaration::XmlStreamEntityDeclaration) -> bool {
    unsafe { ::ffi::qt_core_c_QXmlStreamEntityDeclaration_operator_eq(self as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration, other as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```bool QXmlStreamEntityDeclaration::operator!=(const QXmlStreamEntityDeclaration& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::xml_stream_entity_declaration::XmlStreamEntityDeclaration) -> bool {
    unsafe { ::ffi::qt_core_c_QXmlStreamEntityDeclaration_operator_neq(self as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration, other as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamEntityDeclaration::publicId() const```</span>
  ///
  ///
  pub fn public_id(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamEntityDeclaration_publicId_to_output(self as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamEntityDeclaration::systemId() const```</span>
  ///
  ///
  pub fn system_id(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamEntityDeclaration_systemId_to_output(self as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamEntityDeclaration::value() const```</span>
  ///
  ///
  pub fn value(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamEntityDeclaration_value_to_output(self as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration, &mut object);
      }
      object
    }
  }
}

impl Drop for ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {
  /// C++ method: <span style='color: green;'>```[destructor] void QXmlStreamEntityDeclaration::~QXmlStreamEntityDeclaration()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QXmlStreamEntityDeclaration_destructor(self as *mut ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [XmlStreamEntityDeclaration::new](../struct.XmlStreamEntityDeclaration.html#method.new) method.
  pub trait XmlStreamEntityDeclarationNewArgs {
    fn exec(self) -> ::xml_stream_entity_declaration::XmlStreamEntityDeclaration;
  }
  impl<'a> XmlStreamEntityDeclarationNewArgs for &'a ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {
    fn exec(self) -> ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {
      let arg1 = self;
      {
        let mut object: ::xml_stream_entity_declaration::XmlStreamEntityDeclaration =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QXmlStreamEntityDeclaration_constructor_arg1(arg1 as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration, &mut object);
        }
        object
      }
    }
  }
  impl XmlStreamEntityDeclarationNewArgs for () {
    fn exec(self) -> ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {

      {
        let mut object: ::xml_stream_entity_declaration::XmlStreamEntityDeclaration =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QXmlStreamEntityDeclaration_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
}

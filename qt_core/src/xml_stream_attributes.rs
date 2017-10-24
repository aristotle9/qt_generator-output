/// C++ type: <span style='color: green;'>```QXmlStreamAttributes```</span>
#[repr(C)]
pub struct XmlStreamAttributes([u8; ::type_sizes::QT_CORE_XML_STREAM_ATTRIBUTES_XML_STREAM_ATTRIBUTES]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for XmlStreamAttributes {
  unsafe fn new_uninitialized() -> XmlStreamAttributes {
    XmlStreamAttributes(::std::mem::uninitialized())
  }
}

impl XmlStreamAttributes {
  /// C++ method: <span style='color: green;'>```QXmlStreamAttributes::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, (&::string::String, &::string::String, &::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QXmlStreamAttributes::append(const QString& namespaceUri, const QString& name, const QString& value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, (&::string::String, &::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QXmlStreamAttributes::append(const QString& qualifiedName, const QString& value)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::XmlStreamAttributesAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QXmlStreamAttributes::hasAttribute```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn has_attribute(&self, &::latin1_string::Latin1String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QXmlStreamAttributes::hasAttribute(QLatin1String qualifiedName) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn has_attribute(&self, (&::string::String, &::string::String)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QXmlStreamAttributes::hasAttribute(const QString& namespaceUri, const QString& name) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn has_attribute(&self, &::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QXmlStreamAttributes::hasAttribute(const QString& qualifiedName) const```</span>
  ///
  ///
  pub fn has_attribute<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::XmlStreamAttributesHasAttributeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QXmlStreamAttributes::QXmlStreamAttributes()```</span>
  ///
  ///
  pub fn new() -> ::xml_stream_attributes::XmlStreamAttributes {
    {
      let mut object: ::xml_stream_attributes::XmlStreamAttributes =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamAttributes_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamAttributes::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, (&::latin1_string::Latin1String, &::latin1_string::Latin1String)) -> ::string_ref::StringRef```<br>
  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamAttributes::value(QLatin1String namespaceUri, QLatin1String name) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, &::latin1_string::Latin1String) -> ::string_ref::StringRef```<br>
  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamAttributes::value(QLatin1String qualifiedName) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn value(&self, (&::string::String, &::latin1_string::Latin1String)) -> ::string_ref::StringRef```<br>
  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamAttributes::value(const QString& namespaceUri, QLatin1String name) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn value(&self, (&::string::String, &::string::String)) -> ::string_ref::StringRef```<br>
  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamAttributes::value(const QString& namespaceUri, const QString& name) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn value(&self, &::string::String) -> ::string_ref::StringRef```<br>
  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamAttributes::value(const QString& qualifiedName) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::string_ref::StringRef
    where Args: overloading::XmlStreamAttributesValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::xml_stream_attributes::XmlStreamAttributes {
  /// C++ method: <span style='color: green;'>```[destructor] void QXmlStreamAttributes::~QXmlStreamAttributes()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_core_c_QXmlStreamAttributes_destructor(self as *mut ::xml_stream_attributes::XmlStreamAttributes)
    }
  }
}

impl ::cpp_utils::StaticCast<::vector::VectorXmlStreamAttribute> for ::xml_stream_attributes::XmlStreamAttributes {
  fn static_cast_mut(&mut self) -> &mut ::vector::VectorXmlStreamAttribute {
    let ffi_result = unsafe { ::ffi::qt_core_c_QXmlStreamAttributes_G_static_cast_QVector_QXmlStreamAttribute_ptr(self as *mut ::xml_stream_attributes::XmlStreamAttributes) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::vector::VectorXmlStreamAttribute {
    let ffi_result = unsafe { ::ffi::qt_core_c_QXmlStreamAttributes_G_static_cast_QVector_QXmlStreamAttribute_ptr(self as *const ::xml_stream_attributes::XmlStreamAttributes as *mut ::xml_stream_attributes::XmlStreamAttributes) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::xml_stream_attributes::XmlStreamAttributes> for ::vector::VectorXmlStreamAttribute {
unsafe fn static_cast_mut(&mut self) -> &mut ::xml_stream_attributes::XmlStreamAttributes {
let ffi_result = ::ffi::qt_core_c_QXmlStreamAttributes_G_static_cast_QXmlStreamAttributes_ptr(self as *mut ::vector::VectorXmlStreamAttribute);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::xml_stream_attributes::XmlStreamAttributes {
let ffi_result = ::ffi::qt_core_c_QXmlStreamAttributes_G_static_cast_QXmlStreamAttributes_ptr(self as *const ::vector::VectorXmlStreamAttribute as *mut ::vector::VectorXmlStreamAttribute);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::xml_stream_attributes::XmlStreamAttributes {
  type Target = ::vector::VectorXmlStreamAttribute;
  fn deref(&self) -> &::vector::VectorXmlStreamAttribute {
    let ffi_result = unsafe { ::ffi::qt_core_c_QXmlStreamAttributes_G_static_cast_QVector_QXmlStreamAttribute_ptr(self as *const ::xml_stream_attributes::XmlStreamAttributes as *mut ::xml_stream_attributes::XmlStreamAttributes) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::xml_stream_attributes::XmlStreamAttributes {
  fn deref_mut(&mut self) -> &mut ::vector::VectorXmlStreamAttribute {
    let ffi_result = unsafe { ::ffi::qt_core_c_QXmlStreamAttributes_G_static_cast_QVector_QXmlStreamAttribute_ptr(self as *mut ::xml_stream_attributes::XmlStreamAttributes) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [XmlStreamAttributes::append](../struct.XmlStreamAttributes.html#method.append) method.
  pub trait XmlStreamAttributesAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::xml_stream_attributes::XmlStreamAttributes) -> ();
  }
  impl<'largs> XmlStreamAttributesAppendArgs<'largs>
    for (&'largs ::string::String, &'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::xml_stream_attributes::XmlStreamAttributes) -> () {
      let namespace_uri = self.0;
      let name = self.1;
      let value = self.2;
      unsafe { ::ffi::qt_core_c_QXmlStreamAttributes_append_namespaceUri_name_value(original_self as *mut ::xml_stream_attributes::XmlStreamAttributes, namespace_uri as *const ::string::String, name as *const ::string::String, value as *const ::string::String) }
    }
  }
  impl<'largs> XmlStreamAttributesAppendArgs<'largs> for (&'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::xml_stream_attributes::XmlStreamAttributes) -> () {
      let qualified_name = self.0;
      let value = self.1;
      unsafe { ::ffi::qt_core_c_QXmlStreamAttributes_append_qualifiedName_value(original_self as *mut ::xml_stream_attributes::XmlStreamAttributes, qualified_name as *const ::string::String, value as *const ::string::String) }
    }
  }
  /// This trait represents a set of arguments accepted by [XmlStreamAttributes::has_attribute](../struct.XmlStreamAttributes.html#method.has_attribute) method.
  pub trait XmlStreamAttributesHasAttributeArgs<'largs> {
    fn exec(self, original_self: &'largs ::xml_stream_attributes::XmlStreamAttributes) -> bool;
  }
  impl<'largs> XmlStreamAttributesHasAttributeArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs ::xml_stream_attributes::XmlStreamAttributes) -> bool {
      let qualified_name = self;
      unsafe { ::ffi::qt_core_c_QXmlStreamAttributes_hasAttribute_QLatin1String(original_self as *const ::xml_stream_attributes::XmlStreamAttributes, qualified_name as *const ::latin1_string::Latin1String) }
    }
  }
  impl<'largs> XmlStreamAttributesHasAttributeArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::xml_stream_attributes::XmlStreamAttributes) -> bool {
      let qualified_name = self;
      unsafe { ::ffi::qt_core_c_QXmlStreamAttributes_hasAttribute_QString(original_self as *const ::xml_stream_attributes::XmlStreamAttributes, qualified_name as *const ::string::String) }
    }
  }
  impl<'largs> XmlStreamAttributesHasAttributeArgs<'largs> for (&'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs ::xml_stream_attributes::XmlStreamAttributes) -> bool {
      let namespace_uri = self.0;
      let name = self.1;
      unsafe { ::ffi::qt_core_c_QXmlStreamAttributes_hasAttribute_QString_QString(original_self as *const ::xml_stream_attributes::XmlStreamAttributes, namespace_uri as *const ::string::String, name as *const ::string::String) }
    }
  }
  /// This trait represents a set of arguments accepted by [XmlStreamAttributes::value](../struct.XmlStreamAttributes.html#method.value) method.
  pub trait XmlStreamAttributesValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::xml_stream_attributes::XmlStreamAttributes) -> ::string_ref::StringRef;
  }
  impl<'largs> XmlStreamAttributesValueArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs ::xml_stream_attributes::XmlStreamAttributes) -> ::string_ref::StringRef {
      let qualified_name = self;
      {
        let mut object: ::string_ref::StringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QXmlStreamAttributes_value_to_output_QLatin1String(original_self as *const ::xml_stream_attributes::XmlStreamAttributes, qualified_name as *const ::latin1_string::Latin1String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> XmlStreamAttributesValueArgs<'largs>
    for (&'largs ::latin1_string::Latin1String, &'largs ::latin1_string::Latin1String) {
    fn exec(self, original_self: &'largs ::xml_stream_attributes::XmlStreamAttributes) -> ::string_ref::StringRef {
      let namespace_uri = self.0;
      let name = self.1;
      {
        let mut object: ::string_ref::StringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QXmlStreamAttributes_value_to_output_QLatin1String_QLatin1String(original_self as *const ::xml_stream_attributes::XmlStreamAttributes, namespace_uri as *const ::latin1_string::Latin1String, name as *const ::latin1_string::Latin1String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> XmlStreamAttributesValueArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::xml_stream_attributes::XmlStreamAttributes) -> ::string_ref::StringRef {
      let qualified_name = self;
      {
        let mut object: ::string_ref::StringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QXmlStreamAttributes_value_to_output_QString(original_self as *const ::xml_stream_attributes::XmlStreamAttributes, qualified_name as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> XmlStreamAttributesValueArgs<'largs>
    for (&'largs ::string::String, &'largs ::latin1_string::Latin1String) {
    fn exec(self, original_self: &'largs ::xml_stream_attributes::XmlStreamAttributes) -> ::string_ref::StringRef {
      let namespace_uri = self.0;
      let name = self.1;
      {
        let mut object: ::string_ref::StringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QXmlStreamAttributes_value_to_output_QString_QLatin1String(original_self as *const ::xml_stream_attributes::XmlStreamAttributes, namespace_uri as *const ::string::String, name as *const ::latin1_string::Latin1String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> XmlStreamAttributesValueArgs<'largs> for (&'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs ::xml_stream_attributes::XmlStreamAttributes) -> ::string_ref::StringRef {
      let namespace_uri = self.0;
      let name = self.1;
      {
        let mut object: ::string_ref::StringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QXmlStreamAttributes_value_to_output_QString_QString(original_self as *const ::xml_stream_attributes::XmlStreamAttributes, namespace_uri as *const ::string::String, name as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
}

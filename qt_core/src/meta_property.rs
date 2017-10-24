/// C++ type: <span style='color: green;'>```QMetaProperty```</span>
#[repr(C)]
pub struct MetaProperty([u8; ::type_sizes::QT_CORE_META_PROPERTY_META_PROPERTY]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for MetaProperty {
  unsafe fn new_uninitialized() -> MetaProperty {
    MetaProperty(::std::mem::uninitialized())
  }
}

impl MetaProperty {
  /// C++ method: <span style='color: green;'>```const QMetaObject* QMetaProperty::enclosingMetaObject() const```</span>
  ///
  ///
  pub fn enclosing_meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QMetaProperty_enclosingMetaObject(self as *const ::meta_property::MetaProperty) }
  }

  /// C++ method: <span style='color: green;'>```QMetaEnum QMetaProperty::enumerator() const```</span>
  ///
  ///
  pub fn enumerator(&self) -> ::meta_enum::MetaEnum {
    {
      let mut object: ::meta_enum::MetaEnum =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMetaProperty_enumerator_to_output(self as *const ::meta_property::MetaProperty, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QMetaProperty::hasNotifySignal() const```</span>
  ///
  ///
  pub fn has_notify_signal(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMetaProperty_hasNotifySignal(self as *const ::meta_property::MetaProperty) }
  }

  /// C++ method: <span style='color: green;'>```bool QMetaProperty::hasStdCppSet() const```</span>
  ///
  ///
  pub fn has_std_cpp_set(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMetaProperty_hasStdCppSet(self as *const ::meta_property::MetaProperty) }
  }

  /// C++ method: <span style='color: green;'>```bool QMetaProperty::isConstant() const```</span>
  ///
  ///
  pub fn is_constant(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMetaProperty_isConstant(self as *const ::meta_property::MetaProperty) }
  }

  /// C++ method: <span style='color: green;'>```bool QMetaProperty::isDesignable() const```</span>
  ///
  ///
  pub fn is_designable(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMetaProperty_isDesignable_no_args(self as *const ::meta_property::MetaProperty) }
  }

  /// C++ method: <span style='color: green;'>```bool QMetaProperty::isDesignable(const QObject* obj = ?) const```</span>
  ///
  ///
  pub unsafe fn is_designable_unsafe(&self, obj: *const ::object::Object) -> bool {
    ::ffi::qt_core_c_QMetaProperty_isDesignable_obj(self as *const ::meta_property::MetaProperty, obj)
  }

  /// C++ method: <span style='color: green;'>```bool QMetaProperty::isEditable() const```</span>
  ///
  ///
  pub fn is_editable(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMetaProperty_isEditable_no_args(self as *const ::meta_property::MetaProperty) }
  }

  /// C++ method: <span style='color: green;'>```bool QMetaProperty::isEditable(const QObject* obj = ?) const```</span>
  ///
  ///
  pub unsafe fn is_editable_unsafe(&self, obj: *const ::object::Object) -> bool {
    ::ffi::qt_core_c_QMetaProperty_isEditable_obj(self as *const ::meta_property::MetaProperty, obj)
  }

  /// C++ method: <span style='color: green;'>```bool QMetaProperty::isEnumType() const```</span>
  ///
  ///
  pub fn is_enum_type(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMetaProperty_isEnumType(self as *const ::meta_property::MetaProperty) }
  }

  /// C++ method: <span style='color: green;'>```bool QMetaProperty::isFinal() const```</span>
  ///
  ///
  pub fn is_final(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMetaProperty_isFinal(self as *const ::meta_property::MetaProperty) }
  }

  /// C++ method: <span style='color: green;'>```bool QMetaProperty::isFlagType() const```</span>
  ///
  ///
  pub fn is_flag_type(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMetaProperty_isFlagType(self as *const ::meta_property::MetaProperty) }
  }

  /// C++ method: <span style='color: green;'>```bool QMetaProperty::isReadable() const```</span>
  ///
  ///
  pub fn is_readable(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMetaProperty_isReadable(self as *const ::meta_property::MetaProperty) }
  }

  /// C++ method: <span style='color: green;'>```bool QMetaProperty::isResettable() const```</span>
  ///
  ///
  pub fn is_resettable(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMetaProperty_isResettable(self as *const ::meta_property::MetaProperty) }
  }

  /// C++ method: <span style='color: green;'>```bool QMetaProperty::isScriptable() const```</span>
  ///
  ///
  pub fn is_scriptable(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMetaProperty_isScriptable_no_args(self as *const ::meta_property::MetaProperty) }
  }

  /// C++ method: <span style='color: green;'>```bool QMetaProperty::isScriptable(const QObject* obj = ?) const```</span>
  ///
  ///
  pub unsafe fn is_scriptable_unsafe(&self, obj: *const ::object::Object) -> bool {
    ::ffi::qt_core_c_QMetaProperty_isScriptable_obj(self as *const ::meta_property::MetaProperty, obj)
  }

  /// C++ method: <span style='color: green;'>```bool QMetaProperty::isStored() const```</span>
  ///
  ///
  pub fn is_stored(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMetaProperty_isStored_no_args(self as *const ::meta_property::MetaProperty) }
  }

  /// C++ method: <span style='color: green;'>```bool QMetaProperty::isStored(const QObject* obj = ?) const```</span>
  ///
  ///
  pub unsafe fn is_stored_unsafe(&self, obj: *const ::object::Object) -> bool {
    ::ffi::qt_core_c_QMetaProperty_isStored_obj(self as *const ::meta_property::MetaProperty, obj)
  }

  /// C++ method: <span style='color: green;'>```bool QMetaProperty::isUser() const```</span>
  ///
  ///
  pub fn is_user(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMetaProperty_isUser_no_args(self as *const ::meta_property::MetaProperty) }
  }

  /// C++ method: <span style='color: green;'>```bool QMetaProperty::isUser(const QObject* obj = ?) const```</span>
  ///
  ///
  pub unsafe fn is_user_unsafe(&self, obj: *const ::object::Object) -> bool {
    ::ffi::qt_core_c_QMetaProperty_isUser_obj(self as *const ::meta_property::MetaProperty, obj)
  }

  /// C++ method: <span style='color: green;'>```bool QMetaProperty::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMetaProperty_isValid(self as *const ::meta_property::MetaProperty) }
  }

  /// C++ method: <span style='color: green;'>```bool QMetaProperty::isWritable() const```</span>
  ///
  ///
  pub fn is_writable(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMetaProperty_isWritable(self as *const ::meta_property::MetaProperty) }
  }

  /// C++ method: <span style='color: green;'>```const char* QMetaProperty::name() const```</span>
  ///
  ///
  pub fn name(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QMetaProperty_name(self as *const ::meta_property::MetaProperty) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QMetaProperty::QMetaProperty()```</span>
  ///
  ///
  pub fn new() -> ::meta_property::MetaProperty {
    {
      let mut object: ::meta_property::MetaProperty =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMetaProperty_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMetaMethod QMetaProperty::notifySignal() const```</span>
  ///
  ///
  pub fn notify_signal(&self) -> ::meta_method::MetaMethod {
    {
      let mut object: ::meta_method::MetaMethod =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMetaProperty_notifySignal_to_output(self as *const ::meta_property::MetaProperty,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QMetaProperty::notifySignalIndex() const```</span>
  ///
  ///
  pub fn notify_signal_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMetaProperty_notifySignalIndex(self as *const ::meta_property::MetaProperty) }
  }

  /// C++ method: <span style='color: green;'>```int QMetaProperty::propertyIndex() const```</span>
  ///
  ///
  pub fn property_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMetaProperty_propertyIndex(self as *const ::meta_property::MetaProperty) }
  }

  /// C++ method: <span style='color: green;'>```QVariant QMetaProperty::read(const QObject* obj) const```</span>
  ///
  ///
  pub unsafe fn read(&self, obj: *const ::object::Object) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QMetaProperty_read_to_output(self as *const ::meta_property::MetaProperty,
                                                    obj,
                                                    &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVariant QMetaProperty::readOnGadget(const void* gadget) const```</span>
  ///
  ///
  pub unsafe fn read_on_gadget(&self, gadget: *const ::libc::c_void) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QMetaProperty_readOnGadget_to_output(self as *const ::meta_property::MetaProperty,
                                                            gadget,
                                                            &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QMetaProperty::reset(QObject* obj) const```</span>
  ///
  ///
  pub unsafe fn reset(&self, obj: *mut ::object::Object) -> bool {
    ::ffi::qt_core_c_QMetaProperty_reset(self as *const ::meta_property::MetaProperty, obj)
  }

  /// C++ method: <span style='color: green;'>```bool QMetaProperty::resetOnGadget(void* gadget) const```</span>
  ///
  ///
  pub unsafe fn reset_on_gadget(&self, gadget: *mut ::libc::c_void) -> bool {
    ::ffi::qt_core_c_QMetaProperty_resetOnGadget(self as *const ::meta_property::MetaProperty, gadget)
  }

  /// C++ method: <span style='color: green;'>```int QMetaProperty::revision() const```</span>
  ///
  ///
  pub fn revision(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMetaProperty_revision(self as *const ::meta_property::MetaProperty) }
  }

  /// C++ method: <span style='color: green;'>```const char* QMetaProperty::typeName() const```</span>
  ///
  ///
  pub fn type_name(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QMetaProperty_typeName(self as *const ::meta_property::MetaProperty) }
  }

  /// C++ method: <span style='color: green;'>```int QMetaProperty::userType() const```</span>
  ///
  ///
  pub fn user_type(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMetaProperty_userType(self as *const ::meta_property::MetaProperty) }
  }

  /// C++ method: <span style='color: green;'>```bool QMetaProperty::write(QObject* obj, const QVariant& value) const```</span>
  ///
  ///
  pub unsafe fn write(&self, obj: *mut ::object::Object, value: &::variant::Variant) -> bool {
    ::ffi::qt_core_c_QMetaProperty_write(self as *const ::meta_property::MetaProperty,
                                         obj,
                                         value as *const ::variant::Variant)
  }

  /// C++ method: <span style='color: green;'>```bool QMetaProperty::writeOnGadget(void* gadget, const QVariant& value) const```</span>
  ///
  ///
  pub unsafe fn write_on_gadget(&self, gadget: *mut ::libc::c_void, value: &::variant::Variant) -> bool {
    ::ffi::qt_core_c_QMetaProperty_writeOnGadget(self as *const ::meta_property::MetaProperty,
                                                 gadget,
                                                 value as *const ::variant::Variant)
  }
}

impl Drop for ::meta_property::MetaProperty {
  /// C++ method: <span style='color: green;'>```[destructor] void QMetaProperty::~QMetaProperty()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QMetaProperty_destructor(self as *mut ::meta_property::MetaProperty) }
  }
}

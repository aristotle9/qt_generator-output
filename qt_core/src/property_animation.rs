/// C++ type: <span style='color: green;'>```QPropertyAnimation```</span>
#[repr(C)]
pub struct PropertyAnimation(u8);

impl PropertyAnimation {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QPropertyAnimation::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QPropertyAnimation_metaObject(self as *const ::property_animation::PropertyAnimation) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QPropertyAnimation::QPropertyAnimation()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::property_animation::PropertyAnimation> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPropertyAnimation_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QPropertyAnimation::QPropertyAnimation```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::object::Object) -> ::cpp_utils::CppBox<::property_animation::PropertyAnimation>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPropertyAnimation::QPropertyAnimation(QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::object::Object, &::byte_array::ByteArray)) -> ::cpp_utils::CppBox<::property_animation::PropertyAnimation>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPropertyAnimation::QPropertyAnimation(QObject* target, const QByteArray& propertyName)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::object::Object, &::byte_array::ByteArray, *mut ::object::Object)) -> ::cpp_utils::CppBox<::property_animation::PropertyAnimation>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPropertyAnimation::QPropertyAnimation(QObject* target, const QByteArray& propertyName, QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::property_animation::PropertyAnimation>
    where Args: overloading::PropertyAnimationNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QByteArray QPropertyAnimation::propertyName() const```</span>
  ///
  ///
  pub fn property_name(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QPropertyAnimation_propertyName_to_output(self as *const ::property_animation::PropertyAnimation, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QPropertyAnimation::setPropertyName(const QByteArray& propertyName)```</span>
  ///
  ///
  pub fn set_property_name(&mut self, property_name: &::byte_array::ByteArray) {
    unsafe {
      ::ffi::qt_core_c_QPropertyAnimation_setPropertyName(self as *mut ::property_animation::PropertyAnimation,
                                                          property_name as *const ::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPropertyAnimation::setTargetObject(QObject* target)```</span>
  ///
  ///
  pub unsafe fn set_target_object(&mut self, target: *mut ::object::Object) {
    ::ffi::qt_core_c_QPropertyAnimation_setTargetObject(self as *mut ::property_animation::PropertyAnimation, target)
  }

  /// C++ method: <span style='color: green;'>```QObject* QPropertyAnimation::targetObject() const```</span>
  ///
  ///
  pub fn target_object(&self) -> *mut ::object::Object {
    unsafe { ::ffi::qt_core_c_QPropertyAnimation_targetObject(self as *const ::property_animation::PropertyAnimation) }
  }

  /// C++ method: <span style='color: green;'>```static QString QPropertyAnimation::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QPropertyAnimation_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QPropertyAnimation::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QPropertyAnimation_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::property_animation::PropertyAnimation {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QPropertyAnimation_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `PropertyAnimation`.
  pub struct Signals<'a>(&'a ::property_animation::PropertyAnimation);
  /// Represents a built-in Qt signal `QPropertyAnimation::valueChanged`.
  ///
  /// An object of this type can be created from `PropertyAnimation` with `object.signals().value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PropertyAnimation` object.
  pub struct ValueChanged<'a>(&'a ::property_animation::PropertyAnimation);
  impl<'a> ::connection::Receiver for ValueChanged<'a> {
    type Arguments = (&'static ::variant::Variant,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2valueChanged(const QVariant&)\0"
    }
  }
  impl<'a> ::connection::Signal for ValueChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QPropertyAnimation::valueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn value_changed(&self) -> ValueChanged {
      ValueChanged(self.0)
    }
  }
  impl ::property_animation::PropertyAnimation {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::property_animation::PropertyAnimation> for ::abstract_animation::AbstractAnimation {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::property_animation::PropertyAnimation> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPropertyAnimation_G_dynamic_cast_QPropertyAnimation_ptr_QAbstractAnimation(self as *mut ::abstract_animation::AbstractAnimation) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::property_animation::PropertyAnimation> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPropertyAnimation_G_dynamic_cast_QPropertyAnimation_ptr_QAbstractAnimation(self as *const ::abstract_animation::AbstractAnimation as *mut ::abstract_animation::AbstractAnimation) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::property_animation::PropertyAnimation> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::property_animation::PropertyAnimation> {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QPropertyAnimation_G_dynamic_cast_QPropertyAnimation_ptr_QObject(self as *mut ::object::Object)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::property_animation::PropertyAnimation> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPropertyAnimation_G_dynamic_cast_QPropertyAnimation_ptr_QObject(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::property_animation::PropertyAnimation> for ::variant_animation::VariantAnimation {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::property_animation::PropertyAnimation> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPropertyAnimation_G_dynamic_cast_QPropertyAnimation_ptr_QVariantAnimation(self as *mut ::variant_animation::VariantAnimation) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::property_animation::PropertyAnimation> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPropertyAnimation_G_dynamic_cast_QPropertyAnimation_ptr_QVariantAnimation(self as *const ::variant_animation::VariantAnimation as *mut ::variant_animation::VariantAnimation) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::abstract_animation::AbstractAnimation> for ::property_animation::PropertyAnimation {
  fn static_cast_mut(&mut self) -> &mut ::abstract_animation::AbstractAnimation {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPropertyAnimation_G_static_cast_QAbstractAnimation_ptr(self as *mut ::property_animation::PropertyAnimation) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_animation::AbstractAnimation {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPropertyAnimation_G_static_cast_QAbstractAnimation_ptr(self as *const ::property_animation::PropertyAnimation as *mut ::property_animation::PropertyAnimation) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::property_animation::PropertyAnimation {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPropertyAnimation_G_static_cast_QObject_ptr(self as *mut ::property_animation::PropertyAnimation) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPropertyAnimation_G_static_cast_QObject_ptr(self as *const ::property_animation::PropertyAnimation as *mut ::property_animation::PropertyAnimation) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::variant_animation::VariantAnimation> for ::property_animation::PropertyAnimation {
  fn static_cast_mut(&mut self) -> &mut ::variant_animation::VariantAnimation {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPropertyAnimation_G_static_cast_QVariantAnimation_ptr(self as *mut ::property_animation::PropertyAnimation) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::variant_animation::VariantAnimation {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPropertyAnimation_G_static_cast_QVariantAnimation_ptr(self as *const ::property_animation::PropertyAnimation as *mut ::property_animation::PropertyAnimation) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::property_animation::PropertyAnimation> for ::abstract_animation::AbstractAnimation {
unsafe fn static_cast_mut(&mut self) -> &mut ::property_animation::PropertyAnimation {
let ffi_result = ::ffi::qt_core_c_QPropertyAnimation_G_static_cast_QPropertyAnimation_ptr_QAbstractAnimation(self as *mut ::abstract_animation::AbstractAnimation);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::property_animation::PropertyAnimation {
let ffi_result = ::ffi::qt_core_c_QPropertyAnimation_G_static_cast_QPropertyAnimation_ptr_QAbstractAnimation(self as *const ::abstract_animation::AbstractAnimation as *mut ::abstract_animation::AbstractAnimation);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::property_animation::PropertyAnimation> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::property_animation::PropertyAnimation {
    let ffi_result =
      ::ffi::qt_core_c_QPropertyAnimation_G_static_cast_QPropertyAnimation_ptr_QObject(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::property_animation::PropertyAnimation {
    let ffi_result = ::ffi::qt_core_c_QPropertyAnimation_G_static_cast_QPropertyAnimation_ptr_QObject(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::property_animation::PropertyAnimation> for ::variant_animation::VariantAnimation {
unsafe fn static_cast_mut(&mut self) -> &mut ::property_animation::PropertyAnimation {
let ffi_result = ::ffi::qt_core_c_QPropertyAnimation_G_static_cast_QPropertyAnimation_ptr_QVariantAnimation(self as *mut ::variant_animation::VariantAnimation);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::property_animation::PropertyAnimation {
let ffi_result = ::ffi::qt_core_c_QPropertyAnimation_G_static_cast_QPropertyAnimation_ptr_QVariantAnimation(self as *const ::variant_animation::VariantAnimation as *mut ::variant_animation::VariantAnimation);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::property_animation::PropertyAnimation {
  type Target = ::variant_animation::VariantAnimation;
  fn deref(&self) -> &::variant_animation::VariantAnimation {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPropertyAnimation_G_static_cast_QVariantAnimation_ptr(self as *const ::property_animation::PropertyAnimation as *mut ::property_animation::PropertyAnimation) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::property_animation::PropertyAnimation {
  fn deref_mut(&mut self) -> &mut ::variant_animation::VariantAnimation {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPropertyAnimation_G_static_cast_QVariantAnimation_ptr(self as *mut ::property_animation::PropertyAnimation) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [PropertyAnimation::new_unsafe](../struct.PropertyAnimation.html#method.new_unsafe) method.
  pub trait PropertyAnimationNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::property_animation::PropertyAnimation>;
  }
  impl PropertyAnimationNewUnsafeArgs for *mut ::object::Object {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::property_animation::PropertyAnimation> {
      let parent = self;
      let ffi_result = ::ffi::qt_core_c_QPropertyAnimation_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> PropertyAnimationNewUnsafeArgs for (*mut ::object::Object, &'a ::byte_array::ByteArray) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::property_animation::PropertyAnimation> {
      let target = self.0;
      let property_name = self.1;
      let ffi_result =
        ::ffi::qt_core_c_QPropertyAnimation_new_target_propertyName(target,
                                                                    property_name as *const ::byte_array::ByteArray);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> PropertyAnimationNewUnsafeArgs
    for (*mut ::object::Object, &'a ::byte_array::ByteArray, *mut ::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::property_animation::PropertyAnimation> {
      let target = self.0;
      let property_name = self.1;
      let parent = self.2;
      let ffi_result = ::ffi::qt_core_c_QPropertyAnimation_new_target_propertyName_parent(target, property_name as *const ::byte_array::ByteArray, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}

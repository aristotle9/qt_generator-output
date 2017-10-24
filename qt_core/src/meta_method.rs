/// C++ type: <span style='color: green;'>```QMetaMethod::Access```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Access {
  /// C++ enum variant: <span style='color: green;'>```Private = 0```</span>
  Private = 0,
  /// C++ enum variant: <span style='color: green;'>```Protected = 1```</span>
  Protected = 1,
  /// C++ enum variant: <span style='color: green;'>```Public = 2```</span>
  Public = 2,
}

/// C++ type: <span style='color: green;'>```QMetaMethod::Attributes```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Attributes {
  /// C++ enum variant: <span style='color: green;'>```Compatibility = 1```</span>
  Compatibility = 1,
  /// C++ enum variant: <span style='color: green;'>```Cloned = 2```</span>
  Cloned = 2,
  /// C++ enum variant: <span style='color: green;'>```Scriptable = 4```</span>
  Scriptable = 4,
}

/// C++ type: <span style='color: green;'>```QMetaMethod```</span>
#[repr(C)]
pub struct MetaMethod([u8; ::type_sizes::QT_CORE_META_METHOD_META_METHOD]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for MetaMethod {
  unsafe fn new_uninitialized() -> MetaMethod {
    MetaMethod(::std::mem::uninitialized())
  }
}

impl MetaMethod {
  /// C++ method: <span style='color: green;'>```QMetaMethod::Access QMetaMethod::access() const```</span>
  ///
  ///
  pub fn access(&self) -> ::meta_method::Access {
    unsafe { ::ffi::qt_core_c_QMetaMethod_access(self as *const ::meta_method::MetaMethod) }
  }

  /// C++ method: <span style='color: green;'>```int QMetaMethod::attributes() const```</span>
  ///
  ///
  pub fn attributes(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMetaMethod_attributes(self as *const ::meta_method::MetaMethod) }
  }

  /// C++ method: <span style='color: green;'>```const QMetaObject* QMetaMethod::enclosingMetaObject() const```</span>
  ///
  ///
  pub fn enclosing_meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QMetaMethod_enclosingMetaObject(self as *const ::meta_method::MetaMethod) }
  }

  /// C++ method: <span style='color: green;'>```void QMetaMethod::getParameterTypes(int* types) const```</span>
  ///
  ///
  pub unsafe fn get_parameter_types(&self, types: *mut ::libc::c_int) {
    ::ffi::qt_core_c_QMetaMethod_getParameterTypes(self as *const ::meta_method::MetaMethod, types)
  }

  /// C++ method: <span style='color: green;'>```QMetaMethod::invoke```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn invoke(&self, *mut ::object::Object) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, QGenericArgument val0 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, QGenericArgument val0 = ?, QGenericArgument val1 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?, QGenericArgument val8 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?, QGenericArgument val8 = ?, QGenericArgument val9 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 12
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::generic_return_argument::GenericReturnArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, QGenericReturnArgument returnValue) const```</span>
  ///
  ///
  ///
  /// ## Variant 13
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, QGenericReturnArgument returnValue, QGenericArgument val0 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 14
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 15
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 16
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 17
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 18
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 19
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 20
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 21
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?, QGenericArgument val8 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 22
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?, QGenericArgument val8 = ?, QGenericArgument val9 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 23
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::qt::ConnectionType)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, Qt::ConnectionType connectionType) const```</span>
  ///
  ///
  ///
  /// ## Variant 24
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::qt::ConnectionType, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, Qt::ConnectionType connectionType, QGenericArgument val0 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 25
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::qt::ConnectionType, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, Qt::ConnectionType connectionType, QGenericArgument val0 = ?, QGenericArgument val1 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 26
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::qt::ConnectionType, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, Qt::ConnectionType connectionType, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 27
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::qt::ConnectionType, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, Qt::ConnectionType connectionType, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 28
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::qt::ConnectionType, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, Qt::ConnectionType connectionType, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 29
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::qt::ConnectionType, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, Qt::ConnectionType connectionType, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 30
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::qt::ConnectionType, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, Qt::ConnectionType connectionType, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 31
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::qt::ConnectionType, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, Qt::ConnectionType connectionType, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 32
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::qt::ConnectionType, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, Qt::ConnectionType connectionType, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?, QGenericArgument val8 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 33
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::qt::ConnectionType, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, Qt::ConnectionType connectionType, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?, QGenericArgument val8 = ?, QGenericArgument val9 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 34
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::qt::ConnectionType, &::generic_return_argument::GenericReturnArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, Qt::ConnectionType connectionType, QGenericReturnArgument returnValue) const```</span>
  ///
  ///
  ///
  /// ## Variant 35
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::qt::ConnectionType, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, Qt::ConnectionType connectionType, QGenericReturnArgument returnValue, QGenericArgument val0 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 36
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::qt::ConnectionType, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, Qt::ConnectionType connectionType, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 37
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::qt::ConnectionType, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, Qt::ConnectionType connectionType, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 38
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::qt::ConnectionType, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, Qt::ConnectionType connectionType, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 39
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::qt::ConnectionType, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, Qt::ConnectionType connectionType, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 40
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::qt::ConnectionType, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, Qt::ConnectionType connectionType, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 41
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::qt::ConnectionType, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, Qt::ConnectionType connectionType, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 42
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::qt::ConnectionType, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, Qt::ConnectionType connectionType, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 43
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::qt::ConnectionType, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, Qt::ConnectionType connectionType, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?, QGenericArgument val8 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 44
  ///
  /// Rust arguments: ```fn invoke(&self, (*mut ::object::Object, &::qt::ConnectionType, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invoke(QObject* object, Qt::ConnectionType connectionType, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?, QGenericArgument val8 = ?, QGenericArgument val9 = ?) const```</span>
  ///
  ///
  pub unsafe fn invoke<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::MetaMethodInvokeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMetaMethod::invokeOnGadget```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn invoke_on_gadget(&self, *mut ::libc::c_void) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invokeOnGadget(void* gadget) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn invoke_on_gadget(&self, (*mut ::libc::c_void, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invokeOnGadget(void* gadget, QGenericArgument val0 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn invoke_on_gadget(&self, (*mut ::libc::c_void, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invokeOnGadget(void* gadget, QGenericArgument val0 = ?, QGenericArgument val1 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn invoke_on_gadget(&self, (*mut ::libc::c_void, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invokeOnGadget(void* gadget, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn invoke_on_gadget(&self, (*mut ::libc::c_void, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invokeOnGadget(void* gadget, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn invoke_on_gadget(&self, (*mut ::libc::c_void, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invokeOnGadget(void* gadget, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn invoke_on_gadget(&self, (*mut ::libc::c_void, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invokeOnGadget(void* gadget, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn invoke_on_gadget(&self, (*mut ::libc::c_void, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invokeOnGadget(void* gadget, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn invoke_on_gadget(&self, (*mut ::libc::c_void, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invokeOnGadget(void* gadget, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn invoke_on_gadget(&self, (*mut ::libc::c_void, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invokeOnGadget(void* gadget, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?, QGenericArgument val8 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn invoke_on_gadget(&self, (*mut ::libc::c_void, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invokeOnGadget(void* gadget, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?, QGenericArgument val8 = ?, QGenericArgument val9 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 12
  ///
  /// Rust arguments: ```fn invoke_on_gadget(&self, (*mut ::libc::c_void, &::generic_return_argument::GenericReturnArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invokeOnGadget(void* gadget, QGenericReturnArgument returnValue) const```</span>
  ///
  ///
  ///
  /// ## Variant 13
  ///
  /// Rust arguments: ```fn invoke_on_gadget(&self, (*mut ::libc::c_void, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invokeOnGadget(void* gadget, QGenericReturnArgument returnValue, QGenericArgument val0 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 14
  ///
  /// Rust arguments: ```fn invoke_on_gadget(&self, (*mut ::libc::c_void, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invokeOnGadget(void* gadget, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 15
  ///
  /// Rust arguments: ```fn invoke_on_gadget(&self, (*mut ::libc::c_void, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invokeOnGadget(void* gadget, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 16
  ///
  /// Rust arguments: ```fn invoke_on_gadget(&self, (*mut ::libc::c_void, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invokeOnGadget(void* gadget, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 17
  ///
  /// Rust arguments: ```fn invoke_on_gadget(&self, (*mut ::libc::c_void, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invokeOnGadget(void* gadget, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 18
  ///
  /// Rust arguments: ```fn invoke_on_gadget(&self, (*mut ::libc::c_void, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invokeOnGadget(void* gadget, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 19
  ///
  /// Rust arguments: ```fn invoke_on_gadget(&self, (*mut ::libc::c_void, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invokeOnGadget(void* gadget, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 20
  ///
  /// Rust arguments: ```fn invoke_on_gadget(&self, (*mut ::libc::c_void, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invokeOnGadget(void* gadget, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 21
  ///
  /// Rust arguments: ```fn invoke_on_gadget(&self, (*mut ::libc::c_void, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invokeOnGadget(void* gadget, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?, QGenericArgument val8 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 22
  ///
  /// Rust arguments: ```fn invoke_on_gadget(&self, (*mut ::libc::c_void, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::invokeOnGadget(void* gadget, QGenericReturnArgument returnValue, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?, QGenericArgument val8 = ?, QGenericArgument val9 = ?) const```</span>
  ///
  ///
  pub unsafe fn invoke_on_gadget<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::MetaMethodInvokeOnGadgetArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QMetaMethod::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMetaMethod_isValid(self as *const ::meta_method::MetaMethod) }
  }

  /// C++ method: <span style='color: green;'>```int QMetaMethod::methodIndex() const```</span>
  ///
  ///
  pub fn method_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMetaMethod_methodIndex(self as *const ::meta_method::MetaMethod) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QMetaMethod::methodSignature() const```</span>
  ///
  ///
  pub fn method_signature(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMetaMethod_methodSignature_to_output(self as *const ::meta_method::MetaMethod, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMetaMethod::MethodType QMetaMethod::methodType() const```</span>
  ///
  ///
  pub fn method_type(&self) -> ::meta_method::MethodType {
    unsafe { ::ffi::qt_core_c_QMetaMethod_methodType(self as *const ::meta_method::MetaMethod) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QMetaMethod::name() const```</span>
  ///
  ///
  pub fn name(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMetaMethod_name_to_output(self as *const ::meta_method::MetaMethod, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QMetaMethod::QMetaMethod()```</span>
  ///
  ///
  pub fn new() -> ::meta_method::MetaMethod {
    {
      let mut object: ::meta_method::MetaMethod =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMetaMethod_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QMetaMethod::parameterCount() const```</span>
  ///
  ///
  pub fn parameter_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMetaMethod_parameterCount(self as *const ::meta_method::MetaMethod) }
  }

  /// C++ method: <span style='color: green;'>```QList<QByteArray> QMetaMethod::parameterNames() const```</span>
  ///
  ///
  pub fn parameter_names(&self) -> ::list::ListByteArray {
    {
      let mut object: ::list::ListByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMetaMethod_parameterNames_to_output(self as *const ::meta_method::MetaMethod, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QMetaMethod::parameterType(int index) const```</span>
  ///
  ///
  pub fn parameter_type(&self, index: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMetaMethod_parameterType(self as *const ::meta_method::MetaMethod, index) }
  }

  /// C++ method: <span style='color: green;'>```QList<QByteArray> QMetaMethod::parameterTypes() const```</span>
  ///
  ///
  pub fn parameter_types(&self) -> ::list::ListByteArray {
    {
      let mut object: ::list::ListByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMetaMethod_parameterTypes_to_output(self as *const ::meta_method::MetaMethod, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QMetaMethod::returnType() const```</span>
  ///
  ///
  pub fn return_type(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMetaMethod_returnType(self as *const ::meta_method::MetaMethod) }
  }

  /// C++ method: <span style='color: green;'>```int QMetaMethod::revision() const```</span>
  ///
  ///
  pub fn revision(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMetaMethod_revision(self as *const ::meta_method::MetaMethod) }
  }

  /// C++ method: <span style='color: green;'>```const char* QMetaMethod::tag() const```</span>
  ///
  ///
  pub fn tag(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QMetaMethod_tag(self as *const ::meta_method::MetaMethod) }
  }

  /// C++ method: <span style='color: green;'>```const char* QMetaMethod::typeName() const```</span>
  ///
  ///
  pub fn type_name(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QMetaMethod_typeName(self as *const ::meta_method::MetaMethod) }
  }
}

impl Drop for ::meta_method::MetaMethod {
  /// C++ method: <span style='color: green;'>```[destructor] void QMetaMethod::~QMetaMethod()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QMetaMethod_destructor(self as *mut ::meta_method::MetaMethod) }
  }
}

/// C++ type: <span style='color: green;'>```QMetaMethod::MethodType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum MethodType {
  /// C++ enum variant: <span style='color: green;'>```Method = 0```</span>
  Method = 0,
  /// C++ enum variant: <span style='color: green;'>```Signal = 1```</span>
  Signal = 1,
  /// C++ enum variant: <span style='color: green;'>```Slot = 2```</span>
  Slot = 2,
  /// C++ enum variant: <span style='color: green;'>```Constructor = 3```</span>
  Constructor = 3,
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [MetaMethod::invoke](../struct.MetaMethod.html#method.invoke) method.
  pub trait MetaMethodInvokeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool;
  }
  impl<'largs> MetaMethodInvokeArgs<'largs> for *mut ::object::Object {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self;
      ::ffi::qt_core_c_QMetaMethod_invoke_object(original_self as *const ::meta_method::MetaMethod, object)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs> for (*mut ::object::Object, &'largs ::qt::ConnectionType) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let connection_type = self.1;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_connectionType(original_self as *const ::meta_method::MetaMethod,
                                                                object,
                                                                connection_type as *const ::qt::ConnectionType)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::qt::ConnectionType,
                                                     &'largs ::generic_return_argument::GenericReturnArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let connection_type = self.1;
      let return_value = self.2;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_connectionType_returnValue(original_self as *const ::meta_method::MetaMethod, object, connection_type as *const ::qt::ConnectionType, return_value as *const ::generic_return_argument::GenericReturnArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::qt::ConnectionType,
                                                     &'largs ::generic_return_argument::GenericReturnArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let connection_type = self.1;
      let return_value = self.2;
      let val0 = self.3;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_connectionType_returnValue_val0(original_self as *const ::meta_method::MetaMethod, object, connection_type as *const ::qt::ConnectionType, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::qt::ConnectionType,
                                                     &'largs ::generic_return_argument::GenericReturnArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let connection_type = self.1;
      let return_value = self.2;
      let val0 = self.3;
      let val1 = self.4;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_connectionType_returnValue_val0_val1(original_self as *const ::meta_method::MetaMethod, object, connection_type as *const ::qt::ConnectionType, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::qt::ConnectionType,
                                                     &'largs ::generic_return_argument::GenericReturnArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let connection_type = self.1;
      let return_value = self.2;
      let val0 = self.3;
      let val1 = self.4;
      let val2 = self.5;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_connectionType_returnValue_val0_val1_val2(original_self as *const ::meta_method::MetaMethod, object, connection_type as *const ::qt::ConnectionType, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::qt::ConnectionType,
                                                     &'largs ::generic_return_argument::GenericReturnArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let connection_type = self.1;
      let return_value = self.2;
      let val0 = self.3;
      let val1 = self.4;
      let val2 = self.5;
      let val3 = self.6;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_connectionType_returnValue_val0_val1_val2_val3(original_self as *const ::meta_method::MetaMethod, object, connection_type as *const ::qt::ConnectionType, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::qt::ConnectionType,
                                                     &'largs ::generic_return_argument::GenericReturnArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let connection_type = self.1;
      let return_value = self.2;
      let val0 = self.3;
      let val1 = self.4;
      let val2 = self.5;
      let val3 = self.6;
      let val4 = self.7;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_connectionType_returnValue_val0_val1_val2_val3_val4(original_self as *const ::meta_method::MetaMethod, object, connection_type as *const ::qt::ConnectionType, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::qt::ConnectionType,
                                                     &'largs ::generic_return_argument::GenericReturnArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let connection_type = self.1;
      let return_value = self.2;
      let val0 = self.3;
      let val1 = self.4;
      let val2 = self.5;
      let val3 = self.6;
      let val4 = self.7;
      let val5 = self.8;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_connectionType_returnValue_val0_val1_val2_val3_val4_val5(original_self as *const ::meta_method::MetaMethod, object, connection_type as *const ::qt::ConnectionType, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::qt::ConnectionType,
                                                     &'largs ::generic_return_argument::GenericReturnArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let connection_type = self.1;
      let return_value = self.2;
      let val0 = self.3;
      let val1 = self.4;
      let val2 = self.5;
      let val3 = self.6;
      let val4 = self.7;
      let val5 = self.8;
      let val6 = self.9;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_connectionType_returnValue_val0_val1_val2_val3_val4_val5_val6(original_self as *const ::meta_method::MetaMethod, object, connection_type as *const ::qt::ConnectionType, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::qt::ConnectionType,
                                                     &'largs ::generic_return_argument::GenericReturnArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let connection_type = self.1;
      let return_value = self.2;
      let val0 = self.3;
      let val1 = self.4;
      let val2 = self.5;
      let val3 = self.6;
      let val4 = self.7;
      let val5 = self.8;
      let val6 = self.9;
      let val7 = self.10;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_connectionType_returnValue_val0_val1_val2_val3_val4_val5_val6_val7(original_self as *const ::meta_method::MetaMethod, object, connection_type as *const ::qt::ConnectionType, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::qt::ConnectionType,
                                                     &'largs ::generic_return_argument::GenericReturnArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let connection_type = self.1;
      let return_value = self.2;
      let val0 = self.3;
      let val1 = self.4;
      let val2 = self.5;
      let val3 = self.6;
      let val4 = self.7;
      let val5 = self.8;
      let val6 = self.9;
      let val7 = self.10;
      let val8 = self.11;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_connectionType_returnValue_val0_val1_val2_val3_val4_val5_val6_val7_val8(original_self as *const ::meta_method::MetaMethod, object, connection_type as *const ::qt::ConnectionType, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument, val8 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::qt::ConnectionType,
                                                     &'largs ::generic_return_argument::GenericReturnArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let connection_type = self.1;
      let return_value = self.2;
      let val0 = self.3;
      let val1 = self.4;
      let val2 = self.5;
      let val3 = self.6;
      let val4 = self.7;
      let val5 = self.8;
      let val6 = self.9;
      let val7 = self.10;
      let val8 = self.11;
      let val9 = self.12;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_connectionType_returnValue_val0_val1_val2_val3_val4_val5_val6_val7_val8_val9(original_self as *const ::meta_method::MetaMethod, object, connection_type as *const ::qt::ConnectionType, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument, val8 as *const ::generic_argument::GenericArgument, val9 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object, &'largs ::qt::ConnectionType, &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let connection_type = self.1;
      let val0 = self.2;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_connectionType_val0(original_self as *const ::meta_method::MetaMethod, object, connection_type as *const ::qt::ConnectionType, val0 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::qt::ConnectionType,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let connection_type = self.1;
      let val0 = self.2;
      let val1 = self.3;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_connectionType_val0_val1(original_self as *const ::meta_method::MetaMethod, object, connection_type as *const ::qt::ConnectionType, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::qt::ConnectionType,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let connection_type = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_connectionType_val0_val1_val2(original_self as *const ::meta_method::MetaMethod, object, connection_type as *const ::qt::ConnectionType, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::qt::ConnectionType,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let connection_type = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_connectionType_val0_val1_val2_val3(original_self as *const ::meta_method::MetaMethod, object, connection_type as *const ::qt::ConnectionType, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::qt::ConnectionType,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let connection_type = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      let val4 = self.6;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_connectionType_val0_val1_val2_val3_val4(original_self as *const ::meta_method::MetaMethod, object, connection_type as *const ::qt::ConnectionType, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::qt::ConnectionType,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let connection_type = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      let val4 = self.6;
      let val5 = self.7;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_connectionType_val0_val1_val2_val3_val4_val5(original_self as *const ::meta_method::MetaMethod, object, connection_type as *const ::qt::ConnectionType, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::qt::ConnectionType,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let connection_type = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      let val4 = self.6;
      let val5 = self.7;
      let val6 = self.8;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_connectionType_val0_val1_val2_val3_val4_val5_val6(original_self as *const ::meta_method::MetaMethod, object, connection_type as *const ::qt::ConnectionType, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::qt::ConnectionType,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let connection_type = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      let val4 = self.6;
      let val5 = self.7;
      let val6 = self.8;
      let val7 = self.9;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_connectionType_val0_val1_val2_val3_val4_val5_val6_val7(original_self as *const ::meta_method::MetaMethod, object, connection_type as *const ::qt::ConnectionType, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::qt::ConnectionType,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let connection_type = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      let val4 = self.6;
      let val5 = self.7;
      let val6 = self.8;
      let val7 = self.9;
      let val8 = self.10;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_connectionType_val0_val1_val2_val3_val4_val5_val6_val7_val8(original_self as *const ::meta_method::MetaMethod, object, connection_type as *const ::qt::ConnectionType, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument, val8 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::qt::ConnectionType,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let connection_type = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      let val4 = self.6;
      let val5 = self.7;
      let val6 = self.8;
      let val7 = self.9;
      let val8 = self.10;
      let val9 = self.11;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_connectionType_val0_val1_val2_val3_val4_val5_val6_val7_val8_val9(original_self as *const ::meta_method::MetaMethod, object, connection_type as *const ::qt::ConnectionType, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument, val8 as *const ::generic_argument::GenericArgument, val9 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object, &'largs ::generic_return_argument::GenericReturnArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let return_value = self.1;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_returnValue(original_self as *const ::meta_method::MetaMethod, object, return_value as *const ::generic_return_argument::GenericReturnArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::generic_return_argument::GenericReturnArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let return_value = self.1;
      let val0 = self.2;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_returnValue_val0(original_self as *const ::meta_method::MetaMethod, object, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::generic_return_argument::GenericReturnArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let return_value = self.1;
      let val0 = self.2;
      let val1 = self.3;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_returnValue_val0_val1(original_self as *const ::meta_method::MetaMethod, object, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::generic_return_argument::GenericReturnArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let return_value = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_returnValue_val0_val1_val2(original_self as *const ::meta_method::MetaMethod, object, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::generic_return_argument::GenericReturnArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let return_value = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_returnValue_val0_val1_val2_val3(original_self as *const ::meta_method::MetaMethod, object, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::generic_return_argument::GenericReturnArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let return_value = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      let val4 = self.6;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_returnValue_val0_val1_val2_val3_val4(original_self as *const ::meta_method::MetaMethod, object, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::generic_return_argument::GenericReturnArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let return_value = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      let val4 = self.6;
      let val5 = self.7;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_returnValue_val0_val1_val2_val3_val4_val5(original_self as *const ::meta_method::MetaMethod, object, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::generic_return_argument::GenericReturnArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let return_value = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      let val4 = self.6;
      let val5 = self.7;
      let val6 = self.8;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_returnValue_val0_val1_val2_val3_val4_val5_val6(original_self as *const ::meta_method::MetaMethod, object, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::generic_return_argument::GenericReturnArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let return_value = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      let val4 = self.6;
      let val5 = self.7;
      let val6 = self.8;
      let val7 = self.9;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_returnValue_val0_val1_val2_val3_val4_val5_val6_val7(original_self as *const ::meta_method::MetaMethod, object, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::generic_return_argument::GenericReturnArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let return_value = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      let val4 = self.6;
      let val5 = self.7;
      let val6 = self.8;
      let val7 = self.9;
      let val8 = self.10;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_returnValue_val0_val1_val2_val3_val4_val5_val6_val7_val8(original_self as *const ::meta_method::MetaMethod, object, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument, val8 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::generic_return_argument::GenericReturnArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let return_value = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      let val4 = self.6;
      let val5 = self.7;
      let val6 = self.8;
      let val7 = self.9;
      let val8 = self.10;
      let val9 = self.11;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_returnValue_val0_val1_val2_val3_val4_val5_val6_val7_val8_val9(original_self as *const ::meta_method::MetaMethod, object, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument, val8 as *const ::generic_argument::GenericArgument, val9 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs> for (*mut ::object::Object, &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let val0 = self.1;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_val0(original_self as *const ::meta_method::MetaMethod,
                                                      object,
                                                      val0 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let val0 = self.1;
      let val1 = self.2;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_val0_val1(original_self as *const ::meta_method::MetaMethod,
                                                           object,
                                                           val0 as *const ::generic_argument::GenericArgument,
                                                           val1 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let val0 = self.1;
      let val1 = self.2;
      let val2 = self.3;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_val0_val1_val2(original_self as *const ::meta_method::MetaMethod,
                                                                object,
                                                                val0 as *const ::generic_argument::GenericArgument,
                                                                val1 as *const ::generic_argument::GenericArgument,
                                                                val2 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let val0 = self.1;
      let val1 = self.2;
      let val2 = self.3;
      let val3 = self.4;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_val0_val1_val2_val3(original_self as *const ::meta_method::MetaMethod, object, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let val0 = self.1;
      let val1 = self.2;
      let val2 = self.3;
      let val3 = self.4;
      let val4 = self.5;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_val0_val1_val2_val3_val4(original_self as *const ::meta_method::MetaMethod, object, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let val0 = self.1;
      let val1 = self.2;
      let val2 = self.3;
      let val3 = self.4;
      let val4 = self.5;
      let val5 = self.6;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_val0_val1_val2_val3_val4_val5(original_self as *const ::meta_method::MetaMethod, object, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let val0 = self.1;
      let val1 = self.2;
      let val2 = self.3;
      let val3 = self.4;
      let val4 = self.5;
      let val5 = self.6;
      let val6 = self.7;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_val0_val1_val2_val3_val4_val5_val6(original_self as *const ::meta_method::MetaMethod, object, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let val0 = self.1;
      let val1 = self.2;
      let val2 = self.3;
      let val3 = self.4;
      let val4 = self.5;
      let val5 = self.6;
      let val6 = self.7;
      let val7 = self.8;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_val0_val1_val2_val3_val4_val5_val6_val7(original_self as *const ::meta_method::MetaMethod, object, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let val0 = self.1;
      let val1 = self.2;
      let val2 = self.3;
      let val3 = self.4;
      let val4 = self.5;
      let val5 = self.6;
      let val6 = self.7;
      let val7 = self.8;
      let val8 = self.9;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_val0_val1_val2_val3_val4_val5_val6_val7_val8(original_self as *const ::meta_method::MetaMethod, object, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument, val8 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeArgs<'largs>
    for (*mut ::object::Object,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument,
                                                     &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let object = self.0;
      let val0 = self.1;
      let val1 = self.2;
      let val2 = self.3;
      let val3 = self.4;
      let val4 = self.5;
      let val5 = self.6;
      let val6 = self.7;
      let val7 = self.8;
      let val8 = self.9;
      let val9 = self.10;
      ::ffi::qt_core_c_QMetaMethod_invoke_object_val0_val1_val2_val3_val4_val5_val6_val7_val8_val9(original_self as *const ::meta_method::MetaMethod, object, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument, val8 as *const ::generic_argument::GenericArgument, val9 as *const ::generic_argument::GenericArgument)
    }
  }
  /// This trait represents a set of arguments accepted by [MetaMethod::invoke_on_gadget](../struct.MetaMethod.html#method.invoke_on_gadget) method.
  pub trait MetaMethodInvokeOnGadgetArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool;
  }
  impl<'largs> MetaMethodInvokeOnGadgetArgs<'largs> for *mut ::libc::c_void {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let gadget = self;
      ::ffi::qt_core_c_QMetaMethod_invokeOnGadget_gadget(original_self as *const ::meta_method::MetaMethod, gadget)
    }
  }
  impl<'largs> MetaMethodInvokeOnGadgetArgs<'largs>
    for (*mut ::libc::c_void, &'largs ::generic_return_argument::GenericReturnArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let gadget = self.0;
      let return_value = self.1;
      ::ffi::qt_core_c_QMetaMethod_invokeOnGadget_gadget_returnValue(original_self as *const ::meta_method::MetaMethod, gadget, return_value as *const ::generic_return_argument::GenericReturnArgument)
    }
  }
  impl<'largs> MetaMethodInvokeOnGadgetArgs<'largs>
    for (*mut ::libc::c_void,
                                                             &'largs ::generic_return_argument::GenericReturnArgument,
                                                             &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let gadget = self.0;
      let return_value = self.1;
      let val0 = self.2;
      ::ffi::qt_core_c_QMetaMethod_invokeOnGadget_gadget_returnValue_val0(original_self as *const ::meta_method::MetaMethod, gadget, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeOnGadgetArgs<'largs>
    for (*mut ::libc::c_void,
                                                             &'largs ::generic_return_argument::GenericReturnArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let gadget = self.0;
      let return_value = self.1;
      let val0 = self.2;
      let val1 = self.3;
      ::ffi::qt_core_c_QMetaMethod_invokeOnGadget_gadget_returnValue_val0_val1(original_self as *const ::meta_method::MetaMethod, gadget, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeOnGadgetArgs<'largs>
    for (*mut ::libc::c_void,
                                                             &'largs ::generic_return_argument::GenericReturnArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let gadget = self.0;
      let return_value = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      ::ffi::qt_core_c_QMetaMethod_invokeOnGadget_gadget_returnValue_val0_val1_val2(original_self as *const ::meta_method::MetaMethod, gadget, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeOnGadgetArgs<'largs>
    for (*mut ::libc::c_void,
                                                             &'largs ::generic_return_argument::GenericReturnArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let gadget = self.0;
      let return_value = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      ::ffi::qt_core_c_QMetaMethod_invokeOnGadget_gadget_returnValue_val0_val1_val2_val3(original_self as *const ::meta_method::MetaMethod, gadget, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeOnGadgetArgs<'largs>
    for (*mut ::libc::c_void,
                                                             &'largs ::generic_return_argument::GenericReturnArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let gadget = self.0;
      let return_value = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      let val4 = self.6;
      ::ffi::qt_core_c_QMetaMethod_invokeOnGadget_gadget_returnValue_val0_val1_val2_val3_val4(original_self as *const ::meta_method::MetaMethod, gadget, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeOnGadgetArgs<'largs>
    for (*mut ::libc::c_void,
                                                             &'largs ::generic_return_argument::GenericReturnArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let gadget = self.0;
      let return_value = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      let val4 = self.6;
      let val5 = self.7;
      ::ffi::qt_core_c_QMetaMethod_invokeOnGadget_gadget_returnValue_val0_val1_val2_val3_val4_val5(original_self as *const ::meta_method::MetaMethod, gadget, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeOnGadgetArgs<'largs>
    for (*mut ::libc::c_void,
                                                             &'largs ::generic_return_argument::GenericReturnArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let gadget = self.0;
      let return_value = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      let val4 = self.6;
      let val5 = self.7;
      let val6 = self.8;
      ::ffi::qt_core_c_QMetaMethod_invokeOnGadget_gadget_returnValue_val0_val1_val2_val3_val4_val5_val6(original_self as *const ::meta_method::MetaMethod, gadget, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeOnGadgetArgs<'largs>
    for (*mut ::libc::c_void,
                                                             &'largs ::generic_return_argument::GenericReturnArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let gadget = self.0;
      let return_value = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      let val4 = self.6;
      let val5 = self.7;
      let val6 = self.8;
      let val7 = self.9;
      ::ffi::qt_core_c_QMetaMethod_invokeOnGadget_gadget_returnValue_val0_val1_val2_val3_val4_val5_val6_val7(original_self as *const ::meta_method::MetaMethod, gadget, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeOnGadgetArgs<'largs>
    for (*mut ::libc::c_void,
                                                             &'largs ::generic_return_argument::GenericReturnArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let gadget = self.0;
      let return_value = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      let val4 = self.6;
      let val5 = self.7;
      let val6 = self.8;
      let val7 = self.9;
      let val8 = self.10;
      ::ffi::qt_core_c_QMetaMethod_invokeOnGadget_gadget_returnValue_val0_val1_val2_val3_val4_val5_val6_val7_val8(original_self as *const ::meta_method::MetaMethod, gadget, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument, val8 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeOnGadgetArgs<'largs>
    for (*mut ::libc::c_void,
                                                             &'largs ::generic_return_argument::GenericReturnArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let gadget = self.0;
      let return_value = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      let val4 = self.6;
      let val5 = self.7;
      let val6 = self.8;
      let val7 = self.9;
      let val8 = self.10;
      let val9 = self.11;
      ::ffi::qt_core_c_QMetaMethod_invokeOnGadget_gadget_returnValue_val0_val1_val2_val3_val4_val5_val6_val7_val8_val9(original_self as *const ::meta_method::MetaMethod, gadget, return_value as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument, val8 as *const ::generic_argument::GenericArgument, val9 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeOnGadgetArgs<'largs>
    for (*mut ::libc::c_void, &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let gadget = self.0;
      let val0 = self.1;
      ::ffi::qt_core_c_QMetaMethod_invokeOnGadget_gadget_val0(original_self as *const ::meta_method::MetaMethod,
                                                              gadget,
                                                              val0 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeOnGadgetArgs<'largs>
    for (*mut ::libc::c_void,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let gadget = self.0;
      let val0 = self.1;
      let val1 = self.2;
      ::ffi::qt_core_c_QMetaMethod_invokeOnGadget_gadget_val0_val1(original_self as *const ::meta_method::MetaMethod,
                                                                   gadget,
                                                                   val0 as *const ::generic_argument::GenericArgument,
                                                                   val1 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeOnGadgetArgs<'largs>
    for (*mut ::libc::c_void,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let gadget = self.0;
      let val0 = self.1;
      let val1 = self.2;
      let val2 = self.3;
      ::ffi::qt_core_c_QMetaMethod_invokeOnGadget_gadget_val0_val1_val2(original_self as *const ::meta_method::MetaMethod, gadget, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeOnGadgetArgs<'largs>
    for (*mut ::libc::c_void,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let gadget = self.0;
      let val0 = self.1;
      let val1 = self.2;
      let val2 = self.3;
      let val3 = self.4;
      ::ffi::qt_core_c_QMetaMethod_invokeOnGadget_gadget_val0_val1_val2_val3(original_self as *const ::meta_method::MetaMethod, gadget, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeOnGadgetArgs<'largs>
    for (*mut ::libc::c_void,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let gadget = self.0;
      let val0 = self.1;
      let val1 = self.2;
      let val2 = self.3;
      let val3 = self.4;
      let val4 = self.5;
      ::ffi::qt_core_c_QMetaMethod_invokeOnGadget_gadget_val0_val1_val2_val3_val4(original_self as *const ::meta_method::MetaMethod, gadget, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeOnGadgetArgs<'largs>
    for (*mut ::libc::c_void,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let gadget = self.0;
      let val0 = self.1;
      let val1 = self.2;
      let val2 = self.3;
      let val3 = self.4;
      let val4 = self.5;
      let val5 = self.6;
      ::ffi::qt_core_c_QMetaMethod_invokeOnGadget_gadget_val0_val1_val2_val3_val4_val5(original_self as *const ::meta_method::MetaMethod, gadget, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeOnGadgetArgs<'largs>
    for (*mut ::libc::c_void,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let gadget = self.0;
      let val0 = self.1;
      let val1 = self.2;
      let val2 = self.3;
      let val3 = self.4;
      let val4 = self.5;
      let val5 = self.6;
      let val6 = self.7;
      ::ffi::qt_core_c_QMetaMethod_invokeOnGadget_gadget_val0_val1_val2_val3_val4_val5_val6(original_self as *const ::meta_method::MetaMethod, gadget, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeOnGadgetArgs<'largs>
    for (*mut ::libc::c_void,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let gadget = self.0;
      let val0 = self.1;
      let val1 = self.2;
      let val2 = self.3;
      let val3 = self.4;
      let val4 = self.5;
      let val5 = self.6;
      let val6 = self.7;
      let val7 = self.8;
      ::ffi::qt_core_c_QMetaMethod_invokeOnGadget_gadget_val0_val1_val2_val3_val4_val5_val6_val7(original_self as *const ::meta_method::MetaMethod, gadget, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeOnGadgetArgs<'largs>
    for (*mut ::libc::c_void,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let gadget = self.0;
      let val0 = self.1;
      let val1 = self.2;
      let val2 = self.3;
      let val3 = self.4;
      let val4 = self.5;
      let val5 = self.6;
      let val6 = self.7;
      let val7 = self.8;
      let val8 = self.9;
      ::ffi::qt_core_c_QMetaMethod_invokeOnGadget_gadget_val0_val1_val2_val3_val4_val5_val6_val7_val8(original_self as *const ::meta_method::MetaMethod, gadget, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument, val8 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'largs> MetaMethodInvokeOnGadgetArgs<'largs>
    for (*mut ::libc::c_void,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument,
                                                             &'largs ::generic_argument::GenericArgument) {
    unsafe fn exec(self, original_self: &'largs ::meta_method::MetaMethod) -> bool {
      let gadget = self.0;
      let val0 = self.1;
      let val1 = self.2;
      let val2 = self.3;
      let val3 = self.4;
      let val4 = self.5;
      let val5 = self.6;
      let val6 = self.7;
      let val7 = self.8;
      let val8 = self.9;
      let val9 = self.10;
      ::ffi::qt_core_c_QMetaMethod_invokeOnGadget_gadget_val0_val1_val2_val3_val4_val5_val6_val7_val8_val9(original_self as *const ::meta_method::MetaMethod, gadget, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument, val8 as *const ::generic_argument::GenericArgument, val9 as *const ::generic_argument::GenericArgument)
    }
  }
}

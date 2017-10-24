/// C++ type: <span style='color: green;'>```QMetaObject::Call```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Call {
  /// C++ enum variant: <span style='color: green;'>```InvokeMetaMethod = 0```</span>
  InvokeMetaMethod = 0,
  /// C++ enum variant: <span style='color: green;'>```ReadProperty = 1```</span>
  ReadProperty = 1,
  /// C++ enum variant: <span style='color: green;'>```WriteProperty = 2```</span>
  WriteProperty = 2,
  /// C++ enum variant: <span style='color: green;'>```ResetProperty = 3```</span>
  ResetProperty = 3,
  /// C++ enum variant: <span style='color: green;'>```QueryPropertyDesignable = 4```</span>
  QueryPropertyDesignable = 4,
  /// C++ enum variant: <span style='color: green;'>```QueryPropertyScriptable = 5```</span>
  QueryPropertyScriptable = 5,
  /// C++ enum variant: <span style='color: green;'>```QueryPropertyStored = 6```</span>
  QueryPropertyStored = 6,
  /// C++ enum variant: <span style='color: green;'>```QueryPropertyEditable = 7```</span>
  QueryPropertyEditable = 7,
  /// C++ enum variant: <span style='color: green;'>```QueryPropertyUser = 8```</span>
  QueryPropertyUser = 8,
  /// C++ enum variant: <span style='color: green;'>```CreateInstance = 9```</span>
  CreateInstance = 9,
  /// C++ enum variant: <span style='color: green;'>```IndexOfMethod = 10```</span>
  IndexOfMethod = 10,
  /// C++ enum variant: <span style='color: green;'>```RegisterPropertyMetaType = 11```</span>
  RegisterPropertyMetaType = 11,
  /// C++ enum variant: <span style='color: green;'>```RegisterMethodArgumentMetaType = 12```</span>
  RegisterMethodArgumentMetaType = 12,
}

/// C++ type: <span style='color: green;'>```QMetaObject::Connection```</span>
#[repr(C)]
pub struct Connection([u8; ::type_sizes::QT_CORE_META_OBJECT_CONNECTION]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Connection {
  unsafe fn new_uninitialized() -> Connection {
    Connection(::std::mem::uninitialized())
  }
}

impl Connection {
  /// C++ method: <span style='color: green;'>```QMetaObject::Connection::Connection```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::meta_object::Connection```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMetaObject::Connection::Connection()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::meta_object::Connection) -> ::meta_object::Connection```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMetaObject::Connection::Connection(const QMetaObject::Connection& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::meta_object::Connection
    where Args: overloading::ConnectionNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QMetaObject::Connection& QMetaObject::Connection::operator=(const QMetaObject::Connection& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::meta_object::Connection)
                             -> &'l0 mut ::meta_object::Connection {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QMetaObject_Connection_operator_assign(self as *mut ::meta_object::Connection,
                                                                other as *const ::meta_object::Connection)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

impl Drop for ::meta_object::Connection {
  /// C++ method: <span style='color: green;'>```[destructor] void QMetaObject::Connection::~Connection()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QMetaObject_Connection_destructor(self as *mut ::meta_object::Connection) }
  }
}

/// C++ type: <span style='color: green;'>```QMetaObject```</span>
#[repr(C)]
pub struct MetaObject(u8);

impl MetaObject {
  /// C++ method: <span style='color: green;'>```QMetaObject::activate```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn activate((*mut ::object::Object, *const ::meta_object::MetaObject, ::libc::c_int, *mut *mut ::libc::c_void)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QMetaObject::activate(QObject* sender, const QMetaObject* arg2, int local_signal_index, void** argv)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn activate((*mut ::object::Object, ::libc::c_int, *mut *mut ::libc::c_void)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QMetaObject::activate(QObject* sender, int signal_index, void** argv)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn activate((*mut ::object::Object, ::libc::c_int, ::libc::c_int, *mut *mut ::libc::c_void)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QMetaObject::activate(QObject* sender, int signal_offset, int local_signal_index, void** argv)```</span>
  ///
  ///
  pub unsafe fn activate<Args>(args: Args) -> ()
    where Args: overloading::MetaObjectActivateArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QMetaObject::cast```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn cast(&self, *mut ::object::Object) -> *mut ::object::Object```<br>
  /// C++ method: <span style='color: green;'>```QObject* QMetaObject::cast(QObject* obj) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn cast(&self, *const ::object::Object) -> *const ::object::Object```<br>
  /// C++ method: <span style='color: green;'>```const QObject* QMetaObject::cast(const QObject* obj) const```</span>
  ///
  ///
  pub unsafe fn cast<'largs, Args>(&'largs self, args: Args) -> Args::ReturnType
    where Args: overloading::MetaObjectCastArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::checkConnectArgs(const QMetaMethod& signal, const QMetaMethod& method)```</span>
  ///
  ///
  pub fn check_connect_args(signal: &::meta_method::MetaMethod, method: &::meta_method::MetaMethod) -> bool {
    unsafe {
      ::ffi::qt_core_c_QMetaObject_checkConnectArgs_QMetaMethod_QMetaMethod(signal as *const ::meta_method::MetaMethod, method as *const ::meta_method::MetaMethod)
    }
  }

  /// C++ method: <span style='color: green;'>```static bool QMetaObject::checkConnectArgs(const char* signal, const char* method)```</span>
  ///
  ///
  pub unsafe fn check_connect_args_unsafe(signal: *const ::libc::c_char, method: *const ::libc::c_char) -> bool {
    ::ffi::qt_core_c_QMetaObject_checkConnectArgs_char_char(signal, method)
  }

  /// C++ method: <span style='color: green;'>```QMetaClassInfo QMetaObject::classInfo(int index) const```</span>
  ///
  ///
  pub fn class_info(&self, index: ::libc::c_int) -> ::meta_class_info::MetaClassInfo {
    {
      let mut object: ::meta_class_info::MetaClassInfo =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMetaObject_classInfo_to_output(self as *const ::meta_object::MetaObject, index, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QMetaObject::classInfoCount() const```</span>
  ///
  ///
  pub fn class_info_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMetaObject_classInfoCount(self as *const ::meta_object::MetaObject) }
  }

  /// C++ method: <span style='color: green;'>```int QMetaObject::classInfoOffset() const```</span>
  ///
  ///
  pub fn class_info_offset(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMetaObject_classInfoOffset(self as *const ::meta_object::MetaObject) }
  }

  /// C++ method: <span style='color: green;'>```const char* QMetaObject::className() const```</span>
  ///
  ///
  pub fn class_name(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QMetaObject_className(self as *const ::meta_object::MetaObject) }
  }

  /// C++ method: <span style='color: green;'>```QMetaObject::connect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn connect((*const ::object::Object, ::libc::c_int, *const ::object::Object, ::libc::c_int)) -> ::meta_object::Connection```<br>
  /// C++ method: <span style='color: green;'>```static QMetaObject::Connection QMetaObject::connect(const QObject* sender, int signal_index, const QObject* receiver, int method_index)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn connect((*const ::object::Object, ::libc::c_int, *const ::object::Object, ::libc::c_int, ::libc::c_int)) -> ::meta_object::Connection```<br>
  /// C++ method: <span style='color: green;'>```static QMetaObject::Connection QMetaObject::connect(const QObject* sender, int signal_index, const QObject* receiver, int method_index, int type = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn connect((*const ::object::Object, ::libc::c_int, *const ::object::Object, ::libc::c_int, ::libc::c_int, *mut ::libc::c_int)) -> ::meta_object::Connection```<br>
  /// C++ method: <span style='color: green;'>```static QMetaObject::Connection QMetaObject::connect(const QObject* sender, int signal_index, const QObject* receiver, int method_index, int type = ?, int* types = ?)```</span>
  ///
  ///
  pub unsafe fn connect<Args>(args: Args) -> ::meta_object::Connection
    where Args: overloading::MetaObjectConnectArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static void QMetaObject::connectSlotsByName(QObject* o)```</span>
  ///
  ///
  pub unsafe fn connect_slots_by_name(o: *mut ::object::Object) {
    ::ffi::qt_core_c_QMetaObject_connectSlotsByName(o)
  }

  /// C++ method: <span style='color: green;'>```QMetaMethod QMetaObject::constructor(int index) const```</span>
  ///
  ///
  pub fn constructor(&self, index: ::libc::c_int) -> ::meta_method::MetaMethod {
    {
      let mut object: ::meta_method::MetaMethod =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMetaObject_constructor_to_output(self as *const ::meta_object::MetaObject,
                                                           index,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QMetaObject::constructorCount() const```</span>
  ///
  ///
  pub fn constructor_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMetaObject_constructorCount(self as *const ::meta_object::MetaObject) }
  }

  /// C++ method: <span style='color: green;'>```static bool QMetaObject::disconnect(const QObject* sender, int signal_index, const QObject* receiver, int method_index)```</span>
  ///
  ///
  pub unsafe fn disconnect(sender: *const ::object::Object,
                           signal_index: ::libc::c_int,
                           receiver: *const ::object::Object,
                           method_index: ::libc::c_int)
                           -> bool {
    ::ffi::qt_core_c_QMetaObject_disconnect(sender, signal_index, receiver, method_index)
  }

  /// C++ method: <span style='color: green;'>```static bool QMetaObject::disconnectOne(const QObject* sender, int signal_index, const QObject* receiver, int method_index)```</span>
  ///
  ///
  pub unsafe fn disconnect_one(sender: *const ::object::Object,
                               signal_index: ::libc::c_int,
                               receiver: *const ::object::Object,
                               method_index: ::libc::c_int)
                               -> bool {
    ::ffi::qt_core_c_QMetaObject_disconnectOne(sender, signal_index, receiver, method_index)
  }

  /// C++ method: <span style='color: green;'>```QMetaEnum QMetaObject::enumerator(int index) const```</span>
  ///
  ///
  pub fn enumerator(&self, index: ::libc::c_int) -> ::meta_enum::MetaEnum {
    {
      let mut object: ::meta_enum::MetaEnum =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMetaObject_enumerator_to_output(self as *const ::meta_object::MetaObject, index, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QMetaObject::enumeratorCount() const```</span>
  ///
  ///
  pub fn enumerator_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMetaObject_enumeratorCount(self as *const ::meta_object::MetaObject) }
  }

  /// C++ method: <span style='color: green;'>```int QMetaObject::enumeratorOffset() const```</span>
  ///
  ///
  pub fn enumerator_offset(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMetaObject_enumeratorOffset(self as *const ::meta_object::MetaObject) }
  }

  /// C++ method: <span style='color: green;'>```int QMetaObject::indexOfClassInfo(const char* name) const```</span>
  ///
  ///
  pub unsafe fn index_of_class_info(&self, name: *const ::libc::c_char) -> ::libc::c_int {
    ::ffi::qt_core_c_QMetaObject_indexOfClassInfo(self as *const ::meta_object::MetaObject, name)
  }

  /// C++ method: <span style='color: green;'>```int QMetaObject::indexOfConstructor(const char* constructor) const```</span>
  ///
  ///
  pub unsafe fn index_of_constructor(&self, constructor: *const ::libc::c_char) -> ::libc::c_int {
    ::ffi::qt_core_c_QMetaObject_indexOfConstructor(self as *const ::meta_object::MetaObject, constructor)
  }

  /// C++ method: <span style='color: green;'>```int QMetaObject::indexOfEnumerator(const char* name) const```</span>
  ///
  ///
  pub unsafe fn index_of_enumerator(&self, name: *const ::libc::c_char) -> ::libc::c_int {
    ::ffi::qt_core_c_QMetaObject_indexOfEnumerator(self as *const ::meta_object::MetaObject, name)
  }

  /// C++ method: <span style='color: green;'>```int QMetaObject::indexOfMethod(const char* method) const```</span>
  ///
  ///
  pub unsafe fn index_of_method(&self, method: *const ::libc::c_char) -> ::libc::c_int {
    ::ffi::qt_core_c_QMetaObject_indexOfMethod(self as *const ::meta_object::MetaObject, method)
  }

  /// C++ method: <span style='color: green;'>```int QMetaObject::indexOfProperty(const char* name) const```</span>
  ///
  ///
  pub unsafe fn index_of_property(&self, name: *const ::libc::c_char) -> ::libc::c_int {
    ::ffi::qt_core_c_QMetaObject_indexOfProperty(self as *const ::meta_object::MetaObject, name)
  }

  /// C++ method: <span style='color: green;'>```int QMetaObject::indexOfSignal(const char* signal) const```</span>
  ///
  ///
  pub unsafe fn index_of_signal(&self, signal: *const ::libc::c_char) -> ::libc::c_int {
    ::ffi::qt_core_c_QMetaObject_indexOfSignal(self as *const ::meta_object::MetaObject, signal)
  }

  /// C++ method: <span style='color: green;'>```int QMetaObject::indexOfSlot(const char* slot) const```</span>
  ///
  ///
  pub unsafe fn index_of_slot(&self, slot: *const ::libc::c_char) -> ::libc::c_int {
    ::ffi::qt_core_c_QMetaObject_indexOfSlot(self as *const ::meta_object::MetaObject, slot)
  }

  /// C++ method: <span style='color: green;'>```bool QMetaObject::inherits(const QMetaObject* metaObject) const```</span>
  ///
  ///
  pub unsafe fn inherits(&self, meta_object: *const ::meta_object::MetaObject) -> bool {
    ::ffi::qt_core_c_QMetaObject_inherits(self as *const ::meta_object::MetaObject, meta_object)
  }

  /// C++ method: <span style='color: green;'>```QMetaObject::invokeMethod```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, QGenericArgument val0 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, QGenericArgument val0 = ?, QGenericArgument val1 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?, QGenericArgument val8 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?, QGenericArgument val8 = ?, QGenericArgument val9 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 12
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::generic_return_argument::GenericReturnArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, QGenericReturnArgument ret)```</span>
  ///
  ///
  ///
  /// ## Variant 13
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, QGenericReturnArgument ret, QGenericArgument val0 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 14
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, QGenericReturnArgument ret, QGenericArgument val0 = ?, QGenericArgument val1 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 15
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, QGenericReturnArgument ret, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 16
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, QGenericReturnArgument ret, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 17
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, QGenericReturnArgument ret, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 18
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, QGenericReturnArgument ret, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 19
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, QGenericReturnArgument ret, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 20
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, QGenericReturnArgument ret, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 21
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, QGenericReturnArgument ret, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?, QGenericArgument val8 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 22
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, QGenericReturnArgument ret, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?, QGenericArgument val8 = ?, QGenericArgument val9 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 23
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::qt::ConnectionType, &::generic_return_argument::GenericReturnArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, Qt::ConnectionType arg3, QGenericReturnArgument ret)```</span>
  ///
  ///
  ///
  /// ## Variant 24
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::qt::ConnectionType, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, Qt::ConnectionType arg3, QGenericReturnArgument ret, QGenericArgument val0 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 25
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::qt::ConnectionType, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, Qt::ConnectionType arg3, QGenericReturnArgument ret, QGenericArgument val0 = ?, QGenericArgument val1 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 26
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::qt::ConnectionType, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, Qt::ConnectionType arg3, QGenericReturnArgument ret, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 27
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::qt::ConnectionType, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, Qt::ConnectionType arg3, QGenericReturnArgument ret, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 28
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::qt::ConnectionType, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, Qt::ConnectionType arg3, QGenericReturnArgument ret, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 29
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::qt::ConnectionType, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, Qt::ConnectionType arg3, QGenericReturnArgument ret, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 30
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::qt::ConnectionType, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, Qt::ConnectionType arg3, QGenericReturnArgument ret, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 31
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::qt::ConnectionType, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, Qt::ConnectionType arg3, QGenericReturnArgument ret, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 32
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::qt::ConnectionType, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, Qt::ConnectionType arg3, QGenericReturnArgument ret, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?, QGenericArgument val8 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 33
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::qt::ConnectionType, &::generic_return_argument::GenericReturnArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, Qt::ConnectionType arg3, QGenericReturnArgument ret, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?, QGenericArgument val8 = ?, QGenericArgument val9 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 34
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::qt::ConnectionType)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, Qt::ConnectionType type)```</span>
  ///
  ///
  ///
  /// ## Variant 35
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::qt::ConnectionType, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, Qt::ConnectionType type, QGenericArgument val0 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 36
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::qt::ConnectionType, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, Qt::ConnectionType type, QGenericArgument val0 = ?, QGenericArgument val1 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 37
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::qt::ConnectionType, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, Qt::ConnectionType type, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 38
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::qt::ConnectionType, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, Qt::ConnectionType type, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 39
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::qt::ConnectionType, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, Qt::ConnectionType type, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 40
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::qt::ConnectionType, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, Qt::ConnectionType type, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 41
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::qt::ConnectionType, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, Qt::ConnectionType type, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 42
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::qt::ConnectionType, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, Qt::ConnectionType type, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 43
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::qt::ConnectionType, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, Qt::ConnectionType type, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?, QGenericArgument val8 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 44
  ///
  /// Rust arguments: ```fn invoke_method((*mut ::object::Object, *const ::libc::c_char, &::qt::ConnectionType, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QMetaObject::invokeMethod(QObject* obj, const char* member, Qt::ConnectionType type, QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?, QGenericArgument val8 = ?, QGenericArgument val9 = ?)```</span>
  ///
  ///
  pub unsafe fn invoke_method<Args>(args: Args) -> bool
    where Args: overloading::MetaObjectInvokeMethodArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static int QMetaObject::metacall(QObject* arg1, QMetaObject::Call arg2, int arg3, void** arg4)```</span>
  ///
  ///
  pub unsafe fn metacall(arg1: *mut ::object::Object,
                         arg2: ::meta_object::Call,
                         arg3: ::libc::c_int,
                         arg4: *mut *mut ::libc::c_void)
                         -> ::libc::c_int {
    ::ffi::qt_core_c_QMetaObject_metacall(arg1, arg2, arg3, arg4)
  }

  /// C++ method: <span style='color: green;'>```QMetaMethod QMetaObject::method(int index) const```</span>
  ///
  ///
  pub fn method(&self, index: ::libc::c_int) -> ::meta_method::MetaMethod {
    {
      let mut object: ::meta_method::MetaMethod =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMetaObject_method_to_output(self as *const ::meta_object::MetaObject, index, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QMetaObject::methodCount() const```</span>
  ///
  ///
  pub fn method_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMetaObject_methodCount(self as *const ::meta_object::MetaObject) }
  }

  /// C++ method: <span style='color: green;'>```int QMetaObject::methodOffset() const```</span>
  ///
  ///
  pub fn method_offset(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMetaObject_methodOffset(self as *const ::meta_object::MetaObject) }
  }

  /// C++ method: <span style='color: green;'>```QMetaObject::newInstance```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_instance(&self, ()) -> *mut ::object::Object```<br>
  /// C++ method: <span style='color: green;'>```QObject* QMetaObject::newInstance() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_instance(&self, &::generic_argument::GenericArgument) -> *mut ::object::Object```<br>
  /// C++ method: <span style='color: green;'>```QObject* QMetaObject::newInstance(QGenericArgument val0 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_instance(&self, (&::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> *mut ::object::Object```<br>
  /// C++ method: <span style='color: green;'>```QObject* QMetaObject::newInstance(QGenericArgument val0 = ?, QGenericArgument val1 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new_instance(&self, (&::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> *mut ::object::Object```<br>
  /// C++ method: <span style='color: green;'>```QObject* QMetaObject::newInstance(QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new_instance(&self, (&::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> *mut ::object::Object```<br>
  /// C++ method: <span style='color: green;'>```QObject* QMetaObject::newInstance(QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new_instance(&self, (&::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> *mut ::object::Object```<br>
  /// C++ method: <span style='color: green;'>```QObject* QMetaObject::newInstance(QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn new_instance(&self, (&::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> *mut ::object::Object```<br>
  /// C++ method: <span style='color: green;'>```QObject* QMetaObject::newInstance(QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn new_instance(&self, (&::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> *mut ::object::Object```<br>
  /// C++ method: <span style='color: green;'>```QObject* QMetaObject::newInstance(QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn new_instance(&self, (&::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> *mut ::object::Object```<br>
  /// C++ method: <span style='color: green;'>```QObject* QMetaObject::newInstance(QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn new_instance(&self, (&::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> *mut ::object::Object```<br>
  /// C++ method: <span style='color: green;'>```QObject* QMetaObject::newInstance(QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?, QGenericArgument val8 = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn new_instance(&self, (&::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument, &::generic_argument::GenericArgument)) -> *mut ::object::Object```<br>
  /// C++ method: <span style='color: green;'>```QObject* QMetaObject::newInstance(QGenericArgument val0 = ?, QGenericArgument val1 = ?, QGenericArgument val2 = ?, QGenericArgument val3 = ?, QGenericArgument val4 = ?, QGenericArgument val5 = ?, QGenericArgument val6 = ?, QGenericArgument val7 = ?, QGenericArgument val8 = ?, QGenericArgument val9 = ?) const```</span>
  ///
  ///
  pub fn new_instance<'largs, Args>(&'largs self, args: Args) -> *mut ::object::Object
    where Args: overloading::MetaObjectNewInstanceArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static QByteArray QMetaObject::normalizedSignature(const char* method)```</span>
  ///
  ///
  pub unsafe fn normalized_signature(method: *const ::libc::c_char) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QMetaObject_normalizedSignature_to_output(method, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QByteArray QMetaObject::normalizedType(const char* type)```</span>
  ///
  ///
  pub unsafe fn normalized_type(type_: *const ::libc::c_char) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QMetaObject_normalizedType_to_output(type_, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMetaProperty QMetaObject::property(int index) const```</span>
  ///
  ///
  pub fn property(&self, index: ::libc::c_int) -> ::meta_property::MetaProperty {
    {
      let mut object: ::meta_property::MetaProperty =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMetaObject_property_to_output(self as *const ::meta_object::MetaObject, index, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QMetaObject::propertyCount() const```</span>
  ///
  ///
  pub fn property_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMetaObject_propertyCount(self as *const ::meta_object::MetaObject) }
  }

  /// C++ method: <span style='color: green;'>```int QMetaObject::propertyOffset() const```</span>
  ///
  ///
  pub fn property_offset(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMetaObject_propertyOffset(self as *const ::meta_object::MetaObject) }
  }

  /// C++ method: <span style='color: green;'>```int QMetaObject::static_metacall(QMetaObject::Call arg1, int arg2, void** arg3) const```</span>
  ///
  ///
  pub unsafe fn static_metacall(&self,
                                arg1: ::meta_object::Call,
                                arg2: ::libc::c_int,
                                arg3: *mut *mut ::libc::c_void)
                                -> ::libc::c_int {
    ::ffi::qt_core_c_QMetaObject_static_metacall(self as *const ::meta_object::MetaObject, arg1, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```const QMetaObject* QMetaObject::superClass() const```</span>
  ///
  ///
  pub fn super_class(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QMetaObject_superClass(self as *const ::meta_object::MetaObject) }
  }

  /// C++ method: <span style='color: green;'>```QMetaObject::tr```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn tr(&self, (*const ::libc::c_char, *const ::libc::c_char)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QMetaObject::tr(const char* s, const char* c) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn tr(&self, (*const ::libc::c_char, *const ::libc::c_char, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QMetaObject::tr(const char* s, const char* c, int n = ?) const```</span>
  ///
  ///
  pub unsafe fn tr<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::MetaObjectTrArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMetaProperty QMetaObject::userProperty() const```</span>
  ///
  ///
  pub fn user_property(&self) -> ::meta_property::MetaProperty {
    {
      let mut object: ::meta_property::MetaProperty =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMetaObject_userProperty_to_output(self as *const ::meta_object::MetaObject, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::meta_object::MetaObject {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QMetaObject_delete
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Connection::new](../struct.Connection.html#method.new) method.
  pub trait ConnectionNewArgs {
    fn exec(self) -> ::meta_object::Connection;
  }
  impl ConnectionNewArgs for () {
    fn exec(self) -> ::meta_object::Connection {

      {
        let mut object: ::meta_object::Connection =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMetaObject_Connection_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> ConnectionNewArgs for &'a ::meta_object::Connection {
    fn exec(self) -> ::meta_object::Connection {
      let other = self;
      {
        let mut object: ::meta_object::Connection =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMetaObject_Connection_constructor_other(other as *const ::meta_object::Connection,
                                                                    &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [MetaObject::activate](../struct.MetaObject.html#method.activate) method.
  pub trait MetaObjectActivateArgs {
    unsafe fn exec(self) -> ();
  }
  impl MetaObjectActivateArgs
    for (*mut ::object::Object, *const ::meta_object::MetaObject, ::libc::c_int, *mut *mut ::libc::c_void) {
    unsafe fn exec(self) -> () {
      let sender = self.0;
      let arg2 = self.1;
      let local_signal_index = self.2;
      let argv = self.3;
      ::ffi::qt_core_c_QMetaObject_activate_sender_arg2_local_signal_index_argv(sender, arg2, local_signal_index, argv)
    }
  }
  impl MetaObjectActivateArgs for (*mut ::object::Object, ::libc::c_int, *mut *mut ::libc::c_void) {
    unsafe fn exec(self) -> () {
      let sender = self.0;
      let signal_index = self.1;
      let argv = self.2;
      ::ffi::qt_core_c_QMetaObject_activate_sender_signal_index_argv(sender, signal_index, argv)
    }
  }
  impl MetaObjectActivateArgs for (*mut ::object::Object, ::libc::c_int, ::libc::c_int, *mut *mut ::libc::c_void) {
    unsafe fn exec(self) -> () {
      let sender = self.0;
      let signal_offset = self.1;
      let local_signal_index = self.2;
      let argv = self.3;
      ::ffi::qt_core_c_QMetaObject_activate_sender_signal_offset_local_signal_index_argv(sender,
                                                                                         signal_offset,
                                                                                         local_signal_index,
                                                                                         argv)
    }
  }
  /// This trait represents a set of arguments accepted by [MetaObject::cast](../struct.MetaObject.html#method.cast) method.
  pub trait MetaObjectCastArgs<'largs> {
    type ReturnType;
    unsafe fn exec(self, original_self: &'largs ::meta_object::MetaObject) -> Self::ReturnType;
  }
  impl<'largs> MetaObjectCastArgs<'largs> for *mut ::object::Object {
    type ReturnType = *mut ::object::Object;
    unsafe fn exec(self, original_self: &'largs ::meta_object::MetaObject) -> *mut ::object::Object {
      let obj = self;
      ::ffi::qt_core_c_QMetaObject_cast_QObject_ptr(original_self as *const ::meta_object::MetaObject, obj)
    }
  }
  impl<'largs> MetaObjectCastArgs<'largs> for *const ::object::Object {
    type ReturnType = *const ::object::Object;
    unsafe fn exec(self, original_self: &'largs ::meta_object::MetaObject) -> *const ::object::Object {
      let obj = self;
      ::ffi::qt_core_c_QMetaObject_cast_const_QObject_ptr(original_self as *const ::meta_object::MetaObject, obj)
    }
  }
  /// This trait represents a set of arguments accepted by [MetaObject::connect](../struct.MetaObject.html#method.connect) method.
  pub trait MetaObjectConnectArgs {
    unsafe fn exec(self) -> ::meta_object::Connection;
  }
  impl MetaObjectConnectArgs for (*const ::object::Object, ::libc::c_int, *const ::object::Object, ::libc::c_int) {
    unsafe fn exec(self) -> ::meta_object::Connection {
      let sender = self.0;
      let signal_index = self.1;
      let receiver = self.2;
      let method_index = self.3;
      {
        let mut object: ::meta_object::Connection =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QMetaObject_connect_to_output_sender_signal_index_receiver_method_index(sender,
                                                                                                 signal_index,
                                                                                                 receiver,
                                                                                                 method_index,
                                                                                                 &mut object);
        object
      }
    }
  }
  impl MetaObjectConnectArgs
    for (*const ::object::Object, ::libc::c_int, *const ::object::Object, ::libc::c_int, ::libc::c_int) {
    unsafe fn exec(self) -> ::meta_object::Connection {
      let sender = self.0;
      let signal_index = self.1;
      let receiver = self.2;
      let method_index = self.3;
      let type_ = self.4;
      {
        let mut object: ::meta_object::Connection =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QMetaObject_connect_to_output_sender_signal_index_receiver_method_index_type(sender,
                                                                                                      signal_index,
                                                                                                      receiver,
                                                                                                      method_index,
                                                                                                      type_,
                                                                                                      &mut object);
        object
      }
    }
  }
  impl MetaObjectConnectArgs
    for (*const ::object::Object,
                                      ::libc::c_int,
                                      *const ::object::Object,
                                      ::libc::c_int,
                                      ::libc::c_int,
                                      *mut ::libc::c_int) {
    unsafe fn exec(self) -> ::meta_object::Connection {
      let sender = self.0;
      let signal_index = self.1;
      let receiver = self.2;
      let method_index = self.3;
      let type_ = self.4;
      let types = self.5;
      {
        let mut object: ::meta_object::Connection =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QMetaObject_connect_to_output_sender_signal_index_receiver_method_index_type_types(sender, signal_index, receiver, method_index, type_, types, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [MetaObject::invoke_method](../struct.MetaObject.html#method.invoke_method) method.
  pub trait MetaObjectInvokeMethodArgs {
    unsafe fn exec(self) -> bool;
  }
  impl MetaObjectInvokeMethodArgs for (*mut ::object::Object, *const ::libc::c_char) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member(obj, member)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::qt::ConnectionType,
                                               &'a ::generic_return_argument::GenericReturnArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let arg3 = self.2;
      let ret = self.3;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret(obj, member, arg3 as *const ::qt::ConnectionType, ret as *const ::generic_return_argument::GenericReturnArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::qt::ConnectionType,
                                               &'a ::generic_return_argument::GenericReturnArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let arg3 = self.2;
      let ret = self.3;
      let val0 = self.4;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0(obj, member, arg3 as *const ::qt::ConnectionType, ret as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::qt::ConnectionType,
                                               &'a ::generic_return_argument::GenericReturnArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let arg3 = self.2;
      let ret = self.3;
      let val0 = self.4;
      let val1 = self.5;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1(obj, member, arg3 as *const ::qt::ConnectionType, ret as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::qt::ConnectionType,
                                               &'a ::generic_return_argument::GenericReturnArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let arg3 = self.2;
      let ret = self.3;
      let val0 = self.4;
      let val1 = self.5;
      let val2 = self.6;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1_val2(obj, member, arg3 as *const ::qt::ConnectionType, ret as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::qt::ConnectionType,
                                               &'a ::generic_return_argument::GenericReturnArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let arg3 = self.2;
      let ret = self.3;
      let val0 = self.4;
      let val1 = self.5;
      let val2 = self.6;
      let val3 = self.7;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1_val2_val3(obj, member, arg3 as *const ::qt::ConnectionType, ret as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::qt::ConnectionType,
                                               &'a ::generic_return_argument::GenericReturnArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let arg3 = self.2;
      let ret = self.3;
      let val0 = self.4;
      let val1 = self.5;
      let val2 = self.6;
      let val3 = self.7;
      let val4 = self.8;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1_val2_val3_val4(obj, member, arg3 as *const ::qt::ConnectionType, ret as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::qt::ConnectionType,
                                               &'a ::generic_return_argument::GenericReturnArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let arg3 = self.2;
      let ret = self.3;
      let val0 = self.4;
      let val1 = self.5;
      let val2 = self.6;
      let val3 = self.7;
      let val4 = self.8;
      let val5 = self.9;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1_val2_val3_val4_val5(obj, member, arg3 as *const ::qt::ConnectionType, ret as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::qt::ConnectionType,
                                               &'a ::generic_return_argument::GenericReturnArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let arg3 = self.2;
      let ret = self.3;
      let val0 = self.4;
      let val1 = self.5;
      let val2 = self.6;
      let val3 = self.7;
      let val4 = self.8;
      let val5 = self.9;
      let val6 = self.10;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1_val2_val3_val4_val5_val6(obj, member, arg3 as *const ::qt::ConnectionType, ret as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::qt::ConnectionType,
                                               &'a ::generic_return_argument::GenericReturnArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let arg3 = self.2;
      let ret = self.3;
      let val0 = self.4;
      let val1 = self.5;
      let val2 = self.6;
      let val3 = self.7;
      let val4 = self.8;
      let val5 = self.9;
      let val6 = self.10;
      let val7 = self.11;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1_val2_val3_val4_val5_val6_val7(obj, member, arg3 as *const ::qt::ConnectionType, ret as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::qt::ConnectionType,
                                               &'a ::generic_return_argument::GenericReturnArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let arg3 = self.2;
      let ret = self.3;
      let val0 = self.4;
      let val1 = self.5;
      let val2 = self.6;
      let val3 = self.7;
      let val4 = self.8;
      let val5 = self.9;
      let val6 = self.10;
      let val7 = self.11;
      let val8 = self.12;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1_val2_val3_val4_val5_val6_val7_val8(obj, member, arg3 as *const ::qt::ConnectionType, ret as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument, val8 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::qt::ConnectionType,
                                               &'a ::generic_return_argument::GenericReturnArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let arg3 = self.2;
      let ret = self.3;
      let val0 = self.4;
      let val1 = self.5;
      let val2 = self.6;
      let val3 = self.7;
      let val4 = self.8;
      let val5 = self.9;
      let val6 = self.10;
      let val7 = self.11;
      let val8 = self.12;
      let val9 = self.13;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1_val2_val3_val4_val5_val6_val7_val8_val9(obj, member, arg3 as *const ::qt::ConnectionType, ret as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument, val8 as *const ::generic_argument::GenericArgument, val9 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object, *const ::libc::c_char, &'a ::generic_return_argument::GenericReturnArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let ret = self.2;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_ret(obj, member, ret as *const ::generic_return_argument::GenericReturnArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::generic_return_argument::GenericReturnArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let ret = self.2;
      let val0 = self.3;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0(obj, member, ret as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::generic_return_argument::GenericReturnArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let ret = self.2;
      let val0 = self.3;
      let val1 = self.4;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1(obj, member, ret as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::generic_return_argument::GenericReturnArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let ret = self.2;
      let val0 = self.3;
      let val1 = self.4;
      let val2 = self.5;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1_val2(obj, member, ret as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::generic_return_argument::GenericReturnArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let ret = self.2;
      let val0 = self.3;
      let val1 = self.4;
      let val2 = self.5;
      let val3 = self.6;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1_val2_val3(obj, member, ret as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::generic_return_argument::GenericReturnArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let ret = self.2;
      let val0 = self.3;
      let val1 = self.4;
      let val2 = self.5;
      let val3 = self.6;
      let val4 = self.7;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1_val2_val3_val4(obj, member, ret as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::generic_return_argument::GenericReturnArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let ret = self.2;
      let val0 = self.3;
      let val1 = self.4;
      let val2 = self.5;
      let val3 = self.6;
      let val4 = self.7;
      let val5 = self.8;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1_val2_val3_val4_val5(obj, member, ret as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::generic_return_argument::GenericReturnArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let ret = self.2;
      let val0 = self.3;
      let val1 = self.4;
      let val2 = self.5;
      let val3 = self.6;
      let val4 = self.7;
      let val5 = self.8;
      let val6 = self.9;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1_val2_val3_val4_val5_val6(obj, member, ret as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::generic_return_argument::GenericReturnArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let ret = self.2;
      let val0 = self.3;
      let val1 = self.4;
      let val2 = self.5;
      let val3 = self.6;
      let val4 = self.7;
      let val5 = self.8;
      let val6 = self.9;
      let val7 = self.10;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1_val2_val3_val4_val5_val6_val7(obj, member, ret as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::generic_return_argument::GenericReturnArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let ret = self.2;
      let val0 = self.3;
      let val1 = self.4;
      let val2 = self.5;
      let val3 = self.6;
      let val4 = self.7;
      let val5 = self.8;
      let val6 = self.9;
      let val7 = self.10;
      let val8 = self.11;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1_val2_val3_val4_val5_val6_val7_val8(obj, member, ret as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument, val8 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::generic_return_argument::GenericReturnArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let ret = self.2;
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
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1_val2_val3_val4_val5_val6_val7_val8_val9(obj, member, ret as *const ::generic_return_argument::GenericReturnArgument, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument, val8 as *const ::generic_argument::GenericArgument, val9 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs for (*mut ::object::Object, *const ::libc::c_char, &'a ::qt::ConnectionType) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let type_ = self.2;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_type(obj, member, type_ as *const ::qt::ConnectionType)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::qt::ConnectionType,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let type_ = self.2;
      let val0 = self.3;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0(obj, member, type_ as *const ::qt::ConnectionType, val0 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::qt::ConnectionType,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let type_ = self.2;
      let val0 = self.3;
      let val1 = self.4;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1(obj, member, type_ as *const ::qt::ConnectionType, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::qt::ConnectionType,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let type_ = self.2;
      let val0 = self.3;
      let val1 = self.4;
      let val2 = self.5;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1_val2(obj, member, type_ as *const ::qt::ConnectionType, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::qt::ConnectionType,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let type_ = self.2;
      let val0 = self.3;
      let val1 = self.4;
      let val2 = self.5;
      let val3 = self.6;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1_val2_val3(obj, member, type_ as *const ::qt::ConnectionType, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::qt::ConnectionType,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let type_ = self.2;
      let val0 = self.3;
      let val1 = self.4;
      let val2 = self.5;
      let val3 = self.6;
      let val4 = self.7;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1_val2_val3_val4(obj, member, type_ as *const ::qt::ConnectionType, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::qt::ConnectionType,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let type_ = self.2;
      let val0 = self.3;
      let val1 = self.4;
      let val2 = self.5;
      let val3 = self.6;
      let val4 = self.7;
      let val5 = self.8;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1_val2_val3_val4_val5(obj, member, type_ as *const ::qt::ConnectionType, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::qt::ConnectionType,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let type_ = self.2;
      let val0 = self.3;
      let val1 = self.4;
      let val2 = self.5;
      let val3 = self.6;
      let val4 = self.7;
      let val5 = self.8;
      let val6 = self.9;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1_val2_val3_val4_val5_val6(obj, member, type_ as *const ::qt::ConnectionType, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::qt::ConnectionType,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let type_ = self.2;
      let val0 = self.3;
      let val1 = self.4;
      let val2 = self.5;
      let val3 = self.6;
      let val4 = self.7;
      let val5 = self.8;
      let val6 = self.9;
      let val7 = self.10;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1_val2_val3_val4_val5_val6_val7(obj, member, type_ as *const ::qt::ConnectionType, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::qt::ConnectionType,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let type_ = self.2;
      let val0 = self.3;
      let val1 = self.4;
      let val2 = self.5;
      let val3 = self.6;
      let val4 = self.7;
      let val5 = self.8;
      let val6 = self.9;
      let val7 = self.10;
      let val8 = self.11;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1_val2_val3_val4_val5_val6_val7_val8(obj, member, type_ as *const ::qt::ConnectionType, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument, val8 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::qt::ConnectionType,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let type_ = self.2;
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
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1_val2_val3_val4_val5_val6_val7_val8_val9(obj, member, type_ as *const ::qt::ConnectionType, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument, val8 as *const ::generic_argument::GenericArgument, val9 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object, *const ::libc::c_char, &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let val0 = self.2;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_val0(obj,
                                                                member,
                                                                val0 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let val0 = self.2;
      let val1 = self.3;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1(obj, member, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1_val2(obj, member, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1_val2_val3(obj, member, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      let val4 = self.6;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1_val2_val3_val4(obj, member, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      let val4 = self.6;
      let val5 = self.7;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1_val2_val3_val4_val5(obj, member, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      let val4 = self.6;
      let val5 = self.7;
      let val6 = self.8;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1_val2_val3_val4_val5_val6(obj, member, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      let val4 = self.6;
      let val5 = self.7;
      let val6 = self.8;
      let val7 = self.9;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1_val2_val3_val4_val5_val6_val7(obj, member, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
      let val0 = self.2;
      let val1 = self.3;
      let val2 = self.4;
      let val3 = self.5;
      let val4 = self.6;
      let val5 = self.7;
      let val6 = self.8;
      let val7 = self.9;
      let val8 = self.10;
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1_val2_val3_val4_val5_val6_val7_val8(obj, member, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument, val8 as *const ::generic_argument::GenericArgument)
    }
  }
  impl<'a> MetaObjectInvokeMethodArgs
    for (*mut ::object::Object,
                                               *const ::libc::c_char,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument,
                                               &'a ::generic_argument::GenericArgument) {
    unsafe fn exec(self) -> bool {
      let obj = self.0;
      let member = self.1;
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
      ::ffi::qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1_val2_val3_val4_val5_val6_val7_val8_val9(obj, member, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument, val8 as *const ::generic_argument::GenericArgument, val9 as *const ::generic_argument::GenericArgument)
    }
  }
  /// This trait represents a set of arguments accepted by [MetaObject::new_instance](../struct.MetaObject.html#method.new_instance) method.
  pub trait MetaObjectNewInstanceArgs<'largs> {
    fn exec(self, original_self: &'largs ::meta_object::MetaObject) -> *mut ::object::Object;
  }
  impl<'largs> MetaObjectNewInstanceArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::meta_object::MetaObject) -> *mut ::object::Object {

      unsafe { ::ffi::qt_core_c_QMetaObject_newInstance_no_args(original_self as *const ::meta_object::MetaObject) }
    }
  }
  impl<'largs> MetaObjectNewInstanceArgs<'largs> for &'largs ::generic_argument::GenericArgument {
    fn exec(self, original_self: &'largs ::meta_object::MetaObject) -> *mut ::object::Object {
      let val0 = self;
      unsafe {
        ::ffi::qt_core_c_QMetaObject_newInstance_val0(original_self as *const ::meta_object::MetaObject,
                                                      val0 as *const ::generic_argument::GenericArgument)
      }
    }
  }
  impl<'largs> MetaObjectNewInstanceArgs<'largs>
    for (&'largs ::generic_argument::GenericArgument, &'largs ::generic_argument::GenericArgument) {
    fn exec(self, original_self: &'largs ::meta_object::MetaObject) -> *mut ::object::Object {
      let val0 = self.0;
      let val1 = self.1;
      unsafe {
        ::ffi::qt_core_c_QMetaObject_newInstance_val0_val1(original_self as *const ::meta_object::MetaObject,
                                                           val0 as *const ::generic_argument::GenericArgument,
                                                           val1 as *const ::generic_argument::GenericArgument)
      }
    }
  }
  impl<'largs> MetaObjectNewInstanceArgs<'largs>
    for (&'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument) {
    fn exec(self, original_self: &'largs ::meta_object::MetaObject) -> *mut ::object::Object {
      let val0 = self.0;
      let val1 = self.1;
      let val2 = self.2;
      unsafe {
        ::ffi::qt_core_c_QMetaObject_newInstance_val0_val1_val2(original_self as *const ::meta_object::MetaObject,
                                                                val0 as *const ::generic_argument::GenericArgument,
                                                                val1 as *const ::generic_argument::GenericArgument,
                                                                val2 as *const ::generic_argument::GenericArgument)
      }
    }
  }
  impl<'largs> MetaObjectNewInstanceArgs<'largs>
    for (&'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument) {
    fn exec(self, original_self: &'largs ::meta_object::MetaObject) -> *mut ::object::Object {
      let val0 = self.0;
      let val1 = self.1;
      let val2 = self.2;
      let val3 = self.3;
      unsafe {
        ::ffi::qt_core_c_QMetaObject_newInstance_val0_val1_val2_val3(original_self as *const ::meta_object::MetaObject, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument)
      }
    }
  }
  impl<'largs> MetaObjectNewInstanceArgs<'largs>
    for (&'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument) {
    fn exec(self, original_self: &'largs ::meta_object::MetaObject) -> *mut ::object::Object {
      let val0 = self.0;
      let val1 = self.1;
      let val2 = self.2;
      let val3 = self.3;
      let val4 = self.4;
      unsafe { ::ffi::qt_core_c_QMetaObject_newInstance_val0_val1_val2_val3_val4(original_self as *const ::meta_object::MetaObject, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument) }
    }
  }
  impl<'largs> MetaObjectNewInstanceArgs<'largs>
    for (&'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument) {
    fn exec(self, original_self: &'largs ::meta_object::MetaObject) -> *mut ::object::Object {
      let val0 = self.0;
      let val1 = self.1;
      let val2 = self.2;
      let val3 = self.3;
      let val4 = self.4;
      let val5 = self.5;
      unsafe { ::ffi::qt_core_c_QMetaObject_newInstance_val0_val1_val2_val3_val4_val5(original_self as *const ::meta_object::MetaObject, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument) }
    }
  }
  impl<'largs> MetaObjectNewInstanceArgs<'largs>
    for (&'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument) {
    fn exec(self, original_self: &'largs ::meta_object::MetaObject) -> *mut ::object::Object {
      let val0 = self.0;
      let val1 = self.1;
      let val2 = self.2;
      let val3 = self.3;
      let val4 = self.4;
      let val5 = self.5;
      let val6 = self.6;
      unsafe { ::ffi::qt_core_c_QMetaObject_newInstance_val0_val1_val2_val3_val4_val5_val6(original_self as *const ::meta_object::MetaObject, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument) }
    }
  }
  impl<'largs> MetaObjectNewInstanceArgs<'largs>
    for (&'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument) {
    fn exec(self, original_self: &'largs ::meta_object::MetaObject) -> *mut ::object::Object {
      let val0 = self.0;
      let val1 = self.1;
      let val2 = self.2;
      let val3 = self.3;
      let val4 = self.4;
      let val5 = self.5;
      let val6 = self.6;
      let val7 = self.7;
      unsafe { ::ffi::qt_core_c_QMetaObject_newInstance_val0_val1_val2_val3_val4_val5_val6_val7(original_self as *const ::meta_object::MetaObject, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument) }
    }
  }
  impl<'largs> MetaObjectNewInstanceArgs<'largs>
    for (&'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument) {
    fn exec(self, original_self: &'largs ::meta_object::MetaObject) -> *mut ::object::Object {
      let val0 = self.0;
      let val1 = self.1;
      let val2 = self.2;
      let val3 = self.3;
      let val4 = self.4;
      let val5 = self.5;
      let val6 = self.6;
      let val7 = self.7;
      let val8 = self.8;
      unsafe { ::ffi::qt_core_c_QMetaObject_newInstance_val0_val1_val2_val3_val4_val5_val6_val7_val8(original_self as *const ::meta_object::MetaObject, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument, val8 as *const ::generic_argument::GenericArgument) }
    }
  }
  impl<'largs> MetaObjectNewInstanceArgs<'largs>
    for (&'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument,
                                                          &'largs ::generic_argument::GenericArgument) {
    fn exec(self, original_self: &'largs ::meta_object::MetaObject) -> *mut ::object::Object {
      let val0 = self.0;
      let val1 = self.1;
      let val2 = self.2;
      let val3 = self.3;
      let val4 = self.4;
      let val5 = self.5;
      let val6 = self.6;
      let val7 = self.7;
      let val8 = self.8;
      let val9 = self.9;
      unsafe { ::ffi::qt_core_c_QMetaObject_newInstance_val0_val1_val2_val3_val4_val5_val6_val7_val8_val9(original_self as *const ::meta_object::MetaObject, val0 as *const ::generic_argument::GenericArgument, val1 as *const ::generic_argument::GenericArgument, val2 as *const ::generic_argument::GenericArgument, val3 as *const ::generic_argument::GenericArgument, val4 as *const ::generic_argument::GenericArgument, val5 as *const ::generic_argument::GenericArgument, val6 as *const ::generic_argument::GenericArgument, val7 as *const ::generic_argument::GenericArgument, val8 as *const ::generic_argument::GenericArgument, val9 as *const ::generic_argument::GenericArgument) }
    }
  }
  /// This trait represents a set of arguments accepted by [MetaObject::tr](../struct.MetaObject.html#method.tr) method.
  pub trait MetaObjectTrArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::meta_object::MetaObject) -> ::string::String;
  }
  impl<'largs> MetaObjectTrArgs<'largs> for (*const ::libc::c_char, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs ::meta_object::MetaObject) -> ::string::String {
      let s = self.0;
      let c = self.1;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QMetaObject_tr_to_output_s_c(original_self as *const ::meta_object::MetaObject,
                                                      s,
                                                      c,
                                                      &mut object);
        object
      }
    }
  }
  impl<'largs> MetaObjectTrArgs<'largs> for (*const ::libc::c_char, *const ::libc::c_char, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::meta_object::MetaObject) -> ::string::String {
      let s = self.0;
      let c = self.1;
      let n = self.2;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QMetaObject_tr_to_output_s_c_n(original_self as *const ::meta_object::MetaObject,
                                                        s,
                                                        c,
                                                        n,
                                                        &mut object);
        object
      }
    }
  }
}

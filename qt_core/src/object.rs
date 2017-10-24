/// C++ type: <span style='color: green;'>```QObject```</span>
#[repr(C)]
pub struct Object(u8);

impl Object {
  /// C++ method: <span style='color: green;'>```bool QObject::blockSignals(bool b)```</span>
  ///
  ///
  pub fn block_signals(&mut self, b: bool) -> bool {
    unsafe { ::ffi::qt_core_c_QObject_blockSignals(self as *mut ::object::Object, b) }
  }

  /// C++ method: <span style='color: green;'>```const QList<QObject*>& QObject::children() const```</span>
  ///
  ///
  pub fn children<'l0>(&'l0 self) -> &'l0 ::list::ListObjectMutPtr {
    let ffi_result = unsafe { ::ffi::qt_core_c_QObject_children(self as *const ::object::Object) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QObject::connect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn connect(&self, (*const ::object::Object, *const ::libc::c_char, *const ::libc::c_char)) -> ::meta_object::Connection```<br>
  /// C++ method: <span style='color: green;'>```QMetaObject::Connection QObject::connect(const QObject* sender, const char* signal, const char* member) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn connect(&self, (*const ::object::Object, *const ::libc::c_char, *const ::libc::c_char, &::qt::ConnectionType)) -> ::meta_object::Connection```<br>
  /// C++ method: <span style='color: green;'>```QMetaObject::Connection QObject::connect(const QObject* sender, const char* signal, const char* member, Qt::ConnectionType type = ?) const```</span>
  ///
  ///
  pub unsafe fn connect<'largs, Args>(&'largs self, args: Args) -> ::meta_object::Connection
    where Args: overloading::ObjectConnectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QObject::connect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn connect_static((*const ::object::Object, &::meta_method::MetaMethod, *const ::object::Object, &::meta_method::MetaMethod)) -> ::meta_object::Connection```<br>
  /// C++ method: <span style='color: green;'>```static QMetaObject::Connection QObject::connect(const QObject* sender, const QMetaMethod& signal, const QObject* receiver, const QMetaMethod& method)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn connect_static((*const ::object::Object, &::meta_method::MetaMethod, *const ::object::Object, &::meta_method::MetaMethod, &::qt::ConnectionType)) -> ::meta_object::Connection```<br>
  /// C++ method: <span style='color: green;'>```static QMetaObject::Connection QObject::connect(const QObject* sender, const QMetaMethod& signal, const QObject* receiver, const QMetaMethod& method, Qt::ConnectionType type = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn connect_static((*const ::object::Object, *const ::libc::c_char, *const ::object::Object, *const ::libc::c_char)) -> ::meta_object::Connection```<br>
  /// C++ method: <span style='color: green;'>```static QMetaObject::Connection QObject::connect(const QObject* sender, const char* signal, const QObject* receiver, const char* member)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn connect_static((*const ::object::Object, *const ::libc::c_char, *const ::object::Object, *const ::libc::c_char, &::qt::ConnectionType)) -> ::meta_object::Connection```<br>
  /// C++ method: <span style='color: green;'>```static QMetaObject::Connection QObject::connect(const QObject* sender, const char* signal, const QObject* receiver, const char* member, Qt::ConnectionType arg5 = ?)```</span>
  ///
  ///
  pub unsafe fn connect_static<Args>(args: Args) -> ::meta_object::Connection
    where Args: overloading::ObjectConnectStaticArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QObject::deleteLater()```</span>
  ///
  ///
  pub fn delete_later(&mut self) {
    unsafe { ::ffi::qt_core_c_QObject_deleteLater(self as *mut ::object::Object) }
  }

  /// C++ method: <span style='color: green;'>```void QObject::destroyed()```</span>
  ///
  ///
  pub fn destroyed(&mut self) {
    unsafe { ::ffi::qt_core_c_QObject_destroyed_no_args(self as *mut ::object::Object) }
  }

  /// C++ method: <span style='color: green;'>```void QObject::destroyed(QObject* arg1 = ?)```</span>
  ///
  ///
  pub unsafe fn destroyed_unsafe(&mut self, arg1: *mut ::object::Object) {
    ::ffi::qt_core_c_QObject_destroyed_arg1(self as *mut ::object::Object, arg1)
  }

  /// C++ method: <span style='color: green;'>```QObject::disconnect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn disconnect1(&self, *const ::object::Object) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QObject::disconnect(const QObject* receiver) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn disconnect1(&self, (*const ::object::Object, *const ::libc::c_char)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QObject::disconnect(const QObject* receiver, const char* member = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn disconnect1(&self, *const ::libc::c_char) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QObject::disconnect(const char* signal = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn disconnect1(&self, (*const ::libc::c_char, *const ::object::Object)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QObject::disconnect(const char* signal = ?, const QObject* receiver = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn disconnect1(&self, (*const ::libc::c_char, *const ::object::Object, *const ::libc::c_char)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QObject::disconnect(const char* signal = ?, const QObject* receiver = ?, const char* member = ?) const```</span>
  ///
  ///
  pub unsafe fn disconnect1<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::ObjectDisconnect1Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QObject::disconnect() const```</span>
  ///
  ///
  pub fn disconnect3(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QObject_disconnect_no_args(self as *const ::object::Object) }
  }

  /// C++ method: <span style='color: green;'>```static bool QObject::disconnect(const QMetaObject::Connection& arg1)```</span>
  ///
  ///
  pub fn disconnect_static_0(arg1: &::meta_object::Connection) -> bool {
    unsafe { ::ffi::qt_core_c_QObject_disconnect_QMetaObject_Connection(arg1 as *const ::meta_object::Connection) }
  }

  /// C++ method: <span style='color: green;'>```QObject::disconnect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn disconnect_static_2((*const ::object::Object, &::meta_method::MetaMethod, *const ::object::Object, &::meta_method::MetaMethod)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QObject::disconnect(const QObject* sender, const QMetaMethod& signal, const QObject* receiver, const QMetaMethod& member)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn disconnect_static_2((*const ::object::Object, *const ::libc::c_char, *const ::object::Object, *const ::libc::c_char)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QObject::disconnect(const QObject* sender, const char* signal, const QObject* receiver, const char* member)```</span>
  ///
  ///
  pub unsafe fn disconnect_static_2<Args>(args: Args) -> bool
    where Args: overloading::ObjectDisconnectStatic2Args
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QObject::dumpObjectInfo() const```</span>
  ///
  ///
  pub fn dump_object_info(&self) {
    unsafe { ::ffi::qt_core_c_QObject_dumpObjectInfo_const(self as *const ::object::Object) }
  }

  /// C++ method: <span style='color: green;'>```void QObject::dumpObjectInfo()```</span>
  ///
  ///
  pub fn dump_object_info_mut(&mut self) {
    unsafe { ::ffi::qt_core_c_QObject_dumpObjectInfo(self as *mut ::object::Object) }
  }

  /// C++ method: <span style='color: green;'>```void QObject::dumpObjectTree() const```</span>
  ///
  ///
  pub fn dump_object_tree(&self) {
    unsafe { ::ffi::qt_core_c_QObject_dumpObjectTree_const(self as *const ::object::Object) }
  }

  /// C++ method: <span style='color: green;'>```void QObject::dumpObjectTree()```</span>
  ///
  ///
  pub fn dump_object_tree_mut(&mut self) {
    unsafe { ::ffi::qt_core_c_QObject_dumpObjectTree(self as *mut ::object::Object) }
  }

  /// C++ method: <span style='color: green;'>```QList<QByteArray> QObject::dynamicPropertyNames() const```</span>
  ///
  ///
  pub fn dynamic_property_names(&self) -> ::list::ListByteArray {
    {
      let mut object: ::list::ListByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QObject_dynamicPropertyNames_to_output(self as *const ::object::Object, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QObject::event(QEvent* event)```</span>
  ///
  ///
  pub unsafe fn event(&mut self, event: *mut ::event::Event) -> bool {
    ::ffi::qt_core_c_QObject_event(self as *mut ::object::Object, event)
  }

  /// C++ method: <span style='color: green;'>```virtual bool QObject::eventFilter(QObject* watched, QEvent* event)```</span>
  ///
  ///
  pub unsafe fn event_filter(&mut self, watched: *mut ::object::Object, event: *mut ::event::Event) -> bool {
    ::ffi::qt_core_c_QObject_eventFilter(self as *mut ::object::Object, watched, event)
  }

  /// C++ method: <span style='color: green;'>```bool QObject::inherits(const char* classname) const```</span>
  ///
  ///
  pub unsafe fn inherits(&self, classname: *const ::libc::c_char) -> bool {
    ::ffi::qt_core_c_QObject_inherits(self as *const ::object::Object, classname)
  }

  /// C++ method: <span style='color: green;'>```void QObject::installEventFilter(QObject* filterObj)```</span>
  ///
  ///
  pub unsafe fn install_event_filter(&mut self, filter_obj: *mut ::object::Object) {
    ::ffi::qt_core_c_QObject_installEventFilter(self as *mut ::object::Object, filter_obj)
  }

  /// C++ method: <span style='color: green;'>```bool QObject::isWidgetType() const```</span>
  ///
  ///
  pub fn is_widget_type(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QObject_isWidgetType(self as *const ::object::Object) }
  }

  /// C++ method: <span style='color: green;'>```bool QObject::isWindowType() const```</span>
  ///
  ///
  pub fn is_window_type(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QObject_isWindowType(self as *const ::object::Object) }
  }

  /// C++ method: <span style='color: green;'>```void QObject::killTimer(int id)```</span>
  ///
  ///
  pub fn kill_timer(&mut self, id: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QObject_killTimer(self as *mut ::object::Object, id) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QObject::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QObject_metaObject(self as *const ::object::Object) }
  }

  /// C++ method: <span style='color: green;'>```void QObject::moveToThread(QThread* thread)```</span>
  ///
  ///
  pub unsafe fn move_to_thread(&mut self, thread: *mut ::thread::Thread) {
    ::ffi::qt_core_c_QObject_moveToThread(self as *mut ::object::Object, thread)
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QObject::QObject()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::object::Object> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QObject_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QObject::QObject(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::object::Object) -> ::cpp_utils::CppBox<::object::Object> {
    let ffi_result = ::ffi::qt_core_c_QObject_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QString QObject::objectName() const```</span>
  ///
  ///
  pub fn object_name(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QObject_objectName_to_output(self as *const ::object::Object, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QObject* QObject::parent() const```</span>
  ///
  ///
  pub fn parent(&self) -> *mut ::object::Object {
    unsafe { ::ffi::qt_core_c_QObject_parent(self as *const ::object::Object) }
  }

  /// C++ method: <span style='color: green;'>```QVariant QObject::property(const char* name) const```</span>
  ///
  ///
  pub unsafe fn property(&self, name: *const ::libc::c_char) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QObject_property_to_output(self as *const ::object::Object, name, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QObject::removeEventFilter(QObject* obj)```</span>
  ///
  ///
  pub unsafe fn remove_event_filter(&mut self, obj: *mut ::object::Object) {
    ::ffi::qt_core_c_QObject_removeEventFilter(self as *mut ::object::Object, obj)
  }

  /// C++ method: <span style='color: green;'>```void QObject::setObjectName(const QString& name)```</span>
  ///
  ///
  pub fn set_object_name(&mut self, name: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QObject_setObjectName(self as *mut ::object::Object,
                                             name as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QObject::setParent(QObject* parent)```</span>
  ///
  ///
  pub unsafe fn set_parent(&mut self, parent: *mut ::object::Object) {
    ::ffi::qt_core_c_QObject_setParent(self as *mut ::object::Object, parent)
  }

  /// C++ method: <span style='color: green;'>```bool QObject::setProperty(const char* name, const QVariant& value)```</span>
  ///
  ///
  pub unsafe fn set_property(&mut self, name: *const ::libc::c_char, value: &::variant::Variant) -> bool {
    ::ffi::qt_core_c_QObject_setProperty(self as *mut ::object::Object,
                                         name,
                                         value as *const ::variant::Variant)
  }

  /// C++ method: <span style='color: green;'>```bool QObject::signalsBlocked() const```</span>
  ///
  ///
  pub fn signals_blocked(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QObject_signalsBlocked(self as *const ::object::Object) }
  }

  /// C++ method: <span style='color: green;'>```QObject::startTimer```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn start_timer(&mut self, ::libc::c_int) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QObject::startTimer(int interval)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn start_timer(&mut self, (::libc::c_int, &::qt::TimerType)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QObject::startTimer(int interval, Qt::TimerType timerType = ?)```</span>
  ///
  ///
  pub fn start_timer<'largs, Args>(&'largs mut self, args: Args) -> ::libc::c_int
    where Args: overloading::ObjectStartTimerArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QThread* QObject::thread() const```</span>
  ///
  ///
  pub fn thread(&self) -> *mut ::thread::Thread {
    unsafe { ::ffi::qt_core_c_QObject_thread(self as *const ::object::Object) }
  }

  /// C++ method: <span style='color: green;'>```static QString QObject::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QObject_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QObject::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QObject_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::object::Object {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QObject_delete
  }
}

/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QObject* arg2)```</span>
///
///
pub unsafe fn op_shl(arg1: &::debug::Debug, arg2: *const ::object::Object) -> ::debug::Debug {
  {
    let mut object: ::debug::Debug = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
    ::ffi::qt_core_c_QObject_G_operator_shl_to_output(arg1 as *const ::debug::Debug, arg2, &mut object);
    object
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Object::connect](../struct.Object.html#method.connect) method.
  pub trait ObjectConnectArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::object::Object) -> ::meta_object::Connection;
  }
  impl<'largs> ObjectConnectArgs<'largs> for (*const ::object::Object, *const ::libc::c_char, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs ::object::Object) -> ::meta_object::Connection {
      let sender = self.0;
      let signal = self.1;
      let member = self.2;
      {
        let mut object: ::meta_object::Connection =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QObject_connect_to_output_sender_signal_member(original_self as *const ::object::Object,
                                                                        sender,
                                                                        signal,
                                                                        member,
                                                                        &mut object);
        object
      }
    }
  }
  impl<'largs> ObjectConnectArgs<'largs>
    for (*const ::object::Object, *const ::libc::c_char, *const ::libc::c_char, &'largs ::qt::ConnectionType) {
    unsafe fn exec(self, original_self: &'largs ::object::Object) -> ::meta_object::Connection {
      let sender = self.0;
      let signal = self.1;
      let member = self.2;
      let type_ = self.3;
      {
        let mut object: ::meta_object::Connection =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QObject_connect_to_output_sender_signal_member_type(original_self as *const ::object::Object, sender, signal, member, type_ as *const ::qt::ConnectionType, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Object::connect_static](../struct.Object.html#method.connect_static) method.
  pub trait ObjectConnectStaticArgs {
    unsafe fn exec(self) -> ::meta_object::Connection;
  }
  impl ObjectConnectStaticArgs
    for (*const ::object::Object, *const ::libc::c_char, *const ::object::Object, *const ::libc::c_char) {
    unsafe fn exec(self) -> ::meta_object::Connection {
      let sender = self.0;
      let signal = self.1;
      let receiver = self.2;
      let member = self.3;
      {
        let mut object: ::meta_object::Connection =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QObject_connect_to_output_sender_signal_receiver_member(sender,
                                                                                 signal,
                                                                                 receiver,
                                                                                 member,
                                                                                 &mut object);
        object
      }
    }
  }
  impl<'a> ObjectConnectStaticArgs
    for (*const ::object::Object,
                                            *const ::libc::c_char,
                                            *const ::object::Object,
                                            *const ::libc::c_char,
                                            &'a ::qt::ConnectionType) {
    unsafe fn exec(self) -> ::meta_object::Connection {
      let sender = self.0;
      let signal = self.1;
      let receiver = self.2;
      let member = self.3;
      let arg5 = self.4;
      {
        let mut object: ::meta_object::Connection =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QObject_connect_to_output_sender_signal_receiver_member_arg5(sender, signal, receiver, member, arg5 as *const ::qt::ConnectionType, &mut object);
        object
      }
    }
  }
  impl<'a> ObjectConnectStaticArgs
    for (*const ::object::Object,
                                            &'a ::meta_method::MetaMethod,
                                            *const ::object::Object,
                                            &'a ::meta_method::MetaMethod) {
    unsafe fn exec(self) -> ::meta_object::Connection {
      let sender = self.0;
      let signal = self.1;
      let receiver = self.2;
      let method = self.3;
      {
        let mut object: ::meta_object::Connection =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QObject_connect_to_output_sender_signal_receiver_method(sender, signal as *const ::meta_method::MetaMethod, receiver, method as *const ::meta_method::MetaMethod, &mut object);
        object
      }
    }
  }
  impl<'a> ObjectConnectStaticArgs
    for (*const ::object::Object,
                                            &'a ::meta_method::MetaMethod,
                                            *const ::object::Object,
                                            &'a ::meta_method::MetaMethod,
                                            &'a ::qt::ConnectionType) {
    unsafe fn exec(self) -> ::meta_object::Connection {
      let sender = self.0;
      let signal = self.1;
      let receiver = self.2;
      let method = self.3;
      let type_ = self.4;
      {
        let mut object: ::meta_object::Connection =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QObject_connect_to_output_sender_signal_receiver_method_type(sender, signal as *const ::meta_method::MetaMethod, receiver, method as *const ::meta_method::MetaMethod, type_ as *const ::qt::ConnectionType, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Object::disconnect1](../struct.Object.html#method.disconnect1) method.
  pub trait ObjectDisconnect1Args<'largs> {
    unsafe fn exec(self, original_self: &'largs ::object::Object) -> bool;
  }
  impl<'largs> ObjectDisconnect1Args<'largs> for *const ::object::Object {
    unsafe fn exec(self, original_self: &'largs ::object::Object) -> bool {
      let receiver = self;
      ::ffi::qt_core_c_QObject_disconnect_QObject(original_self as *const ::object::Object, receiver)
    }
  }
  impl<'largs> ObjectDisconnect1Args<'largs> for (*const ::object::Object, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs ::object::Object) -> bool {
      let receiver = self.0;
      let member = self.1;
      ::ffi::qt_core_c_QObject_disconnect_QObject_char(original_self as *const ::object::Object, receiver, member)
    }
  }
  impl<'largs> ObjectDisconnect1Args<'largs> for *const ::libc::c_char {
    unsafe fn exec(self, original_self: &'largs ::object::Object) -> bool {
      let signal = self;
      ::ffi::qt_core_c_QObject_disconnect_char(original_self as *const ::object::Object, signal)
    }
  }
  impl<'largs> ObjectDisconnect1Args<'largs> for (*const ::libc::c_char, *const ::object::Object) {
    unsafe fn exec(self, original_self: &'largs ::object::Object) -> bool {
      let signal = self.0;
      let receiver = self.1;
      ::ffi::qt_core_c_QObject_disconnect_char_QObject(original_self as *const ::object::Object, signal, receiver)
    }
  }
  impl<'largs> ObjectDisconnect1Args<'largs> for (*const ::libc::c_char, *const ::object::Object, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs ::object::Object) -> bool {
      let signal = self.0;
      let receiver = self.1;
      let member = self.2;
      ::ffi::qt_core_c_QObject_disconnect_char_QObject_char(original_self as *const ::object::Object,
                                                            signal,
                                                            receiver,
                                                            member)
    }
  }
  /// This trait represents a set of arguments accepted by [Object::disconnect_static_2](../struct.Object.html#method.disconnect_static_2) method.
  pub trait ObjectDisconnectStatic2Args {
    unsafe fn exec(self) -> bool;
  }
  impl<'a> ObjectDisconnectStatic2Args
    for (*const ::object::Object,
                                                &'a ::meta_method::MetaMethod,
                                                *const ::object::Object,
                                                &'a ::meta_method::MetaMethod) {
    unsafe fn exec(self) -> bool {
      let sender = self.0;
      let signal = self.1;
      let receiver = self.2;
      let member = self.3;
      ::ffi::qt_core_c_QObject_disconnect_QObject_QMetaMethod_QObject_QMetaMethod(sender, signal as *const ::meta_method::MetaMethod, receiver, member as *const ::meta_method::MetaMethod)
    }
  }
  impl ObjectDisconnectStatic2Args
    for (*const ::object::Object, *const ::libc::c_char, *const ::object::Object, *const ::libc::c_char) {
    unsafe fn exec(self) -> bool {
      let sender = self.0;
      let signal = self.1;
      let receiver = self.2;
      let member = self.3;
      ::ffi::qt_core_c_QObject_disconnect_QObject_char_QObject_char(sender, signal, receiver, member)
    }
  }
  /// This trait represents a set of arguments accepted by [Object::start_timer](../struct.Object.html#method.start_timer) method.
  pub trait ObjectStartTimerArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::object::Object) -> ::libc::c_int;
  }
  impl<'largs> ObjectStartTimerArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::object::Object) -> ::libc::c_int {
      let interval = self;
      unsafe { ::ffi::qt_core_c_QObject_startTimer_interval(original_self as *mut ::object::Object, interval) }
    }
  }
  impl<'largs> ObjectStartTimerArgs<'largs> for (::libc::c_int, &'largs ::qt::TimerType) {
    fn exec(self, original_self: &'largs mut ::object::Object) -> ::libc::c_int {
      let interval = self.0;
      let timer_type = self.1;
      unsafe {
        ::ffi::qt_core_c_QObject_startTimer_interval_timerType(original_self as *mut ::object::Object,
                                                               interval,
                                                               timer_type as *const ::qt::TimerType)
      }
    }
  }
}

/// C++ type: <span style='color: green;'>```QAbstractEventDispatcher```</span>
#[repr(C)]
pub struct AbstractEventDispatcher(u8);

impl AbstractEventDispatcher {
  /// C++ method: <span style='color: green;'>```virtual void QAbstractEventDispatcher::closingDown()```</span>
  ///
  ///
  pub fn closing_down(&mut self) {
    unsafe { ::ffi::qt_core_c_QAbstractEventDispatcher_closingDown(self as *mut ::abstract_event_dispatcher::AbstractEventDispatcher) }
  }

  /// C++ method: <span style='color: green;'>```bool QAbstractEventDispatcher::filterNativeEvent(const QByteArray& eventType, void* message, long* result)```</span>
  ///
  ///
  pub unsafe fn filter_native_event(&mut self,
                                    event_type: &::byte_array::ByteArray,
                                    message: *mut ::libc::c_void,
                                    result: *mut ::libc::c_long)
                                    -> bool {
    ::ffi::qt_core_c_QAbstractEventDispatcher_filterNativeEvent(self as *mut ::abstract_event_dispatcher::AbstractEventDispatcher, event_type as *const ::byte_array::ByteArray, message, result)
  }

  /// C++ method: <span style='color: green;'>```pure virtual void QAbstractEventDispatcher::flush()```</span>
  ///
  ///
  pub fn flush(&mut self) {
    unsafe {
      ::ffi::qt_core_c_QAbstractEventDispatcher_flush(self as *mut ::abstract_event_dispatcher::AbstractEventDispatcher)
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual bool QAbstractEventDispatcher::hasPendingEvents()```</span>
  ///
  ///
  pub fn has_pending_events(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QAbstractEventDispatcher_hasPendingEvents(self as *mut ::abstract_event_dispatcher::AbstractEventDispatcher) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractEventDispatcher::installNativeEventFilter(QAbstractNativeEventFilter* filterObj)```</span>
  ///
  ///
pub unsafe fn install_native_event_filter(&mut self, filter_obj: *mut ::abstract_native_event_filter::AbstractNativeEventFilter) {
    ::ffi::qt_core_c_QAbstractEventDispatcher_installNativeEventFilter(self as *mut ::abstract_event_dispatcher::AbstractEventDispatcher, filter_obj)
  }

  /// C++ method: <span style='color: green;'>```static QAbstractEventDispatcher* QAbstractEventDispatcher::instance()```</span>
  ///
  ///
  pub fn instance() -> *mut ::abstract_event_dispatcher::AbstractEventDispatcher {
    unsafe { ::ffi::qt_core_c_QAbstractEventDispatcher_instance_no_args() }
  }

  /// C++ method: <span style='color: green;'>```static QAbstractEventDispatcher* QAbstractEventDispatcher::instance(QThread* thread = ?)```</span>
  ///
  ///
  pub unsafe fn instance_unsafe(thread: *mut ::thread::Thread)
                                -> *mut ::abstract_event_dispatcher::AbstractEventDispatcher {
    ::ffi::qt_core_c_QAbstractEventDispatcher_instance_thread(thread)
  }

  /// C++ method: <span style='color: green;'>```pure virtual void QAbstractEventDispatcher::interrupt()```</span>
  ///
  ///
  pub fn interrupt(&mut self) {
    unsafe { ::ffi::qt_core_c_QAbstractEventDispatcher_interrupt(self as *mut ::abstract_event_dispatcher::AbstractEventDispatcher) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QAbstractEventDispatcher::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QAbstractEventDispatcher_metaObject(self as *const ::abstract_event_dispatcher::AbstractEventDispatcher) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual void QAbstractEventDispatcher::registerSocketNotifier(QSocketNotifier* notifier)```</span>
  ///
  ///
  pub unsafe fn register_socket_notifier(&mut self, notifier: *mut ::socket_notifier::SocketNotifier) {
    ::ffi::qt_core_c_QAbstractEventDispatcher_registerSocketNotifier(self as *mut ::abstract_event_dispatcher::AbstractEventDispatcher, notifier)
  }

  /// C++ method: <span style='color: green;'>```QAbstractEventDispatcher::registerTimer```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn register_timer(&mut self, (::libc::c_int, &::qt::TimerType, *mut ::object::Object)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QAbstractEventDispatcher::registerTimer(int interval, Qt::TimerType timerType, QObject* object)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn register_timer(&mut self, (::libc::c_int, ::libc::c_int, &::qt::TimerType, *mut ::object::Object)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```pure virtual void QAbstractEventDispatcher::registerTimer(int timerId, int interval, Qt::TimerType timerType, QObject* object)```</span>
  ///
  ///
  pub unsafe fn register_timer<'largs, Args>(&'largs mut self, args: Args) -> Args::ReturnType
    where Args: overloading::AbstractEventDispatcherRegisterTimerArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```pure virtual QList<QAbstractEventDispatcher::TimerInfo> QAbstractEventDispatcher::registeredTimers(QObject* object) const```</span>
  ///
  ///
  pub unsafe fn registered_timers(&self,
                                  object: *mut ::object::Object)
                                  -> ::list::ListAbstractEventDispatcherTimerInfo {
    {
      let mut object2: ::list::ListAbstractEventDispatcherTimerInfo =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QAbstractEventDispatcher_registeredTimers_to_output(self as *const ::abstract_event_dispatcher::AbstractEventDispatcher, object, &mut object2);
      object2
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual int QAbstractEventDispatcher::remainingTime(int timerId)```</span>
  ///
  ///
  pub fn remaining_time(&mut self, timer_id: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QAbstractEventDispatcher_remainingTime(self as *mut ::abstract_event_dispatcher::AbstractEventDispatcher, timer_id) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractEventDispatcher::removeNativeEventFilter(QAbstractNativeEventFilter* filterObj)```</span>
  ///
  ///
pub unsafe fn remove_native_event_filter(&mut self, filter_obj: *mut ::abstract_native_event_filter::AbstractNativeEventFilter) {
    ::ffi::qt_core_c_QAbstractEventDispatcher_removeNativeEventFilter(self as *mut ::abstract_event_dispatcher::AbstractEventDispatcher, filter_obj)
  }

  /// C++ method: <span style='color: green;'>```virtual void QAbstractEventDispatcher::startingUp()```</span>
  ///
  ///
  pub fn starting_up(&mut self) {
    unsafe { ::ffi::qt_core_c_QAbstractEventDispatcher_startingUp(self as *mut ::abstract_event_dispatcher::AbstractEventDispatcher) }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractEventDispatcher::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QAbstractEventDispatcher_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractEventDispatcher::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QAbstractEventDispatcher_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual void QAbstractEventDispatcher::unregisterSocketNotifier(QSocketNotifier* notifier)```</span>
  ///
  ///
  pub unsafe fn unregister_socket_notifier(&mut self, notifier: *mut ::socket_notifier::SocketNotifier) {
    ::ffi::qt_core_c_QAbstractEventDispatcher_unregisterSocketNotifier(self as *mut ::abstract_event_dispatcher::AbstractEventDispatcher, notifier)
  }

  /// C++ method: <span style='color: green;'>```pure virtual bool QAbstractEventDispatcher::unregisterTimer(int timerId)```</span>
  ///
  ///
  pub fn unregister_timer(&mut self, timer_id: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_core_c_QAbstractEventDispatcher_unregisterTimer(self as *mut ::abstract_event_dispatcher::AbstractEventDispatcher, timer_id) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual bool QAbstractEventDispatcher::unregisterTimers(QObject* object)```</span>
  ///
  ///
  pub unsafe fn unregister_timers(&mut self, object: *mut ::object::Object) -> bool {
    ::ffi::qt_core_c_QAbstractEventDispatcher_unregisterTimers(self as *mut ::abstract_event_dispatcher::AbstractEventDispatcher, object)
  }

  /// C++ method: <span style='color: green;'>```pure virtual void QAbstractEventDispatcher::wakeUp()```</span>
  ///
  ///
  pub fn wake_up(&mut self) {
    unsafe { ::ffi::qt_core_c_QAbstractEventDispatcher_wakeUp(self as *mut ::abstract_event_dispatcher::AbstractEventDispatcher) }
  }
}

impl ::cpp_utils::CppDeletable for ::abstract_event_dispatcher::AbstractEventDispatcher {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QAbstractEventDispatcher_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `AbstractEventDispatcher`.
  pub struct Signals<'a>(&'a ::abstract_event_dispatcher::AbstractEventDispatcher);
  /// Represents a built-in Qt signal `QAbstractEventDispatcher::aboutToBlock`.
  ///
  /// An object of this type can be created from `AbstractEventDispatcher` with `object.signals().about_to_block()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractEventDispatcher` object.
  pub struct AboutToBlock<'a>(&'a ::abstract_event_dispatcher::AbstractEventDispatcher);
  impl<'a> ::connection::Receiver for AboutToBlock<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2aboutToBlock()\0"
    }
  }
  impl<'a> ::connection::Signal for AboutToBlock<'a> {}
  /// Represents a built-in Qt signal `QAbstractEventDispatcher::objectNameChanged`.
  ///
  /// An object of this type can be created from `AbstractEventDispatcher` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractEventDispatcher` object.
  pub struct ObjectNameChanged<'a>(&'a ::abstract_event_dispatcher::AbstractEventDispatcher);
  impl<'a> ::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::string::String,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::connection::Signal for ObjectNameChanged<'a> {}
  /// Represents a built-in Qt signal `QAbstractEventDispatcher::awake`.
  ///
  /// An object of this type can be created from `AbstractEventDispatcher` with `object.signals().awake()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractEventDispatcher` object.
  pub struct Awake<'a>(&'a ::abstract_event_dispatcher::AbstractEventDispatcher);
  impl<'a> ::connection::Receiver for Awake<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2awake()\0"
    }
  }
  impl<'a> ::connection::Signal for Awake<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QAbstractEventDispatcher::aboutToBlock`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn about_to_block(&self) -> AboutToBlock {
      AboutToBlock(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractEventDispatcher::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractEventDispatcher::awake`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn awake(&self) -> Awake {
      Awake(self.0)
    }
  }
  impl ::abstract_event_dispatcher::AbstractEventDispatcher {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

/// C++ type: <span style='color: green;'>```QAbstractEventDispatcher::TimerInfo```</span>
#[repr(C)]
pub struct TimerInfo([u8; ::type_sizes::QT_CORE_ABSTRACT_EVENT_DISPATCHER_TIMER_INFO]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TimerInfo {
  unsafe fn new_uninitialized() -> TimerInfo {
    TimerInfo(::std::mem::uninitialized())
  }
}

impl TimerInfo {
  /// C++ method: <span style='color: green;'>```int QAbstractEventDispatcher::TimerInfo::interval() const```</span>
  ///
  ///
  pub fn interval(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QAbstractEventDispatcher_TimerInfo_interval(self as *const ::abstract_event_dispatcher::TimerInfo) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QAbstractEventDispatcher::TimerInfo::TimerInfo(int id, int i, Qt::TimerType t)```</span>
  ///
  ///
  pub fn new(id: ::libc::c_int, i: ::libc::c_int, t: &::qt::TimerType) -> ::abstract_event_dispatcher::TimerInfo {
    {
      let mut object: ::abstract_event_dispatcher::TimerInfo =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAbstractEventDispatcher_TimerInfo_constructor(id,
                                                                        i,
                                                                        t as *const ::qt::TimerType,
                                                                        &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractEventDispatcher::TimerInfo::set_interval(int value)```</span>
  ///
  ///
  pub fn set_interval(&mut self, value: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QAbstractEventDispatcher_TimerInfo_set_interval(self as *mut ::abstract_event_dispatcher::TimerInfo, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractEventDispatcher::TimerInfo::set_timerId(int value)```</span>
  ///
  ///
  pub fn set_timer_id(&mut self, value: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QAbstractEventDispatcher_TimerInfo_set_timerId(self as *mut ::abstract_event_dispatcher::TimerInfo, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractEventDispatcher::TimerInfo::set_timerType(Qt::TimerType value)```</span>
  ///
  ///
  pub fn set_timer_type(&mut self, value: &::qt::TimerType) {
    unsafe { ::ffi::qt_core_c_QAbstractEventDispatcher_TimerInfo_set_timerType(self as *mut ::abstract_event_dispatcher::TimerInfo, value as *const ::qt::TimerType) }
  }

  /// C++ method: <span style='color: green;'>```int QAbstractEventDispatcher::TimerInfo::timerId() const```</span>
  ///
  ///
  pub fn timer_id(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QAbstractEventDispatcher_TimerInfo_timerId(self as *const ::abstract_event_dispatcher::TimerInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```const Qt::TimerType& QAbstractEventDispatcher::TimerInfo::timerType() const```</span>
  ///
  ///
  pub fn timer_type<'l0>(&'l0 self) -> &'l0 ::qt::TimerType {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractEventDispatcher_TimerInfo_timerType(self as *const ::abstract_event_dispatcher::TimerInfo) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt::TimerType& QAbstractEventDispatcher::TimerInfo::timerType_mut()```</span>
  ///
  ///
  pub fn timer_type_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt::TimerType {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractEventDispatcher_TimerInfo_timerType_mut(self as *mut ::abstract_event_dispatcher::TimerInfo) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

impl Drop for ::abstract_event_dispatcher::TimerInfo {
  /// C++ method: <span style='color: green;'>```[destructor] void QAbstractEventDispatcher::TimerInfo::~QAbstractEventDispatcher::TimerInfo()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QAbstractEventDispatcher_TimerInfo_destructor(self as *mut ::abstract_event_dispatcher::TimerInfo) }
  }
}

impl ::cpp_utils::DynamicCast<::abstract_event_dispatcher::AbstractEventDispatcher> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::abstract_event_dispatcher::AbstractEventDispatcher> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractEventDispatcher_G_dynamic_cast_QAbstractEventDispatcher_ptr(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::abstract_event_dispatcher::AbstractEventDispatcher> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractEventDispatcher_G_dynamic_cast_QAbstractEventDispatcher_ptr(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::abstract_event_dispatcher::AbstractEventDispatcher {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractEventDispatcher_G_static_cast_QObject_ptr(self as *mut ::abstract_event_dispatcher::AbstractEventDispatcher) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractEventDispatcher_G_static_cast_QObject_ptr(self as *const ::abstract_event_dispatcher::AbstractEventDispatcher as *mut ::abstract_event_dispatcher::AbstractEventDispatcher) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_event_dispatcher::AbstractEventDispatcher> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_event_dispatcher::AbstractEventDispatcher {
    let ffi_result = ::ffi::qt_core_c_QAbstractEventDispatcher_G_static_cast_QAbstractEventDispatcher_ptr(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_event_dispatcher::AbstractEventDispatcher {
    let ffi_result = ::ffi::qt_core_c_QAbstractEventDispatcher_G_static_cast_QAbstractEventDispatcher_ptr(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::abstract_event_dispatcher::AbstractEventDispatcher {
  type Target = ::object::Object;
  fn deref(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractEventDispatcher_G_static_cast_QObject_ptr(self as *const ::abstract_event_dispatcher::AbstractEventDispatcher as *mut ::abstract_event_dispatcher::AbstractEventDispatcher) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::abstract_event_dispatcher::AbstractEventDispatcher {
  fn deref_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractEventDispatcher_G_static_cast_QObject_ptr(self as *mut ::abstract_event_dispatcher::AbstractEventDispatcher) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [AbstractEventDispatcher::register_timer](../struct.AbstractEventDispatcher.html#method.register_timer) method.
  pub trait AbstractEventDispatcherRegisterTimerArgs<'largs> {
    type ReturnType;
    unsafe fn exec(self,
                   original_self: &'largs mut ::abstract_event_dispatcher::AbstractEventDispatcher)
                   -> Self::ReturnType;
  }
  impl<'largs> AbstractEventDispatcherRegisterTimerArgs<'largs>
    for (::libc::c_int, &'largs ::qt::TimerType, *mut ::object::Object) {
    type ReturnType = ::libc::c_int;
    unsafe fn exec(self,
                   original_self: &'largs mut ::abstract_event_dispatcher::AbstractEventDispatcher)
                   -> ::libc::c_int {
      let interval = self.0;
      let timer_type = self.1;
      let object = self.2;
      ::ffi::qt_core_c_QAbstractEventDispatcher_registerTimer_interval_timerType_object(original_self as *mut ::abstract_event_dispatcher::AbstractEventDispatcher, interval, timer_type as *const ::qt::TimerType, object)
    }
  }
  impl<'largs> AbstractEventDispatcherRegisterTimerArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::qt::TimerType, *mut ::object::Object) {
    type ReturnType = ();
    unsafe fn exec(self, original_self: &'largs mut ::abstract_event_dispatcher::AbstractEventDispatcher) -> () {
      let timer_id = self.0;
      let interval = self.1;
      let timer_type = self.2;
      let object = self.3;
      ::ffi::qt_core_c_QAbstractEventDispatcher_registerTimer_timerId_interval_timerType_object(original_self as *mut ::abstract_event_dispatcher::AbstractEventDispatcher, timer_id, interval, timer_type as *const ::qt::TimerType, object)
    }
  }
}

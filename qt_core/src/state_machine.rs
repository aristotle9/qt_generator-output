/// C++ type: <span style='color: green;'>```QStateMachine::Error```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Error {
  /// C++ enum variant: <span style='color: green;'>```NoError = 0```</span>
  NoError = 0,
  /// C++ enum variant: <span style='color: green;'>```NoInitialStateError = 1```</span>
  NoInitialStateError = 1,
  /// C++ enum variant: <span style='color: green;'>```NoDefaultStateInHistoryStateError = 2```</span>
  NoDefaultStateInHistoryStateError = 2,
  /// C++ enum variant: <span style='color: green;'>```NoCommonAncestorForTransitionError = 3```</span>
  NoCommonAncestorForTransitionError = 3,
}

/// C++ type: <span style='color: green;'>```QStateMachine::EventPriority```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum EventPriority {
  /// C++ enum variant: <span style='color: green;'>```NormalPriority = 0```</span>
  Normal = 0,
  /// C++ enum variant: <span style='color: green;'>```HighPriority = 1```</span>
  High = 1,
}

/// C++ type: <span style='color: green;'>```QStateMachine::SignalEvent```</span>
#[repr(C)]
pub struct SignalEvent(u8);

impl SignalEvent {
  /// C++ method: <span style='color: green;'>```QList<QVariant> QStateMachine::SignalEvent::arguments() const```</span>
  ///
  ///
  pub fn arguments(&self) -> ::list::ListVariant {
    {
      let mut object: ::list::ListVariant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStateMachine_SignalEvent_arguments_to_output(self as *const ::state_machine::SignalEvent,
                                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QStateMachine::SignalEvent::SignalEvent(QObject* sender, int signalIndex, const QList<QVariant>& arguments)```</span>
  ///
  ///
  pub unsafe fn new(sender: *mut ::object::Object,
                    signal_index: ::libc::c_int,
                    arguments: &::list::ListVariant)
                    -> ::cpp_utils::CppBox<::state_machine::SignalEvent> {
    let ffi_result = ::ffi::qt_core_c_QStateMachine_SignalEvent_new(sender,
                                                                    signal_index,
                                                                    arguments as *const ::list::ListVariant);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QObject* QStateMachine::SignalEvent::sender() const```</span>
  ///
  ///
  pub fn sender(&self) -> *mut ::object::Object {
    unsafe { ::ffi::qt_core_c_QStateMachine_SignalEvent_sender(self as *const ::state_machine::SignalEvent) }
  }

  /// C++ method: <span style='color: green;'>```int QStateMachine::SignalEvent::signalIndex() const```</span>
  ///
  ///
  pub fn signal_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QStateMachine_SignalEvent_signalIndex(self as *const ::state_machine::SignalEvent) }
  }
}

impl ::cpp_utils::CppDeletable for ::state_machine::SignalEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QStateMachine_SignalEvent_delete
  }
}

/// C++ type: <span style='color: green;'>```QStateMachine```</span>
#[repr(C)]
pub struct StateMachine(u8);

impl StateMachine {
  /// C++ method: <span style='color: green;'>```void QStateMachine::addDefaultAnimation(QAbstractAnimation* animation)```</span>
  ///
  ///
  pub unsafe fn add_default_animation(&mut self, animation: *mut ::abstract_animation::AbstractAnimation) {
    ::ffi::qt_core_c_QStateMachine_addDefaultAnimation(self as *mut ::state_machine::StateMachine, animation)
  }

  /// C++ method: <span style='color: green;'>```void QStateMachine::addState(QAbstractState* state)```</span>
  ///
  ///
  pub unsafe fn add_state(&mut self, state: *mut ::abstract_state::AbstractState) {
    ::ffi::qt_core_c_QStateMachine_addState(self as *mut ::state_machine::StateMachine, state)
  }

  /// C++ method: <span style='color: green;'>```bool QStateMachine::cancelDelayedEvent(int id)```</span>
  ///
  ///
  pub fn cancel_delayed_event(&mut self, id: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_core_c_QStateMachine_cancelDelayedEvent(self as *mut ::state_machine::StateMachine, id) }
  }

  /// C++ method: <span style='color: green;'>```void QStateMachine::clearError()```</span>
  ///
  ///
  pub fn clear_error(&mut self) {
    unsafe { ::ffi::qt_core_c_QStateMachine_clearError(self as *mut ::state_machine::StateMachine) }
  }

  /// C++ method: <span style='color: green;'>```QSet<QAbstractState*> QStateMachine::configuration() const```</span>
  ///
  ///
  pub fn configuration(&self) -> ::set::SetAbstractStateMutPtr {
    {
      let mut object: ::set::SetAbstractStateMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStateMachine_configuration_to_output(self as *const ::state_machine::StateMachine,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractAnimation*> QStateMachine::defaultAnimations() const```</span>
  ///
  ///
  pub fn default_animations(&self) -> ::list::ListAbstractAnimationMutPtr {
    {
      let mut object: ::list::ListAbstractAnimationMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStateMachine_defaultAnimations_to_output(self as *const ::state_machine::StateMachine,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStateMachine::Error QStateMachine::error() const```</span>
  ///
  ///
  pub fn error(&self) -> ::state_machine::Error {
    unsafe { ::ffi::qt_core_c_QStateMachine_error(self as *const ::state_machine::StateMachine) }
  }

  /// C++ method: <span style='color: green;'>```QString QStateMachine::errorString() const```</span>
  ///
  ///
  pub fn error_string(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStateMachine_errorString_to_output(self as *const ::state_machine::StateMachine, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QStateMachine::eventFilter(QObject* watched, QEvent* event)```</span>
  ///
  ///
  pub unsafe fn event_filter(&mut self, watched: *mut ::object::Object, event: *mut ::event::Event) -> bool {
    ::ffi::qt_core_c_QStateMachine_eventFilter(self as *mut ::state_machine::StateMachine, watched, event)
  }

  /// C++ method: <span style='color: green;'>```bool QStateMachine::isAnimated() const```</span>
  ///
  ///
  pub fn is_animated(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QStateMachine_isAnimated(self as *const ::state_machine::StateMachine) }
  }

  /// C++ method: <span style='color: green;'>```bool QStateMachine::isRunning() const```</span>
  ///
  ///
  pub fn is_running(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QStateMachine_isRunning(self as *const ::state_machine::StateMachine) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QStateMachine::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QStateMachine_metaObject(self as *const ::state_machine::StateMachine) }
  }

  /// C++ method: <span style='color: green;'>```QStateMachine::QStateMachine```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::state_machine::StateMachine>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStateMachine::QStateMachine()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::state::ChildMode) -> ::cpp_utils::CppBox<::state_machine::StateMachine>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStateMachine::QStateMachine(QState::ChildMode childMode)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::state_machine::StateMachine>
    where Args: overloading::StateMachineNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QStateMachine::QStateMachine```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::object::Object) -> ::cpp_utils::CppBox<::state_machine::StateMachine>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStateMachine::QStateMachine(QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::state::ChildMode, *mut ::object::Object)) -> ::cpp_utils::CppBox<::state_machine::StateMachine>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStateMachine::QStateMachine(QState::ChildMode childMode, QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::state_machine::StateMachine>
    where Args: overloading::StateMachineNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```int QStateMachine::postDelayedEvent(QEvent* event, int delay)```</span>
  ///
  ///
  pub unsafe fn post_delayed_event(&mut self, event: *mut ::event::Event, delay: ::libc::c_int) -> ::libc::c_int {
    ::ffi::qt_core_c_QStateMachine_postDelayedEvent(self as *mut ::state_machine::StateMachine, event, delay)
  }

  /// C++ method: <span style='color: green;'>```QStateMachine::postEvent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn post_event(&mut self, *mut ::event::Event) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QStateMachine::postEvent(QEvent* event)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn post_event(&mut self, (*mut ::event::Event, ::state_machine::EventPriority)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QStateMachine::postEvent(QEvent* event, QStateMachine::EventPriority priority = ?)```</span>
  ///
  ///
  pub unsafe fn post_event<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::StateMachinePostEventArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QStateMachine::removeDefaultAnimation(QAbstractAnimation* animation)```</span>
  ///
  ///
  pub unsafe fn remove_default_animation(&mut self, animation: *mut ::abstract_animation::AbstractAnimation) {
    ::ffi::qt_core_c_QStateMachine_removeDefaultAnimation(self as *mut ::state_machine::StateMachine, animation)
  }

  /// C++ method: <span style='color: green;'>```void QStateMachine::removeState(QAbstractState* state)```</span>
  ///
  ///
  pub unsafe fn remove_state(&mut self, state: *mut ::abstract_state::AbstractState) {
    ::ffi::qt_core_c_QStateMachine_removeState(self as *mut ::state_machine::StateMachine, state)
  }

  /// C++ method: <span style='color: green;'>```void QStateMachine::setAnimated(bool enabled)```</span>
  ///
  ///
  pub fn set_animated(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_core_c_QStateMachine_setAnimated(self as *mut ::state_machine::StateMachine, enabled) }
  }

  /// C++ method: <span style='color: green;'>```void QStateMachine::setGlobalRestorePolicy(QState::RestorePolicy restorePolicy)```</span>
  ///
  ///
  pub fn set_global_restore_policy(&mut self, restore_policy: &::state::RestorePolicy) {
    unsafe {
      ::ffi::qt_core_c_QStateMachine_setGlobalRestorePolicy(self as *mut ::state_machine::StateMachine,
                                                            restore_policy as *const ::state::RestorePolicy)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QStateMachine::setRunning(bool running)```</span>
  ///
  ///
  pub fn set_running(&mut self, running: bool) {
    unsafe { ::ffi::qt_core_c_QStateMachine_setRunning(self as *mut ::state_machine::StateMachine, running) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QStateMachine::start()```</span>
  ///
  ///
  pub fn start(&mut self) {
    unsafe { ::ffi::qt_core_c_QStateMachine_start(self as *mut ::state_machine::StateMachine) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QStateMachine::stop()```</span>
  ///
  ///
  pub fn stop(&mut self) {
    unsafe { ::ffi::qt_core_c_QStateMachine_stop(self as *mut ::state_machine::StateMachine) }
  }

  /// C++ method: <span style='color: green;'>```static QString QStateMachine::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QStateMachine_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QStateMachine::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QStateMachine_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::state_machine::StateMachine {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QStateMachine_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `StateMachine`.
  pub struct Signals<'a>(&'a ::state_machine::StateMachine);
  /// Represents a built-in Qt signal `QStateMachine::propertiesAssigned`.
  ///
  /// An object of this type can be created from `StateMachine` with `object.signals().properties_assigned()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StateMachine` object.
  pub struct PropertiesAssigned<'a>(&'a ::state_machine::StateMachine);
  impl<'a> ::connection::Receiver for PropertiesAssigned<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2propertiesAssigned()\0"
    }
  }
  impl<'a> ::connection::Signal for PropertiesAssigned<'a> {}
  /// Represents a built-in Qt signal `QStateMachine::initialStateChanged`.
  ///
  /// An object of this type can be created from `StateMachine` with `object.signals().initial_state_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StateMachine` object.
  pub struct InitialStateChanged<'a>(&'a ::state_machine::StateMachine);
  impl<'a> ::connection::Receiver for InitialStateChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2initialStateChanged()\0"
    }
  }
  impl<'a> ::connection::Signal for InitialStateChanged<'a> {}
  /// Represents a built-in Qt signal `QStateMachine::runningChanged`.
  ///
  /// An object of this type can be created from `StateMachine` with `object.signals().running_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StateMachine` object.
  pub struct RunningChanged<'a>(&'a ::state_machine::StateMachine);
  impl<'a> ::connection::Receiver for RunningChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2runningChanged(bool)\0"
    }
  }
  impl<'a> ::connection::Signal for RunningChanged<'a> {}
  /// Represents a built-in Qt signal `QStateMachine::started`.
  ///
  /// An object of this type can be created from `StateMachine` with `object.signals().started()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StateMachine` object.
  pub struct Started<'a>(&'a ::state_machine::StateMachine);
  impl<'a> ::connection::Receiver for Started<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2started()\0"
    }
  }
  impl<'a> ::connection::Signal for Started<'a> {}
  /// Represents a built-in Qt signal `QStateMachine::errorStateChanged`.
  ///
  /// An object of this type can be created from `StateMachine` with `object.signals().error_state_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StateMachine` object.
  pub struct ErrorStateChanged<'a>(&'a ::state_machine::StateMachine);
  impl<'a> ::connection::Receiver for ErrorStateChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2errorStateChanged()\0"
    }
  }
  impl<'a> ::connection::Signal for ErrorStateChanged<'a> {}
  /// Represents a built-in Qt signal `QStateMachine::stopped`.
  ///
  /// An object of this type can be created from `StateMachine` with `object.signals().stopped()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StateMachine` object.
  pub struct Stopped<'a>(&'a ::state_machine::StateMachine);
  impl<'a> ::connection::Receiver for Stopped<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2stopped()\0"
    }
  }
  impl<'a> ::connection::Signal for Stopped<'a> {}
  /// Represents a built-in Qt signal `QStateMachine::finished`.
  ///
  /// An object of this type can be created from `StateMachine` with `object.signals().finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StateMachine` object.
  pub struct Finished<'a>(&'a ::state_machine::StateMachine);
  impl<'a> ::connection::Receiver for Finished<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2finished()\0"
    }
  }
  impl<'a> ::connection::Signal for Finished<'a> {}
  /// Represents a built-in Qt signal `QStateMachine::childModeChanged`.
  ///
  /// An object of this type can be created from `StateMachine` with `object.signals().child_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StateMachine` object.
  pub struct ChildModeChanged<'a>(&'a ::state_machine::StateMachine);
  impl<'a> ::connection::Receiver for ChildModeChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2childModeChanged()\0"
    }
  }
  impl<'a> ::connection::Signal for ChildModeChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QStateMachine::propertiesAssigned`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn properties_assigned(&self) -> PropertiesAssigned {
      PropertiesAssigned(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStateMachine::initialStateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn initial_state_changed(&self) -> InitialStateChanged {
      InitialStateChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStateMachine::runningChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn running_changed(&self) -> RunningChanged {
      RunningChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStateMachine::started`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn started(&self) -> Started {
      Started(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStateMachine::errorStateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn error_state_changed(&self) -> ErrorStateChanged {
      ErrorStateChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStateMachine::stopped`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn stopped(&self) -> Stopped {
      Stopped(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStateMachine::finished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn finished(&self) -> Finished {
      Finished(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStateMachine::childModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn child_mode_changed(&self) -> ChildModeChanged {
      ChildModeChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `StateMachine`.
  pub struct Slots<'a>(&'a ::state_machine::StateMachine);
  /// Represents a built-in Qt slot `QStateMachine::stop`.
  ///
  /// An object of this type can be created from `StateMachine` with `object.slots().stop()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StateMachine` object.
  pub struct Stop<'a>(&'a ::state_machine::StateMachine);
  impl<'a> ::connection::Receiver for Stop<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1stop()\0"
    }
  }
  /// Represents a built-in Qt slot `QStateMachine::start`.
  ///
  /// An object of this type can be created from `StateMachine` with `object.slots().start()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StateMachine` object.
  pub struct Start<'a>(&'a ::state_machine::StateMachine);
  impl<'a> ::connection::Receiver for Start<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1start()\0"
    }
  }
  /// Represents a built-in Qt slot `QStateMachine::setRunning`.
  ///
  /// An object of this type can be created from `StateMachine` with `object.slots().set_running()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StateMachine` object.
  pub struct SetRunning<'a>(&'a ::state_machine::StateMachine);
  impl<'a> ::connection::Receiver for SetRunning<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRunning(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QStateMachine::stop`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn stop(&self) -> Stop {
      Stop(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStateMachine::start`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn start(&self) -> Start {
      Start(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStateMachine::setRunning`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_running(&self) -> SetRunning {
      SetRunning(self.0)
    }
  }
  impl ::state_machine::StateMachine {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
    /// Provides access to built-in Qt slots of this type
    pub fn slots(&self) -> Slots {
      Slots(self)
    }
  }

}

/// C++ type: <span style='color: green;'>```QStateMachine::WrappedEvent```</span>
#[repr(C)]
pub struct WrappedEvent(u8);

impl WrappedEvent {
  /// C++ method: <span style='color: green;'>```QEvent* QStateMachine::WrappedEvent::event() const```</span>
  ///
  ///
  pub fn event(&self) -> *mut ::event::Event {
    unsafe { ::ffi::qt_core_c_QStateMachine_WrappedEvent_event(self as *const ::state_machine::WrappedEvent) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QStateMachine::WrappedEvent::WrappedEvent(QObject* object, QEvent* event)```</span>
  ///
  ///
  pub unsafe fn new(object: *mut ::object::Object,
                    event: *mut ::event::Event)
                    -> ::cpp_utils::CppBox<::state_machine::WrappedEvent> {
    let ffi_result = ::ffi::qt_core_c_QStateMachine_WrappedEvent_new(object, event);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QObject* QStateMachine::WrappedEvent::object() const```</span>
  ///
  ///
  pub fn object(&self) -> *mut ::object::Object {
    unsafe { ::ffi::qt_core_c_QStateMachine_WrappedEvent_object(self as *const ::state_machine::WrappedEvent) }
  }
}

impl ::cpp_utils::CppDeletable for ::state_machine::WrappedEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QStateMachine_WrappedEvent_delete
  }
}

impl ::cpp_utils::DynamicCast<::state_machine::SignalEvent> for ::event::Event {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::state_machine::SignalEvent> {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QStateMachine_G_dynamic_cast_QStateMachine_SignalEvent_ptr(self as *mut ::event::Event)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::state_machine::SignalEvent> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStateMachine_G_dynamic_cast_QStateMachine_SignalEvent_ptr(self as *const ::event::Event as *mut ::event::Event) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::state_machine::StateMachine> for ::abstract_state::AbstractState {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::state_machine::StateMachine> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStateMachine_G_dynamic_cast_QStateMachine_ptr_QAbstractState(self as *mut ::abstract_state::AbstractState) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::state_machine::StateMachine> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStateMachine_G_dynamic_cast_QStateMachine_ptr_QAbstractState(self as *const ::abstract_state::AbstractState as *mut ::abstract_state::AbstractState) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::state_machine::StateMachine> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::state_machine::StateMachine> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QStateMachine_G_dynamic_cast_QStateMachine_ptr_QObject(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::state_machine::StateMachine> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStateMachine_G_dynamic_cast_QStateMachine_ptr_QObject(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::state_machine::StateMachine> for ::state::State {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::state_machine::StateMachine> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QStateMachine_G_dynamic_cast_QStateMachine_ptr_QState(self as *mut ::state::State) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::state_machine::StateMachine> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStateMachine_G_dynamic_cast_QStateMachine_ptr_QState(self as *const ::state::State as *mut ::state::State) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::state_machine::WrappedEvent> for ::event::Event {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::state_machine::WrappedEvent> {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QStateMachine_G_dynamic_cast_QStateMachine_WrappedEvent_ptr(self as *mut ::event::Event)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::state_machine::WrappedEvent> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStateMachine_G_dynamic_cast_QStateMachine_WrappedEvent_ptr(self as *const ::event::Event as *mut ::event::Event) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::abstract_state::AbstractState> for ::state_machine::StateMachine {
  fn static_cast_mut(&mut self) -> &mut ::abstract_state::AbstractState {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QStateMachine_G_static_cast_QAbstractState_ptr(self as *mut ::state_machine::StateMachine)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_state::AbstractState {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStateMachine_G_static_cast_QAbstractState_ptr(self as *const ::state_machine::StateMachine as *mut ::state_machine::StateMachine) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::event::Event> for ::state_machine::SignalEvent {
  fn static_cast_mut(&mut self) -> &mut ::event::Event {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStateMachine_G_static_cast_QEvent_ptr_QStateMachine_SignalEvent(self as *mut ::state_machine::SignalEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::event::Event {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStateMachine_G_static_cast_QEvent_ptr_QStateMachine_SignalEvent(self as *const ::state_machine::SignalEvent as *mut ::state_machine::SignalEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::event::Event> for ::state_machine::WrappedEvent {
  fn static_cast_mut(&mut self) -> &mut ::event::Event {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStateMachine_G_static_cast_QEvent_ptr_QStateMachine_WrappedEvent(self as *mut ::state_machine::WrappedEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::event::Event {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStateMachine_G_static_cast_QEvent_ptr_QStateMachine_WrappedEvent(self as *const ::state_machine::WrappedEvent as *mut ::state_machine::WrappedEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::state_machine::StateMachine {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QStateMachine_G_static_cast_QObject_ptr(self as *mut ::state_machine::StateMachine) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStateMachine_G_static_cast_QObject_ptr(self as *const ::state_machine::StateMachine as *mut ::state_machine::StateMachine) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::state::State> for ::state_machine::StateMachine {
  fn static_cast_mut(&mut self) -> &mut ::state::State {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QStateMachine_G_static_cast_QState_ptr(self as *mut ::state_machine::StateMachine) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::state::State {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStateMachine_G_static_cast_QState_ptr(self as *const ::state_machine::StateMachine as *mut ::state_machine::StateMachine) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::state_machine::SignalEvent> for ::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::state_machine::SignalEvent {
    let ffi_result =
      ::ffi::qt_core_c_QStateMachine_G_static_cast_QStateMachine_SignalEvent_ptr(self as *mut ::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::state_machine::SignalEvent {
    let ffi_result = ::ffi::qt_core_c_QStateMachine_G_static_cast_QStateMachine_SignalEvent_ptr(self as *const ::event::Event as *mut ::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::state_machine::StateMachine> for ::abstract_state::AbstractState {
  unsafe fn static_cast_mut(&mut self) -> &mut ::state_machine::StateMachine {
    let ffi_result = ::ffi::qt_core_c_QStateMachine_G_static_cast_QStateMachine_ptr_QAbstractState(self as *mut ::abstract_state::AbstractState);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::state_machine::StateMachine {
    let ffi_result = ::ffi::qt_core_c_QStateMachine_G_static_cast_QStateMachine_ptr_QAbstractState(self as *const ::abstract_state::AbstractState as *mut ::abstract_state::AbstractState);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::state_machine::StateMachine> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::state_machine::StateMachine {
    let ffi_result =
      ::ffi::qt_core_c_QStateMachine_G_static_cast_QStateMachine_ptr_QObject(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::state_machine::StateMachine {
    let ffi_result = ::ffi::qt_core_c_QStateMachine_G_static_cast_QStateMachine_ptr_QObject(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::state_machine::StateMachine> for ::state::State {
  unsafe fn static_cast_mut(&mut self) -> &mut ::state_machine::StateMachine {
    let ffi_result =
      ::ffi::qt_core_c_QStateMachine_G_static_cast_QStateMachine_ptr_QState(self as *mut ::state::State);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::state_machine::StateMachine {
    let ffi_result = ::ffi::qt_core_c_QStateMachine_G_static_cast_QStateMachine_ptr_QState(self as *const ::state::State as *mut ::state::State);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::state_machine::WrappedEvent> for ::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::state_machine::WrappedEvent {
    let ffi_result =
      ::ffi::qt_core_c_QStateMachine_G_static_cast_QStateMachine_WrappedEvent_ptr(self as *mut ::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::state_machine::WrappedEvent {
    let ffi_result = ::ffi::qt_core_c_QStateMachine_G_static_cast_QStateMachine_WrappedEvent_ptr(self as *const ::event::Event as *mut ::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::state_machine::SignalEvent {
  type Target = ::event::Event;
  fn deref(&self) -> &::event::Event {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStateMachine_G_static_cast_QEvent_ptr_QStateMachine_SignalEvent(self as *const ::state_machine::SignalEvent as *mut ::state_machine::SignalEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::state_machine::WrappedEvent {
  type Target = ::event::Event;
  fn deref(&self) -> &::event::Event {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStateMachine_G_static_cast_QEvent_ptr_QStateMachine_WrappedEvent(self as *const ::state_machine::WrappedEvent as *mut ::state_machine::WrappedEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::state_machine::StateMachine {
  type Target = ::state::State;
  fn deref(&self) -> &::state::State {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStateMachine_G_static_cast_QState_ptr(self as *const ::state_machine::StateMachine as *mut ::state_machine::StateMachine) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::state_machine::SignalEvent {
  fn deref_mut(&mut self) -> &mut ::event::Event {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStateMachine_G_static_cast_QEvent_ptr_QStateMachine_SignalEvent(self as *mut ::state_machine::SignalEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::state_machine::WrappedEvent {
  fn deref_mut(&mut self) -> &mut ::event::Event {
    let ffi_result = unsafe { ::ffi::qt_core_c_QStateMachine_G_static_cast_QEvent_ptr_QStateMachine_WrappedEvent(self as *mut ::state_machine::WrappedEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::state_machine::StateMachine {
  fn deref_mut(&mut self) -> &mut ::state::State {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QStateMachine_G_static_cast_QState_ptr(self as *mut ::state_machine::StateMachine) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StateMachine::new](../struct.StateMachine.html#method.new) method.
  pub trait StateMachineNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::state_machine::StateMachine>;
  }
  impl<'a> StateMachineNewArgs for &'a ::state::ChildMode {
    fn exec(self) -> ::cpp_utils::CppBox<::state_machine::StateMachine> {
      let child_mode = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QStateMachine_new_childMode(child_mode as *const ::state::ChildMode) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl StateMachineNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::state_machine::StateMachine> {

      let ffi_result = unsafe { ::ffi::qt_core_c_QStateMachine_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [StateMachine::new_unsafe](../struct.StateMachine.html#method.new_unsafe) method.
  pub trait StateMachineNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::state_machine::StateMachine>;
  }
  impl<'a> StateMachineNewUnsafeArgs for (&'a ::state::ChildMode, *mut ::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::state_machine::StateMachine> {
      let child_mode = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_core_c_QStateMachine_new_childMode_parent(child_mode as *const ::state::ChildMode,
                                                                           parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl StateMachineNewUnsafeArgs for *mut ::object::Object {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::state_machine::StateMachine> {
      let parent = self;
      let ffi_result = ::ffi::qt_core_c_QStateMachine_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [StateMachine::post_event](../struct.StateMachine.html#method.post_event) method.
  pub trait StateMachinePostEventArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::state_machine::StateMachine) -> ();
  }
  impl<'largs> StateMachinePostEventArgs<'largs> for *mut ::event::Event {
    unsafe fn exec(self, original_self: &'largs mut ::state_machine::StateMachine) -> () {
      let event = self;
      ::ffi::qt_core_c_QStateMachine_postEvent_event(original_self as *mut ::state_machine::StateMachine, event)
    }
  }
  impl<'largs> StateMachinePostEventArgs<'largs> for (*mut ::event::Event, ::state_machine::EventPriority) {
    unsafe fn exec(self, original_self: &'largs mut ::state_machine::StateMachine) -> () {
      let event = self.0;
      let priority = self.1;
      ::ffi::qt_core_c_QStateMachine_postEvent_event_priority(original_self as *mut ::state_machine::StateMachine,
                                                              event,
                                                              priority)
    }
  }
}

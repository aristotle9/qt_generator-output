/// C++ type: <span style='color: green;'>```QState::ChildMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ChildMode {
  /// C++ enum variant: <span style='color: green;'>```ExclusiveStates = 0```</span>
  Exclusive = 0,
  /// C++ enum variant: <span style='color: green;'>```ParallelStates = 1```</span>
  Parallel = 1,
}

/// C++ type: <span style='color: green;'>```QState::RestorePolicy```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum RestorePolicy {
  /// C++ enum variant: <span style='color: green;'>```DontRestoreProperties = 0```</span>
  DontRestoreProperties = 0,
  /// C++ enum variant: <span style='color: green;'>```RestoreProperties = 1```</span>
  RestoreProperties = 1,
}

/// C++ type: <span style='color: green;'>```QState```</span>
#[repr(C)]
pub struct State(u8);

impl State {
  /// C++ method: <span style='color: green;'>```QState::addTransition```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_transition(&mut self, *mut ::abstract_state::AbstractState) -> *mut ::abstract_transition::AbstractTransition```<br>
  /// C++ method: <span style='color: green;'>```QAbstractTransition* QState::addTransition(QAbstractState* target)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_transition(&mut self, (*const ::object::Object, *const ::libc::c_char, *mut ::abstract_state::AbstractState)) -> *mut ::signal_transition::SignalTransition```<br>
  /// C++ method: <span style='color: green;'>```QSignalTransition* QState::addTransition(const QObject* sender, const char* signal, QAbstractState* target)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn add_transition(&mut self, *mut ::abstract_transition::AbstractTransition) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QState::addTransition(QAbstractTransition* transition)```</span>
  ///
  ///
  pub unsafe fn add_transition<'largs, Args>(&'largs mut self, args: Args) -> Args::ReturnType
    where Args: overloading::StateAddTransitionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QState::assignProperty(QObject* object, const char* name, const QVariant& value)```</span>
  ///
  ///
  pub unsafe fn assign_property(&mut self,
                                object: *mut ::object::Object,
                                name: *const ::libc::c_char,
                                value: &::variant::Variant) {
    ::ffi::qt_core_c_QState_assignProperty(self as *mut ::state::State,
                                           object,
                                           name,
                                           value as *const ::variant::Variant)
  }

  /// C++ method: <span style='color: green;'>```QState::ChildMode QState::childMode() const```</span>
  ///
  ///
  pub fn child_mode(&self) -> ::state::ChildMode {
    unsafe { ::ffi::qt_core_c_QState_childMode(self as *const ::state::State) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractState* QState::errorState() const```</span>
  ///
  ///
  pub fn error_state(&self) -> *mut ::abstract_state::AbstractState {
    unsafe { ::ffi::qt_core_c_QState_errorState(self as *const ::state::State) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractState* QState::initialState() const```</span>
  ///
  ///
  pub fn initial_state(&self) -> *mut ::abstract_state::AbstractState {
    unsafe { ::ffi::qt_core_c_QState_initialState(self as *const ::state::State) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QState::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QState_metaObject(self as *const ::state::State) }
  }

  /// C++ method: <span style='color: green;'>```QState::QState```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::state::State>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QState::QState()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::state::ChildMode) -> ::cpp_utils::CppBox<::state::State>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QState::QState(QState::ChildMode childMode)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::state::State>
    where Args: overloading::StateNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QState::QState```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::state::State) -> ::cpp_utils::CppBox<::state::State>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QState::QState(QState* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((::state::ChildMode, *mut ::state::State)) -> ::cpp_utils::CppBox<::state::State>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QState::QState(QState::ChildMode childMode, QState* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::state::State>
    where Args: overloading::StateNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QState::removeTransition(QAbstractTransition* transition)```</span>
  ///
  ///
  pub unsafe fn remove_transition(&mut self, transition: *mut ::abstract_transition::AbstractTransition) {
    ::ffi::qt_core_c_QState_removeTransition(self as *mut ::state::State, transition)
  }

  /// C++ method: <span style='color: green;'>```void QState::setChildMode(QState::ChildMode mode)```</span>
  ///
  ///
  pub fn set_child_mode(&mut self, mode: ::state::ChildMode) {
    unsafe { ::ffi::qt_core_c_QState_setChildMode(self as *mut ::state::State, mode) }
  }

  /// C++ method: <span style='color: green;'>```void QState::setErrorState(QAbstractState* state)```</span>
  ///
  ///
  pub unsafe fn set_error_state(&mut self, state: *mut ::abstract_state::AbstractState) {
    ::ffi::qt_core_c_QState_setErrorState(self as *mut ::state::State, state)
  }

  /// C++ method: <span style='color: green;'>```void QState::setInitialState(QAbstractState* state)```</span>
  ///
  ///
  pub unsafe fn set_initial_state(&mut self, state: *mut ::abstract_state::AbstractState) {
    ::ffi::qt_core_c_QState_setInitialState(self as *mut ::state::State, state)
  }

  /// C++ method: <span style='color: green;'>```static QString QState::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QState_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QState::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QState_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractTransition*> QState::transitions() const```</span>
  ///
  ///
  pub fn transitions(&self) -> ::list::ListAbstractTransitionMutPtr {
    {
      let mut object: ::list::ListAbstractTransitionMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QState_transitions_to_output(self as *const ::state::State, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::state::State {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QState_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `State`.
  pub struct Signals<'a>(&'a ::state::State);
  /// Represents a built-in Qt signal `QState::exited`.
  ///
  /// An object of this type can be created from `State` with `object.signals().exited()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `State` object.
  pub struct Exited<'a>(&'a ::state::State);
  impl<'a> ::connection::Receiver for Exited<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2exited()\0"
    }
  }
  impl<'a> ::connection::Signal for Exited<'a> {}
  /// Represents a built-in Qt signal `QState::errorStateChanged`.
  ///
  /// An object of this type can be created from `State` with `object.signals().error_state_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `State` object.
  pub struct ErrorStateChanged<'a>(&'a ::state::State);
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
  /// Represents a built-in Qt signal `QState::entered`.
  ///
  /// An object of this type can be created from `State` with `object.signals().entered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `State` object.
  pub struct Entered<'a>(&'a ::state::State);
  impl<'a> ::connection::Receiver for Entered<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2entered()\0"
    }
  }
  impl<'a> ::connection::Signal for Entered<'a> {}
  /// Represents a built-in Qt signal `QState::finished`.
  ///
  /// An object of this type can be created from `State` with `object.signals().finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `State` object.
  pub struct Finished<'a>(&'a ::state::State);
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
  /// Represents a built-in Qt signal `QState::propertiesAssigned`.
  ///
  /// An object of this type can be created from `State` with `object.signals().properties_assigned()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `State` object.
  pub struct PropertiesAssigned<'a>(&'a ::state::State);
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
  /// Represents a built-in Qt signal `QState::initialStateChanged`.
  ///
  /// An object of this type can be created from `State` with `object.signals().initial_state_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `State` object.
  pub struct InitialStateChanged<'a>(&'a ::state::State);
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
  /// Represents a built-in Qt signal `QState::activeChanged`.
  ///
  /// An object of this type can be created from `State` with `object.signals().active_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `State` object.
  pub struct ActiveChanged<'a>(&'a ::state::State);
  impl<'a> ::connection::Receiver for ActiveChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2activeChanged(bool)\0"
    }
  }
  impl<'a> ::connection::Signal for ActiveChanged<'a> {}
  /// Represents a built-in Qt signal `QState::childModeChanged`.
  ///
  /// An object of this type can be created from `State` with `object.signals().child_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `State` object.
  pub struct ChildModeChanged<'a>(&'a ::state::State);
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
    /// Returns an object representing a built-in Qt signal `QState::exited`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn exited(&self) -> Exited {
      Exited(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QState::errorStateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn error_state_changed(&self) -> ErrorStateChanged {
      ErrorStateChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QState::entered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn entered(&self) -> Entered {
      Entered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QState::finished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn finished(&self) -> Finished {
      Finished(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QState::propertiesAssigned`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn properties_assigned(&self) -> PropertiesAssigned {
      PropertiesAssigned(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QState::initialStateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn initial_state_changed(&self) -> InitialStateChanged {
      InitialStateChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QState::activeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn active_changed(&self) -> ActiveChanged {
      ActiveChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QState::childModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn child_mode_changed(&self) -> ChildModeChanged {
      ChildModeChanged(self.0)
    }
  }
  impl ::state::State {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::state::State> for ::abstract_state::AbstractState {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::state::State> {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QState_G_dynamic_cast_QState_ptr_QAbstractState(self as *mut ::abstract_state::AbstractState)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::state::State> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QState_G_dynamic_cast_QState_ptr_QAbstractState(self as *const ::abstract_state::AbstractState as *mut ::abstract_state::AbstractState) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::state::State> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::state::State> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QState_G_dynamic_cast_QState_ptr_QObject(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::state::State> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QState_G_dynamic_cast_QState_ptr_QObject(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::abstract_state::AbstractState> for ::state::State {
  fn static_cast_mut(&mut self) -> &mut ::abstract_state::AbstractState {
    let ffi_result = unsafe { ::ffi::qt_core_c_QState_G_static_cast_QAbstractState_ptr(self as *mut ::state::State) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_state::AbstractState {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QState_G_static_cast_QAbstractState_ptr(self as *const ::state::State as *mut ::state::State)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::state::State {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QState_G_static_cast_QObject_ptr(self as *mut ::state::State) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QState_G_static_cast_QObject_ptr(self as *const ::state::State as *mut ::state::State)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::state::State> for ::abstract_state::AbstractState {
  unsafe fn static_cast_mut(&mut self) -> &mut ::state::State {
    let ffi_result =
      ::ffi::qt_core_c_QState_G_static_cast_QState_ptr_QAbstractState(self as *mut ::abstract_state::AbstractState);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::state::State {
    let ffi_result = ::ffi::qt_core_c_QState_G_static_cast_QState_ptr_QAbstractState(self as *const ::abstract_state::AbstractState as *mut ::abstract_state::AbstractState);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::state::State> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::state::State {
    let ffi_result = ::ffi::qt_core_c_QState_G_static_cast_QState_ptr_QObject(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::state::State {
    let ffi_result = ::ffi::qt_core_c_QState_G_static_cast_QState_ptr_QObject(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::state::State {
  type Target = ::abstract_state::AbstractState;
  fn deref(&self) -> &::abstract_state::AbstractState {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QState_G_static_cast_QAbstractState_ptr(self as *const ::state::State as *mut ::state::State)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::state::State {
  fn deref_mut(&mut self) -> &mut ::abstract_state::AbstractState {
    let ffi_result = unsafe { ::ffi::qt_core_c_QState_G_static_cast_QAbstractState_ptr(self as *mut ::state::State) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [State::add_transition](../struct.State.html#method.add_transition) method.
  pub trait StateAddTransitionArgs<'largs> {
    type ReturnType;
    unsafe fn exec(self, original_self: &'largs mut ::state::State) -> Self::ReturnType;
  }
  impl<'largs> StateAddTransitionArgs<'largs>
    for (*const ::object::Object, *const ::libc::c_char, *mut ::abstract_state::AbstractState) {
    type ReturnType = *mut ::signal_transition::SignalTransition;
    unsafe fn exec(self, original_self: &'largs mut ::state::State) -> *mut ::signal_transition::SignalTransition {
      let sender = self.0;
      let signal = self.1;
      let target = self.2;
      ::ffi::qt_core_c_QState_addTransition_sender_signal_target(original_self as *mut ::state::State,
                                                                 sender,
                                                                 signal,
                                                                 target)
    }
  }
  impl<'largs> StateAddTransitionArgs<'largs> for *mut ::abstract_state::AbstractState {
    type ReturnType = *mut ::abstract_transition::AbstractTransition;
    unsafe fn exec(self, original_self: &'largs mut ::state::State) -> *mut ::abstract_transition::AbstractTransition {
      let target = self;
      ::ffi::qt_core_c_QState_addTransition_target(original_self as *mut ::state::State, target)
    }
  }
  impl<'largs> StateAddTransitionArgs<'largs> for *mut ::abstract_transition::AbstractTransition {
    type ReturnType = ();
    unsafe fn exec(self, original_self: &'largs mut ::state::State) -> () {
      let transition = self;
      ::ffi::qt_core_c_QState_addTransition_transition(original_self as *mut ::state::State, transition)
    }
  }
  /// This trait represents a set of arguments accepted by [State::new](../struct.State.html#method.new) method.
  pub trait StateNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::state::State>;
  }
  impl StateNewArgs for ::state::ChildMode {
    fn exec(self) -> ::cpp_utils::CppBox<::state::State> {
      let child_mode = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QState_new_childMode(child_mode) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl StateNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::state::State> {

      let ffi_result = unsafe { ::ffi::qt_core_c_QState_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [State::new_unsafe](../struct.State.html#method.new_unsafe) method.
  pub trait StateNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::state::State>;
  }
  impl StateNewUnsafeArgs for (::state::ChildMode, *mut ::state::State) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::state::State> {
      let child_mode = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_core_c_QState_new_childMode_parent(child_mode, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl StateNewUnsafeArgs for *mut ::state::State {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::state::State> {
      let parent = self;
      let ffi_result = ::ffi::qt_core_c_QState_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}

/// C++ type: <span style='color: green;'>```QHistoryState```</span>
#[repr(C)]
pub struct HistoryState(u8);

impl HistoryState {
  /// C++ method: <span style='color: green;'>```QAbstractState* QHistoryState::defaultState() const```</span>
  ///
  ///
  pub fn default_state(&self) -> *mut ::abstract_state::AbstractState {
    unsafe { ::ffi::qt_core_c_QHistoryState_defaultState(self as *const ::history_state::HistoryState) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractTransition* QHistoryState::defaultTransition() const```</span>
  ///
  ///
  pub fn default_transition(&self) -> *mut ::abstract_transition::AbstractTransition {
    unsafe { ::ffi::qt_core_c_QHistoryState_defaultTransition(self as *const ::history_state::HistoryState) }
  }

  /// C++ method: <span style='color: green;'>```QHistoryState::HistoryType QHistoryState::historyType() const```</span>
  ///
  ///
  pub fn history_type(&self) -> ::history_state::HistoryType {
    unsafe { ::ffi::qt_core_c_QHistoryState_historyType(self as *const ::history_state::HistoryState) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QHistoryState::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QHistoryState_metaObject(self as *const ::history_state::HistoryState) }
  }

  /// C++ method: <span style='color: green;'>```QHistoryState::QHistoryState```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::history_state::HistoryState>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QHistoryState::QHistoryState()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::history_state::HistoryType) -> ::cpp_utils::CppBox<::history_state::HistoryState>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QHistoryState::QHistoryState(QHistoryState::HistoryType type)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::history_state::HistoryState>
    where Args: overloading::HistoryStateNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QHistoryState::QHistoryState```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe((::history_state::HistoryType, *mut ::state::State)) -> ::cpp_utils::CppBox<::history_state::HistoryState>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QHistoryState::QHistoryState(QHistoryState::HistoryType type, QState* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::state::State) -> ::cpp_utils::CppBox<::history_state::HistoryState>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QHistoryState::QHistoryState(QState* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::history_state::HistoryState>
    where Args: overloading::HistoryStateNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QHistoryState::setDefaultState(QAbstractState* state)```</span>
  ///
  ///
  pub unsafe fn set_default_state(&mut self, state: *mut ::abstract_state::AbstractState) {
    ::ffi::qt_core_c_QHistoryState_setDefaultState(self as *mut ::history_state::HistoryState, state)
  }

  /// C++ method: <span style='color: green;'>```void QHistoryState::setDefaultTransition(QAbstractTransition* transition)```</span>
  ///
  ///
  pub unsafe fn set_default_transition(&mut self, transition: *mut ::abstract_transition::AbstractTransition) {
    ::ffi::qt_core_c_QHistoryState_setDefaultTransition(self as *mut ::history_state::HistoryState, transition)
  }

  /// C++ method: <span style='color: green;'>```void QHistoryState::setHistoryType(QHistoryState::HistoryType type)```</span>
  ///
  ///
  pub fn set_history_type(&mut self, type_: ::history_state::HistoryType) {
    unsafe { ::ffi::qt_core_c_QHistoryState_setHistoryType(self as *mut ::history_state::HistoryState, type_) }
  }

  /// C++ method: <span style='color: green;'>```static QString QHistoryState::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QHistoryState_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QHistoryState::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QHistoryState_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::history_state::HistoryState {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QHistoryState_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `HistoryState`.
  pub struct Signals<'a>(&'a ::history_state::HistoryState);
  /// Represents a built-in Qt signal `QHistoryState::historyTypeChanged`.
  ///
  /// An object of this type can be created from `HistoryState` with `object.signals().history_type_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HistoryState` object.
  pub struct HistoryTypeChanged<'a>(&'a ::history_state::HistoryState);
  impl<'a> ::connection::Receiver for HistoryTypeChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2historyTypeChanged()\0"
    }
  }
  impl<'a> ::connection::Signal for HistoryTypeChanged<'a> {}
  /// Represents a built-in Qt signal `QHistoryState::defaultStateChanged`.
  ///
  /// An object of this type can be created from `HistoryState` with `object.signals().default_state_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HistoryState` object.
  pub struct DefaultStateChanged<'a>(&'a ::history_state::HistoryState);
  impl<'a> ::connection::Receiver for DefaultStateChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2defaultStateChanged()\0"
    }
  }
  impl<'a> ::connection::Signal for DefaultStateChanged<'a> {}
  /// Represents a built-in Qt signal `QHistoryState::activeChanged`.
  ///
  /// An object of this type can be created from `HistoryState` with `object.signals().active_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HistoryState` object.
  pub struct ActiveChanged<'a>(&'a ::history_state::HistoryState);
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
  /// Represents a built-in Qt signal `QHistoryState::entered`.
  ///
  /// An object of this type can be created from `HistoryState` with `object.signals().entered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HistoryState` object.
  pub struct Entered<'a>(&'a ::history_state::HistoryState);
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
  /// Represents a built-in Qt signal `QHistoryState::exited`.
  ///
  /// An object of this type can be created from `HistoryState` with `object.signals().exited()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HistoryState` object.
  pub struct Exited<'a>(&'a ::history_state::HistoryState);
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
  /// Represents a built-in Qt signal `QHistoryState::defaultTransitionChanged`.
  ///
  /// An object of this type can be created from `HistoryState` with `object.signals().default_transition_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `HistoryState` object.
  pub struct DefaultTransitionChanged<'a>(&'a ::history_state::HistoryState);
  impl<'a> ::connection::Receiver for DefaultTransitionChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2defaultTransitionChanged()\0"
    }
  }
  impl<'a> ::connection::Signal for DefaultTransitionChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QHistoryState::historyTypeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn history_type_changed(&self) -> HistoryTypeChanged {
      HistoryTypeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QHistoryState::defaultStateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn default_state_changed(&self) -> DefaultStateChanged {
      DefaultStateChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QHistoryState::activeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn active_changed(&self) -> ActiveChanged {
      ActiveChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QHistoryState::entered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn entered(&self) -> Entered {
      Entered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QHistoryState::exited`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn exited(&self) -> Exited {
      Exited(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QHistoryState::defaultTransitionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn default_transition_changed(&self) -> DefaultTransitionChanged {
      DefaultTransitionChanged(self.0)
    }
  }
  impl ::history_state::HistoryState {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

/// C++ type: <span style='color: green;'>```QHistoryState::HistoryType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum HistoryType {
  /// C++ enum variant: <span style='color: green;'>```ShallowHistory = 0```</span>
  Shallow = 0,
  /// C++ enum variant: <span style='color: green;'>```DeepHistory = 1```</span>
  Deep = 1,
}

impl ::cpp_utils::DynamicCast<::history_state::HistoryState> for ::abstract_state::AbstractState {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::history_state::HistoryState> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QHistoryState_G_dynamic_cast_QHistoryState_ptr_QAbstractState(self as *mut ::abstract_state::AbstractState) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::history_state::HistoryState> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QHistoryState_G_dynamic_cast_QHistoryState_ptr_QAbstractState(self as *const ::abstract_state::AbstractState as *mut ::abstract_state::AbstractState) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::history_state::HistoryState> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::history_state::HistoryState> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QHistoryState_G_dynamic_cast_QHistoryState_ptr_QObject(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::history_state::HistoryState> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QHistoryState_G_dynamic_cast_QHistoryState_ptr_QObject(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::abstract_state::AbstractState> for ::history_state::HistoryState {
  fn static_cast_mut(&mut self) -> &mut ::abstract_state::AbstractState {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QHistoryState_G_static_cast_QAbstractState_ptr(self as *mut ::history_state::HistoryState)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_state::AbstractState {
    let ffi_result = unsafe { ::ffi::qt_core_c_QHistoryState_G_static_cast_QAbstractState_ptr(self as *const ::history_state::HistoryState as *mut ::history_state::HistoryState) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::history_state::HistoryState {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QHistoryState_G_static_cast_QObject_ptr(self as *mut ::history_state::HistoryState) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QHistoryState_G_static_cast_QObject_ptr(self as *const ::history_state::HistoryState as *mut ::history_state::HistoryState) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::history_state::HistoryState> for ::abstract_state::AbstractState {
  unsafe fn static_cast_mut(&mut self) -> &mut ::history_state::HistoryState {
    let ffi_result = ::ffi::qt_core_c_QHistoryState_G_static_cast_QHistoryState_ptr_QAbstractState(self as *mut ::abstract_state::AbstractState);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::history_state::HistoryState {
    let ffi_result = ::ffi::qt_core_c_QHistoryState_G_static_cast_QHistoryState_ptr_QAbstractState(self as *const ::abstract_state::AbstractState as *mut ::abstract_state::AbstractState);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::history_state::HistoryState> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::history_state::HistoryState {
    let ffi_result =
      ::ffi::qt_core_c_QHistoryState_G_static_cast_QHistoryState_ptr_QObject(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::history_state::HistoryState {
    let ffi_result = ::ffi::qt_core_c_QHistoryState_G_static_cast_QHistoryState_ptr_QObject(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::history_state::HistoryState {
  type Target = ::abstract_state::AbstractState;
  fn deref(&self) -> &::abstract_state::AbstractState {
    let ffi_result = unsafe { ::ffi::qt_core_c_QHistoryState_G_static_cast_QAbstractState_ptr(self as *const ::history_state::HistoryState as *mut ::history_state::HistoryState) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::history_state::HistoryState {
  fn deref_mut(&mut self) -> &mut ::abstract_state::AbstractState {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QHistoryState_G_static_cast_QAbstractState_ptr(self as *mut ::history_state::HistoryState)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [HistoryState::new](../struct.HistoryState.html#method.new) method.
  pub trait HistoryStateNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::history_state::HistoryState>;
  }
  impl HistoryStateNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::history_state::HistoryState> {

      let ffi_result = unsafe { ::ffi::qt_core_c_QHistoryState_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl HistoryStateNewArgs for ::history_state::HistoryType {
    fn exec(self) -> ::cpp_utils::CppBox<::history_state::HistoryState> {
      let type_ = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QHistoryState_new_type(type_) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [HistoryState::new_unsafe](../struct.HistoryState.html#method.new_unsafe) method.
  pub trait HistoryStateNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::history_state::HistoryState>;
  }
  impl HistoryStateNewUnsafeArgs for *mut ::state::State {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::history_state::HistoryState> {
      let parent = self;
      let ffi_result = ::ffi::qt_core_c_QHistoryState_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl HistoryStateNewUnsafeArgs for (::history_state::HistoryType, *mut ::state::State) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::history_state::HistoryState> {
      let type_ = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_core_c_QHistoryState_new_type_parent(type_, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}

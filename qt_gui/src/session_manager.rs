/// C++ type: <span style='color: green;'>```QSessionManager::RestartHint```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum RestartHint {
  /// C++ enum variant: <span style='color: green;'>```RestartIfRunning = 0```</span>
  IfRunning = 0,
  /// C++ enum variant: <span style='color: green;'>```RestartAnyway = 1```</span>
  Anyway = 1,
  /// C++ enum variant: <span style='color: green;'>```RestartImmediately = 2```</span>
  Immediately = 2,
  /// C++ enum variant: <span style='color: green;'>```RestartNever = 3```</span>
  Never = 3,
}

/// C++ type: <span style='color: green;'>```QSessionManager```</span>
#[repr(C)]
pub struct SessionManager(u8);

impl SessionManager {
  /// C++ method: <span style='color: green;'>```bool QSessionManager::allowsErrorInteraction()```</span>
  ///
  ///
  pub fn allows_error_interaction(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QSessionManager_allowsErrorInteraction(self as *mut ::session_manager::SessionManager) }
  }

  /// C++ method: <span style='color: green;'>```bool QSessionManager::allowsInteraction()```</span>
  ///
  ///
  pub fn allows_interaction(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QSessionManager_allowsInteraction(self as *mut ::session_manager::SessionManager) }
  }

  /// C++ method: <span style='color: green;'>```void QSessionManager::cancel()```</span>
  ///
  ///
  pub fn cancel(&mut self) {
    unsafe { ::ffi::qt_gui_c_QSessionManager_cancel(self as *mut ::session_manager::SessionManager) }
  }

  /// C++ method: <span style='color: green;'>```QStringList QSessionManager::discardCommand() const```</span>
  ///
  ///
  pub fn discard_command(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QSessionManager_discardCommand_to_output(self as *const ::session_manager::SessionManager,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QSessionManager::isPhase2() const```</span>
  ///
  ///
  pub fn is_phase2(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QSessionManager_isPhase2(self as *const ::session_manager::SessionManager) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QSessionManager::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QSessionManager_metaObject(self as *const ::session_manager::SessionManager) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QSessionManager::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QSessionManager_qt_metacall(self as *mut ::session_manager::SessionManager,
                                                arg1 as *const ::qt_core::meta_object::Call,
                                                arg2,
                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QSessionManager::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QSessionManager_qt_metacast(self as *mut ::session_manager::SessionManager, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QSessionManager::release()```</span>
  ///
  ///
  pub fn release(&mut self) {
    unsafe { ::ffi::qt_gui_c_QSessionManager_release(self as *mut ::session_manager::SessionManager) }
  }

  /// C++ method: <span style='color: green;'>```void QSessionManager::requestPhase2()```</span>
  ///
  ///
  pub fn request_phase2(&mut self) {
    unsafe { ::ffi::qt_gui_c_QSessionManager_requestPhase2(self as *mut ::session_manager::SessionManager) }
  }

  /// C++ method: <span style='color: green;'>```QStringList QSessionManager::restartCommand() const```</span>
  ///
  ///
  pub fn restart_command(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QSessionManager_restartCommand_to_output(self as *const ::session_manager::SessionManager,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSessionManager::RestartHint QSessionManager::restartHint() const```</span>
  ///
  ///
  pub fn restart_hint(&self) -> ::session_manager::RestartHint {
    unsafe { ::ffi::qt_gui_c_QSessionManager_restartHint(self as *const ::session_manager::SessionManager) }
  }

  /// C++ method: <span style='color: green;'>```QString QSessionManager::sessionId() const```</span>
  ///
  ///
  pub fn session_id(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QSessionManager_sessionId_to_output(self as *const ::session_manager::SessionManager,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QSessionManager::sessionKey() const```</span>
  ///
  ///
  pub fn session_key(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QSessionManager_sessionKey_to_output(self as *const ::session_manager::SessionManager,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QSessionManager::setDiscardCommand(const QStringList& arg1)```</span>
  ///
  ///
  pub fn set_discard_command(&mut self, arg1: &::qt_core::string_list::StringList) {
    unsafe {
      ::ffi::qt_gui_c_QSessionManager_setDiscardCommand(self as *mut ::session_manager::SessionManager,
                                                        arg1 as *const ::qt_core::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```QSessionManager::setManagerProperty```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_manager_property(&mut self, (&::qt_core::string::String, &::qt_core::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSessionManager::setManagerProperty(const QString& name, const QString& value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_manager_property(&mut self, (&::qt_core::string::String, &::qt_core::string_list::StringList)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSessionManager::setManagerProperty(const QString& name, const QStringList& value)```</span>
  ///
  ///
  pub fn set_manager_property<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::SessionManagerSetManagerPropertyArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QSessionManager::setRestartCommand(const QStringList& arg1)```</span>
  ///
  ///
  pub fn set_restart_command(&mut self, arg1: &::qt_core::string_list::StringList) {
    unsafe {
      ::ffi::qt_gui_c_QSessionManager_setRestartCommand(self as *mut ::session_manager::SessionManager,
                                                        arg1 as *const ::qt_core::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```void QSessionManager::setRestartHint(QSessionManager::RestartHint arg1)```</span>
  ///
  ///
  pub fn set_restart_hint(&mut self, arg1: ::session_manager::RestartHint) {
    unsafe { ::ffi::qt_gui_c_QSessionManager_setRestartHint(self as *mut ::session_manager::SessionManager, arg1) }
  }

  /// C++ method: <span style='color: green;'>```static QString QSessionManager::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QSessionManager_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSessionManager::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QSessionManager_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `SessionManager`.
  pub struct Signals<'a>(&'a ::session_manager::SessionManager);
  /// Represents a built-in Qt signal `QSessionManager::objectNameChanged`.
  ///
  /// An object of this type can be created from `SessionManager` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SessionManager` object.
  pub struct ObjectNameChanged<'a>(&'a ::session_manager::SessionManager);
  impl<'a> ::qt_core::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ObjectNameChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QSessionManager::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::session_manager::SessionManager {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::session_manager::SessionManager {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QSessionManager_G_static_cast_QObject_ptr(self as *mut ::session_manager::SessionManager)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QSessionManager_G_static_cast_QObject_ptr(self as *const ::session_manager::SessionManager as *mut ::session_manager::SessionManager) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::session_manager::SessionManager> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::session_manager::SessionManager {
    let ffi_result =
      ::ffi::qt_gui_c_QSessionManager_G_static_cast_QSessionManager_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::session_manager::SessionManager {
    let ffi_result = ::ffi::qt_gui_c_QSessionManager_G_static_cast_QSessionManager_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::session_manager::SessionManager {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QSessionManager_G_static_cast_QObject_ptr(self as *const ::session_manager::SessionManager as *mut ::session_manager::SessionManager) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::session_manager::SessionManager {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QSessionManager_G_static_cast_QObject_ptr(self as *mut ::session_manager::SessionManager)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [SessionManager::set_manager_property](../struct.SessionManager.html#method.set_manager_property) method.
  pub trait SessionManagerSetManagerPropertyArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::session_manager::SessionManager) -> ();
  }
  impl<'largs> SessionManagerSetManagerPropertyArgs<'largs>
    for (&'largs ::qt_core::string::String, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::session_manager::SessionManager) -> () {
      let name = self.0;
      let value = self.1;
      unsafe { ::ffi::qt_gui_c_QSessionManager_setManagerProperty_QString_QString(original_self as *mut ::session_manager::SessionManager, name as *const ::qt_core::string::String, value as *const ::qt_core::string::String) }
    }
  }
  impl<'largs> SessionManagerSetManagerPropertyArgs<'largs>
    for (&'largs ::qt_core::string::String, &'largs ::qt_core::string_list::StringList) {
    fn exec(self, original_self: &'largs mut ::session_manager::SessionManager) -> () {
      let name = self.0;
      let value = self.1;
      unsafe { ::ffi::qt_gui_c_QSessionManager_setManagerProperty_QString_QStringList(original_self as *mut ::session_manager::SessionManager, name as *const ::qt_core::string::String, value as *const ::qt_core::string_list::StringList) }
    }
  }
}

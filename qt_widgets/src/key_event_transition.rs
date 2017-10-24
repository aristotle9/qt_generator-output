/// C++ type: <span style='color: green;'>```QKeyEventTransition```</span>
#[repr(C)]
pub struct KeyEventTransition(u8);

impl KeyEventTransition {
  /// C++ method: <span style='color: green;'>```int QKeyEventTransition::key() const```</span>
  ///
  ///
  pub fn key(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QKeyEventTransition_key(self as *const ::key_event_transition::KeyEventTransition) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QKeyEventTransition::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_widgets_c_QKeyEventTransition_metaObject(self as *const ::key_event_transition::KeyEventTransition)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QKeyEventTransition::QKeyEventTransition()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::key_event_transition::KeyEventTransition> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QKeyEventTransition_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QKeyEventTransition::QKeyEventTransition```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::qt_core::object::Object, &::qt_core::event::Type, ::libc::c_int)) -> ::cpp_utils::CppBox<::key_event_transition::KeyEventTransition>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QKeyEventTransition::QKeyEventTransition(QObject* object, QEvent::Type type, int key)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::qt_core::object::Object, &::qt_core::event::Type, ::libc::c_int, *mut ::qt_core::state::State)) -> ::cpp_utils::CppBox<::key_event_transition::KeyEventTransition>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QKeyEventTransition::QKeyEventTransition(QObject* object, QEvent::Type type, int key, QState* sourceState = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::qt_core::state::State) -> ::cpp_utils::CppBox<::key_event_transition::KeyEventTransition>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QKeyEventTransition::QKeyEventTransition(QState* sourceState = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::key_event_transition::KeyEventTransition>
    where Args: overloading::KeyEventTransitionNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int QKeyEventTransition::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QKeyEventTransition_qt_metacall(self as *mut ::key_event_transition::KeyEventTransition,
                                                        arg1 as *const ::qt_core::meta_object::Call,
                                                        arg2,
                                                        arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QKeyEventTransition::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QKeyEventTransition_qt_metacast(self as *mut ::key_event_transition::KeyEventTransition,
                                                        arg1)
  }

  /// C++ method: <span style='color: green;'>```void QKeyEventTransition::setKey(int key)```</span>
  ///
  ///
  pub fn set_key(&mut self, key: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QKeyEventTransition_setKey(self as *mut ::key_event_transition::KeyEventTransition, key)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QKeyEventTransition::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QKeyEventTransition_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QKeyEventTransition::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QKeyEventTransition_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::key_event_transition::KeyEventTransition {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QKeyEventTransition_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::abstract_transition::AbstractTransition> for ::key_event_transition::KeyEventTransition {
fn static_cast_mut(&mut self) -> &mut ::qt_core::abstract_transition::AbstractTransition {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QKeyEventTransition_G_static_cast_QAbstractTransition_ptr(self as *mut ::key_event_transition::KeyEventTransition) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::abstract_transition::AbstractTransition {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QKeyEventTransition_G_static_cast_QAbstractTransition_ptr(self as *const ::key_event_transition::KeyEventTransition as *mut ::key_event_transition::KeyEventTransition) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_core::event_transition::EventTransition> for ::key_event_transition::KeyEventTransition {
fn static_cast_mut(&mut self) -> &mut ::qt_core::event_transition::EventTransition {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QKeyEventTransition_G_static_cast_QEventTransition_ptr(self as *mut ::key_event_transition::KeyEventTransition) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::event_transition::EventTransition {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QKeyEventTransition_G_static_cast_QEventTransition_ptr(self as *const ::key_event_transition::KeyEventTransition as *mut ::key_event_transition::KeyEventTransition) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::key_event_transition::KeyEventTransition {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QKeyEventTransition_G_static_cast_QObject_ptr(self as *mut ::key_event_transition::KeyEventTransition) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QKeyEventTransition_G_static_cast_QObject_ptr(self as *const ::key_event_transition::KeyEventTransition as *mut ::key_event_transition::KeyEventTransition) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::key_event_transition::KeyEventTransition> for ::qt_core::abstract_transition::AbstractTransition {
unsafe fn static_cast_mut(&mut self) -> &mut ::key_event_transition::KeyEventTransition {
let ffi_result = ::ffi::qt_widgets_c_QKeyEventTransition_G_static_cast_QKeyEventTransition_ptr_QAbstractTransition(self as *mut ::qt_core::abstract_transition::AbstractTransition);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::key_event_transition::KeyEventTransition {
let ffi_result = ::ffi::qt_widgets_c_QKeyEventTransition_G_static_cast_QKeyEventTransition_ptr_QAbstractTransition(self as *const ::qt_core::abstract_transition::AbstractTransition as *mut ::qt_core::abstract_transition::AbstractTransition);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::key_event_transition::KeyEventTransition> for ::qt_core::event_transition::EventTransition {
unsafe fn static_cast_mut(&mut self) -> &mut ::key_event_transition::KeyEventTransition {
let ffi_result = ::ffi::qt_widgets_c_QKeyEventTransition_G_static_cast_QKeyEventTransition_ptr_QEventTransition(self as *mut ::qt_core::event_transition::EventTransition);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::key_event_transition::KeyEventTransition {
let ffi_result = ::ffi::qt_widgets_c_QKeyEventTransition_G_static_cast_QKeyEventTransition_ptr_QEventTransition(self as *const ::qt_core::event_transition::EventTransition as *mut ::qt_core::event_transition::EventTransition);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::key_event_transition::KeyEventTransition> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::key_event_transition::KeyEventTransition {
    let ffi_result = ::ffi::qt_widgets_c_QKeyEventTransition_G_static_cast_QKeyEventTransition_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::key_event_transition::KeyEventTransition {
    let ffi_result = ::ffi::qt_widgets_c_QKeyEventTransition_G_static_cast_QKeyEventTransition_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::key_event_transition::KeyEventTransition {
  type Target = ::qt_core::event_transition::EventTransition;
  fn deref(&self) -> &::qt_core::event_transition::EventTransition {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QKeyEventTransition_G_static_cast_QEventTransition_ptr(self as *const ::key_event_transition::KeyEventTransition as *mut ::key_event_transition::KeyEventTransition) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::key_event_transition::KeyEventTransition {
  fn deref_mut(&mut self) -> &mut ::qt_core::event_transition::EventTransition {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QKeyEventTransition_G_static_cast_QEventTransition_ptr(self as *mut ::key_event_transition::KeyEventTransition) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [KeyEventTransition::new_unsafe](../struct.KeyEventTransition.html#method.new_unsafe) method.
  pub trait KeyEventTransitionNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::key_event_transition::KeyEventTransition>;
  }
  impl<'a> KeyEventTransitionNewUnsafeArgs
    for (*mut ::qt_core::object::Object, &'a ::qt_core::event::Type, ::libc::c_int) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::key_event_transition::KeyEventTransition> {
      let object = self.0;
      let type_ = self.1;
      let key = self.2;
      let ffi_result =
        ::ffi::qt_widgets_c_QKeyEventTransition_new_object_type_key(object,
                                                                    type_ as *const ::qt_core::event::Type,
                                                                    key);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> KeyEventTransitionNewUnsafeArgs
    for (*mut ::qt_core::object::Object, &'a ::qt_core::event::Type, ::libc::c_int, *mut ::qt_core::state::State) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::key_event_transition::KeyEventTransition> {
      let object = self.0;
      let type_ = self.1;
      let key = self.2;
      let source_state = self.3;
      let ffi_result = ::ffi::qt_widgets_c_QKeyEventTransition_new_object_type_key_sourceState(object, type_ as *const ::qt_core::event::Type, key, source_state);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl KeyEventTransitionNewUnsafeArgs for *mut ::qt_core::state::State {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::key_event_transition::KeyEventTransition> {
      let source_state = self;
      let ffi_result = ::ffi::qt_widgets_c_QKeyEventTransition_new_sourceState(source_state);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}

/// C++ type: <span style='color: green;'>```QMouseEventTransition```</span>
#[repr(C)]
pub struct MouseEventTransition(u8);

impl MouseEventTransition {
  /// C++ method: <span style='color: green;'>```QPainterPath QMouseEventTransition::hitTestPath() const```</span>
  ///
  ///
  pub fn hit_test_path(&self) -> ::qt_gui::painter_path::PainterPath {
    {
      let mut object: ::qt_gui::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QMouseEventTransition_hitTestPath_to_output(self as *const ::mouse_event_transition::MouseEventTransition, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QMouseEventTransition::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QMouseEventTransition_metaObject(self as *const ::mouse_event_transition::MouseEventTransition) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QMouseEventTransition::QMouseEventTransition()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::mouse_event_transition::MouseEventTransition> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMouseEventTransition_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QMouseEventTransition::QMouseEventTransition```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::qt_core::object::Object, &::qt_core::event::Type, &::qt_core::qt::MouseButton)) -> ::cpp_utils::CppBox<::mouse_event_transition::MouseEventTransition>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMouseEventTransition::QMouseEventTransition(QObject* object, QEvent::Type type, Qt::MouseButton button)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::qt_core::object::Object, &::qt_core::event::Type, &::qt_core::qt::MouseButton, *mut ::qt_core::state::State)) -> ::cpp_utils::CppBox<::mouse_event_transition::MouseEventTransition>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMouseEventTransition::QMouseEventTransition(QObject* object, QEvent::Type type, Qt::MouseButton button, QState* sourceState = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::qt_core::state::State) -> ::cpp_utils::CppBox<::mouse_event_transition::MouseEventTransition>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMouseEventTransition::QMouseEventTransition(QState* sourceState = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::mouse_event_transition::MouseEventTransition>
    where Args: overloading::MouseEventTransitionNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int QMouseEventTransition::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QMouseEventTransition_qt_metacall(self as *mut ::mouse_event_transition::MouseEventTransition,
                                                          arg1 as *const ::qt_core::meta_object::Call,
                                                          arg2,
                                                          arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QMouseEventTransition::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QMouseEventTransition_qt_metacast(self as *mut ::mouse_event_transition::MouseEventTransition,
                                                          arg1)
  }

  /// C++ method: <span style='color: green;'>```void QMouseEventTransition::setButton(Qt::MouseButton button)```</span>
  ///
  ///
  pub fn set_button(&mut self, button: &::qt_core::qt::MouseButton) {
    unsafe {
      ::ffi::qt_widgets_c_QMouseEventTransition_setButton(self as *mut ::mouse_event_transition::MouseEventTransition,
                                                          button as *const ::qt_core::qt::MouseButton)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMouseEventTransition::setHitTestPath(const QPainterPath& path)```</span>
  ///
  ///
  pub fn set_hit_test_path(&mut self, path: &::qt_gui::painter_path::PainterPath) {
    unsafe { ::ffi::qt_widgets_c_QMouseEventTransition_setHitTestPath(self as *mut ::mouse_event_transition::MouseEventTransition, path as *const ::qt_gui::painter_path::PainterPath) }
  }

  /// C++ method: <span style='color: green;'>```static QString QMouseEventTransition::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QMouseEventTransition_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QMouseEventTransition::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QMouseEventTransition_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::mouse_event_transition::MouseEventTransition {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QMouseEventTransition_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::abstract_transition::AbstractTransition> for ::mouse_event_transition::MouseEventTransition {
fn static_cast_mut(&mut self) -> &mut ::qt_core::abstract_transition::AbstractTransition {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QMouseEventTransition_G_static_cast_QAbstractTransition_ptr(self as *mut ::mouse_event_transition::MouseEventTransition) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::abstract_transition::AbstractTransition {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QMouseEventTransition_G_static_cast_QAbstractTransition_ptr(self as *const ::mouse_event_transition::MouseEventTransition as *mut ::mouse_event_transition::MouseEventTransition) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_core::event_transition::EventTransition> for ::mouse_event_transition::MouseEventTransition {
fn static_cast_mut(&mut self) -> &mut ::qt_core::event_transition::EventTransition {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QMouseEventTransition_G_static_cast_QEventTransition_ptr(self as *mut ::mouse_event_transition::MouseEventTransition) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::event_transition::EventTransition {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QMouseEventTransition_G_static_cast_QEventTransition_ptr(self as *const ::mouse_event_transition::MouseEventTransition as *mut ::mouse_event_transition::MouseEventTransition) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::mouse_event_transition::MouseEventTransition {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMouseEventTransition_G_static_cast_QObject_ptr(self as *mut ::mouse_event_transition::MouseEventTransition) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMouseEventTransition_G_static_cast_QObject_ptr(self as *const ::mouse_event_transition::MouseEventTransition as *mut ::mouse_event_transition::MouseEventTransition) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::mouse_event_transition::MouseEventTransition> for ::qt_core::abstract_transition::AbstractTransition {
unsafe fn static_cast_mut(&mut self) -> &mut ::mouse_event_transition::MouseEventTransition {
let ffi_result = ::ffi::qt_widgets_c_QMouseEventTransition_G_static_cast_QMouseEventTransition_ptr_QAbstractTransition(self as *mut ::qt_core::abstract_transition::AbstractTransition);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::mouse_event_transition::MouseEventTransition {
let ffi_result = ::ffi::qt_widgets_c_QMouseEventTransition_G_static_cast_QMouseEventTransition_ptr_QAbstractTransition(self as *const ::qt_core::abstract_transition::AbstractTransition as *mut ::qt_core::abstract_transition::AbstractTransition);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::mouse_event_transition::MouseEventTransition> for ::qt_core::event_transition::EventTransition {
unsafe fn static_cast_mut(&mut self) -> &mut ::mouse_event_transition::MouseEventTransition {
let ffi_result = ::ffi::qt_widgets_c_QMouseEventTransition_G_static_cast_QMouseEventTransition_ptr_QEventTransition(self as *mut ::qt_core::event_transition::EventTransition);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::mouse_event_transition::MouseEventTransition {
let ffi_result = ::ffi::qt_widgets_c_QMouseEventTransition_G_static_cast_QMouseEventTransition_ptr_QEventTransition(self as *const ::qt_core::event_transition::EventTransition as *mut ::qt_core::event_transition::EventTransition);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::mouse_event_transition::MouseEventTransition> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::mouse_event_transition::MouseEventTransition {
    let ffi_result = ::ffi::qt_widgets_c_QMouseEventTransition_G_static_cast_QMouseEventTransition_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::mouse_event_transition::MouseEventTransition {
    let ffi_result = ::ffi::qt_widgets_c_QMouseEventTransition_G_static_cast_QMouseEventTransition_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::mouse_event_transition::MouseEventTransition {
  type Target = ::qt_core::event_transition::EventTransition;
  fn deref(&self) -> &::qt_core::event_transition::EventTransition {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMouseEventTransition_G_static_cast_QEventTransition_ptr(self as *const ::mouse_event_transition::MouseEventTransition as *mut ::mouse_event_transition::MouseEventTransition) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::mouse_event_transition::MouseEventTransition {
  fn deref_mut(&mut self) -> &mut ::qt_core::event_transition::EventTransition {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMouseEventTransition_G_static_cast_QEventTransition_ptr(self as *mut ::mouse_event_transition::MouseEventTransition) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [MouseEventTransition::new_unsafe](../struct.MouseEventTransition.html#method.new_unsafe) method.
  pub trait MouseEventTransitionNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::mouse_event_transition::MouseEventTransition>;
  }
  impl<'a> MouseEventTransitionNewUnsafeArgs
    for (*mut ::qt_core::object::Object, &'a ::qt_core::event::Type, &'a ::qt_core::qt::MouseButton) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::mouse_event_transition::MouseEventTransition> {
      let object = self.0;
      let type_ = self.1;
      let button = self.2;
      let ffi_result =
        ::ffi::qt_widgets_c_QMouseEventTransition_new_object_type_button(object,
                                                                         type_ as *const ::qt_core::event::Type,
                                                                         button as *const ::qt_core::qt::MouseButton);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> MouseEventTransitionNewUnsafeArgs
    for (*mut ::qt_core::object::Object,
                                                      &'a ::qt_core::event::Type,
                                                      &'a ::qt_core::qt::MouseButton,
                                                      *mut ::qt_core::state::State) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::mouse_event_transition::MouseEventTransition> {
      let object = self.0;
      let type_ = self.1;
      let button = self.2;
      let source_state = self.3;
      let ffi_result = ::ffi::qt_widgets_c_QMouseEventTransition_new_object_type_button_sourceState(object, type_ as *const ::qt_core::event::Type, button as *const ::qt_core::qt::MouseButton, source_state);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl MouseEventTransitionNewUnsafeArgs for *mut ::qt_core::state::State {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::mouse_event_transition::MouseEventTransition> {
      let source_state = self;
      let ffi_result = ::ffi::qt_widgets_c_QMouseEventTransition_new_sourceState(source_state);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}

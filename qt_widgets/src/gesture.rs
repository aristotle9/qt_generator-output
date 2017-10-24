/// C++ type: <span style='color: green;'>```QGesture```</span>
#[repr(C)]
pub struct Gesture(u8);

impl Gesture {
  /// C++ method: <span style='color: green;'>```QGesture::GestureCancelPolicy QGesture::gestureCancelPolicy() const```</span>
  ///
  ///
  pub fn gesture_cancel_policy(&self) -> ::gesture::GestureCancelPolicy {
    unsafe { ::ffi::qt_widgets_c_QGesture_gestureCancelPolicy(self as *const ::gesture::Gesture) }
  }

  /// C++ method: <span style='color: green;'>```bool QGesture::hasHotSpot() const```</span>
  ///
  ///
  pub fn has_hot_spot(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGesture_hasHotSpot(self as *const ::gesture::Gesture) }
  }

  /// C++ method: <span style='color: green;'>```QPointF QGesture::hotSpot() const```</span>
  ///
  ///
  pub fn hot_spot(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGesture_hotSpot_to_output(self as *const ::gesture::Gesture, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QGesture::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QGesture_metaObject(self as *const ::gesture::Gesture) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGesture::QGesture()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::gesture::Gesture> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGesture_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGesture::QGesture(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object) -> ::cpp_utils::CppBox<::gesture::Gesture> {
    let ffi_result = ::ffi::qt_widgets_c_QGesture_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QGesture::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QGesture_qt_metacall(self as *mut ::gesture::Gesture,
                                             arg1 as *const ::qt_core::meta_object::Call,
                                             arg2,
                                             arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QGesture::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QGesture_qt_metacast(self as *mut ::gesture::Gesture, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QGesture::setGestureCancelPolicy(QGesture::GestureCancelPolicy policy)```</span>
  ///
  ///
  pub fn set_gesture_cancel_policy(&mut self, policy: ::gesture::GestureCancelPolicy) {
    unsafe { ::ffi::qt_widgets_c_QGesture_setGestureCancelPolicy(self as *mut ::gesture::Gesture, policy) }
  }

  /// C++ method: <span style='color: green;'>```void QGesture::setHotSpot(const QPointF& value)```</span>
  ///
  ///
  pub fn set_hot_spot(&mut self, value: &::qt_core::point_f::PointF) {
    unsafe {
      ::ffi::qt_widgets_c_QGesture_setHotSpot(self as *mut ::gesture::Gesture,
                                              value as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGesture::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGesture_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGesture::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGesture_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QGesture::unsetHotSpot()```</span>
  ///
  ///
  pub fn unset_hot_spot(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGesture_unsetHotSpot(self as *mut ::gesture::Gesture) }
  }
}

impl ::cpp_utils::CppDeletable for ::gesture::Gesture {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGesture_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Gesture`.
  pub struct Signals<'a>(&'a ::gesture::Gesture);
  /// Represents a built-in Qt signal `QGesture::objectNameChanged`.
  ///
  /// An object of this type can be created from `Gesture` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Gesture` object.
  pub struct ObjectNameChanged<'a>(&'a ::gesture::Gesture);
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
    /// Returns an object representing a built-in Qt signal `QGesture::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::gesture::Gesture {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

/// C++ type: <span style='color: green;'>```QGesture::GestureCancelPolicy```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum GestureCancelPolicy {
  /// C++ enum variant: <span style='color: green;'>```CancelNone = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```CancelAllInContext = 1```</span>
  AllInContext = 1,
}

/// C++ method: <span style='color: green;'>```operator<<```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shl((&::qt_core::debug::Debug, *const ::gesture::Gesture)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QGesture* arg2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&::qt_core::debug::Debug, *const ::gesture_event::GestureEvent)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QGestureEvent* arg2)```</span>
///
///
pub unsafe fn op_shl<Args>(args: Args) -> ::qt_core::debug::Debug
  where Args: overloading::OpShlArgs
{
  args.exec()
}
impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::gesture::Gesture {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGesture_G_static_cast_QObject_ptr(self as *mut ::gesture::Gesture) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGesture_G_static_cast_QObject_ptr(self as *const ::gesture::Gesture as *mut ::gesture::Gesture) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::gesture::Gesture> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::gesture::Gesture {
    let ffi_result = ::ffi::qt_widgets_c_QGesture_G_static_cast_QGesture_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::gesture::Gesture {
    let ffi_result = ::ffi::qt_widgets_c_QGesture_G_static_cast_QGesture_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::gesture::Gesture {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGesture_G_static_cast_QObject_ptr(self as *const ::gesture::Gesture as *mut ::gesture::Gesture) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::gesture::Gesture {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGesture_G_static_cast_QObject_ptr(self as *mut ::gesture::Gesture) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [op_shl](../fn.op_shl.html) method.
  pub trait OpShlArgs {
    unsafe fn exec(self) -> ::qt_core::debug::Debug;
  }
  impl<'a> OpShlArgs for (&'a ::qt_core::debug::Debug, *const ::gesture::Gesture) {
    unsafe fn exec(self) -> ::qt_core::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QGesture_G_operator_shl_to_output_QDebug_QGesture(arg1 as *const ::qt_core::debug::Debug,
                                                                              arg2,
                                                                              &mut object);
        object
      }
    }
  }
  impl<'a> OpShlArgs for (&'a ::qt_core::debug::Debug, *const ::gesture_event::GestureEvent) {
    unsafe fn exec(self) -> ::qt_core::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QGesture_G_operator_shl_to_output_QDebug_QGestureEvent(arg1 as *const ::qt_core::debug::Debug, arg2, &mut object);
        object
      }
    }
  }
}

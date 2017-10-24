/// C++ type: <span style='color: green;'>```QSequentialAnimationGroup```</span>
#[repr(C)]
pub struct SequentialAnimationGroup(u8);

impl SequentialAnimationGroup {
  /// C++ method: <span style='color: green;'>```QPauseAnimation* QSequentialAnimationGroup::addPause(int msecs)```</span>
  ///
  ///
  pub fn add_pause(&mut self, msecs: ::libc::c_int) -> *mut ::pause_animation::PauseAnimation {
    unsafe { ::ffi::qt_core_c_QSequentialAnimationGroup_addPause(self as *mut ::sequential_animation_group::SequentialAnimationGroup, msecs) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractAnimation* QSequentialAnimationGroup::currentAnimation() const```</span>
  ///
  ///
  pub fn current_animation(&self) -> *mut ::abstract_animation::AbstractAnimation {
    unsafe { ::ffi::qt_core_c_QSequentialAnimationGroup_currentAnimation(self as *const ::sequential_animation_group::SequentialAnimationGroup) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QSequentialAnimationGroup::duration() const```</span>
  ///
  ///
  pub fn duration(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QSequentialAnimationGroup_duration(self as *const ::sequential_animation_group::SequentialAnimationGroup) }
  }

  /// C++ method: <span style='color: green;'>```QPauseAnimation* QSequentialAnimationGroup::insertPause(int index, int msecs)```</span>
  ///
  ///
  pub fn insert_pause(&mut self, index: ::libc::c_int, msecs: ::libc::c_int) -> *mut ::pause_animation::PauseAnimation {
    unsafe { ::ffi::qt_core_c_QSequentialAnimationGroup_insertPause(self as *mut ::sequential_animation_group::SequentialAnimationGroup, index, msecs) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QSequentialAnimationGroup::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QSequentialAnimationGroup_metaObject(self as *const ::sequential_animation_group::SequentialAnimationGroup) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QSequentialAnimationGroup::QSequentialAnimationGroup()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::sequential_animation_group::SequentialAnimationGroup> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSequentialAnimationGroup_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QSequentialAnimationGroup::QSequentialAnimationGroup(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::object::Object)
                           -> ::cpp_utils::CppBox<::sequential_animation_group::SequentialAnimationGroup> {
    let ffi_result = ::ffi::qt_core_c_QSequentialAnimationGroup_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```static QString QSequentialAnimationGroup::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QSequentialAnimationGroup_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSequentialAnimationGroup::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QSequentialAnimationGroup_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::sequential_animation_group::SequentialAnimationGroup {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QSequentialAnimationGroup_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `SequentialAnimationGroup`.
  pub struct Signals<'a>(&'a ::sequential_animation_group::SequentialAnimationGroup);
  /// Represents a built-in Qt signal `QSequentialAnimationGroup::currentAnimationChanged`.
  ///
  /// An object of this type can be created from `SequentialAnimationGroup` with `object.signals().current_animation_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SequentialAnimationGroup` object.
  pub struct CurrentAnimationChanged<'a>(&'a ::sequential_animation_group::SequentialAnimationGroup);
  impl<'a> ::connection::Receiver for CurrentAnimationChanged<'a> {
    type Arguments = (*mut ::abstract_animation::AbstractAnimation,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentAnimationChanged(QAbstractAnimation*)\0"
    }
  }
  impl<'a> ::connection::Signal for CurrentAnimationChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QSequentialAnimationGroup::currentAnimationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_animation_changed(&self) -> CurrentAnimationChanged {
      CurrentAnimationChanged(self.0)
    }
  }
  impl ::sequential_animation_group::SequentialAnimationGroup {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::sequential_animation_group::SequentialAnimationGroup> for ::abstract_animation::AbstractAnimation {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::sequential_animation_group::SequentialAnimationGroup> {
let ffi_result = unsafe { ::ffi::qt_core_c_QSequentialAnimationGroup_G_dynamic_cast_QSequentialAnimationGroup_ptr_QAbstractAnimation(self as *mut ::abstract_animation::AbstractAnimation) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::sequential_animation_group::SequentialAnimationGroup> {
let ffi_result = unsafe { ::ffi::qt_core_c_QSequentialAnimationGroup_G_dynamic_cast_QSequentialAnimationGroup_ptr_QAbstractAnimation(self as *const ::abstract_animation::AbstractAnimation as *mut ::abstract_animation::AbstractAnimation) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::sequential_animation_group::SequentialAnimationGroup> for ::animation_group::AnimationGroup {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::sequential_animation_group::SequentialAnimationGroup> {
let ffi_result = unsafe { ::ffi::qt_core_c_QSequentialAnimationGroup_G_dynamic_cast_QSequentialAnimationGroup_ptr_QAnimationGroup(self as *mut ::animation_group::AnimationGroup) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::sequential_animation_group::SequentialAnimationGroup> {
let ffi_result = unsafe { ::ffi::qt_core_c_QSequentialAnimationGroup_G_dynamic_cast_QSequentialAnimationGroup_ptr_QAnimationGroup(self as *const ::animation_group::AnimationGroup as *mut ::animation_group::AnimationGroup) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::sequential_animation_group::SequentialAnimationGroup> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::sequential_animation_group::SequentialAnimationGroup> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSequentialAnimationGroup_G_dynamic_cast_QSequentialAnimationGroup_ptr_QObject(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::sequential_animation_group::SequentialAnimationGroup> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSequentialAnimationGroup_G_dynamic_cast_QSequentialAnimationGroup_ptr_QObject(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::abstract_animation::AbstractAnimation> for ::sequential_animation_group::SequentialAnimationGroup {
fn static_cast_mut(&mut self) -> &mut ::abstract_animation::AbstractAnimation {
let ffi_result = unsafe { ::ffi::qt_core_c_QSequentialAnimationGroup_G_static_cast_QAbstractAnimation_ptr(self as *mut ::sequential_animation_group::SequentialAnimationGroup) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::abstract_animation::AbstractAnimation {
let ffi_result = unsafe { ::ffi::qt_core_c_QSequentialAnimationGroup_G_static_cast_QAbstractAnimation_ptr(self as *const ::sequential_animation_group::SequentialAnimationGroup as *mut ::sequential_animation_group::SequentialAnimationGroup) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::animation_group::AnimationGroup> for ::sequential_animation_group::SequentialAnimationGroup {
fn static_cast_mut(&mut self) -> &mut ::animation_group::AnimationGroup {
let ffi_result = unsafe { ::ffi::qt_core_c_QSequentialAnimationGroup_G_static_cast_QAnimationGroup_ptr(self as *mut ::sequential_animation_group::SequentialAnimationGroup) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::animation_group::AnimationGroup {
let ffi_result = unsafe { ::ffi::qt_core_c_QSequentialAnimationGroup_G_static_cast_QAnimationGroup_ptr(self as *const ::sequential_animation_group::SequentialAnimationGroup as *mut ::sequential_animation_group::SequentialAnimationGroup) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::object::Object> for ::sequential_animation_group::SequentialAnimationGroup {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSequentialAnimationGroup_G_static_cast_QObject_ptr(self as *mut ::sequential_animation_group::SequentialAnimationGroup) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSequentialAnimationGroup_G_static_cast_QObject_ptr(self as *const ::sequential_animation_group::SequentialAnimationGroup as *mut ::sequential_animation_group::SequentialAnimationGroup) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::sequential_animation_group::SequentialAnimationGroup> for ::abstract_animation::AbstractAnimation {
unsafe fn static_cast_mut(&mut self) -> &mut ::sequential_animation_group::SequentialAnimationGroup {
let ffi_result = ::ffi::qt_core_c_QSequentialAnimationGroup_G_static_cast_QSequentialAnimationGroup_ptr_QAbstractAnimation(self as *mut ::abstract_animation::AbstractAnimation);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::sequential_animation_group::SequentialAnimationGroup {
let ffi_result = ::ffi::qt_core_c_QSequentialAnimationGroup_G_static_cast_QSequentialAnimationGroup_ptr_QAbstractAnimation(self as *const ::abstract_animation::AbstractAnimation as *mut ::abstract_animation::AbstractAnimation);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::sequential_animation_group::SequentialAnimationGroup> for ::animation_group::AnimationGroup {
unsafe fn static_cast_mut(&mut self) -> &mut ::sequential_animation_group::SequentialAnimationGroup {
let ffi_result = ::ffi::qt_core_c_QSequentialAnimationGroup_G_static_cast_QSequentialAnimationGroup_ptr_QAnimationGroup(self as *mut ::animation_group::AnimationGroup);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::sequential_animation_group::SequentialAnimationGroup {
let ffi_result = ::ffi::qt_core_c_QSequentialAnimationGroup_G_static_cast_QSequentialAnimationGroup_ptr_QAnimationGroup(self as *const ::animation_group::AnimationGroup as *mut ::animation_group::AnimationGroup);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::sequential_animation_group::SequentialAnimationGroup> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::sequential_animation_group::SequentialAnimationGroup {
    let ffi_result = ::ffi::qt_core_c_QSequentialAnimationGroup_G_static_cast_QSequentialAnimationGroup_ptr_QObject(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::sequential_animation_group::SequentialAnimationGroup {
    let ffi_result = ::ffi::qt_core_c_QSequentialAnimationGroup_G_static_cast_QSequentialAnimationGroup_ptr_QObject(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::sequential_animation_group::SequentialAnimationGroup {
  type Target = ::animation_group::AnimationGroup;
  fn deref(&self) -> &::animation_group::AnimationGroup {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSequentialAnimationGroup_G_static_cast_QAnimationGroup_ptr(self as *const ::sequential_animation_group::SequentialAnimationGroup as *mut ::sequential_animation_group::SequentialAnimationGroup) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::sequential_animation_group::SequentialAnimationGroup {
  fn deref_mut(&mut self) -> &mut ::animation_group::AnimationGroup {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSequentialAnimationGroup_G_static_cast_QAnimationGroup_ptr(self as *mut ::sequential_animation_group::SequentialAnimationGroup) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

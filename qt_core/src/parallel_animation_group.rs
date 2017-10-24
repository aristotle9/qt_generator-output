/// C++ type: <span style='color: green;'>```QParallelAnimationGroup```</span>
#[repr(C)]
pub struct ParallelAnimationGroup(u8);

impl ParallelAnimationGroup {
  /// C++ method: <span style='color: green;'>```virtual int QParallelAnimationGroup::duration() const```</span>
  ///
  ///
  pub fn duration(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QParallelAnimationGroup_duration(self as *const ::parallel_animation_group::ParallelAnimationGroup) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QParallelAnimationGroup::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QParallelAnimationGroup_metaObject(self as *const ::parallel_animation_group::ParallelAnimationGroup) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QParallelAnimationGroup::QParallelAnimationGroup()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::parallel_animation_group::ParallelAnimationGroup> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QParallelAnimationGroup_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QParallelAnimationGroup::QParallelAnimationGroup(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::object::Object)
                           -> ::cpp_utils::CppBox<::parallel_animation_group::ParallelAnimationGroup> {
    let ffi_result = ::ffi::qt_core_c_QParallelAnimationGroup_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```static QString QParallelAnimationGroup::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QParallelAnimationGroup_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QParallelAnimationGroup::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QParallelAnimationGroup_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::parallel_animation_group::ParallelAnimationGroup {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QParallelAnimationGroup_delete
  }
}

impl ::cpp_utils::DynamicCast<::parallel_animation_group::ParallelAnimationGroup> for ::abstract_animation::AbstractAnimation {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::parallel_animation_group::ParallelAnimationGroup> {
let ffi_result = unsafe { ::ffi::qt_core_c_QParallelAnimationGroup_G_dynamic_cast_QParallelAnimationGroup_ptr_QAbstractAnimation(self as *mut ::abstract_animation::AbstractAnimation) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::parallel_animation_group::ParallelAnimationGroup> {
let ffi_result = unsafe { ::ffi::qt_core_c_QParallelAnimationGroup_G_dynamic_cast_QParallelAnimationGroup_ptr_QAbstractAnimation(self as *const ::abstract_animation::AbstractAnimation as *mut ::abstract_animation::AbstractAnimation) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::parallel_animation_group::ParallelAnimationGroup> for ::animation_group::AnimationGroup {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::parallel_animation_group::ParallelAnimationGroup> {
let ffi_result = unsafe { ::ffi::qt_core_c_QParallelAnimationGroup_G_dynamic_cast_QParallelAnimationGroup_ptr_QAnimationGroup(self as *mut ::animation_group::AnimationGroup) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::parallel_animation_group::ParallelAnimationGroup> {
let ffi_result = unsafe { ::ffi::qt_core_c_QParallelAnimationGroup_G_dynamic_cast_QParallelAnimationGroup_ptr_QAnimationGroup(self as *const ::animation_group::AnimationGroup as *mut ::animation_group::AnimationGroup) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::parallel_animation_group::ParallelAnimationGroup> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::parallel_animation_group::ParallelAnimationGroup> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QParallelAnimationGroup_G_dynamic_cast_QParallelAnimationGroup_ptr_QObject(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::parallel_animation_group::ParallelAnimationGroup> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QParallelAnimationGroup_G_dynamic_cast_QParallelAnimationGroup_ptr_QObject(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::abstract_animation::AbstractAnimation> for ::parallel_animation_group::ParallelAnimationGroup {
fn static_cast_mut(&mut self) -> &mut ::abstract_animation::AbstractAnimation {
let ffi_result = unsafe { ::ffi::qt_core_c_QParallelAnimationGroup_G_static_cast_QAbstractAnimation_ptr(self as *mut ::parallel_animation_group::ParallelAnimationGroup) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::abstract_animation::AbstractAnimation {
let ffi_result = unsafe { ::ffi::qt_core_c_QParallelAnimationGroup_G_static_cast_QAbstractAnimation_ptr(self as *const ::parallel_animation_group::ParallelAnimationGroup as *mut ::parallel_animation_group::ParallelAnimationGroup) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::animation_group::AnimationGroup> for ::parallel_animation_group::ParallelAnimationGroup {
fn static_cast_mut(&mut self) -> &mut ::animation_group::AnimationGroup {
let ffi_result = unsafe { ::ffi::qt_core_c_QParallelAnimationGroup_G_static_cast_QAnimationGroup_ptr(self as *mut ::parallel_animation_group::ParallelAnimationGroup) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::animation_group::AnimationGroup {
let ffi_result = unsafe { ::ffi::qt_core_c_QParallelAnimationGroup_G_static_cast_QAnimationGroup_ptr(self as *const ::parallel_animation_group::ParallelAnimationGroup as *mut ::parallel_animation_group::ParallelAnimationGroup) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::object::Object> for ::parallel_animation_group::ParallelAnimationGroup {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QParallelAnimationGroup_G_static_cast_QObject_ptr(self as *mut ::parallel_animation_group::ParallelAnimationGroup) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QParallelAnimationGroup_G_static_cast_QObject_ptr(self as *const ::parallel_animation_group::ParallelAnimationGroup as *mut ::parallel_animation_group::ParallelAnimationGroup) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::parallel_animation_group::ParallelAnimationGroup> for ::abstract_animation::AbstractAnimation {
unsafe fn static_cast_mut(&mut self) -> &mut ::parallel_animation_group::ParallelAnimationGroup {
let ffi_result = ::ffi::qt_core_c_QParallelAnimationGroup_G_static_cast_QParallelAnimationGroup_ptr_QAbstractAnimation(self as *mut ::abstract_animation::AbstractAnimation);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::parallel_animation_group::ParallelAnimationGroup {
let ffi_result = ::ffi::qt_core_c_QParallelAnimationGroup_G_static_cast_QParallelAnimationGroup_ptr_QAbstractAnimation(self as *const ::abstract_animation::AbstractAnimation as *mut ::abstract_animation::AbstractAnimation);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::parallel_animation_group::ParallelAnimationGroup> for ::animation_group::AnimationGroup {
unsafe fn static_cast_mut(&mut self) -> &mut ::parallel_animation_group::ParallelAnimationGroup {
let ffi_result = ::ffi::qt_core_c_QParallelAnimationGroup_G_static_cast_QParallelAnimationGroup_ptr_QAnimationGroup(self as *mut ::animation_group::AnimationGroup);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::parallel_animation_group::ParallelAnimationGroup {
let ffi_result = ::ffi::qt_core_c_QParallelAnimationGroup_G_static_cast_QParallelAnimationGroup_ptr_QAnimationGroup(self as *const ::animation_group::AnimationGroup as *mut ::animation_group::AnimationGroup);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::parallel_animation_group::ParallelAnimationGroup> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::parallel_animation_group::ParallelAnimationGroup {
    let ffi_result = ::ffi::qt_core_c_QParallelAnimationGroup_G_static_cast_QParallelAnimationGroup_ptr_QObject(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::parallel_animation_group::ParallelAnimationGroup {
    let ffi_result = ::ffi::qt_core_c_QParallelAnimationGroup_G_static_cast_QParallelAnimationGroup_ptr_QObject(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::parallel_animation_group::ParallelAnimationGroup {
  type Target = ::animation_group::AnimationGroup;
  fn deref(&self) -> &::animation_group::AnimationGroup {
    let ffi_result = unsafe { ::ffi::qt_core_c_QParallelAnimationGroup_G_static_cast_QAnimationGroup_ptr(self as *const ::parallel_animation_group::ParallelAnimationGroup as *mut ::parallel_animation_group::ParallelAnimationGroup) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::parallel_animation_group::ParallelAnimationGroup {
  fn deref_mut(&mut self) -> &mut ::animation_group::AnimationGroup {
    let ffi_result = unsafe { ::ffi::qt_core_c_QParallelAnimationGroup_G_static_cast_QAnimationGroup_ptr(self as *mut ::parallel_animation_group::ParallelAnimationGroup) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

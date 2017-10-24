/// C++ type: <span style='color: green;'>```QWeakPointer<Qt3DCore::QAspectJob>```</span>
#[repr(C)]
pub struct WeakPointerAspectJob([u8; ::type_sizes::QT_3D_CORE_WEAK_POINTER_WEAK_POINTER_ASPECT_JOB]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for WeakPointerAspectJob {
  unsafe fn new_uninitialized() -> WeakPointerAspectJob {
    WeakPointerAspectJob(::std::mem::uninitialized())
  }
}

impl WeakPointerAspectJob {
  /// C++ method: <span style='color: green;'>```void QWeakPointer<Qt3DCore::QAspectJob>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe {
      ::ffi::qt_3d_core_c_QWeakPointer_Qt3DCore_QAspectJob_clear(self as *mut ::weak_pointer::WeakPointerAspectJob)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QAspectJob* QWeakPointer<Qt3DCore::QAspectJob>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *mut ::aspect_job::AspectJob {
    unsafe {
      ::ffi::qt_3d_core_c_QWeakPointer_Qt3DCore_QAspectJob_data(self as *const ::weak_pointer::WeakPointerAspectJob)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QWeakPointer<Qt3DCore::QAspectJob>::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe {
      ::ffi::qt_3d_core_c_QWeakPointer_Qt3DCore_QAspectJob_isNull(self as *const ::weak_pointer::WeakPointerAspectJob)
    }
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DCore::QAspectJob> QWeakPointer<Qt3DCore::QAspectJob>::lock() const```</span>
  ///
  ///
  pub fn lock(&self) -> ::shared_pointer::SharedPointerAspectJob {
    {
      let mut object: ::shared_pointer::SharedPointerAspectJob =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_QWeakPointer_Qt3DCore_QAspectJob_lock_to_output(self as *const ::weak_pointer::WeakPointerAspectJob, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QWeakPointer<Qt3DCore::QAspectJob>::QWeakPointer```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::weak_pointer::WeakPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QWeakPointer<Qt3DCore::QAspectJob>::QWeakPointer()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::shared_pointer::SharedPointerAspectJob) -> ::weak_pointer::WeakPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QWeakPointer<Qt3DCore::QAspectJob>::QWeakPointer(const QSharedPointer<Qt3DCore::QAspectJob>& o)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::weak_pointer::WeakPointerAspectJob) -> ::weak_pointer::WeakPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QWeakPointer<Qt3DCore::QAspectJob>::QWeakPointer(const QWeakPointer<Qt3DCore::QAspectJob>& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::weak_pointer::WeakPointerAspectJob
    where Args: overloading::WeakPointerAspectJobNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QWeakPointer<Qt3DCore::QAspectJob>::operator=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_assign(&mut self, &'l1 ::shared_pointer::SharedPointerAspectJob) -> &'l0 mut ::weak_pointer::WeakPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```QWeakPointer<Qt3DCore::QAspectJob>& QWeakPointer<Qt3DCore::QAspectJob>::operator=(const QSharedPointer<Qt3DCore::QAspectJob>& o)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_assign(&mut self, &'l1 ::weak_pointer::WeakPointerAspectJob) -> &'l0 mut ::weak_pointer::WeakPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```QWeakPointer<Qt3DCore::QAspectJob>& QWeakPointer<Qt3DCore::QAspectJob>::operator=(const QWeakPointer<Qt3DCore::QAspectJob>& other)```</span>
  ///
  ///
  pub fn op_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::weak_pointer::WeakPointerAspectJob
    where Args: overloading::WeakPointerAspectJobOpAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QWeakPointer<Qt3DCore::QAspectJob>::operator!() const```</span>
  ///
  ///
  pub fn op_not(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QWeakPointer_Qt3DCore_QAspectJob_operator_not(self as *const ::weak_pointer::WeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```void QWeakPointer<Qt3DCore::QAspectJob>::swap(QWeakPointer<Qt3DCore::QAspectJob>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::weak_pointer::WeakPointerAspectJob) {
    unsafe {
      ::ffi::qt_3d_core_c_QWeakPointer_Qt3DCore_QAspectJob_swap(self as *mut ::weak_pointer::WeakPointerAspectJob,
                                                                other as *mut ::weak_pointer::WeakPointerAspectJob)
    }
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DCore::QAspectJob> QWeakPointer<Qt3DCore::QAspectJob>::toStrongRef() const```</span>
  ///
  ///
  pub fn to_strong_ref(&self) -> ::shared_pointer::SharedPointerAspectJob {
    {
      let mut object: ::shared_pointer::SharedPointerAspectJob =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_QWeakPointer_Qt3DCore_QAspectJob_toStrongRef_to_output(self as *const ::weak_pointer::WeakPointerAspectJob, &mut object);
      }
      object
    }
  }
}

impl Drop for ::weak_pointer::WeakPointerAspectJob {
  /// C++ method: <span style='color: green;'>```[destructor] void QWeakPointer<Qt3DCore::QAspectJob>::~QWeakPointer()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_3d_core_c_QWeakPointer_Qt3DCore_QAspectJob_destructor(self as *mut ::weak_pointer::WeakPointerAspectJob)
    }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [WeakPointerAspectJob::new](../struct.WeakPointerAspectJob.html#method.new) method.
  pub trait WeakPointerAspectJobNewArgs {
    fn exec(self) -> ::weak_pointer::WeakPointerAspectJob;
  }
  impl WeakPointerAspectJobNewArgs for () {
    fn exec(self) -> ::weak_pointer::WeakPointerAspectJob {

      {
        let mut object: ::weak_pointer::WeakPointerAspectJob =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QWeakPointer_Qt3DCore_QAspectJob_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> WeakPointerAspectJobNewArgs for &'a ::shared_pointer::SharedPointerAspectJob {
    fn exec(self) -> ::weak_pointer::WeakPointerAspectJob {
      let o = self;
      {
        let mut object: ::weak_pointer::WeakPointerAspectJob =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QWeakPointer_Qt3DCore_QAspectJob_constructor_o(o as *const ::shared_pointer::SharedPointerAspectJob, &mut object);
        }
        object
      }
    }
  }
  impl<'a> WeakPointerAspectJobNewArgs for &'a ::weak_pointer::WeakPointerAspectJob {
    fn exec(self) -> ::weak_pointer::WeakPointerAspectJob {
      let other = self;
      {
        let mut object: ::weak_pointer::WeakPointerAspectJob =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QWeakPointer_Qt3DCore_QAspectJob_constructor_other(other as *const ::weak_pointer::WeakPointerAspectJob, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [WeakPointerAspectJob::op_assign](../struct.WeakPointerAspectJob.html#method.op_assign) method.
  pub trait WeakPointerAspectJobOpAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::weak_pointer::WeakPointerAspectJob)
            -> &'largs mut ::weak_pointer::WeakPointerAspectJob;
  }
  impl<'largs> WeakPointerAspectJobOpAssignArgs<'largs> for &'largs ::shared_pointer::SharedPointerAspectJob {
    fn exec(self,
            original_self: &'largs mut ::weak_pointer::WeakPointerAspectJob)
            -> &'largs mut ::weak_pointer::WeakPointerAspectJob {
      let o = self;
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_QWeakPointer_Qt3DCore_QAspectJob_operator_assign_o(original_self as *mut ::weak_pointer::WeakPointerAspectJob, o as *const ::shared_pointer::SharedPointerAspectJob) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> WeakPointerAspectJobOpAssignArgs<'largs> for &'largs ::weak_pointer::WeakPointerAspectJob {
    fn exec(self,
            original_self: &'largs mut ::weak_pointer::WeakPointerAspectJob)
            -> &'largs mut ::weak_pointer::WeakPointerAspectJob {
      let other = self;
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_QWeakPointer_Qt3DCore_QAspectJob_operator_assign_other(original_self as *mut ::weak_pointer::WeakPointerAspectJob, other as *const ::weak_pointer::WeakPointerAspectJob) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
}

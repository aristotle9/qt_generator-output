/// C++ type: <span style='color: green;'>```Qt3DCore::QAspectJob```</span>
#[repr(C)]
pub struct AspectJob(u8);

impl AspectJob {
  /// C++ method: <span style='color: green;'>```void Qt3DCore::QAspectJob::addDependency(QWeakPointer<Qt3DCore::QAspectJob> dependency)```</span>
  ///
  ///
  pub fn add_dependency(&mut self, dependency: &::weak_pointer::WeakPointerAspectJob) {
    unsafe {
      ::ffi::qt_3d_core_c_Qt3DCore_QAspectJob_addDependency(self as *mut ::aspect_job::AspectJob,
                                                            dependency as *const ::weak_pointer::WeakPointerAspectJob)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QWeakPointer<Qt3DCore::QAspectJob>> Qt3DCore::QAspectJob::dependencies() const```</span>
  ///
  ///
  pub fn dependencies(&self) -> ::vector::VectorWeakPointerWeakPointerAspectJob {
    {
      let mut object: ::vector::VectorWeakPointerWeakPointerAspectJob =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QAspectJob_dependencies_to_output(self as *const ::aspect_job::AspectJob,
                                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DCore::QAspectJob::removeDependency(QWeakPointer<Qt3DCore::QAspectJob> dependency)```</span>
  ///
  ///
  pub fn remove_dependency(&mut self, dependency: &::weak_pointer::WeakPointerAspectJob) {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QAspectJob_removeDependency(self as *mut ::aspect_job::AspectJob, dependency as *const ::weak_pointer::WeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual void Qt3DCore::QAspectJob::run()```</span>
  ///
  ///
  pub fn run(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QAspectJob_run(self as *mut ::aspect_job::AspectJob) }
  }
}

impl ::cpp_utils::CppDeletable for ::aspect_job::AspectJob {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_core_c_Qt3DCore_QAspectJob_delete
  }
}

/// C++ type: <span style='color: green;'>```QSharedPointer<Qt3DCore::QAspectJob>```</span>
#[repr(C)]
pub struct SharedPointerAspectJob([u8; ::type_sizes::QT_3D_CORE_SHARED_POINTER_SHARED_POINTER_ASPECT_JOB]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for SharedPointerAspectJob {
  unsafe fn new_uninitialized() -> SharedPointerAspectJob {
    SharedPointerAspectJob(::std::mem::uninitialized())
  }
}

impl SharedPointerAspectJob {
  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DCore::QAspectJob>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QAspectJob_clear(self as *mut ::shared_pointer::SharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QAspectJob* QSharedPointer<Qt3DCore::QAspectJob>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *mut ::aspect_job::AspectJob {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QAspectJob_data(self as *const ::shared_pointer::SharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```bool QSharedPointer<Qt3DCore::QAspectJob>::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QAspectJob_isNull(self as *const ::shared_pointer::SharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DCore::QAspectJob>::QSharedPointer```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::shared_pointer::SharedPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedPointer<Qt3DCore::QAspectJob>::QSharedPointer()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::shared_pointer::SharedPointerAspectJob) -> ::shared_pointer::SharedPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedPointer<Qt3DCore::QAspectJob>::QSharedPointer(const QSharedPointer<Qt3DCore::QAspectJob>& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::shared_pointer::SharedPointerAspectJob
    where Args: overloading::SharedPointerAspectJobNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DCore::QAspectJob>& QSharedPointer<Qt3DCore::QAspectJob>::operator=(const QSharedPointer<Qt3DCore::QAspectJob>& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::shared_pointer::SharedPointerAspectJob)
                             -> &'l0 mut ::shared_pointer::SharedPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QAspectJob_operator_assign(self as *mut ::shared_pointer::SharedPointerAspectJob, other as *const ::shared_pointer::SharedPointerAspectJob) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QAspectJob& QSharedPointer<Qt3DCore::QAspectJob>::operator*() const```</span>
  ///
  ///
  pub fn op_indirection<'l0>(&'l0 self) -> &'l0 mut ::aspect_job::AspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QAspectJob_operator_indirection(self as *const ::shared_pointer::SharedPointerAspectJob) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QSharedPointer<Qt3DCore::QAspectJob>::operator!() const```</span>
  ///
  ///
  pub fn op_not(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QAspectJob_operator_not(self as *const ::shared_pointer::SharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QAspectJob* QSharedPointer<Qt3DCore::QAspectJob>::operator->() const```</span>
  ///
  ///
  pub fn op_struct_deref(&self) -> *mut ::aspect_job::AspectJob {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QAspectJob_operator_struct_deref(self as *const ::shared_pointer::SharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DCore::QAspectJob>::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QAspectJob_reset_no_args(self as *mut ::shared_pointer::SharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DCore::QAspectJob>::reset(Qt3DCore::QAspectJob* t)```</span>
  ///
  ///
  pub unsafe fn reset_unsafe(&mut self, t: *mut ::aspect_job::AspectJob) {
    ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QAspectJob_reset_t(self as *mut ::shared_pointer::SharedPointerAspectJob, t)
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DCore::QAspectJob>::swap(QSharedPointer<Qt3DCore::QAspectJob>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::shared_pointer::SharedPointerAspectJob) {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QAspectJob_swap(self as *mut ::shared_pointer::SharedPointerAspectJob, other as *mut ::shared_pointer::SharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```QWeakPointer<Qt3DCore::QAspectJob> QSharedPointer<Qt3DCore::QAspectJob>::toWeakRef() const```</span>
  ///
  ///
  pub fn to_weak_ref(&self) -> ::weak_pointer::WeakPointerAspectJob {
    {
      let mut object: ::weak_pointer::WeakPointerAspectJob =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QAspectJob_toWeakRef_to_output(self as *const ::shared_pointer::SharedPointerAspectJob, &mut object);
      }
      object
    }
  }
}

impl Drop for ::shared_pointer::SharedPointerAspectJob {
  /// C++ method: <span style='color: green;'>```[destructor] void QSharedPointer<Qt3DCore::QAspectJob>::~QSharedPointer()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QAspectJob_destructor(self as *mut ::shared_pointer::SharedPointerAspectJob) }
  }
}

/// C++ type: <span style='color: green;'>```QSharedPointer<Qt3DCore::QBackendNodeMapper>```</span>
#[repr(C)]
pub struct SharedPointerBackendNodeBackendNodeMapper([u8; ::type_sizes::QT_3D_CORE_SHARED_POINTER_SHARED_POINTER_BACKEND_NODE_BACKEND_NODE_MAPPER]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for SharedPointerBackendNodeBackendNodeMapper {
  unsafe fn new_uninitialized() -> SharedPointerBackendNodeBackendNodeMapper {
    SharedPointerBackendNodeBackendNodeMapper(::std::mem::uninitialized())
  }
}

impl SharedPointerBackendNodeBackendNodeMapper {
  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DCore::QBackendNodeMapper>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QBackendNodeMapper_clear(self as *mut ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QBackendNodeMapper* QSharedPointer<Qt3DCore::QBackendNodeMapper>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *mut ::backend_node::BackendNodeMapper {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QBackendNodeMapper_data(self as *const ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper) }
  }

  /// C++ method: <span style='color: green;'>```bool QSharedPointer<Qt3DCore::QBackendNodeMapper>::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QBackendNodeMapper_isNull(self as *const ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper) }
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DCore::QBackendNodeMapper>::QSharedPointer```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedPointer<Qt3DCore::QBackendNodeMapper>::QSharedPointer()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::shared_pointer::SharedPointerBackendNodeBackendNodeMapper) -> ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedPointer<Qt3DCore::QBackendNodeMapper>::QSharedPointer(const QSharedPointer<Qt3DCore::QBackendNodeMapper>& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper
    where Args: overloading::SharedPointerBackendNodeBackendNodeMapperNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DCore::QBackendNodeMapper>& QSharedPointer<Qt3DCore::QBackendNodeMapper>::operator=(const QSharedPointer<Qt3DCore::QBackendNodeMapper>& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper)
                             -> &'l0 mut ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QBackendNodeMapper_operator_assign(self as *mut ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper, other as *const ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QBackendNodeMapper& QSharedPointer<Qt3DCore::QBackendNodeMapper>::operator*() const```</span>
  ///
  ///
  pub fn op_indirection<'l0>(&'l0 self) -> &'l0 mut ::backend_node::BackendNodeMapper {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QBackendNodeMapper_operator_indirection(self as *const ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QSharedPointer<Qt3DCore::QBackendNodeMapper>::operator!() const```</span>
  ///
  ///
  pub fn op_not(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QBackendNodeMapper_operator_not(self as *const ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QBackendNodeMapper* QSharedPointer<Qt3DCore::QBackendNodeMapper>::operator->() const```</span>
  ///
  ///
  pub fn op_struct_deref(&self) -> *mut ::backend_node::BackendNodeMapper {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QBackendNodeMapper_operator_struct_deref(self as *const ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper) }
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DCore::QBackendNodeMapper>::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QBackendNodeMapper_reset_no_args(self as *mut ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper) }
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DCore::QBackendNodeMapper>::reset(Qt3DCore::QBackendNodeMapper* t)```</span>
  ///
  ///
  pub unsafe fn reset_unsafe(&mut self, t: *mut ::backend_node::BackendNodeMapper) {
    ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QBackendNodeMapper_reset_t(self as *mut ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper, t)
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DCore::QBackendNodeMapper>::swap(QSharedPointer<Qt3DCore::QBackendNodeMapper>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper) {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QBackendNodeMapper_swap(self as *mut ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper, other as *mut ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper) }
  }
}

impl Drop for ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper {
  /// C++ method: <span style='color: green;'>```[destructor] void QSharedPointer<Qt3DCore::QBackendNodeMapper>::~QSharedPointer()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QBackendNodeMapper_destructor(self as *mut ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper) }
  }
}

/// C++ type: <span style='color: green;'>```QSharedPointer<Qt3DCore::QEntity>```</span>
#[repr(C)]
pub struct SharedPointerEntity([u8; ::type_sizes::QT_3D_CORE_SHARED_POINTER_SHARED_POINTER_ENTITY]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for SharedPointerEntity {
  unsafe fn new_uninitialized() -> SharedPointerEntity {
    SharedPointerEntity(::std::mem::uninitialized())
  }
}

impl SharedPointerEntity {
  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DCore::QEntity>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe {
      ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QEntity_clear(self as *mut ::shared_pointer::SharedPointerEntity)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity* QSharedPointer<Qt3DCore::QEntity>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *mut ::entity::Entity {
    unsafe {
      ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QEntity_data(self as *const ::shared_pointer::SharedPointerEntity)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QSharedPointer<Qt3DCore::QEntity>::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe {
      ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QEntity_isNull(self as *const ::shared_pointer::SharedPointerEntity)
    }
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DCore::QEntity>::QSharedPointer```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::shared_pointer::SharedPointerEntity```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedPointer<Qt3DCore::QEntity>::QSharedPointer()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::shared_pointer::SharedPointerEntity) -> ::shared_pointer::SharedPointerEntity```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedPointer<Qt3DCore::QEntity>::QSharedPointer(const QSharedPointer<Qt3DCore::QEntity>& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::shared_pointer::SharedPointerEntity
    where Args: overloading::SharedPointerEntityNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DCore::QEntity>& QSharedPointer<Qt3DCore::QEntity>::operator=(const QSharedPointer<Qt3DCore::QEntity>& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::shared_pointer::SharedPointerEntity)
                             -> &'l0 mut ::shared_pointer::SharedPointerEntity {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QEntity_operator_assign(self as *mut ::shared_pointer::SharedPointerEntity, other as *const ::shared_pointer::SharedPointerEntity) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity& QSharedPointer<Qt3DCore::QEntity>::operator*() const```</span>
  ///
  ///
  pub fn op_indirection<'l0>(&'l0 self) -> &'l0 mut ::entity::Entity {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QEntity_operator_indirection(self as *const ::shared_pointer::SharedPointerEntity) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QSharedPointer<Qt3DCore::QEntity>::operator!() const```</span>
  ///
  ///
  pub fn op_not(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QEntity_operator_not(self as *const ::shared_pointer::SharedPointerEntity) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity* QSharedPointer<Qt3DCore::QEntity>::operator->() const```</span>
  ///
  ///
  pub fn op_struct_deref(&self) -> *mut ::entity::Entity {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QEntity_operator_struct_deref(self as *const ::shared_pointer::SharedPointerEntity) }
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DCore::QEntity>::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QEntity_reset_no_args(self as *mut ::shared_pointer::SharedPointerEntity) }
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DCore::QEntity>::reset(Qt3DCore::QEntity* t)```</span>
  ///
  ///
  pub unsafe fn reset_unsafe(&mut self, t: *mut ::entity::Entity) {
    ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QEntity_reset_t(self as *mut ::shared_pointer::SharedPointerEntity, t)
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DCore::QEntity>::swap(QSharedPointer<Qt3DCore::QEntity>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::shared_pointer::SharedPointerEntity) {
    unsafe {
      ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QEntity_swap(self as *mut ::shared_pointer::SharedPointerEntity,
                                                               other as *mut ::shared_pointer::SharedPointerEntity)
    }
  }
}

impl Drop for ::shared_pointer::SharedPointerEntity {
  /// C++ method: <span style='color: green;'>```[destructor] void QSharedPointer<Qt3DCore::QEntity>::~QSharedPointer()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QEntity_destructor(self as *mut ::shared_pointer::SharedPointerEntity)
    }
  }
}

/// C++ type: <span style='color: green;'>```QSharedPointer<Qt3DCore::QNodeCreatedChangeBase>```</span>
#[repr(C)]
pub struct SharedPointerNodeCreatedChangeNodeCreatedChangeBase([u8; ::type_sizes::QT_3D_CORE_SHARED_POINTER_SHARED_POINTER_NODE_CREATED_CHANGE_NODE_CREATED_CHANGE_BASE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for SharedPointerNodeCreatedChangeNodeCreatedChangeBase {
  unsafe fn new_uninitialized() -> SharedPointerNodeCreatedChangeNodeCreatedChangeBase {
    SharedPointerNodeCreatedChangeNodeCreatedChangeBase(::std::mem::uninitialized())
  }
}

impl SharedPointerNodeCreatedChangeNodeCreatedChangeBase {
  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DCore::QNodeCreatedChangeBase>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QNodeCreatedChangeBase_clear(self as *mut ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeCreatedChangeBase* QSharedPointer<Qt3DCore::QNodeCreatedChangeBase>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *mut ::node_created_change::NodeCreatedChangeBase {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QNodeCreatedChangeBase_data(self as *const ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase) }
  }

  /// C++ method: <span style='color: green;'>```bool QSharedPointer<Qt3DCore::QNodeCreatedChangeBase>::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QNodeCreatedChangeBase_isNull(self as *const ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase) }
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DCore::QNodeCreatedChangeBase>::QSharedPointer```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedPointer<Qt3DCore::QNodeCreatedChangeBase>::QSharedPointer()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase) -> ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedPointer<Qt3DCore::QNodeCreatedChangeBase>::QSharedPointer(const QSharedPointer<Qt3DCore::QNodeCreatedChangeBase>& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase
    where Args: overloading::SharedPointerNodeCreatedChangeNodeCreatedChangeBaseNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DCore::QNodeCreatedChangeBase>& QSharedPointer<Qt3DCore::QNodeCreatedChangeBase>::operator=(const QSharedPointer<Qt3DCore::QNodeCreatedChangeBase>& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase)
                             -> &'l0 mut ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QNodeCreatedChangeBase_operator_assign(self as *mut ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase, other as *const ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeCreatedChangeBase& QSharedPointer<Qt3DCore::QNodeCreatedChangeBase>::operator*() const```</span>
  ///
  ///
  pub fn op_indirection<'l0>(&'l0 self) -> &'l0 mut ::node_created_change::NodeCreatedChangeBase {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QNodeCreatedChangeBase_operator_indirection(self as *const ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QSharedPointer<Qt3DCore::QNodeCreatedChangeBase>::operator!() const```</span>
  ///
  ///
  pub fn op_not(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QNodeCreatedChangeBase_operator_not(self as *const ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeCreatedChangeBase* QSharedPointer<Qt3DCore::QNodeCreatedChangeBase>::operator->() const```</span>
  ///
  ///
  pub fn op_struct_deref(&self) -> *mut ::node_created_change::NodeCreatedChangeBase {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QNodeCreatedChangeBase_operator_struct_deref(self as *const ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase) }
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DCore::QNodeCreatedChangeBase>::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QNodeCreatedChangeBase_reset_no_args(self as *mut ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase) }
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DCore::QNodeCreatedChangeBase>::reset(Qt3DCore::QNodeCreatedChangeBase* t)```</span>
  ///
  ///
  pub unsafe fn reset_unsafe(&mut self, t: *mut ::node_created_change::NodeCreatedChangeBase) {
    ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QNodeCreatedChangeBase_reset_t(self as *mut ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase, t)
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DCore::QNodeCreatedChangeBase>::swap(QSharedPointer<Qt3DCore::QNodeCreatedChangeBase>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase) {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QNodeCreatedChangeBase_swap(self as *mut ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase, other as *mut ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase) }
  }
}

impl Drop for ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase {
  /// C++ method: <span style='color: green;'>```[destructor] void QSharedPointer<Qt3DCore::QNodeCreatedChangeBase>::~QSharedPointer()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QNodeCreatedChangeBase_destructor(self as *mut ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase) }
  }
}

/// C++ type: <span style='color: green;'>```QSharedPointer<Qt3DCore::QSceneChange>```</span>
#[repr(C)]
pub struct SharedPointerSceneChange([u8; ::type_sizes::QT_3D_CORE_SHARED_POINTER_SHARED_POINTER_SCENE_CHANGE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for SharedPointerSceneChange {
  unsafe fn new_uninitialized() -> SharedPointerSceneChange {
    SharedPointerSceneChange(::std::mem::uninitialized())
  }
}

impl SharedPointerSceneChange {
  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DCore::QSceneChange>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QSceneChange_clear(self as *mut ::shared_pointer::SharedPointerSceneChange) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QSceneChange* QSharedPointer<Qt3DCore::QSceneChange>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *mut ::scene_change::SceneChange {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QSceneChange_data(self as *const ::shared_pointer::SharedPointerSceneChange) }
  }

  /// C++ method: <span style='color: green;'>```bool QSharedPointer<Qt3DCore::QSceneChange>::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QSceneChange_isNull(self as *const ::shared_pointer::SharedPointerSceneChange) }
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DCore::QSceneChange>::QSharedPointer```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::shared_pointer::SharedPointerSceneChange```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedPointer<Qt3DCore::QSceneChange>::QSharedPointer()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::shared_pointer::SharedPointerSceneChange) -> ::shared_pointer::SharedPointerSceneChange```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedPointer<Qt3DCore::QSceneChange>::QSharedPointer(const QSharedPointer<Qt3DCore::QSceneChange>& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::shared_pointer::SharedPointerSceneChange
    where Args: overloading::SharedPointerSceneChangeNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DCore::QSceneChange>& QSharedPointer<Qt3DCore::QSceneChange>::operator=(const QSharedPointer<Qt3DCore::QSceneChange>& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::shared_pointer::SharedPointerSceneChange)
                             -> &'l0 mut ::shared_pointer::SharedPointerSceneChange {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QSceneChange_operator_assign(self as *mut ::shared_pointer::SharedPointerSceneChange, other as *const ::shared_pointer::SharedPointerSceneChange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QSceneChange& QSharedPointer<Qt3DCore::QSceneChange>::operator*() const```</span>
  ///
  ///
  pub fn op_indirection<'l0>(&'l0 self) -> &'l0 mut ::scene_change::SceneChange {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QSceneChange_operator_indirection(self as *const ::shared_pointer::SharedPointerSceneChange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QSharedPointer<Qt3DCore::QSceneChange>::operator!() const```</span>
  ///
  ///
  pub fn op_not(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QSceneChange_operator_not(self as *const ::shared_pointer::SharedPointerSceneChange) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QSceneChange* QSharedPointer<Qt3DCore::QSceneChange>::operator->() const```</span>
  ///
  ///
  pub fn op_struct_deref(&self) -> *mut ::scene_change::SceneChange {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QSceneChange_operator_struct_deref(self as *const ::shared_pointer::SharedPointerSceneChange) }
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DCore::QSceneChange>::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QSceneChange_reset_no_args(self as *mut ::shared_pointer::SharedPointerSceneChange) }
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DCore::QSceneChange>::reset(Qt3DCore::QSceneChange* t)```</span>
  ///
  ///
  pub unsafe fn reset_unsafe(&mut self, t: *mut ::scene_change::SceneChange) {
    ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QSceneChange_reset_t(self as *mut ::shared_pointer::SharedPointerSceneChange, t)
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DCore::QSceneChange>::swap(QSharedPointer<Qt3DCore::QSceneChange>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::shared_pointer::SharedPointerSceneChange) {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QSceneChange_swap(self as *mut ::shared_pointer::SharedPointerSceneChange, other as *mut ::shared_pointer::SharedPointerSceneChange) }
  }
}

impl Drop for ::shared_pointer::SharedPointerSceneChange {
  /// C++ method: <span style='color: green;'>```[destructor] void QSharedPointer<Qt3DCore::QSceneChange>::~QSharedPointer()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QSceneChange_destructor(self as *mut ::shared_pointer::SharedPointerSceneChange) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [SharedPointerAspectJob::new](../struct.SharedPointerAspectJob.html#method.new) method.
  pub trait SharedPointerAspectJobNewArgs {
    fn exec(self) -> ::shared_pointer::SharedPointerAspectJob;
  }
  impl SharedPointerAspectJobNewArgs for () {
    fn exec(self) -> ::shared_pointer::SharedPointerAspectJob {

      {
        let mut object: ::shared_pointer::SharedPointerAspectJob =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QAspectJob_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> SharedPointerAspectJobNewArgs for &'a ::shared_pointer::SharedPointerAspectJob {
    fn exec(self) -> ::shared_pointer::SharedPointerAspectJob {
      let other = self;
      {
        let mut object: ::shared_pointer::SharedPointerAspectJob =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QAspectJob_constructor_other(other as *const ::shared_pointer::SharedPointerAspectJob, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [SharedPointerBackendNodeBackendNodeMapper::new](../struct.SharedPointerBackendNodeBackendNodeMapper.html#method.new) method.
  pub trait SharedPointerBackendNodeBackendNodeMapperNewArgs {
    fn exec(self) -> ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper;
  }
  impl SharedPointerBackendNodeBackendNodeMapperNewArgs for () {
    fn exec(self) -> ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper {

      {
        let mut object: ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QBackendNodeMapper_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> SharedPointerBackendNodeBackendNodeMapperNewArgs for &'a ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper {

  fn exec(self, ) -> ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper {
    let other = self;
    {
let mut object: ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QBackendNodeMapper_constructor_other(other as *const ::shared_pointer::SharedPointerBackendNodeBackendNodeMapper, &mut object); }object
}
  }
}
  /// This trait represents a set of arguments accepted by [SharedPointerEntity::new](../struct.SharedPointerEntity.html#method.new) method.
  pub trait SharedPointerEntityNewArgs {
    fn exec(self) -> ::shared_pointer::SharedPointerEntity;
  }
  impl SharedPointerEntityNewArgs for () {
    fn exec(self) -> ::shared_pointer::SharedPointerEntity {

      {
        let mut object: ::shared_pointer::SharedPointerEntity =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QEntity_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> SharedPointerEntityNewArgs for &'a ::shared_pointer::SharedPointerEntity {
    fn exec(self) -> ::shared_pointer::SharedPointerEntity {
      let other = self;
      {
        let mut object: ::shared_pointer::SharedPointerEntity =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QEntity_constructor_other(other as *const ::shared_pointer::SharedPointerEntity, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [SharedPointerNodeCreatedChangeNodeCreatedChangeBase::new](../struct.SharedPointerNodeCreatedChangeNodeCreatedChangeBase.html#method.new) method.
  pub trait SharedPointerNodeCreatedChangeNodeCreatedChangeBaseNewArgs {
    fn exec(self) -> ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase;
  }
  impl SharedPointerNodeCreatedChangeNodeCreatedChangeBaseNewArgs for () {
    fn exec(self) -> ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase {

      {
        let mut object: ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QNodeCreatedChangeBase_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> SharedPointerNodeCreatedChangeNodeCreatedChangeBaseNewArgs for &'a ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase {

  fn exec(self, ) -> ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase {
    let other = self;
    {
let mut object: ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QNodeCreatedChangeBase_constructor_other(other as *const ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase, &mut object); }object
}
  }
}
  /// This trait represents a set of arguments accepted by [SharedPointerSceneChange::new](../struct.SharedPointerSceneChange.html#method.new) method.
  pub trait SharedPointerSceneChangeNewArgs {
    fn exec(self) -> ::shared_pointer::SharedPointerSceneChange;
  }
  impl SharedPointerSceneChangeNewArgs for () {
    fn exec(self) -> ::shared_pointer::SharedPointerSceneChange {

      {
        let mut object: ::shared_pointer::SharedPointerSceneChange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QSceneChange_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> SharedPointerSceneChangeNewArgs for &'a ::shared_pointer::SharedPointerSceneChange {
    fn exec(self) -> ::shared_pointer::SharedPointerSceneChange {
      let other = self;
      {
        let mut object: ::shared_pointer::SharedPointerSceneChange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QSharedPointer_Qt3DCore_QSceneChange_constructor_other(other as *const ::shared_pointer::SharedPointerSceneChange, &mut object);
        }
        object
      }
    }
  }
}

/// C++ type: <span style='color: green;'>```QSharedPointer<Qt3DRender::QBufferDataGenerator>```</span>
#[repr(C)]
pub struct SharedPointerBufferDataGenerator([u8; ::type_sizes::QT_3D_RENDER_SHARED_POINTER_SHARED_POINTER_BUFFER_DATA_GENERATOR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for SharedPointerBufferDataGenerator {
  unsafe fn new_uninitialized() -> SharedPointerBufferDataGenerator {
    SharedPointerBufferDataGenerator(::std::mem::uninitialized())
  }
}

impl SharedPointerBufferDataGenerator {
  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::QBufferDataGenerator>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QBufferDataGenerator_clear(self as *mut ::shared_pointer::SharedPointerBufferDataGenerator) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QBufferDataGenerator* QSharedPointer<Qt3DRender::QBufferDataGenerator>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *mut ::buffer_data_generator::BufferDataGenerator {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QBufferDataGenerator_data(self as *const ::shared_pointer::SharedPointerBufferDataGenerator) }
  }

  /// C++ method: <span style='color: green;'>```bool QSharedPointer<Qt3DRender::QBufferDataGenerator>::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QBufferDataGenerator_isNull(self as *const ::shared_pointer::SharedPointerBufferDataGenerator) }
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::QBufferDataGenerator>::QSharedPointer```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::shared_pointer::SharedPointerBufferDataGenerator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedPointer<Qt3DRender::QBufferDataGenerator>::QSharedPointer()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::shared_pointer::SharedPointerBufferDataGenerator) -> ::shared_pointer::SharedPointerBufferDataGenerator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedPointer<Qt3DRender::QBufferDataGenerator>::QSharedPointer(const QSharedPointer<Qt3DRender::QBufferDataGenerator>& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::shared_pointer::SharedPointerBufferDataGenerator
    where Args: overloading::SharedPointerBufferDataGeneratorNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::QBufferDataGenerator>& QSharedPointer<Qt3DRender::QBufferDataGenerator>::operator=(const QSharedPointer<Qt3DRender::QBufferDataGenerator>& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::shared_pointer::SharedPointerBufferDataGenerator)
                             -> &'l0 mut ::shared_pointer::SharedPointerBufferDataGenerator {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QBufferDataGenerator_operator_assign(self as *mut ::shared_pointer::SharedPointerBufferDataGenerator, other as *const ::shared_pointer::SharedPointerBufferDataGenerator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QBufferDataGenerator& QSharedPointer<Qt3DRender::QBufferDataGenerator>::operator*() const```</span>
  ///
  ///
  pub fn op_indirection<'l0>(&'l0 self) -> &'l0 mut ::buffer_data_generator::BufferDataGenerator {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QBufferDataGenerator_operator_indirection(self as *const ::shared_pointer::SharedPointerBufferDataGenerator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QSharedPointer<Qt3DRender::QBufferDataGenerator>::operator!() const```</span>
  ///
  ///
  pub fn op_not(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QBufferDataGenerator_operator_not(self as *const ::shared_pointer::SharedPointerBufferDataGenerator) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QBufferDataGenerator* QSharedPointer<Qt3DRender::QBufferDataGenerator>::operator->() const```</span>
  ///
  ///
  pub fn op_struct_deref(&self) -> *mut ::buffer_data_generator::BufferDataGenerator {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QBufferDataGenerator_operator_struct_deref(self as *const ::shared_pointer::SharedPointerBufferDataGenerator) }
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::QBufferDataGenerator>::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QBufferDataGenerator_reset_no_args(self as *mut ::shared_pointer::SharedPointerBufferDataGenerator) }
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::QBufferDataGenerator>::reset(Qt3DRender::QBufferDataGenerator* t)```</span>
  ///
  ///
  pub unsafe fn reset_unsafe(&mut self, t: *mut ::buffer_data_generator::BufferDataGenerator) {
    ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QBufferDataGenerator_reset_t(self as *mut ::shared_pointer::SharedPointerBufferDataGenerator, t)
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::QBufferDataGenerator>::swap(QSharedPointer<Qt3DRender::QBufferDataGenerator>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::shared_pointer::SharedPointerBufferDataGenerator) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QBufferDataGenerator_swap(self as *mut ::shared_pointer::SharedPointerBufferDataGenerator, other as *mut ::shared_pointer::SharedPointerBufferDataGenerator) }
  }
}

impl Drop for ::shared_pointer::SharedPointerBufferDataGenerator {
  /// C++ method: <span style='color: green;'>```[destructor] void QSharedPointer<Qt3DRender::QBufferDataGenerator>::~QSharedPointer()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QBufferDataGenerator_destructor(self as *mut ::shared_pointer::SharedPointerBufferDataGenerator) }
  }
}

/// C++ type: <span style='color: green;'>```QSharedPointer<Qt3DRender::QGeometryFactory>```</span>
#[repr(C)]
pub struct SharedPointerGeometryFactory([u8; ::type_sizes::QT_3D_RENDER_SHARED_POINTER_SHARED_POINTER_GEOMETRY_FACTORY]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for SharedPointerGeometryFactory {
  unsafe fn new_uninitialized() -> SharedPointerGeometryFactory {
    SharedPointerGeometryFactory(::std::mem::uninitialized())
  }
}

impl SharedPointerGeometryFactory {
  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::QGeometryFactory>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QGeometryFactory_clear(self as *mut ::shared_pointer::SharedPointerGeometryFactory) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QGeometryFactory* QSharedPointer<Qt3DRender::QGeometryFactory>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *mut ::geometry_factory::GeometryFactory {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QGeometryFactory_data(self as *const ::shared_pointer::SharedPointerGeometryFactory) }
  }

  /// C++ method: <span style='color: green;'>```bool QSharedPointer<Qt3DRender::QGeometryFactory>::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QGeometryFactory_isNull(self as *const ::shared_pointer::SharedPointerGeometryFactory) }
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::QGeometryFactory>::QSharedPointer```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::shared_pointer::SharedPointerGeometryFactory```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedPointer<Qt3DRender::QGeometryFactory>::QSharedPointer()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::shared_pointer::SharedPointerGeometryFactory) -> ::shared_pointer::SharedPointerGeometryFactory```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedPointer<Qt3DRender::QGeometryFactory>::QSharedPointer(const QSharedPointer<Qt3DRender::QGeometryFactory>& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::shared_pointer::SharedPointerGeometryFactory
    where Args: overloading::SharedPointerGeometryFactoryNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::QGeometryFactory>& QSharedPointer<Qt3DRender::QGeometryFactory>::operator=(const QSharedPointer<Qt3DRender::QGeometryFactory>& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::shared_pointer::SharedPointerGeometryFactory)
                             -> &'l0 mut ::shared_pointer::SharedPointerGeometryFactory {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QGeometryFactory_operator_assign(self as *mut ::shared_pointer::SharedPointerGeometryFactory, other as *const ::shared_pointer::SharedPointerGeometryFactory) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QGeometryFactory& QSharedPointer<Qt3DRender::QGeometryFactory>::operator*() const```</span>
  ///
  ///
  pub fn op_indirection<'l0>(&'l0 self) -> &'l0 mut ::geometry_factory::GeometryFactory {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QGeometryFactory_operator_indirection(self as *const ::shared_pointer::SharedPointerGeometryFactory) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QSharedPointer<Qt3DRender::QGeometryFactory>::operator!() const```</span>
  ///
  ///
  pub fn op_not(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QGeometryFactory_operator_not(self as *const ::shared_pointer::SharedPointerGeometryFactory) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QGeometryFactory* QSharedPointer<Qt3DRender::QGeometryFactory>::operator->() const```</span>
  ///
  ///
  pub fn op_struct_deref(&self) -> *mut ::geometry_factory::GeometryFactory {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QGeometryFactory_operator_struct_deref(self as *const ::shared_pointer::SharedPointerGeometryFactory) }
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::QGeometryFactory>::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QGeometryFactory_reset_no_args(self as *mut ::shared_pointer::SharedPointerGeometryFactory) }
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::QGeometryFactory>::reset(Qt3DRender::QGeometryFactory* t)```</span>
  ///
  ///
  pub unsafe fn reset_unsafe(&mut self, t: *mut ::geometry_factory::GeometryFactory) {
    ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QGeometryFactory_reset_t(self as *mut ::shared_pointer::SharedPointerGeometryFactory, t)
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::QGeometryFactory>::swap(QSharedPointer<Qt3DRender::QGeometryFactory>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::shared_pointer::SharedPointerGeometryFactory) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QGeometryFactory_swap(self as *mut ::shared_pointer::SharedPointerGeometryFactory, other as *mut ::shared_pointer::SharedPointerGeometryFactory) }
  }
}

impl Drop for ::shared_pointer::SharedPointerGeometryFactory {
  /// C++ method: <span style='color: green;'>```[destructor] void QSharedPointer<Qt3DRender::QGeometryFactory>::~QSharedPointer()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QGeometryFactory_destructor(self as *mut ::shared_pointer::SharedPointerGeometryFactory) }
  }
}

/// C++ type: <span style='color: green;'>```QSharedPointer<Qt3DRender::PropertyReaderInterface>```</span>
#[repr(C)]
pub struct SharedPointerShaderDataPropertyReaderInterface([u8; ::type_sizes::QT_3D_RENDER_SHARED_POINTER_SHARED_POINTER_SHADER_DATA_PROPERTY_READER_INTERFACE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for SharedPointerShaderDataPropertyReaderInterface {
  unsafe fn new_uninitialized() -> SharedPointerShaderDataPropertyReaderInterface {
    SharedPointerShaderDataPropertyReaderInterface(::std::mem::uninitialized())
  }
}

impl SharedPointerShaderDataPropertyReaderInterface {
  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::PropertyReaderInterface>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_PropertyReaderInterface_clear(self as *mut ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::PropertyReaderInterface* QSharedPointer<Qt3DRender::PropertyReaderInterface>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *mut ::shader_data::PropertyReaderInterface {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_PropertyReaderInterface_data(self as *const ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface) }
  }

  /// C++ method: <span style='color: green;'>```bool QSharedPointer<Qt3DRender::PropertyReaderInterface>::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_PropertyReaderInterface_isNull(self as *const ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface) }
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::PropertyReaderInterface>::QSharedPointer```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedPointer<Qt3DRender::PropertyReaderInterface>::QSharedPointer()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::shared_pointer::SharedPointerShaderDataPropertyReaderInterface) -> ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedPointer<Qt3DRender::PropertyReaderInterface>::QSharedPointer(const QSharedPointer<Qt3DRender::PropertyReaderInterface>& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface
    where Args: overloading::SharedPointerShaderDataPropertyReaderInterfaceNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::PropertyReaderInterface>& QSharedPointer<Qt3DRender::PropertyReaderInterface>::operator=(const QSharedPointer<Qt3DRender::PropertyReaderInterface>& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface)
                             -> &'l0 mut ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_PropertyReaderInterface_operator_assign(self as *mut ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface, other as *const ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::PropertyReaderInterface& QSharedPointer<Qt3DRender::PropertyReaderInterface>::operator*() const```</span>
  ///
  ///
  pub fn op_indirection<'l0>(&'l0 self) -> &'l0 mut ::shader_data::PropertyReaderInterface {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_PropertyReaderInterface_operator_indirection(self as *const ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QSharedPointer<Qt3DRender::PropertyReaderInterface>::operator!() const```</span>
  ///
  ///
  pub fn op_not(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_PropertyReaderInterface_operator_not(self as *const ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::PropertyReaderInterface* QSharedPointer<Qt3DRender::PropertyReaderInterface>::operator->() const```</span>
  ///
  ///
  pub fn op_struct_deref(&self) -> *mut ::shader_data::PropertyReaderInterface {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_PropertyReaderInterface_operator_struct_deref(self as *const ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface) }
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::PropertyReaderInterface>::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_PropertyReaderInterface_reset_no_args(self as *mut ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface) }
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::PropertyReaderInterface>::reset(Qt3DRender::PropertyReaderInterface* t)```</span>
  ///
  ///
  pub unsafe fn reset_unsafe(&mut self, t: *mut ::shader_data::PropertyReaderInterface) {
    ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_PropertyReaderInterface_reset_t(self as *mut ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface, t)
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::PropertyReaderInterface>::swap(QSharedPointer<Qt3DRender::PropertyReaderInterface>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_PropertyReaderInterface_swap(self as *mut ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface, other as *mut ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface) }
  }
}

impl Drop for ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface {
  /// C++ method: <span style='color: green;'>```[destructor] void QSharedPointer<Qt3DRender::PropertyReaderInterface>::~QSharedPointer()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_PropertyReaderInterface_destructor(self as *mut ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface) }
  }
}

/// C++ type: <span style='color: green;'>```QSharedPointer<Qt3DRender::QTextureData>```</span>
#[repr(C)]
pub struct SharedPointerTextureData([u8; ::type_sizes::QT_3D_RENDER_SHARED_POINTER_SHARED_POINTER_TEXTURE_DATA]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for SharedPointerTextureData {
  unsafe fn new_uninitialized() -> SharedPointerTextureData {
    SharedPointerTextureData(::std::mem::uninitialized())
  }
}

impl SharedPointerTextureData {
  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::QTextureData>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureData_clear(self as *mut ::shared_pointer::SharedPointerTextureData) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTextureData* QSharedPointer<Qt3DRender::QTextureData>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *mut ::texture_data::TextureData {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureData_data(self as *const ::shared_pointer::SharedPointerTextureData) }
  }

  /// C++ method: <span style='color: green;'>```bool QSharedPointer<Qt3DRender::QTextureData>::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureData_isNull(self as *const ::shared_pointer::SharedPointerTextureData) }
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::QTextureData>::QSharedPointer```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::shared_pointer::SharedPointerTextureData```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedPointer<Qt3DRender::QTextureData>::QSharedPointer()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::shared_pointer::SharedPointerTextureData) -> ::shared_pointer::SharedPointerTextureData```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedPointer<Qt3DRender::QTextureData>::QSharedPointer(const QSharedPointer<Qt3DRender::QTextureData>& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::shared_pointer::SharedPointerTextureData
    where Args: overloading::SharedPointerTextureDataNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::QTextureData>& QSharedPointer<Qt3DRender::QTextureData>::operator=(const QSharedPointer<Qt3DRender::QTextureData>& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::shared_pointer::SharedPointerTextureData)
                             -> &'l0 mut ::shared_pointer::SharedPointerTextureData {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureData_operator_assign(self as *mut ::shared_pointer::SharedPointerTextureData, other as *const ::shared_pointer::SharedPointerTextureData) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTextureData& QSharedPointer<Qt3DRender::QTextureData>::operator*() const```</span>
  ///
  ///
  pub fn op_indirection<'l0>(&'l0 self) -> &'l0 mut ::texture_data::TextureData {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureData_operator_indirection(self as *const ::shared_pointer::SharedPointerTextureData) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QSharedPointer<Qt3DRender::QTextureData>::operator!() const```</span>
  ///
  ///
  pub fn op_not(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureData_operator_not(self as *const ::shared_pointer::SharedPointerTextureData) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTextureData* QSharedPointer<Qt3DRender::QTextureData>::operator->() const```</span>
  ///
  ///
  pub fn op_struct_deref(&self) -> *mut ::texture_data::TextureData {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureData_operator_struct_deref(self as *const ::shared_pointer::SharedPointerTextureData) }
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::QTextureData>::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureData_reset_no_args(self as *mut ::shared_pointer::SharedPointerTextureData) }
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::QTextureData>::reset(Qt3DRender::QTextureData* t)```</span>
  ///
  ///
  pub unsafe fn reset_unsafe(&mut self, t: *mut ::texture_data::TextureData) {
    ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureData_reset_t(self as *mut ::shared_pointer::SharedPointerTextureData, t)
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::QTextureData>::swap(QSharedPointer<Qt3DRender::QTextureData>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::shared_pointer::SharedPointerTextureData) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureData_swap(self as *mut ::shared_pointer::SharedPointerTextureData, other as *mut ::shared_pointer::SharedPointerTextureData) }
  }
}

impl Drop for ::shared_pointer::SharedPointerTextureData {
  /// C++ method: <span style='color: green;'>```[destructor] void QSharedPointer<Qt3DRender::QTextureData>::~QSharedPointer()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureData_destructor(self as *mut ::shared_pointer::SharedPointerTextureData) }
  }
}

/// C++ type: <span style='color: green;'>```QSharedPointer<Qt3DRender::QTextureGenerator>```</span>
#[repr(C)]
pub struct SharedPointerTextureGenerator([u8; ::type_sizes::QT_3D_RENDER_SHARED_POINTER_SHARED_POINTER_TEXTURE_GENERATOR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for SharedPointerTextureGenerator {
  unsafe fn new_uninitialized() -> SharedPointerTextureGenerator {
    SharedPointerTextureGenerator(::std::mem::uninitialized())
  }
}

impl SharedPointerTextureGenerator {
  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::QTextureGenerator>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureGenerator_clear(self as *mut ::shared_pointer::SharedPointerTextureGenerator) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTextureGenerator* QSharedPointer<Qt3DRender::QTextureGenerator>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *mut ::texture_generator::TextureGenerator {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureGenerator_data(self as *const ::shared_pointer::SharedPointerTextureGenerator) }
  }

  /// C++ method: <span style='color: green;'>```bool QSharedPointer<Qt3DRender::QTextureGenerator>::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureGenerator_isNull(self as *const ::shared_pointer::SharedPointerTextureGenerator) }
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::QTextureGenerator>::QSharedPointer```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::shared_pointer::SharedPointerTextureGenerator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedPointer<Qt3DRender::QTextureGenerator>::QSharedPointer()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::shared_pointer::SharedPointerTextureGenerator) -> ::shared_pointer::SharedPointerTextureGenerator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedPointer<Qt3DRender::QTextureGenerator>::QSharedPointer(const QSharedPointer<Qt3DRender::QTextureGenerator>& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::shared_pointer::SharedPointerTextureGenerator
    where Args: overloading::SharedPointerTextureGeneratorNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::QTextureGenerator>& QSharedPointer<Qt3DRender::QTextureGenerator>::operator=(const QSharedPointer<Qt3DRender::QTextureGenerator>& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::shared_pointer::SharedPointerTextureGenerator)
                             -> &'l0 mut ::shared_pointer::SharedPointerTextureGenerator {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureGenerator_operator_assign(self as *mut ::shared_pointer::SharedPointerTextureGenerator, other as *const ::shared_pointer::SharedPointerTextureGenerator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTextureGenerator& QSharedPointer<Qt3DRender::QTextureGenerator>::operator*() const```</span>
  ///
  ///
  pub fn op_indirection<'l0>(&'l0 self) -> &'l0 mut ::texture_generator::TextureGenerator {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureGenerator_operator_indirection(self as *const ::shared_pointer::SharedPointerTextureGenerator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QSharedPointer<Qt3DRender::QTextureGenerator>::operator!() const```</span>
  ///
  ///
  pub fn op_not(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureGenerator_operator_not(self as *const ::shared_pointer::SharedPointerTextureGenerator) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTextureGenerator* QSharedPointer<Qt3DRender::QTextureGenerator>::operator->() const```</span>
  ///
  ///
  pub fn op_struct_deref(&self) -> *mut ::texture_generator::TextureGenerator {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureGenerator_operator_struct_deref(self as *const ::shared_pointer::SharedPointerTextureGenerator) }
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::QTextureGenerator>::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureGenerator_reset_no_args(self as *mut ::shared_pointer::SharedPointerTextureGenerator) }
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::QTextureGenerator>::reset(Qt3DRender::QTextureGenerator* t)```</span>
  ///
  ///
  pub unsafe fn reset_unsafe(&mut self, t: *mut ::texture_generator::TextureGenerator) {
    ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureGenerator_reset_t(self as *mut ::shared_pointer::SharedPointerTextureGenerator, t)
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::QTextureGenerator>::swap(QSharedPointer<Qt3DRender::QTextureGenerator>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::shared_pointer::SharedPointerTextureGenerator) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureGenerator_swap(self as *mut ::shared_pointer::SharedPointerTextureGenerator, other as *mut ::shared_pointer::SharedPointerTextureGenerator) }
  }
}

impl Drop for ::shared_pointer::SharedPointerTextureGenerator {
  /// C++ method: <span style='color: green;'>```[destructor] void QSharedPointer<Qt3DRender::QTextureGenerator>::~QSharedPointer()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureGenerator_destructor(self as *mut ::shared_pointer::SharedPointerTextureGenerator) }
  }
}

/// C++ type: <span style='color: green;'>```QSharedPointer<Qt3DRender::QTextureImageData>```</span>
#[repr(C)]
pub struct SharedPointerTextureImageData([u8; ::type_sizes::QT_3D_RENDER_SHARED_POINTER_SHARED_POINTER_TEXTURE_IMAGE_DATA]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for SharedPointerTextureImageData {
  unsafe fn new_uninitialized() -> SharedPointerTextureImageData {
    SharedPointerTextureImageData(::std::mem::uninitialized())
  }
}

impl SharedPointerTextureImageData {
  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::QTextureImageData>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureImageData_clear(self as *mut ::shared_pointer::SharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTextureImageData* QSharedPointer<Qt3DRender::QTextureImageData>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *mut ::texture_image_data::TextureImageData {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureImageData_data(self as *const ::shared_pointer::SharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```bool QSharedPointer<Qt3DRender::QTextureImageData>::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureImageData_isNull(self as *const ::shared_pointer::SharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::QTextureImageData>::QSharedPointer```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::shared_pointer::SharedPointerTextureImageData```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedPointer<Qt3DRender::QTextureImageData>::QSharedPointer()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::shared_pointer::SharedPointerTextureImageData) -> ::shared_pointer::SharedPointerTextureImageData```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedPointer<Qt3DRender::QTextureImageData>::QSharedPointer(const QSharedPointer<Qt3DRender::QTextureImageData>& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::shared_pointer::SharedPointerTextureImageData
    where Args: overloading::SharedPointerTextureImageDataNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::QTextureImageData>& QSharedPointer<Qt3DRender::QTextureImageData>::operator=(const QSharedPointer<Qt3DRender::QTextureImageData>& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::shared_pointer::SharedPointerTextureImageData)
                             -> &'l0 mut ::shared_pointer::SharedPointerTextureImageData {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureImageData_operator_assign(self as *mut ::shared_pointer::SharedPointerTextureImageData, other as *const ::shared_pointer::SharedPointerTextureImageData) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTextureImageData& QSharedPointer<Qt3DRender::QTextureImageData>::operator*() const```</span>
  ///
  ///
  pub fn op_indirection<'l0>(&'l0 self) -> &'l0 mut ::texture_image_data::TextureImageData {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureImageData_operator_indirection(self as *const ::shared_pointer::SharedPointerTextureImageData) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QSharedPointer<Qt3DRender::QTextureImageData>::operator!() const```</span>
  ///
  ///
  pub fn op_not(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureImageData_operator_not(self as *const ::shared_pointer::SharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTextureImageData* QSharedPointer<Qt3DRender::QTextureImageData>::operator->() const```</span>
  ///
  ///
  pub fn op_struct_deref(&self) -> *mut ::texture_image_data::TextureImageData {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureImageData_operator_struct_deref(self as *const ::shared_pointer::SharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::QTextureImageData>::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureImageData_reset_no_args(self as *mut ::shared_pointer::SharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::QTextureImageData>::reset(Qt3DRender::QTextureImageData* t)```</span>
  ///
  ///
  pub unsafe fn reset_unsafe(&mut self, t: *mut ::texture_image_data::TextureImageData) {
    ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureImageData_reset_t(self as *mut ::shared_pointer::SharedPointerTextureImageData, t)
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::QTextureImageData>::swap(QSharedPointer<Qt3DRender::QTextureImageData>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::shared_pointer::SharedPointerTextureImageData) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureImageData_swap(self as *mut ::shared_pointer::SharedPointerTextureImageData, other as *mut ::shared_pointer::SharedPointerTextureImageData) }
  }
}

impl Drop for ::shared_pointer::SharedPointerTextureImageData {
  /// C++ method: <span style='color: green;'>```[destructor] void QSharedPointer<Qt3DRender::QTextureImageData>::~QSharedPointer()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureImageData_destructor(self as *mut ::shared_pointer::SharedPointerTextureImageData) }
  }
}

/// C++ type: <span style='color: green;'>```QSharedPointer<Qt3DRender::QTextureImageDataGenerator>```</span>
#[repr(C)]
pub struct SharedPointerTextureImageDataGenerator([u8; ::type_sizes::QT_3D_RENDER_SHARED_POINTER_SHARED_POINTER_TEXTURE_IMAGE_DATA_GENERATOR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for SharedPointerTextureImageDataGenerator {
  unsafe fn new_uninitialized() -> SharedPointerTextureImageDataGenerator {
    SharedPointerTextureImageDataGenerator(::std::mem::uninitialized())
  }
}

impl SharedPointerTextureImageDataGenerator {
  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::QTextureImageDataGenerator>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureImageDataGenerator_clear(self as *mut ::shared_pointer::SharedPointerTextureImageDataGenerator) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTextureImageDataGenerator* QSharedPointer<Qt3DRender::QTextureImageDataGenerator>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *mut ::texture_image_data_generator::TextureImageDataGenerator {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureImageDataGenerator_data(self as *const ::shared_pointer::SharedPointerTextureImageDataGenerator) }
  }

  /// C++ method: <span style='color: green;'>```bool QSharedPointer<Qt3DRender::QTextureImageDataGenerator>::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureImageDataGenerator_isNull(self as *const ::shared_pointer::SharedPointerTextureImageDataGenerator) }
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::QTextureImageDataGenerator>::QSharedPointer```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::shared_pointer::SharedPointerTextureImageDataGenerator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedPointer<Qt3DRender::QTextureImageDataGenerator>::QSharedPointer()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::shared_pointer::SharedPointerTextureImageDataGenerator) -> ::shared_pointer::SharedPointerTextureImageDataGenerator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedPointer<Qt3DRender::QTextureImageDataGenerator>::QSharedPointer(const QSharedPointer<Qt3DRender::QTextureImageDataGenerator>& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::shared_pointer::SharedPointerTextureImageDataGenerator
    where Args: overloading::SharedPointerTextureImageDataGeneratorNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::QTextureImageDataGenerator>& QSharedPointer<Qt3DRender::QTextureImageDataGenerator>::operator=(const QSharedPointer<Qt3DRender::QTextureImageDataGenerator>& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::shared_pointer::SharedPointerTextureImageDataGenerator)
                             -> &'l0 mut ::shared_pointer::SharedPointerTextureImageDataGenerator {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureImageDataGenerator_operator_assign(self as *mut ::shared_pointer::SharedPointerTextureImageDataGenerator, other as *const ::shared_pointer::SharedPointerTextureImageDataGenerator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTextureImageDataGenerator& QSharedPointer<Qt3DRender::QTextureImageDataGenerator>::operator*() const```</span>
  ///
  ///
  pub fn op_indirection<'l0>(&'l0 self) -> &'l0 mut ::texture_image_data_generator::TextureImageDataGenerator {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureImageDataGenerator_operator_indirection(self as *const ::shared_pointer::SharedPointerTextureImageDataGenerator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QSharedPointer<Qt3DRender::QTextureImageDataGenerator>::operator!() const```</span>
  ///
  ///
  pub fn op_not(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureImageDataGenerator_operator_not(self as *const ::shared_pointer::SharedPointerTextureImageDataGenerator) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTextureImageDataGenerator* QSharedPointer<Qt3DRender::QTextureImageDataGenerator>::operator->() const```</span>
  ///
  ///
  pub fn op_struct_deref(&self) -> *mut ::texture_image_data_generator::TextureImageDataGenerator {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureImageDataGenerator_operator_struct_deref(self as *const ::shared_pointer::SharedPointerTextureImageDataGenerator) }
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::QTextureImageDataGenerator>::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureImageDataGenerator_reset_no_args(self as *mut ::shared_pointer::SharedPointerTextureImageDataGenerator) }
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::QTextureImageDataGenerator>::reset(Qt3DRender::QTextureImageDataGenerator* t)```</span>
  ///
  ///
  pub unsafe fn reset_unsafe(&mut self, t: *mut ::texture_image_data_generator::TextureImageDataGenerator) {
    ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureImageDataGenerator_reset_t(self as *mut ::shared_pointer::SharedPointerTextureImageDataGenerator, t)
  }

  /// C++ method: <span style='color: green;'>```void QSharedPointer<Qt3DRender::QTextureImageDataGenerator>::swap(QSharedPointer<Qt3DRender::QTextureImageDataGenerator>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::shared_pointer::SharedPointerTextureImageDataGenerator) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureImageDataGenerator_swap(self as *mut ::shared_pointer::SharedPointerTextureImageDataGenerator, other as *mut ::shared_pointer::SharedPointerTextureImageDataGenerator) }
  }
}

impl Drop for ::shared_pointer::SharedPointerTextureImageDataGenerator {
  /// C++ method: <span style='color: green;'>```[destructor] void QSharedPointer<Qt3DRender::QTextureImageDataGenerator>::~QSharedPointer()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureImageDataGenerator_destructor(self as *mut ::shared_pointer::SharedPointerTextureImageDataGenerator) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [SharedPointerBufferDataGenerator::new](../struct.SharedPointerBufferDataGenerator.html#method.new) method.
  pub trait SharedPointerBufferDataGeneratorNewArgs {
    fn exec(self) -> ::shared_pointer::SharedPointerBufferDataGenerator;
  }
  impl SharedPointerBufferDataGeneratorNewArgs for () {
    fn exec(self) -> ::shared_pointer::SharedPointerBufferDataGenerator {

      {
        let mut object: ::shared_pointer::SharedPointerBufferDataGenerator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QBufferDataGenerator_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> SharedPointerBufferDataGeneratorNewArgs for &'a ::shared_pointer::SharedPointerBufferDataGenerator {
    fn exec(self) -> ::shared_pointer::SharedPointerBufferDataGenerator {
      let other = self;
      {
        let mut object: ::shared_pointer::SharedPointerBufferDataGenerator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QBufferDataGenerator_constructor_other(other as *const ::shared_pointer::SharedPointerBufferDataGenerator, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [SharedPointerGeometryFactory::new](../struct.SharedPointerGeometryFactory.html#method.new) method.
  pub trait SharedPointerGeometryFactoryNewArgs {
    fn exec(self) -> ::shared_pointer::SharedPointerGeometryFactory;
  }
  impl SharedPointerGeometryFactoryNewArgs for () {
    fn exec(self) -> ::shared_pointer::SharedPointerGeometryFactory {

      {
        let mut object: ::shared_pointer::SharedPointerGeometryFactory =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QGeometryFactory_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> SharedPointerGeometryFactoryNewArgs for &'a ::shared_pointer::SharedPointerGeometryFactory {
    fn exec(self) -> ::shared_pointer::SharedPointerGeometryFactory {
      let other = self;
      {
        let mut object: ::shared_pointer::SharedPointerGeometryFactory =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QGeometryFactory_constructor_other(other as *const ::shared_pointer::SharedPointerGeometryFactory, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [SharedPointerShaderDataPropertyReaderInterface::new](../struct.SharedPointerShaderDataPropertyReaderInterface.html#method.new) method.
  pub trait SharedPointerShaderDataPropertyReaderInterfaceNewArgs {
    fn exec(self) -> ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface;
  }
  impl SharedPointerShaderDataPropertyReaderInterfaceNewArgs for () {
    fn exec(self) -> ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface {

      {
        let mut object: ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_PropertyReaderInterface_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> SharedPointerShaderDataPropertyReaderInterfaceNewArgs for &'a ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface {

  fn exec(self, ) -> ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface {
    let other = self;
    {
let mut object: ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_PropertyReaderInterface_constructor_other(other as *const ::shared_pointer::SharedPointerShaderDataPropertyReaderInterface, &mut object); }object
}
  }
}
  /// This trait represents a set of arguments accepted by [SharedPointerTextureData::new](../struct.SharedPointerTextureData.html#method.new) method.
  pub trait SharedPointerTextureDataNewArgs {
    fn exec(self) -> ::shared_pointer::SharedPointerTextureData;
  }
  impl SharedPointerTextureDataNewArgs for () {
    fn exec(self) -> ::shared_pointer::SharedPointerTextureData {

      {
        let mut object: ::shared_pointer::SharedPointerTextureData =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureData_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> SharedPointerTextureDataNewArgs for &'a ::shared_pointer::SharedPointerTextureData {
    fn exec(self) -> ::shared_pointer::SharedPointerTextureData {
      let other = self;
      {
        let mut object: ::shared_pointer::SharedPointerTextureData =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureData_constructor_other(other as *const ::shared_pointer::SharedPointerTextureData, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [SharedPointerTextureGenerator::new](../struct.SharedPointerTextureGenerator.html#method.new) method.
  pub trait SharedPointerTextureGeneratorNewArgs {
    fn exec(self) -> ::shared_pointer::SharedPointerTextureGenerator;
  }
  impl SharedPointerTextureGeneratorNewArgs for () {
    fn exec(self) -> ::shared_pointer::SharedPointerTextureGenerator {

      {
        let mut object: ::shared_pointer::SharedPointerTextureGenerator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureGenerator_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> SharedPointerTextureGeneratorNewArgs for &'a ::shared_pointer::SharedPointerTextureGenerator {
    fn exec(self) -> ::shared_pointer::SharedPointerTextureGenerator {
      let other = self;
      {
        let mut object: ::shared_pointer::SharedPointerTextureGenerator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureGenerator_constructor_other(other as *const ::shared_pointer::SharedPointerTextureGenerator, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [SharedPointerTextureImageDataGenerator::new](../struct.SharedPointerTextureImageDataGenerator.html#method.new) method.
  pub trait SharedPointerTextureImageDataGeneratorNewArgs {
    fn exec(self) -> ::shared_pointer::SharedPointerTextureImageDataGenerator;
  }
  impl SharedPointerTextureImageDataGeneratorNewArgs for () {
    fn exec(self) -> ::shared_pointer::SharedPointerTextureImageDataGenerator {

      {
        let mut object: ::shared_pointer::SharedPointerTextureImageDataGenerator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureImageDataGenerator_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> SharedPointerTextureImageDataGeneratorNewArgs for &'a ::shared_pointer::SharedPointerTextureImageDataGenerator {

  fn exec(self, ) -> ::shared_pointer::SharedPointerTextureImageDataGenerator {
    let other = self;
    {
let mut object: ::shared_pointer::SharedPointerTextureImageDataGenerator = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureImageDataGenerator_constructor_other(other as *const ::shared_pointer::SharedPointerTextureImageDataGenerator, &mut object); }object
}
  }
}
  /// This trait represents a set of arguments accepted by [SharedPointerTextureImageData::new](../struct.SharedPointerTextureImageData.html#method.new) method.
  pub trait SharedPointerTextureImageDataNewArgs {
    fn exec(self) -> ::shared_pointer::SharedPointerTextureImageData;
  }
  impl SharedPointerTextureImageDataNewArgs for () {
    fn exec(self) -> ::shared_pointer::SharedPointerTextureImageData {

      {
        let mut object: ::shared_pointer::SharedPointerTextureImageData =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureImageData_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> SharedPointerTextureImageDataNewArgs for &'a ::shared_pointer::SharedPointerTextureImageData {
    fn exec(self) -> ::shared_pointer::SharedPointerTextureImageData {
      let other = self;
      {
        let mut object: ::shared_pointer::SharedPointerTextureImageData =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QSharedPointer_Qt3DRender_QTextureImageData_constructor_other(other as *const ::shared_pointer::SharedPointerTextureImageData, &mut object);
        }
        object
      }
    }
  }
}

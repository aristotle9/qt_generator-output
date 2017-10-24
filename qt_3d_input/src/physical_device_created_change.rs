/// C++ type: <span style='color: green;'>```Qt3DInput::QPhysicalDeviceCreatedChangeBase```</span>
#[repr(C)]
pub struct PhysicalDeviceCreatedChangeBase(u8);

impl PhysicalDeviceCreatedChangeBase {
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QPhysicalDeviceCreatedChangeBase::QPhysicalDeviceCreatedChangeBase(const Qt3DInput::QAbstractPhysicalDevice* device)```</span>
  ///
  ///
  pub unsafe fn new(device: *const ::abstract_physical_device::AbstractPhysicalDevice)
                    -> ::cpp_utils::CppBox<::physical_device_created_change::PhysicalDeviceCreatedChangeBase> {
    let ffi_result = ::ffi::qt_3d_input_c_Qt3DInput_QPhysicalDeviceCreatedChangeBase_new(device);
    ::cpp_utils::CppBox::new(ffi_result)
  }
}

impl ::cpp_utils::CppDeletable for ::physical_device_created_change::PhysicalDeviceCreatedChangeBase {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_input_c_Qt3DInput_QPhysicalDeviceCreatedChangeBase_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node_created_change::NodeCreatedChangeBase> for ::physical_device_created_change::PhysicalDeviceCreatedChangeBase {
fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node_created_change::NodeCreatedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_input_c_QPhysicalDeviceCreatedChange_G_static_cast_Qt3DCore_QNodeCreatedChangeBase_ptr(self as *mut ::physical_device_created_change::PhysicalDeviceCreatedChangeBase) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_3d_core::node_created_change::NodeCreatedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_input_c_QPhysicalDeviceCreatedChange_G_static_cast_Qt3DCore_QNodeCreatedChangeBase_ptr(self as *const ::physical_device_created_change::PhysicalDeviceCreatedChangeBase as *mut ::physical_device_created_change::PhysicalDeviceCreatedChangeBase) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_3d_core::scene_change::SceneChange> for ::physical_device_created_change::PhysicalDeviceCreatedChangeBase {
fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::scene_change::SceneChange {
let ffi_result = unsafe { ::ffi::qt_3d_input_c_QPhysicalDeviceCreatedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *mut ::physical_device_created_change::PhysicalDeviceCreatedChangeBase) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_3d_core::scene_change::SceneChange {
let ffi_result = unsafe { ::ffi::qt_3d_input_c_QPhysicalDeviceCreatedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *const ::physical_device_created_change::PhysicalDeviceCreatedChangeBase as *mut ::physical_device_created_change::PhysicalDeviceCreatedChangeBase) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::physical_device_created_change::PhysicalDeviceCreatedChangeBase> for ::qt_3d_core::node_created_change::NodeCreatedChangeBase {
unsafe fn static_cast_mut(&mut self) -> &mut ::physical_device_created_change::PhysicalDeviceCreatedChangeBase {
let ffi_result = ::ffi::qt_3d_input_c_QPhysicalDeviceCreatedChange_G_static_cast_Qt3DInput_QPhysicalDeviceCreatedChangeBase_ptr_Qt3DCore_QNodeCreatedChangeBase(self as *mut ::qt_3d_core::node_created_change::NodeCreatedChangeBase);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::physical_device_created_change::PhysicalDeviceCreatedChangeBase {
let ffi_result = ::ffi::qt_3d_input_c_QPhysicalDeviceCreatedChange_G_static_cast_Qt3DInput_QPhysicalDeviceCreatedChangeBase_ptr_Qt3DCore_QNodeCreatedChangeBase(self as *const ::qt_3d_core::node_created_change::NodeCreatedChangeBase as *mut ::qt_3d_core::node_created_change::NodeCreatedChangeBase);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::physical_device_created_change::PhysicalDeviceCreatedChangeBase> for ::qt_3d_core::scene_change::SceneChange {
unsafe fn static_cast_mut(&mut self) -> &mut ::physical_device_created_change::PhysicalDeviceCreatedChangeBase {
let ffi_result = ::ffi::qt_3d_input_c_QPhysicalDeviceCreatedChange_G_static_cast_Qt3DInput_QPhysicalDeviceCreatedChangeBase_ptr_Qt3DCore_QSceneChange(self as *mut ::qt_3d_core::scene_change::SceneChange);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::physical_device_created_change::PhysicalDeviceCreatedChangeBase {
let ffi_result = ::ffi::qt_3d_input_c_QPhysicalDeviceCreatedChange_G_static_cast_Qt3DInput_QPhysicalDeviceCreatedChangeBase_ptr_Qt3DCore_QSceneChange(self as *const ::qt_3d_core::scene_change::SceneChange as *mut ::qt_3d_core::scene_change::SceneChange);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::physical_device_created_change::PhysicalDeviceCreatedChangeBase {
  type Target = ::qt_3d_core::node_created_change::NodeCreatedChangeBase;
  fn deref(&self) -> &::qt_3d_core::node_created_change::NodeCreatedChangeBase {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QPhysicalDeviceCreatedChange_G_static_cast_Qt3DCore_QNodeCreatedChangeBase_ptr(self as *const ::physical_device_created_change::PhysicalDeviceCreatedChangeBase as *mut ::physical_device_created_change::PhysicalDeviceCreatedChangeBase) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::physical_device_created_change::PhysicalDeviceCreatedChangeBase {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::node_created_change::NodeCreatedChangeBase {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QPhysicalDeviceCreatedChange_G_static_cast_Qt3DCore_QNodeCreatedChangeBase_ptr(self as *mut ::physical_device_created_change::PhysicalDeviceCreatedChangeBase) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

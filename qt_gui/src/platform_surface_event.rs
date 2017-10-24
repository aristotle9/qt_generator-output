/// C++ type: <span style='color: green;'>```QPlatformSurfaceEvent```</span>
#[repr(C)]
pub struct PlatformSurfaceEvent(u8);

impl PlatformSurfaceEvent {
  /// C++ method: <span style='color: green;'>```[constructor] void QPlatformSurfaceEvent::QPlatformSurfaceEvent(QPlatformSurfaceEvent::SurfaceEventType surfaceEventType)```</span>
  ///
  ///
  pub fn new(surface_event_type: ::platform_surface_event::SurfaceEventType)
             -> ::cpp_utils::CppBox<::platform_surface_event::PlatformSurfaceEvent> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPlatformSurfaceEvent_new(surface_event_type) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QPlatformSurfaceEvent::SurfaceEventType QPlatformSurfaceEvent::surfaceEventType() const```</span>
  ///
  ///
  pub fn surface_event_type(&self) -> ::platform_surface_event::SurfaceEventType {
    unsafe { ::ffi::qt_gui_c_QPlatformSurfaceEvent_surfaceEventType(self as *const ::platform_surface_event::PlatformSurfaceEvent) }
  }
}

impl ::cpp_utils::CppDeletable for ::platform_surface_event::PlatformSurfaceEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QPlatformSurfaceEvent_delete
  }
}

/// C++ type: <span style='color: green;'>```QPlatformSurfaceEvent::SurfaceEventType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SurfaceEventType {
  /// C++ enum variant: <span style='color: green;'>```SurfaceCreated = 0```</span>
  Created = 0,
  /// C++ enum variant: <span style='color: green;'>```SurfaceAboutToBeDestroyed = 1```</span>
  AboutToBeDestroyed = 1,
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::platform_surface_event::PlatformSurfaceEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPlatformSurfaceEvent_G_static_cast_QEvent_ptr(self as *mut ::platform_surface_event::PlatformSurfaceEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPlatformSurfaceEvent_G_static_cast_QEvent_ptr(self as *const ::platform_surface_event::PlatformSurfaceEvent as *mut ::platform_surface_event::PlatformSurfaceEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::platform_surface_event::PlatformSurfaceEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::platform_surface_event::PlatformSurfaceEvent {
    let ffi_result = ::ffi::qt_gui_c_QPlatformSurfaceEvent_G_static_cast_QPlatformSurfaceEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::platform_surface_event::PlatformSurfaceEvent {
    let ffi_result = ::ffi::qt_gui_c_QPlatformSurfaceEvent_G_static_cast_QPlatformSurfaceEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::platform_surface_event::PlatformSurfaceEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPlatformSurfaceEvent_G_static_cast_QEvent_ptr(self as *const ::platform_surface_event::PlatformSurfaceEvent as *mut ::platform_surface_event::PlatformSurfaceEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::platform_surface_event::PlatformSurfaceEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPlatformSurfaceEvent_G_static_cast_QEvent_ptr(self as *mut ::platform_surface_event::PlatformSurfaceEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

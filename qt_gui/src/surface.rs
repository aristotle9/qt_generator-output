/// C++ type: <span style='color: green;'>```QSurface```</span>
#[repr(C)]
pub struct Surface(u8);

impl Surface {
  /// C++ method: <span style='color: green;'>```pure virtual QSurfaceFormat QSurface::format() const```</span>
  ///
  ///
  pub fn format(&self) -> ::surface_format::SurfaceFormat {
    {
      let mut object: ::surface_format::SurfaceFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QSurface_format_to_output(self as *const ::surface::Surface, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QSize QSurface::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QSurface_size_to_output(self as *const ::surface::Surface, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QSurface::supportsOpenGL() const```</span>
  ///
  ///
  pub fn supports_opengl(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QSurface_supportsOpenGL(self as *const ::surface::Surface) }
  }

  /// C++ method: <span style='color: green;'>```QSurface::SurfaceClass QSurface::surfaceClass() const```</span>
  ///
  ///
  pub fn surface_class(&self) -> ::surface::SurfaceClass {
    unsafe { ::ffi::qt_gui_c_QSurface_surfaceClass(self as *const ::surface::Surface) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QSurface::SurfaceType QSurface::surfaceType() const```</span>
  ///
  ///
  pub fn surface_type(&self) -> ::surface::SurfaceType {
    unsafe { ::ffi::qt_gui_c_QSurface_surfaceType(self as *const ::surface::Surface) }
  }
}

impl ::cpp_utils::CppDeletable for ::surface::Surface {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QSurface_delete
  }
}

/// C++ type: <span style='color: green;'>```QSurface::SurfaceClass```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SurfaceClass {
  /// C++ enum variant: <span style='color: green;'>```Window = 0```</span>
  Window = 0,
  /// C++ enum variant: <span style='color: green;'>```Offscreen = 1```</span>
  Offscreen = 1,
}

/// C++ type: <span style='color: green;'>```QSurface::SurfaceType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SurfaceType {
  /// C++ enum variant: <span style='color: green;'>```RasterSurface = 0```</span>
  Raster = 0,
  /// C++ enum variant: <span style='color: green;'>```OpenGLSurface = 1```</span>
  OpenGL = 1,
  /// C++ enum variant: <span style='color: green;'>```RasterGLSurface = 2```</span>
  RasterGL = 2,
  /// C++ enum variant: <span style='color: green;'>```OpenVGSurface = 3```</span>
  OpenVG = 3,
}

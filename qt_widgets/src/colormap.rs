/// C++ type: <span style='color: green;'>```QColormap```</span>
#[repr(C)]
pub struct Colormap([u8; ::type_sizes::QT_WIDGETS_COLORMAP_COLORMAP]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Colormap {
  unsafe fn new_uninitialized() -> Colormap {
    Colormap(::std::mem::uninitialized())
  }
}

impl Colormap {
  /// C++ method: <span style='color: green;'>```static void QColormap::cleanup()```</span>
  ///
  ///
  pub fn cleanup() {
    unsafe { ::ffi::qt_widgets_c_QColormap_cleanup() }
  }

  /// C++ method: <span style='color: green;'>```const QColor QColormap::colorAt(unsigned int pixel) const```</span>
  ///
  ///
  pub fn color_at(&self, pixel: ::libc::c_uint) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QColormap_colorAt_to_output(self as *const ::colormap::Colormap, pixel, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QVector<QColor> QColormap::colormap() const```</span>
  ///
  ///
  pub fn colormap(&self) -> ::vector::VectorQtGuiColor {
    {
      let mut object: ::vector::VectorQtGuiColor =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QColormap_colormap_to_output(self as *const ::colormap::Colormap, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QColormap::depth() const```</span>
  ///
  ///
  pub fn depth(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QColormap_depth(self as *const ::colormap::Colormap) }
  }

  /// C++ method: <span style='color: green;'>```static void QColormap::initialize()```</span>
  ///
  ///
  pub fn initialize() {
    unsafe { ::ffi::qt_widgets_c_QColormap_initialize() }
  }

  /// C++ method: <span style='color: green;'>```QColormap::instance```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn instance(()) -> ::colormap::Colormap```<br>
  /// C++ method: <span style='color: green;'>```static QColormap QColormap::instance()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn instance(::libc::c_int) -> ::colormap::Colormap```<br>
  /// C++ method: <span style='color: green;'>```static QColormap QColormap::instance(int screen = ?)```</span>
  ///
  ///
  pub fn instance<Args>(args: Args) -> ::colormap::Colormap
    where Args: overloading::ColormapInstanceArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QColormap::Mode QColormap::mode() const```</span>
  ///
  ///
  pub fn mode(&self) -> ::colormap::Mode {
    unsafe { ::ffi::qt_widgets_c_QColormap_mode(self as *const ::colormap::Colormap) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QColormap::QColormap(const QColormap& colormap)```</span>
  ///
  ///
  pub fn new(colormap: &::colormap::Colormap) -> ::colormap::Colormap {
    {
      let mut object: ::colormap::Colormap =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QColormap_constructor(colormap as *const ::colormap::Colormap, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QColormap& QColormap::operator=(const QColormap& colormap)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, colormap: &'l1 ::colormap::Colormap) -> &'l0 mut ::colormap::Colormap {
    let ffi_result = unsafe {
      ::ffi::qt_widgets_c_QColormap_operator_assign(self as *mut ::colormap::Colormap,
                                                    colormap as *const ::colormap::Colormap)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```unsigned int QColormap::pixel(const QColor& color) const```</span>
  ///
  ///
  pub fn pixel(&self, color: &::qt_gui::color::Color) -> ::libc::c_uint {
    unsafe {
      ::ffi::qt_widgets_c_QColormap_pixel(self as *const ::colormap::Colormap,
                                          color as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```int QColormap::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QColormap_size(self as *const ::colormap::Colormap) }
  }
}

impl Drop for ::colormap::Colormap {
  /// C++ method: <span style='color: green;'>```[destructor] void QColormap::~QColormap()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QColormap_destructor(self as *mut ::colormap::Colormap) }
  }
}

/// C++ type: <span style='color: green;'>```QColormap::Mode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Mode {
  /// C++ enum variant: <span style='color: green;'>```Direct = 0```</span>
  Direct = 0,
  /// C++ enum variant: <span style='color: green;'>```Indexed = 1```</span>
  Indexed = 1,
  /// C++ enum variant: <span style='color: green;'>```Gray = 2```</span>
  Gray = 2,
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Colormap::instance](../struct.Colormap.html#method.instance) method.
  pub trait ColormapInstanceArgs {
    fn exec(self) -> ::colormap::Colormap;
  }
  impl ColormapInstanceArgs for () {
    fn exec(self) -> ::colormap::Colormap {

      {
        let mut object: ::colormap::Colormap =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QColormap_instance_to_output_no_args(&mut object);
        }
        object
      }
    }
  }
  impl ColormapInstanceArgs for ::libc::c_int {
    fn exec(self) -> ::colormap::Colormap {
      let screen = self;
      {
        let mut object: ::colormap::Colormap =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QColormap_instance_to_output_screen(screen, &mut object);
        }
        object
      }
    }
  }
}

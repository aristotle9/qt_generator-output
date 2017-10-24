/// C++ type: <span style='color: green;'>```QPainterPathStroker```</span>
#[repr(C)]
pub struct PainterPathStroker([u8; ::type_sizes::QT_GUI_PAINTER_PATH_STROKER_PAINTER_PATH_STROKER]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for PainterPathStroker {
  unsafe fn new_uninitialized() -> PainterPathStroker {
    PainterPathStroker(::std::mem::uninitialized())
  }
}

impl PainterPathStroker {
  /// C++ method: <span style='color: green;'>```QPainterPath QPainterPathStroker::createStroke(const QPainterPath& path) const```</span>
  ///
  ///
  pub fn create_stroke(&self, path: &::painter_path::PainterPath) -> ::painter_path::PainterPath {
    {
      let mut object: ::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainterPathStroker_createStroke_to_output(self as *const ::painter_path_stroker::PainterPathStroker, path as *const ::painter_path::PainterPath, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QPainterPathStroker::curveThreshold() const```</span>
  ///
  ///
  pub fn curve_threshold(&self) -> ::libc::c_double {
    unsafe {
      ::ffi::qt_gui_c_QPainterPathStroker_curveThreshold(self as *const ::painter_path_stroker::PainterPathStroker)
    }
  }

  /// C++ method: <span style='color: green;'>```double QPainterPathStroker::dashOffset() const```</span>
  ///
  ///
  pub fn dash_offset(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPainterPathStroker_dashOffset(self as *const ::painter_path_stroker::PainterPathStroker) }
  }

  /// C++ method: <span style='color: green;'>```QVector<double> QPainterPathStroker::dashPattern() const```</span>
  ///
  ///
  pub fn dash_pattern(&self) -> ::vector::VectorCDouble {
    {
      let mut object: ::vector::VectorCDouble =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainterPathStroker_dashPattern_to_output(self as *const ::painter_path_stroker::PainterPathStroker, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QPainterPathStroker::miterLimit() const```</span>
  ///
  ///
  pub fn miter_limit(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPainterPathStroker_miterLimit(self as *const ::painter_path_stroker::PainterPathStroker) }
  }

  /// C++ method: <span style='color: green;'>```QPainterPathStroker::QPainterPathStroker```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::painter_path_stroker::PainterPathStroker```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPainterPathStroker::QPainterPathStroker()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::pen::Pen) -> ::painter_path_stroker::PainterPathStroker```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPainterPathStroker::QPainterPathStroker(const QPen& pen)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::painter_path_stroker::PainterPathStroker
    where Args: overloading::PainterPathStrokerNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QPainterPathStroker::setCapStyle(Qt::PenCapStyle style)```</span>
  ///
  ///
  pub fn set_cap_style(&mut self, style: &::qt_core::qt::PenCapStyle) {
    unsafe {
      ::ffi::qt_gui_c_QPainterPathStroker_setCapStyle(self as *mut ::painter_path_stroker::PainterPathStroker,
                                                      style as *const ::qt_core::qt::PenCapStyle)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPainterPathStroker::setCurveThreshold(double threshold)```</span>
  ///
  ///
  pub fn set_curve_threshold(&mut self, threshold: ::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QPainterPathStroker_setCurveThreshold(self as *mut ::painter_path_stroker::PainterPathStroker,
                                                            threshold)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPainterPathStroker::setDashOffset(double offset)```</span>
  ///
  ///
  pub fn set_dash_offset(&mut self, offset: ::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QPainterPathStroker_setDashOffset(self as *mut ::painter_path_stroker::PainterPathStroker,
                                                        offset)
    }
  }

  /// C++ method: <span style='color: green;'>```QPainterPathStroker::setDashPattern```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_dash_pattern(&mut self, &::qt_core::qt::PenStyle) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPathStroker::setDashPattern(Qt::PenStyle arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_dash_pattern(&mut self, &::vector::VectorCDouble) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPathStroker::setDashPattern(const QVector<double>& dashPattern)```</span>
  ///
  ///
  pub fn set_dash_pattern<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterPathStrokerSetDashPatternArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QPainterPathStroker::setJoinStyle(Qt::PenJoinStyle style)```</span>
  ///
  ///
  pub fn set_join_style(&mut self, style: &::qt_core::qt::PenJoinStyle) {
    unsafe {
      ::ffi::qt_gui_c_QPainterPathStroker_setJoinStyle(self as *mut ::painter_path_stroker::PainterPathStroker,
                                                       style as *const ::qt_core::qt::PenJoinStyle)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPainterPathStroker::setMiterLimit(double length)```</span>
  ///
  ///
  pub fn set_miter_limit(&mut self, length: ::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QPainterPathStroker_setMiterLimit(self as *mut ::painter_path_stroker::PainterPathStroker,
                                                        length)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPainterPathStroker::setWidth(double width)```</span>
  ///
  ///
  pub fn set_width(&mut self, width: ::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QPainterPathStroker_setWidth(self as *mut ::painter_path_stroker::PainterPathStroker,
                                                   width)
    }
  }

  /// C++ method: <span style='color: green;'>```double QPainterPathStroker::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPainterPathStroker_width(self as *const ::painter_path_stroker::PainterPathStroker) }
  }
}

impl Drop for ::painter_path_stroker::PainterPathStroker {
  /// C++ method: <span style='color: green;'>```[destructor] void QPainterPathStroker::~QPainterPathStroker()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPainterPathStroker_destructor(self as *mut ::painter_path_stroker::PainterPathStroker) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [PainterPathStroker::new](../struct.PainterPathStroker.html#method.new) method.
  pub trait PainterPathStrokerNewArgs {
    fn exec(self) -> ::painter_path_stroker::PainterPathStroker;
  }
  impl PainterPathStrokerNewArgs for () {
    fn exec(self) -> ::painter_path_stroker::PainterPathStroker {

      {
        let mut object: ::painter_path_stroker::PainterPathStroker =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPainterPathStroker_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> PainterPathStrokerNewArgs for &'a ::pen::Pen {
    fn exec(self) -> ::painter_path_stroker::PainterPathStroker {
      let pen = self;
      {
        let mut object: ::painter_path_stroker::PainterPathStroker =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPainterPathStroker_constructor_pen(pen as *const ::pen::Pen, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PainterPathStroker::set_dash_pattern](../struct.PainterPathStroker.html#method.set_dash_pattern) method.
  pub trait PainterPathStrokerSetDashPatternArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter_path_stroker::PainterPathStroker) -> ();
  }
  impl<'largs> PainterPathStrokerSetDashPatternArgs<'largs> for &'largs ::qt_core::qt::PenStyle {
    fn exec(self, original_self: &'largs mut ::painter_path_stroker::PainterPathStroker) -> () {
      let arg1 = self;
      unsafe { ::ffi::qt_gui_c_QPainterPathStroker_setDashPattern_arg1(original_self as *mut ::painter_path_stroker::PainterPathStroker, arg1 as *const ::qt_core::qt::PenStyle) }
    }
  }
  impl<'largs> PainterPathStrokerSetDashPatternArgs<'largs> for &'largs ::vector::VectorCDouble {
    fn exec(self, original_self: &'largs mut ::painter_path_stroker::PainterPathStroker) -> () {
      let dash_pattern = self;
      unsafe { ::ffi::qt_gui_c_QPainterPathStroker_setDashPattern_dashPattern(original_self as *mut ::painter_path_stroker::PainterPathStroker, dash_pattern as *const ::vector::VectorCDouble) }
    }
  }
}

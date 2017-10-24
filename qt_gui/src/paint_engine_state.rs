/// C++ type: <span style='color: green;'>```QPaintEngineState```</span>
#[repr(C)]
pub struct PaintEngineState([u8; ::type_sizes::QT_GUI_PAINT_ENGINE_STATE_PAINT_ENGINE_STATE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for PaintEngineState {
  unsafe fn new_uninitialized() -> PaintEngineState {
    PaintEngineState(::std::mem::uninitialized())
  }
}

impl PaintEngineState {
  /// C++ method: <span style='color: green;'>```QBrush QPaintEngineState::backgroundBrush() const```</span>
  ///
  ///
  pub fn background_brush(&self) -> ::brush::Brush {
    {
      let mut object: ::brush::Brush = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPaintEngineState_backgroundBrush_to_output(self as *const ::paint_engine_state::PaintEngineState, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QBrush QPaintEngineState::brush() const```</span>
  ///
  ///
  pub fn brush(&self) -> ::brush::Brush {
    {
      let mut object: ::brush::Brush = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPaintEngineState_brush_to_output(self as *const ::paint_engine_state::PaintEngineState,
                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPaintEngineState::brushNeedsResolving() const```</span>
  ///
  ///
  pub fn brush_needs_resolving(&self) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QPaintEngineState_brushNeedsResolving(self as *const ::paint_engine_state::PaintEngineState)
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QPaintEngineState::brushOrigin() const```</span>
  ///
  ///
  pub fn brush_origin(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPaintEngineState_brushOrigin_to_output(self as *const ::paint_engine_state::PaintEngineState, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPainterPath QPaintEngineState::clipPath() const```</span>
  ///
  ///
  pub fn clip_path(&self) -> ::painter_path::PainterPath {
    {
      let mut object: ::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPaintEngineState_clipPath_to_output(self as *const ::paint_engine_state::PaintEngineState,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRegion QPaintEngineState::clipRegion() const```</span>
  ///
  ///
  pub fn clip_region(&self) -> ::cpp_utils::CppBox<::region::Region> {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QPaintEngineState_clipRegion_as_ptr(self as *const ::paint_engine_state::PaintEngineState)
      };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QFont QPaintEngineState::font() const```</span>
  ///
  ///
  pub fn font(&self) -> ::font::Font {
    {
      let mut object: ::font::Font = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPaintEngineState_font_to_output(self as *const ::paint_engine_state::PaintEngineState,
                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPaintEngineState::isClipEnabled() const```</span>
  ///
  ///
  pub fn is_clip_enabled(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPaintEngineState_isClipEnabled(self as *const ::paint_engine_state::PaintEngineState) }
  }

  /// C++ method: <span style='color: green;'>```QMatrix QPaintEngineState::matrix() const```</span>
  ///
  ///
  pub fn matrix(&self) -> ::matrix::Matrix {
    {
      let mut object: ::matrix::Matrix =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPaintEngineState_matrix_to_output(self as *const ::paint_engine_state::PaintEngineState,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QPaintEngineState::opacity() const```</span>
  ///
  ///
  pub fn opacity(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPaintEngineState_opacity(self as *const ::paint_engine_state::PaintEngineState) }
  }

  /// C++ method: <span style='color: green;'>```QPainter* QPaintEngineState::painter() const```</span>
  ///
  ///
  pub fn painter(&self) -> *mut ::painter::Painter {
    unsafe { ::ffi::qt_gui_c_QPaintEngineState_painter(self as *const ::paint_engine_state::PaintEngineState) }
  }

  /// C++ method: <span style='color: green;'>```QPen QPaintEngineState::pen() const```</span>
  ///
  ///
  pub fn pen(&self) -> ::pen::Pen {
    {
      let mut object: ::pen::Pen = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPaintEngineState_pen_to_output(self as *const ::paint_engine_state::PaintEngineState,
                                                        &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPaintEngineState::penNeedsResolving() const```</span>
  ///
  ///
  pub fn pen_needs_resolving(&self) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QPaintEngineState_penNeedsResolving(self as *const ::paint_engine_state::PaintEngineState)
    }
  }

  /// C++ method: <span style='color: green;'>```QTransform QPaintEngineState::transform() const```</span>
  ///
  ///
  pub fn transform(&self) -> ::transform::Transform {
    {
      let mut object: ::transform::Transform =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPaintEngineState_transform_to_output(self as *const ::paint_engine_state::PaintEngineState,
                                                              &mut object);
      }
      object
    }
  }
}

impl Drop for ::paint_engine_state::PaintEngineState {
  /// C++ method: <span style='color: green;'>```[destructor] void QPaintEngineState::~QPaintEngineState()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPaintEngineState_destructor(self as *mut ::paint_engine_state::PaintEngineState) }
  }
}

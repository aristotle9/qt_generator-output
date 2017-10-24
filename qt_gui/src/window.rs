/// C++ type: <span style='color: green;'>```QWindow::AncestorMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum AncestorMode {
  /// C++ enum variant: <span style='color: green;'>```ExcludeTransients = 0```</span>
  Exclude = 0,
  /// C++ enum variant: <span style='color: green;'>```IncludeTransients = 1```</span>
  Include = 1,
}

/// C++ type: <span style='color: green;'>```QWindow::Visibility```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Visibility {
  /// C++ enum variant: <span style='color: green;'>```Hidden = 0```</span>
  Hidden = 0,
  /// C++ enum variant: <span style='color: green;'>```AutomaticVisibility = 1```</span>
  AutomaticVisibility = 1,
  /// C++ enum variant: <span style='color: green;'>```Windowed = 2```</span>
  Windowed = 2,
  /// C++ enum variant: <span style='color: green;'>```Minimized = 3```</span>
  Minimized = 3,
  /// C++ enum variant: <span style='color: green;'>```Maximized = 4```</span>
  Maximized = 4,
  /// C++ enum variant: <span style='color: green;'>```FullScreen = 5```</span>
  FullScreen = 5,
}

/// C++ type: <span style='color: green;'>```QWindow```</span>
#[repr(C)]
pub struct Window(u8);

impl Window {
  /// C++ method: <span style='color: green;'>```virtual QAccessibleInterface* QWindow::accessibleRoot() const```</span>
  ///
  ///
  pub fn accessible_root(&self) -> *mut ::accessible_interface::AccessibleInterface {
    unsafe { ::ffi::qt_gui_c_QWindow_accessibleRoot(self as *const ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWindow::alert(int msec)```</span>
  ///
  ///
  pub fn alert(&mut self, msec: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QWindow_alert(self as *mut ::window::Window, msec) }
  }

  /// C++ method: <span style='color: green;'>```QSize QWindow::baseSize() const```</span>
  ///
  ///
  pub fn base_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QWindow_baseSize_to_output(self as *const ::window::Window, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] bool QWindow::close()```</span>
  ///
  ///
  pub fn close(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QWindow_close(self as *mut ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```void QWindow::create()```</span>
  ///
  ///
  pub fn create(&mut self) {
    unsafe { ::ffi::qt_gui_c_QWindow_create(self as *mut ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```QCursor QWindow::cursor() const```</span>
  ///
  ///
  pub fn cursor(&self) -> ::cpp_utils::CppBox<::cursor::Cursor> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QWindow_cursor_as_ptr(self as *const ::window::Window) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```void QWindow::destroy()```</span>
  ///
  ///
  pub fn destroy(&mut self) {
    unsafe { ::ffi::qt_gui_c_QWindow_destroy(self as *mut ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```double QWindow::devicePixelRatio() const```</span>
  ///
  ///
  pub fn device_pixel_ratio(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QWindow_devicePixelRatio(self as *const ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```QString QWindow::filePath() const```</span>
  ///
  ///
  pub fn file_path(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QWindow_filePath_to_output(self as *const ::window::Window, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QObject* QWindow::focusObject() const```</span>
  ///
  ///
  pub fn focus_object(&self) -> *mut ::qt_core::object::Object {
    unsafe { ::ffi::qt_gui_c_QWindow_focusObject(self as *const ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSurfaceFormat QWindow::format() const```</span>
  ///
  ///
  pub fn format(&self) -> ::surface_format::SurfaceFormat {
    {
      let mut object: ::surface_format::SurfaceFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QWindow_format_to_output(self as *const ::window::Window, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QWindow::frameGeometry() const```</span>
  ///
  ///
  pub fn frame_geometry(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QWindow_frameGeometry_to_output(self as *const ::window::Window, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMargins QWindow::frameMargins() const```</span>
  ///
  ///
  pub fn frame_margins(&self) -> ::qt_core::margins::Margins {
    {
      let mut object: ::qt_core::margins::Margins =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QWindow_frameMargins_to_output(self as *const ::window::Window, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPoint QWindow::framePosition() const```</span>
  ///
  ///
  pub fn frame_position(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QWindow_framePosition_to_output(self as *const ::window::Window, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QWindow* QWindow::fromWinId(unsigned long long id)```</span>
  ///
  ///
  pub fn from_win_id(id: ::libc::c_ulonglong) -> *mut ::window::Window {
    unsafe { ::ffi::qt_gui_c_QWindow_fromWinId(id) }
  }

  /// C++ method: <span style='color: green;'>```QRect QWindow::geometry() const```</span>
  ///
  ///
  pub fn geometry(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QWindow_geometry_to_output(self as *const ::window::Window, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QWindow::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QWindow_height(self as *const ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWindow::hide()```</span>
  ///
  ///
  pub fn hide(&mut self) {
    unsafe { ::ffi::qt_gui_c_QWindow_hide(self as *mut ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```QIcon QWindow::icon() const```</span>
  ///
  ///
  pub fn icon(&self) -> ::icon::Icon {
    {
      let mut object: ::icon::Icon = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QWindow_icon_to_output(self as *const ::window::Window, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QWindow::isActive() const```</span>
  ///
  ///
  pub fn is_active(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QWindow_isActive(self as *const ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```QWindow::isAncestorOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn is_ancestor_of(&self, *const ::window::Window) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QWindow::isAncestorOf(const QWindow* child) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn is_ancestor_of(&self, (*const ::window::Window, ::window::AncestorMode)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QWindow::isAncestorOf(const QWindow* child, QWindow::AncestorMode mode = ?) const```</span>
  ///
  ///
  pub unsafe fn is_ancestor_of<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::WindowIsAncestorOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QWindow::isExposed() const```</span>
  ///
  ///
  pub fn is_exposed(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QWindow_isExposed(self as *const ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```bool QWindow::isModal() const```</span>
  ///
  ///
  pub fn is_modal(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QWindow_isModal(self as *const ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```bool QWindow::isTopLevel() const```</span>
  ///
  ///
  pub fn is_top_level(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QWindow_isTopLevel(self as *const ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```bool QWindow::isVisible() const```</span>
  ///
  ///
  pub fn is_visible(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QWindow_isVisible(self as *const ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWindow::lower()```</span>
  ///
  ///
  pub fn lower(&mut self) {
    unsafe { ::ffi::qt_gui_c_QWindow_lower(self as *mut ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```QPoint QWindow::mapFromGlobal(const QPoint& pos) const```</span>
  ///
  ///
  pub fn map_from_global(&self, pos: &::qt_core::point::Point) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QWindow_mapFromGlobal_to_output(self as *const ::window::Window,
                                                        pos as *const ::qt_core::point::Point,
                                                        &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPoint QWindow::mapToGlobal(const QPoint& pos) const```</span>
  ///
  ///
  pub fn map_to_global(&self, pos: &::qt_core::point::Point) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QWindow_mapToGlobal_to_output(self as *const ::window::Window,
                                                      pos as *const ::qt_core::point::Point,
                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRegion QWindow::mask() const```</span>
  ///
  ///
  pub fn mask(&self) -> ::cpp_utils::CppBox<::region::Region> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QWindow_mask_as_ptr(self as *const ::window::Window) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```int QWindow::maximumHeight() const```</span>
  ///
  ///
  pub fn maximum_height(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QWindow_maximumHeight(self as *const ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```QSize QWindow::maximumSize() const```</span>
  ///
  ///
  pub fn maximum_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QWindow_maximumSize_to_output(self as *const ::window::Window, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QWindow::maximumWidth() const```</span>
  ///
  ///
  pub fn maximum_width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QWindow_maximumWidth(self as *const ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QWindow::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QWindow_metaObject(self as *const ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```int QWindow::minimumHeight() const```</span>
  ///
  ///
  pub fn minimum_height(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QWindow_minimumHeight(self as *const ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```QSize QWindow::minimumSize() const```</span>
  ///
  ///
  pub fn minimum_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QWindow_minimumSize_to_output(self as *const ::window::Window, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QWindow::minimumWidth() const```</span>
  ///
  ///
  pub fn minimum_width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QWindow_minimumWidth(self as *const ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QWindow::QWindow()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::window::Window> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QWindow_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QWindow::QWindow```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::screen::Screen) -> ::cpp_utils::CppBox<::window::Window>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QWindow::QWindow(QScreen* screen = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::window::Window) -> ::cpp_utils::CppBox<::window::Window>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QWindow::QWindow(QWindow* parent)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::window::Window>
    where Args: overloading::WindowNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```double QWindow::opacity() const```</span>
  ///
  ///
  pub fn opacity(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QWindow_opacity(self as *const ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```QWindow::parent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn parent(&self, ()) -> *mut ::window::Window```<br>
  /// C++ method: <span style='color: green;'>```QWindow* QWindow::parent() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn parent(&self, ::window::AncestorMode) -> *mut ::window::Window```<br>
  /// C++ method: <span style='color: green;'>```QWindow* QWindow::parent(QWindow::AncestorMode mode) const```</span>
  ///
  ///
  pub fn parent<'largs, Args>(&'largs self, args: Args) -> *mut ::window::Window
    where Args: overloading::WindowParentArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPoint QWindow::position() const```</span>
  ///
  ///
  pub fn position(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QWindow_position_to_output(self as *const ::window::Window, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QWindow::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QWindow_qt_metacall(self as *mut ::window::Window,
                                        arg1 as *const ::qt_core::meta_object::Call,
                                        arg2,
                                        arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QWindow::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QWindow_qt_metacast(self as *mut ::window::Window, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWindow::raise()```</span>
  ///
  ///
  pub fn raise(&mut self) {
    unsafe { ::ffi::qt_gui_c_QWindow_raise(self as *mut ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```void QWindow::reportContentOrientationChange(Qt::ScreenOrientation orientation)```</span>
  ///
  ///
  pub fn report_content_orientation_change(&mut self, orientation: &::qt_core::qt::ScreenOrientation) {
    unsafe {
      ::ffi::qt_gui_c_QWindow_reportContentOrientationChange(self as *mut ::window::Window,
                                                             orientation as *const ::qt_core::qt::ScreenOrientation)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWindow::requestActivate()```</span>
  ///
  ///
  pub fn request_activate(&mut self) {
    unsafe { ::ffi::qt_gui_c_QWindow_requestActivate(self as *mut ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWindow::requestUpdate()```</span>
  ///
  ///
  pub fn request_update(&mut self) {
    unsafe { ::ffi::qt_gui_c_QWindow_requestUpdate(self as *mut ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```QSurfaceFormat QWindow::requestedFormat() const```</span>
  ///
  ///
  pub fn requested_format(&self) -> ::surface_format::SurfaceFormat {
    {
      let mut object: ::surface_format::SurfaceFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QWindow_requestedFormat_to_output(self as *const ::window::Window, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QWindow::resize```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn resize(&mut self, &::qt_core::size::Size) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWindow::resize(const QSize& newSize)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn resize(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWindow::resize(int w, int h)```</span>
  ///
  ///
  pub fn resize<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::WindowResizeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QScreen* QWindow::screen() const```</span>
  ///
  ///
  pub fn screen(&self) -> *mut ::screen::Screen {
    unsafe { ::ffi::qt_gui_c_QWindow_screen(self as *const ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```void QWindow::setBaseSize(const QSize& size)```</span>
  ///
  ///
  pub fn set_base_size(&mut self, size: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_gui_c_QWindow_setBaseSize(self as *mut ::window::Window,
                                          size as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QWindow::setCursor(const QCursor& arg1)```</span>
  ///
  ///
  pub fn set_cursor(&mut self, arg1: &::cursor::Cursor) {
    unsafe {
      ::ffi::qt_gui_c_QWindow_setCursor(self as *mut ::window::Window,
                                        arg1 as *const ::cursor::Cursor)
    }
  }

  /// C++ method: <span style='color: green;'>```void QWindow::setFilePath(const QString& filePath)```</span>
  ///
  ///
  pub fn set_file_path(&mut self, file_path: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QWindow_setFilePath(self as *mut ::window::Window,
                                          file_path as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QWindow::setFlag```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_flag(&mut self, &::qt_core::qt::WindowType) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWindow::setFlag(Qt::WindowType arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_flag(&mut self, (&::qt_core::qt::WindowType, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWindow::setFlag(Qt::WindowType arg1, bool on = ?)```</span>
  ///
  ///
  pub fn set_flag<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::WindowSetFlagArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QWindow::setFormat(const QSurfaceFormat& format)```</span>
  ///
  ///
  pub fn set_format(&mut self, format: &::surface_format::SurfaceFormat) {
    unsafe {
      ::ffi::qt_gui_c_QWindow_setFormat(self as *mut ::window::Window,
                                        format as *const ::surface_format::SurfaceFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```void QWindow::setFramePosition(const QPoint& point)```</span>
  ///
  ///
  pub fn set_frame_position(&mut self, point: &::qt_core::point::Point) {
    unsafe {
      ::ffi::qt_gui_c_QWindow_setFramePosition(self as *mut ::window::Window,
                                               point as *const ::qt_core::point::Point)
    }
  }

  /// C++ method: <span style='color: green;'>```QWindow::setGeometry```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_geometry(&mut self, &::qt_core::rect::Rect) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWindow::setGeometry(const QRect& rect)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_geometry(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWindow::setGeometry(int posx, int posy, int w, int h)```</span>
  ///
  ///
  pub fn set_geometry<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::WindowSetGeometryArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```[slot] void QWindow::setHeight(int arg)```</span>
  ///
  ///
  pub fn set_height(&mut self, arg: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QWindow_setHeight(self as *mut ::window::Window, arg) }
  }

  /// C++ method: <span style='color: green;'>```void QWindow::setIcon(const QIcon& icon)```</span>
  ///
  ///
  pub fn set_icon(&mut self, icon: &::icon::Icon) {
    unsafe { ::ffi::qt_gui_c_QWindow_setIcon(self as *mut ::window::Window, icon as *const ::icon::Icon) }
  }

  /// C++ method: <span style='color: green;'>```bool QWindow::setKeyboardGrabEnabled(bool grab)```</span>
  ///
  ///
  pub fn set_keyboard_grab_enabled(&mut self, grab: bool) -> bool {
    unsafe { ::ffi::qt_gui_c_QWindow_setKeyboardGrabEnabled(self as *mut ::window::Window, grab) }
  }

  /// C++ method: <span style='color: green;'>```void QWindow::setMask(const QRegion& region)```</span>
  ///
  ///
  pub fn set_mask(&mut self, region: &::region::Region) {
    unsafe {
      ::ffi::qt_gui_c_QWindow_setMask(self as *mut ::window::Window,
                                      region as *const ::region::Region)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWindow::setMaximumHeight(int h)```</span>
  ///
  ///
  pub fn set_maximum_height(&mut self, h: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QWindow_setMaximumHeight(self as *mut ::window::Window, h) }
  }

  /// C++ method: <span style='color: green;'>```void QWindow::setMaximumSize(const QSize& size)```</span>
  ///
  ///
  pub fn set_maximum_size(&mut self, size: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_gui_c_QWindow_setMaximumSize(self as *mut ::window::Window,
                                             size as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWindow::setMaximumWidth(int w)```</span>
  ///
  ///
  pub fn set_maximum_width(&mut self, w: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QWindow_setMaximumWidth(self as *mut ::window::Window, w) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWindow::setMinimumHeight(int h)```</span>
  ///
  ///
  pub fn set_minimum_height(&mut self, h: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QWindow_setMinimumHeight(self as *mut ::window::Window, h) }
  }

  /// C++ method: <span style='color: green;'>```void QWindow::setMinimumSize(const QSize& size)```</span>
  ///
  ///
  pub fn set_minimum_size(&mut self, size: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_gui_c_QWindow_setMinimumSize(self as *mut ::window::Window,
                                             size as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWindow::setMinimumWidth(int w)```</span>
  ///
  ///
  pub fn set_minimum_width(&mut self, w: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QWindow_setMinimumWidth(self as *mut ::window::Window, w) }
  }

  /// C++ method: <span style='color: green;'>```void QWindow::setModality(Qt::WindowModality modality)```</span>
  ///
  ///
  pub fn set_modality(&mut self, modality: &::qt_core::qt::WindowModality) {
    unsafe {
      ::ffi::qt_gui_c_QWindow_setModality(self as *mut ::window::Window,
                                          modality as *const ::qt_core::qt::WindowModality)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QWindow::setMouseGrabEnabled(bool grab)```</span>
  ///
  ///
  pub fn set_mouse_grab_enabled(&mut self, grab: bool) -> bool {
    unsafe { ::ffi::qt_gui_c_QWindow_setMouseGrabEnabled(self as *mut ::window::Window, grab) }
  }

  /// C++ method: <span style='color: green;'>```void QWindow::setOpacity(double level)```</span>
  ///
  ///
  pub fn set_opacity(&mut self, level: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QWindow_setOpacity(self as *mut ::window::Window, level) }
  }

  /// C++ method: <span style='color: green;'>```void QWindow::setParent(QWindow* parent)```</span>
  ///
  ///
  pub unsafe fn set_parent(&mut self, parent: *mut ::window::Window) {
    ::ffi::qt_gui_c_QWindow_setParent(self as *mut ::window::Window, parent)
  }

  /// C++ method: <span style='color: green;'>```QWindow::setPosition```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_position(&mut self, &::qt_core::point::Point) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWindow::setPosition(const QPoint& pt)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_position(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QWindow::setPosition(int posx, int posy)```</span>
  ///
  ///
  pub fn set_position<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::WindowSetPositionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QWindow::setScreen(QScreen* screen)```</span>
  ///
  ///
  pub unsafe fn set_screen(&mut self, screen: *mut ::screen::Screen) {
    ::ffi::qt_gui_c_QWindow_setScreen(self as *mut ::window::Window, screen)
  }

  /// C++ method: <span style='color: green;'>```void QWindow::setSizeIncrement(const QSize& size)```</span>
  ///
  ///
  pub fn set_size_increment(&mut self, size: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_gui_c_QWindow_setSizeIncrement(self as *mut ::window::Window,
                                               size as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QWindow::setSurfaceType(QSurface::SurfaceType surfaceType)```</span>
  ///
  ///
  pub fn set_surface_type(&mut self, surface_type: ::surface::SurfaceType) {
    unsafe { ::ffi::qt_gui_c_QWindow_setSurfaceType(self as *mut ::window::Window, surface_type) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWindow::setTitle(const QString& arg1)```</span>
  ///
  ///
  pub fn set_title(&mut self, arg1: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QWindow_setTitle(self as *mut ::window::Window,
                                       arg1 as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QWindow::setTransientParent(QWindow* parent)```</span>
  ///
  ///
  pub unsafe fn set_transient_parent(&mut self, parent: *mut ::window::Window) {
    ::ffi::qt_gui_c_QWindow_setTransientParent(self as *mut ::window::Window, parent)
  }

  /// C++ method: <span style='color: green;'>```void QWindow::setVisibility(QWindow::Visibility v)```</span>
  ///
  ///
  pub fn set_visibility(&mut self, v: ::window::Visibility) {
    unsafe { ::ffi::qt_gui_c_QWindow_setVisibility(self as *mut ::window::Window, v) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWindow::setVisible(bool visible)```</span>
  ///
  ///
  pub fn set_visible(&mut self, visible: bool) {
    unsafe { ::ffi::qt_gui_c_QWindow_setVisible(self as *mut ::window::Window, visible) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWindow::setWidth(int arg)```</span>
  ///
  ///
  pub fn set_width(&mut self, arg: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QWindow_setWidth(self as *mut ::window::Window, arg) }
  }

  /// C++ method: <span style='color: green;'>```void QWindow::setWindowState(Qt::WindowState state)```</span>
  ///
  ///
  pub fn set_window_state(&mut self, state: &::qt_core::qt::WindowState) {
    unsafe {
      ::ffi::qt_gui_c_QWindow_setWindowState(self as *mut ::window::Window,
                                             state as *const ::qt_core::qt::WindowState)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWindow::setX(int arg)```</span>
  ///
  ///
  pub fn set_x(&mut self, arg: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QWindow_setX(self as *mut ::window::Window, arg) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWindow::setY(int arg)```</span>
  ///
  ///
  pub fn set_y(&mut self, arg: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QWindow_setY(self as *mut ::window::Window, arg) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWindow::show()```</span>
  ///
  ///
  pub fn show(&mut self) {
    unsafe { ::ffi::qt_gui_c_QWindow_show(self as *mut ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWindow::showFullScreen()```</span>
  ///
  ///
  pub fn show_full_screen(&mut self) {
    unsafe { ::ffi::qt_gui_c_QWindow_showFullScreen(self as *mut ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWindow::showMaximized()```</span>
  ///
  ///
  pub fn show_maximized(&mut self) {
    unsafe { ::ffi::qt_gui_c_QWindow_showMaximized(self as *mut ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWindow::showMinimized()```</span>
  ///
  ///
  pub fn show_minimized(&mut self) {
    unsafe { ::ffi::qt_gui_c_QWindow_showMinimized(self as *mut ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QWindow::showNormal()```</span>
  ///
  ///
  pub fn show_normal(&mut self) {
    unsafe { ::ffi::qt_gui_c_QWindow_showNormal(self as *mut ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QWindow::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QWindow_size_to_output(self as *const ::window::Window, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QWindow::sizeIncrement() const```</span>
  ///
  ///
  pub fn size_increment(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QWindow_sizeIncrement_to_output(self as *const ::window::Window, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QSurface::SurfaceType QWindow::surfaceType() const```</span>
  ///
  ///
  pub fn surface_type(&self) -> ::surface::SurfaceType {
    unsafe { ::ffi::qt_gui_c_QWindow_surfaceType(self as *const ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```QString QWindow::title() const```</span>
  ///
  ///
  pub fn title(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QWindow_title_to_output(self as *const ::window::Window, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QWindow::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QWindow_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QWindow::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QWindow_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QWindow* QWindow::transientParent() const```</span>
  ///
  ///
  pub fn transient_parent(&self) -> *mut ::window::Window {
    unsafe { ::ffi::qt_gui_c_QWindow_transientParent(self as *const ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```void QWindow::unsetCursor()```</span>
  ///
  ///
  pub fn unset_cursor(&mut self) {
    unsafe { ::ffi::qt_gui_c_QWindow_unsetCursor(self as *mut ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```QWindow::Visibility QWindow::visibility() const```</span>
  ///
  ///
  pub fn visibility(&self) -> ::window::Visibility {
    unsafe { ::ffi::qt_gui_c_QWindow_visibility(self as *const ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```int QWindow::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QWindow_width(self as *const ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```unsigned long long QWindow::winId() const```</span>
  ///
  ///
  pub fn win_id(&self) -> ::libc::c_ulonglong {
    unsafe { ::ffi::qt_gui_c_QWindow_winId(self as *const ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```int QWindow::x() const```</span>
  ///
  ///
  pub fn x(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QWindow_x(self as *const ::window::Window) }
  }

  /// C++ method: <span style='color: green;'>```int QWindow::y() const```</span>
  ///
  ///
  pub fn y(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QWindow_y(self as *const ::window::Window) }
  }
}

impl ::cpp_utils::CppDeletable for ::window::Window {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QWindow_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Window`.
  pub struct Signals<'a>(&'a ::window::Window);
  /// Represents a built-in Qt signal `QWindow::windowTitleChanged`.
  ///
  /// An object of this type can be created from `Window` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct WindowTitleChanged<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for WindowTitleChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2windowTitleChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WindowTitleChanged<'a> {}
  /// Represents a built-in Qt signal `QWindow::visibleChanged`.
  ///
  /// An object of this type can be created from `Window` with `object.signals().visible_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct VisibleChanged<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for VisibleChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2visibleChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for VisibleChanged<'a> {}
  /// Represents a built-in Qt signal `QWindow::widthChanged`.
  ///
  /// An object of this type can be created from `Window` with `object.signals().width_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct WidthChanged<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for WidthChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2widthChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WidthChanged<'a> {}
  /// Represents a built-in Qt signal `QWindow::objectNameChanged`.
  ///
  /// An object of this type can be created from `Window` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct ObjectNameChanged<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ObjectNameChanged<'a> {}
  /// Represents a built-in Qt signal `QWindow::visibilityChanged`.
  ///
  /// An object of this type can be created from `Window` with `object.signals().visibility_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct VisibilityChanged<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for VisibilityChanged<'a> {
    type Arguments = (&'static ::window::Visibility,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2visibilityChanged(QWindow::Visibility)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for VisibilityChanged<'a> {}
  /// Represents a built-in Qt signal `QWindow::windowStateChanged`.
  ///
  /// An object of this type can be created from `Window` with `object.signals().window_state_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct WindowStateChanged<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for WindowStateChanged<'a> {
    type Arguments = (&'static ::qt_core::qt::WindowState,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2windowStateChanged(Qt::WindowState)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WindowStateChanged<'a> {}
  /// Represents a built-in Qt signal `QWindow::heightChanged`.
  ///
  /// An object of this type can be created from `Window` with `object.signals().height_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct HeightChanged<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for HeightChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2heightChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for HeightChanged<'a> {}
  /// Represents a built-in Qt signal `QWindow::contentOrientationChanged`.
  ///
  /// An object of this type can be created from `Window` with `object.signals().content_orientation_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct ContentOrientationChanged<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for ContentOrientationChanged<'a> {
    type Arguments = (&'static ::qt_core::qt::ScreenOrientation,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2contentOrientationChanged(Qt::ScreenOrientation)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ContentOrientationChanged<'a> {}
  /// Represents a built-in Qt signal `QWindow::focusObjectChanged`.
  ///
  /// An object of this type can be created from `Window` with `object.signals().focus_object_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct FocusObjectChanged<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for FocusObjectChanged<'a> {
    type Arguments = (*mut ::qt_core::object::Object,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2focusObjectChanged(QObject*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FocusObjectChanged<'a> {}
  /// Represents a built-in Qt signal `QWindow::minimumHeightChanged`.
  ///
  /// An object of this type can be created from `Window` with `object.signals().minimum_height_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct MinimumHeightChanged<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for MinimumHeightChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2minimumHeightChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MinimumHeightChanged<'a> {}
  /// Represents a built-in Qt signal `QWindow::opacityChanged`.
  ///
  /// An object of this type can be created from `Window` with `object.signals().opacity_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct OpacityChanged<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for OpacityChanged<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2opacityChanged(double)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for OpacityChanged<'a> {}
  /// Represents a built-in Qt signal `QWindow::activeChanged`.
  ///
  /// An object of this type can be created from `Window` with `object.signals().active_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct ActiveChanged<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for ActiveChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2activeChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ActiveChanged<'a> {}
  /// Represents a built-in Qt signal `QWindow::minimumWidthChanged`.
  ///
  /// An object of this type can be created from `Window` with `object.signals().minimum_width_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct MinimumWidthChanged<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for MinimumWidthChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2minimumWidthChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MinimumWidthChanged<'a> {}
  /// Represents a built-in Qt signal `QWindow::yChanged`.
  ///
  /// An object of this type can be created from `Window` with `object.signals().y_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct YChanged<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for YChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2yChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for YChanged<'a> {}
  /// Represents a built-in Qt signal `QWindow::maximumHeightChanged`.
  ///
  /// An object of this type can be created from `Window` with `object.signals().maximum_height_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct MaximumHeightChanged<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for MaximumHeightChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2maximumHeightChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MaximumHeightChanged<'a> {}
  /// Represents a built-in Qt signal `QWindow::modalityChanged`.
  ///
  /// An object of this type can be created from `Window` with `object.signals().modality_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct ModalityChanged<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for ModalityChanged<'a> {
    type Arguments = (&'static ::qt_core::qt::WindowModality,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2modalityChanged(Qt::WindowModality)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ModalityChanged<'a> {}
  /// Represents a built-in Qt signal `QWindow::xChanged`.
  ///
  /// An object of this type can be created from `Window` with `object.signals().x_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct XChanged<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for XChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2xChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for XChanged<'a> {}
  /// Represents a built-in Qt signal `QWindow::maximumWidthChanged`.
  ///
  /// An object of this type can be created from `Window` with `object.signals().maximum_width_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct MaximumWidthChanged<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for MaximumWidthChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2maximumWidthChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MaximumWidthChanged<'a> {}
  /// Represents a built-in Qt signal `QWindow::screenChanged`.
  ///
  /// An object of this type can be created from `Window` with `object.signals().screen_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct ScreenChanged<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for ScreenChanged<'a> {
    type Arguments = (*mut ::screen::Screen,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2screenChanged(QScreen*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ScreenChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QWindow::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWindow::visibleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn visible_changed(&self) -> VisibleChanged {
      VisibleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWindow::widthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn width_changed(&self) -> WidthChanged {
      WidthChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWindow::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWindow::visibilityChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn visibility_changed(&self) -> VisibilityChanged {
      VisibilityChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWindow::windowStateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_state_changed(&self) -> WindowStateChanged {
      WindowStateChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWindow::heightChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn height_changed(&self) -> HeightChanged {
      HeightChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWindow::contentOrientationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn content_orientation_changed(&self) -> ContentOrientationChanged {
      ContentOrientationChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWindow::focusObjectChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn focus_object_changed(&self) -> FocusObjectChanged {
      FocusObjectChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWindow::minimumHeightChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn minimum_height_changed(&self) -> MinimumHeightChanged {
      MinimumHeightChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWindow::opacityChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn opacity_changed(&self) -> OpacityChanged {
      OpacityChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWindow::activeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn active_changed(&self) -> ActiveChanged {
      ActiveChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWindow::minimumWidthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn minimum_width_changed(&self) -> MinimumWidthChanged {
      MinimumWidthChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWindow::yChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn y_changed(&self) -> YChanged {
      YChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWindow::maximumHeightChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn maximum_height_changed(&self) -> MaximumHeightChanged {
      MaximumHeightChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWindow::modalityChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn modality_changed(&self) -> ModalityChanged {
      ModalityChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWindow::xChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn x_changed(&self) -> XChanged {
      XChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWindow::maximumWidthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn maximum_width_changed(&self) -> MaximumWidthChanged {
      MaximumWidthChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QWindow::screenChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn screen_changed(&self) -> ScreenChanged {
      ScreenChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Window`.
  pub struct Slots<'a>(&'a ::window::Window);
  /// Represents a built-in Qt slot `QWindow::showMaximized`.
  ///
  /// An object of this type can be created from `Window` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct ShowMaximized<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QWindow::lower`.
  ///
  /// An object of this type can be created from `Window` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct Lower<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  /// Represents a built-in Qt slot `QWindow::showMinimized`.
  ///
  /// An object of this type can be created from `Window` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct ShowMinimized<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QWindow::setMaximumHeight`.
  ///
  /// An object of this type can be created from `Window` with `object.slots().set_maximum_height()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct SetMaximumHeight<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for SetMaximumHeight<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMaximumHeight(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QWindow::setWidth`.
  ///
  /// An object of this type can be created from `Window` with `object.slots().set_width()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct SetWidth<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for SetWidth<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWidth(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QWindow::show`.
  ///
  /// An object of this type can be created from `Window` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct Show<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QWindow::raise`.
  ///
  /// An object of this type can be created from `Window` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct Raise<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QWindow::setHeight`.
  ///
  /// An object of this type can be created from `Window` with `object.slots().set_height()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct SetHeight<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for SetHeight<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHeight(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QWindow::setTitle`.
  ///
  /// An object of this type can be created from `Window` with `object.slots().set_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct SetTitle<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for SetTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QWindow::alert`.
  ///
  /// An object of this type can be created from `Window` with `object.slots().alert()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct Alert<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for Alert<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1alert(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QWindow::showNormal`.
  ///
  /// An object of this type can be created from `Window` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct ShowNormal<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QWindow::setMaximumWidth`.
  ///
  /// An object of this type can be created from `Window` with `object.slots().set_maximum_width()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct SetMaximumWidth<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for SetMaximumWidth<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMaximumWidth(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QWindow::setY`.
  ///
  /// An object of this type can be created from `Window` with `object.slots().set_y()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct SetY<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for SetY<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setY(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QWindow::setVisible`.
  ///
  /// An object of this type can be created from `Window` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct SetVisible<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QWindow::setX`.
  ///
  /// An object of this type can be created from `Window` with `object.slots().set_x()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct SetX<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for SetX<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setX(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QWindow::requestActivate`.
  ///
  /// An object of this type can be created from `Window` with `object.slots().request_activate()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct RequestActivate<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for RequestActivate<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1requestActivate()\0"
    }
  }
  /// Represents a built-in Qt slot `QWindow::hide`.
  ///
  /// An object of this type can be created from `Window` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct Hide<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QWindow::showFullScreen`.
  ///
  /// An object of this type can be created from `Window` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct ShowFullScreen<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `QWindow::requestUpdate`.
  ///
  /// An object of this type can be created from `Window` with `object.slots().request_update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct RequestUpdate<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for RequestUpdate<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1requestUpdate()\0"
    }
  }
  /// Represents a built-in Qt slot `QWindow::setMinimumWidth`.
  ///
  /// An object of this type can be created from `Window` with `object.slots().set_minimum_width()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct SetMinimumWidth<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for SetMinimumWidth<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMinimumWidth(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QWindow::close`.
  ///
  /// An object of this type can be created from `Window` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct Close<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QWindow::setMinimumHeight`.
  ///
  /// An object of this type can be created from `Window` with `object.slots().set_minimum_height()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Window` object.
  pub struct SetMinimumHeight<'a>(&'a ::window::Window);
  impl<'a> ::qt_core::connection::Receiver for SetMinimumHeight<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMinimumHeight(int)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QWindow::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWindow::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWindow::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWindow::setMaximumHeight`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_maximum_height(&self) -> SetMaximumHeight {
      SetMaximumHeight(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWindow::setWidth`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_width(&self) -> SetWidth {
      SetWidth(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWindow::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWindow::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWindow::setHeight`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_height(&self) -> SetHeight {
      SetHeight(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWindow::setTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_title(&self) -> SetTitle {
      SetTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWindow::alert`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn alert(&self) -> Alert {
      Alert(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWindow::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWindow::setMaximumWidth`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_maximum_width(&self) -> SetMaximumWidth {
      SetMaximumWidth(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWindow::setY`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_y(&self) -> SetY {
      SetY(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWindow::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWindow::setX`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_x(&self) -> SetX {
      SetX(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWindow::requestActivate`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn request_activate(&self) -> RequestActivate {
      RequestActivate(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWindow::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWindow::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWindow::requestUpdate`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn request_update(&self) -> RequestUpdate {
      RequestUpdate(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWindow::setMinimumWidth`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_minimum_width(&self) -> SetMinimumWidth {
      SetMinimumWidth(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWindow::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QWindow::setMinimumHeight`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_minimum_height(&self) -> SetMinimumHeight {
      SetMinimumHeight(self.0)
    }
  }
  impl ::window::Window {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
    /// Provides access to built-in Qt slots of this type
    pub fn slots(&self) -> Slots {
      Slots(self)
    }
  }

}

/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QWindow* arg2)```</span>
///
///
pub unsafe fn op_shl(arg1: &::qt_core::debug::Debug, arg2: *const ::window::Window) -> ::qt_core::debug::Debug {
  {
    let mut object: ::qt_core::debug::Debug = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
    ::ffi::qt_gui_c_QWindow_G_operator_shl_to_output(arg1 as *const ::qt_core::debug::Debug, arg2, &mut object);
    object
  }
}

impl ::cpp_utils::DynamicCast<::window::Window> for ::surface::Surface {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::window::Window> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QWindow_G_dynamic_cast_QWindow_ptr(self as *mut ::surface::Surface) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::window::Window> {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QWindow_G_dynamic_cast_QWindow_ptr(self as *const ::surface::Surface as *mut ::surface::Surface)
      };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::window::Window {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QWindow_G_static_cast_QObject_ptr(self as *mut ::window::Window) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QWindow_G_static_cast_QObject_ptr(self as *const ::window::Window as *mut ::window::Window)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::surface::Surface> for ::window::Window {
  fn static_cast_mut(&mut self) -> &mut ::surface::Surface {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QWindow_G_static_cast_QSurface_ptr(self as *mut ::window::Window) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::surface::Surface {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QWindow_G_static_cast_QSurface_ptr(self as *const ::window::Window as *mut ::window::Window)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::window::Window> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::window::Window {
    let ffi_result =
      ::ffi::qt_gui_c_QWindow_G_static_cast_QWindow_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::window::Window {
    let ffi_result = ::ffi::qt_gui_c_QWindow_G_static_cast_QWindow_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::window::Window> for ::surface::Surface {
  unsafe fn static_cast_mut(&mut self) -> &mut ::window::Window {
    let ffi_result = ::ffi::qt_gui_c_QWindow_G_static_cast_QWindow_ptr_QSurface(self as *mut ::surface::Surface);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::window::Window {
    let ffi_result = ::ffi::qt_gui_c_QWindow_G_static_cast_QWindow_ptr_QSurface(self as *const ::surface::Surface as *mut ::surface::Surface);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Window::is_ancestor_of](../struct.Window.html#method.is_ancestor_of) method.
  pub trait WindowIsAncestorOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::window::Window) -> bool;
  }
  impl<'largs> WindowIsAncestorOfArgs<'largs> for *const ::window::Window {
    unsafe fn exec(self, original_self: &'largs ::window::Window) -> bool {
      let child = self;
      ::ffi::qt_gui_c_QWindow_isAncestorOf_child(original_self as *const ::window::Window, child)
    }
  }
  impl<'largs> WindowIsAncestorOfArgs<'largs> for (*const ::window::Window, ::window::AncestorMode) {
    unsafe fn exec(self, original_self: &'largs ::window::Window) -> bool {
      let child = self.0;
      let mode = self.1;
      ::ffi::qt_gui_c_QWindow_isAncestorOf_child_mode(original_self as *const ::window::Window, child, mode)
    }
  }
  /// This trait represents a set of arguments accepted by [Window::new_unsafe](../struct.Window.html#method.new_unsafe) method.
  pub trait WindowNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::window::Window>;
  }
  impl WindowNewUnsafeArgs for *mut ::window::Window {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::window::Window> {
      let parent = self;
      let ffi_result = ::ffi::qt_gui_c_QWindow_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl WindowNewUnsafeArgs for *mut ::screen::Screen {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::window::Window> {
      let screen = self;
      let ffi_result = ::ffi::qt_gui_c_QWindow_new_screen(screen);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [Window::parent](../struct.Window.html#method.parent) method.
  pub trait WindowParentArgs<'largs> {
    fn exec(self, original_self: &'largs ::window::Window) -> *mut ::window::Window;
  }
  impl<'largs> WindowParentArgs<'largs> for ::window::AncestorMode {
    fn exec(self, original_self: &'largs ::window::Window) -> *mut ::window::Window {
      let mode = self;
      unsafe { ::ffi::qt_gui_c_QWindow_parent_mode(original_self as *const ::window::Window, mode) }
    }
  }
  impl<'largs> WindowParentArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::window::Window) -> *mut ::window::Window {

      unsafe { ::ffi::qt_gui_c_QWindow_parent_no_args(original_self as *const ::window::Window) }
    }
  }
  /// This trait represents a set of arguments accepted by [Window::resize](../struct.Window.html#method.resize) method.
  pub trait WindowResizeArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::window::Window) -> ();
  }
  impl<'largs> WindowResizeArgs<'largs> for &'largs ::qt_core::size::Size {
    fn exec(self, original_self: &'largs mut ::window::Window) -> () {
      let new_size = self;
      unsafe {
        ::ffi::qt_gui_c_QWindow_resize_newSize(original_self as *mut ::window::Window,
                                               new_size as *const ::qt_core::size::Size)
      }
    }
  }
  impl<'largs> WindowResizeArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::window::Window) -> () {
      let w = self.0;
      let h = self.1;
      unsafe { ::ffi::qt_gui_c_QWindow_resize_w_h(original_self as *mut ::window::Window, w, h) }
    }
  }
  /// This trait represents a set of arguments accepted by [Window::set_flag](../struct.Window.html#method.set_flag) method.
  pub trait WindowSetFlagArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::window::Window) -> ();
  }
  impl<'largs> WindowSetFlagArgs<'largs> for &'largs ::qt_core::qt::WindowType {
    fn exec(self, original_self: &'largs mut ::window::Window) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_gui_c_QWindow_setFlag_arg1(original_self as *mut ::window::Window,
                                             arg1 as *const ::qt_core::qt::WindowType)
      }
    }
  }
  impl<'largs> WindowSetFlagArgs<'largs> for (&'largs ::qt_core::qt::WindowType, bool) {
    fn exec(self, original_self: &'largs mut ::window::Window) -> () {
      let arg1 = self.0;
      let on = self.1;
      unsafe {
        ::ffi::qt_gui_c_QWindow_setFlag_arg1_on(original_self as *mut ::window::Window,
                                                arg1 as *const ::qt_core::qt::WindowType,
                                                on)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Window::set_geometry](../struct.Window.html#method.set_geometry) method.
  pub trait WindowSetGeometryArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::window::Window) -> ();
  }
  impl<'largs> WindowSetGeometryArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::window::Window) -> () {
      let posx = self.0;
      let posy = self.1;
      let w = self.2;
      let h = self.3;
      unsafe {
        ::ffi::qt_gui_c_QWindow_setGeometry_posx_posy_w_h(original_self as *mut ::window::Window, posx, posy, w, h)
      }
    }
  }
  impl<'largs> WindowSetGeometryArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs mut ::window::Window) -> () {
      let rect = self;
      unsafe {
        ::ffi::qt_gui_c_QWindow_setGeometry_rect(original_self as *mut ::window::Window,
                                                 rect as *const ::qt_core::rect::Rect)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Window::set_position](../struct.Window.html#method.set_position) method.
  pub trait WindowSetPositionArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::window::Window) -> ();
  }
  impl<'largs> WindowSetPositionArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::window::Window) -> () {
      let posx = self.0;
      let posy = self.1;
      unsafe { ::ffi::qt_gui_c_QWindow_setPosition_posx_posy(original_self as *mut ::window::Window, posx, posy) }
    }
  }
  impl<'largs> WindowSetPositionArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs mut ::window::Window) -> () {
      let pt = self;
      unsafe {
        ::ffi::qt_gui_c_QWindow_setPosition_pt(original_self as *mut ::window::Window,
                                               pt as *const ::qt_core::point::Point)
      }
    }
  }
}

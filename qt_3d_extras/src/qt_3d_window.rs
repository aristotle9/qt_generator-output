/// C++ type: <span style='color: green;'>```Qt3DExtras::Qt3DWindow```</span>
#[repr(C)]
pub struct Qt3DWindow(u8);

impl Qt3DWindow {
  /// C++ method: <span style='color: green;'>```Qt3DRender::QFrameGraphNode* Qt3DExtras::Qt3DWindow::activeFrameGraph() const```</span>
  ///
  ///
  pub fn active_frame_graph(&self) -> *mut ::qt_3d_render::frame_graph_node::FrameGraphNode {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_Qt3DWindow_activeFrameGraph(self as *const ::qt_3d_window::Qt3DWindow) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QCamera* Qt3DExtras::Qt3DWindow::camera() const```</span>
  ///
  ///
  pub fn camera(&self) -> *mut ::qt_3d_render::camera::Camera {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_Qt3DWindow_camera(self as *const ::qt_3d_window::Qt3DWindow) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DExtras::QForwardRenderer* Qt3DExtras::Qt3DWindow::defaultFrameGraph() const```</span>
  ///
  ///
  pub fn default_frame_graph(&self) -> *mut ::forward_renderer::ForwardRenderer {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_Qt3DWindow_defaultFrameGraph(self as *const ::qt_3d_window::Qt3DWindow) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::Qt3DWindow::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_Qt3DWindow_metaObject(self as *const ::qt_3d_window::Qt3DWindow) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::Qt3DWindow::Qt3DWindow()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::qt_3d_window::Qt3DWindow> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_Qt3DWindow_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::Qt3DWindow::Qt3DWindow(QScreen* screen = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(screen: *mut ::qt_gui::screen::Screen) -> ::cpp_utils::CppBox<::qt_3d_window::Qt3DWindow> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_Qt3DWindow_new_screen(screen);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::Qt3DWindow::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_Qt3DWindow_qt_metacall(self as *mut ::qt_3d_window::Qt3DWindow,
                                                            arg1 as *const ::qt_core::meta_object::Call,
                                                            arg2,
                                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::Qt3DWindow::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_Qt3DWindow_qt_metacast(self as *mut ::qt_3d_window::Qt3DWindow, arg1)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::Qt3DWindow::registerAspect(const QString& name)```</span>
  ///
  ///
  pub fn register_aspect(&mut self, name: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_Qt3DWindow_registerAspect_name(self as *mut ::qt_3d_window::Qt3DWindow,
                                                                      name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::Qt3DWindow::registerAspect(Qt3DCore::QAbstractAspect* aspect)```</span>
  ///
  ///
  pub unsafe fn register_aspect_unsafe(&mut self, aspect: *mut ::qt_3d_core::abstract_aspect::AbstractAspect) {
    ::ffi::qt_3d_extras_c_Qt3DExtras_Qt3DWindow_registerAspect_aspect(self as *mut ::qt_3d_window::Qt3DWindow, aspect)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderSettings* Qt3DExtras::Qt3DWindow::renderSettings() const```</span>
  ///
  ///
  pub fn render_settings(&self) -> *mut ::qt_3d_render::render_settings::RenderSettings {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_Qt3DWindow_renderSettings(self as *const ::qt_3d_window::Qt3DWindow) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::Qt3DWindow::setActiveFrameGraph(Qt3DRender::QFrameGraphNode* activeFrameGraph)```</span>
  ///
  ///
  pub unsafe fn set_active_frame_graph(&mut self,
                                       active_frame_graph: *mut ::qt_3d_render::frame_graph_node::FrameGraphNode) {
    ::ffi::qt_3d_extras_c_Qt3DExtras_Qt3DWindow_setActiveFrameGraph(self as *mut ::qt_3d_window::Qt3DWindow,
                                                                    active_frame_graph)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::Qt3DWindow::setRootEntity(Qt3DCore::QEntity* root)```</span>
  ///
  ///
  pub unsafe fn set_root_entity(&mut self, root: *mut ::qt_3d_core::entity::Entity) {
    ::ffi::qt_3d_extras_c_Qt3DExtras_Qt3DWindow_setRootEntity(self as *mut ::qt_3d_window::Qt3DWindow, root)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::Qt3DWindow::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_Qt3DWindow_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::Qt3DWindow::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_Qt3DWindow_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::qt_3d_window::Qt3DWindow {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_Qt3DWindow_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Qt3DWindow`.
  pub struct Signals<'a>(&'a ::qt_3d_window::Qt3DWindow);
  /// Represents a built-in Qt signal `Qt3DExtras::Qt3DWindow::modalityChanged`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.signals().modality_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct ModalityChanged<'a>(&'a ::qt_3d_window::Qt3DWindow);
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
  /// Represents a built-in Qt signal `Qt3DExtras::Qt3DWindow::opacityChanged`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.signals().opacity_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct OpacityChanged<'a>(&'a ::qt_3d_window::Qt3DWindow);
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
  /// Represents a built-in Qt signal `Qt3DExtras::Qt3DWindow::visibleChanged`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.signals().visible_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct VisibleChanged<'a>(&'a ::qt_3d_window::Qt3DWindow);
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
  /// Represents a built-in Qt signal `Qt3DExtras::Qt3DWindow::minimumWidthChanged`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.signals().minimum_width_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct MinimumWidthChanged<'a>(&'a ::qt_3d_window::Qt3DWindow);
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
  /// Represents a built-in Qt signal `Qt3DExtras::Qt3DWindow::windowTitleChanged`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct WindowTitleChanged<'a>(&'a ::qt_3d_window::Qt3DWindow);
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
  /// Represents a built-in Qt signal `Qt3DExtras::Qt3DWindow::maximumWidthChanged`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.signals().maximum_width_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct MaximumWidthChanged<'a>(&'a ::qt_3d_window::Qt3DWindow);
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
  /// Represents a built-in Qt signal `Qt3DExtras::Qt3DWindow::maximumHeightChanged`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.signals().maximum_height_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct MaximumHeightChanged<'a>(&'a ::qt_3d_window::Qt3DWindow);
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
  /// Represents a built-in Qt signal `Qt3DExtras::Qt3DWindow::activeChanged`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.signals().active_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct ActiveChanged<'a>(&'a ::qt_3d_window::Qt3DWindow);
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
  /// Represents a built-in Qt signal `Qt3DExtras::Qt3DWindow::windowStateChanged`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.signals().window_state_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct WindowStateChanged<'a>(&'a ::qt_3d_window::Qt3DWindow);
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
  /// Represents a built-in Qt signal `Qt3DExtras::Qt3DWindow::heightChanged`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.signals().height_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct HeightChanged<'a>(&'a ::qt_3d_window::Qt3DWindow);
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
  /// Represents a built-in Qt signal `Qt3DExtras::Qt3DWindow::widthChanged`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.signals().width_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct WidthChanged<'a>(&'a ::qt_3d_window::Qt3DWindow);
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
  /// Represents a built-in Qt signal `Qt3DExtras::Qt3DWindow::focusObjectChanged`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.signals().focus_object_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct FocusObjectChanged<'a>(&'a ::qt_3d_window::Qt3DWindow);
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
  /// Represents a built-in Qt signal `Qt3DExtras::Qt3DWindow::visibilityChanged`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.signals().visibility_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct VisibilityChanged<'a>(&'a ::qt_3d_window::Qt3DWindow);
  impl<'a> ::qt_core::connection::Receiver for VisibilityChanged<'a> {
    type Arguments = (&'static ::qt_gui::window::Visibility,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2visibilityChanged(QWindow::Visibility)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for VisibilityChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::Qt3DWindow::minimumHeightChanged`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.signals().minimum_height_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct MinimumHeightChanged<'a>(&'a ::qt_3d_window::Qt3DWindow);
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
  /// Represents a built-in Qt signal `Qt3DExtras::Qt3DWindow::xChanged`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.signals().x_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct XChanged<'a>(&'a ::qt_3d_window::Qt3DWindow);
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
  /// Represents a built-in Qt signal `Qt3DExtras::Qt3DWindow::screenChanged`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.signals().screen_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct ScreenChanged<'a>(&'a ::qt_3d_window::Qt3DWindow);
  impl<'a> ::qt_core::connection::Receiver for ScreenChanged<'a> {
    type Arguments = (*mut ::qt_gui::screen::Screen,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2screenChanged(QScreen*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ScreenChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::Qt3DWindow::contentOrientationChanged`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.signals().content_orientation_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct ContentOrientationChanged<'a>(&'a ::qt_3d_window::Qt3DWindow);
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
  /// Represents a built-in Qt signal `Qt3DExtras::Qt3DWindow::yChanged`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.signals().y_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct YChanged<'a>(&'a ::qt_3d_window::Qt3DWindow);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::Qt3DWindow::modalityChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn modality_changed(&self) -> ModalityChanged {
      ModalityChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::Qt3DWindow::opacityChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn opacity_changed(&self) -> OpacityChanged {
      OpacityChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::Qt3DWindow::visibleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn visible_changed(&self) -> VisibleChanged {
      VisibleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::Qt3DWindow::minimumWidthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn minimum_width_changed(&self) -> MinimumWidthChanged {
      MinimumWidthChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::Qt3DWindow::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::Qt3DWindow::maximumWidthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn maximum_width_changed(&self) -> MaximumWidthChanged {
      MaximumWidthChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::Qt3DWindow::maximumHeightChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn maximum_height_changed(&self) -> MaximumHeightChanged {
      MaximumHeightChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::Qt3DWindow::activeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn active_changed(&self) -> ActiveChanged {
      ActiveChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::Qt3DWindow::windowStateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_state_changed(&self) -> WindowStateChanged {
      WindowStateChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::Qt3DWindow::heightChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn height_changed(&self) -> HeightChanged {
      HeightChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::Qt3DWindow::widthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn width_changed(&self) -> WidthChanged {
      WidthChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::Qt3DWindow::focusObjectChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn focus_object_changed(&self) -> FocusObjectChanged {
      FocusObjectChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::Qt3DWindow::visibilityChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn visibility_changed(&self) -> VisibilityChanged {
      VisibilityChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::Qt3DWindow::minimumHeightChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn minimum_height_changed(&self) -> MinimumHeightChanged {
      MinimumHeightChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::Qt3DWindow::xChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn x_changed(&self) -> XChanged {
      XChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::Qt3DWindow::screenChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn screen_changed(&self) -> ScreenChanged {
      ScreenChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::Qt3DWindow::contentOrientationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn content_orientation_changed(&self) -> ContentOrientationChanged {
      ContentOrientationChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::Qt3DWindow::yChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn y_changed(&self) -> YChanged {
      YChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Qt3DWindow`.
  pub struct Slots<'a>(&'a ::qt_3d_window::Qt3DWindow);
  /// Represents a built-in Qt slot `Qt3DExtras::Qt3DWindow::requestUpdate`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.slots().request_update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct RequestUpdate<'a>(&'a ::qt_3d_window::Qt3DWindow);
  impl<'a> ::qt_core::connection::Receiver for RequestUpdate<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1requestUpdate()\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::Qt3DWindow::setMinimumHeight`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.slots().set_minimum_height()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct SetMinimumHeight<'a>(&'a ::qt_3d_window::Qt3DWindow);
  impl<'a> ::qt_core::connection::Receiver for SetMinimumHeight<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMinimumHeight(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::Qt3DWindow::setMaximumHeight`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.slots().set_maximum_height()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct SetMaximumHeight<'a>(&'a ::qt_3d_window::Qt3DWindow);
  impl<'a> ::qt_core::connection::Receiver for SetMaximumHeight<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMaximumHeight(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::Qt3DWindow::show`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct Show<'a>(&'a ::qt_3d_window::Qt3DWindow);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::Qt3DWindow::setMinimumWidth`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.slots().set_minimum_width()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct SetMinimumWidth<'a>(&'a ::qt_3d_window::Qt3DWindow);
  impl<'a> ::qt_core::connection::Receiver for SetMinimumWidth<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMinimumWidth(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::Qt3DWindow::setVisible`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct SetVisible<'a>(&'a ::qt_3d_window::Qt3DWindow);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::Qt3DWindow::hide`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct Hide<'a>(&'a ::qt_3d_window::Qt3DWindow);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::Qt3DWindow::showMaximized`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct ShowMaximized<'a>(&'a ::qt_3d_window::Qt3DWindow);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::Qt3DWindow::showFullScreen`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct ShowFullScreen<'a>(&'a ::qt_3d_window::Qt3DWindow);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::Qt3DWindow::requestActivate`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.slots().request_activate()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct RequestActivate<'a>(&'a ::qt_3d_window::Qt3DWindow);
  impl<'a> ::qt_core::connection::Receiver for RequestActivate<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1requestActivate()\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::Qt3DWindow::lower`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct Lower<'a>(&'a ::qt_3d_window::Qt3DWindow);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::Qt3DWindow::setTitle`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.slots().set_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct SetTitle<'a>(&'a ::qt_3d_window::Qt3DWindow);
  impl<'a> ::qt_core::connection::Receiver for SetTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::Qt3DWindow::setMaximumWidth`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.slots().set_maximum_width()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct SetMaximumWidth<'a>(&'a ::qt_3d_window::Qt3DWindow);
  impl<'a> ::qt_core::connection::Receiver for SetMaximumWidth<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMaximumWidth(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::Qt3DWindow::alert`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.slots().alert()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct Alert<'a>(&'a ::qt_3d_window::Qt3DWindow);
  impl<'a> ::qt_core::connection::Receiver for Alert<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1alert(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::Qt3DWindow::close`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct Close<'a>(&'a ::qt_3d_window::Qt3DWindow);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::Qt3DWindow::showMinimized`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct ShowMinimized<'a>(&'a ::qt_3d_window::Qt3DWindow);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::Qt3DWindow::setWidth`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.slots().set_width()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct SetWidth<'a>(&'a ::qt_3d_window::Qt3DWindow);
  impl<'a> ::qt_core::connection::Receiver for SetWidth<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWidth(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::Qt3DWindow::setHeight`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.slots().set_height()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct SetHeight<'a>(&'a ::qt_3d_window::Qt3DWindow);
  impl<'a> ::qt_core::connection::Receiver for SetHeight<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHeight(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::Qt3DWindow::showNormal`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct ShowNormal<'a>(&'a ::qt_3d_window::Qt3DWindow);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::Qt3DWindow::setX`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.slots().set_x()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct SetX<'a>(&'a ::qt_3d_window::Qt3DWindow);
  impl<'a> ::qt_core::connection::Receiver for SetX<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setX(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::Qt3DWindow::setY`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.slots().set_y()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct SetY<'a>(&'a ::qt_3d_window::Qt3DWindow);
  impl<'a> ::qt_core::connection::Receiver for SetY<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setY(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::Qt3DWindow::raise`.
  ///
  /// An object of this type can be created from `Qt3DWindow` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Qt3DWindow` object.
  pub struct Raise<'a>(&'a ::qt_3d_window::Qt3DWindow);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::Qt3DWindow::requestUpdate`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn request_update(&self) -> RequestUpdate {
      RequestUpdate(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::Qt3DWindow::setMinimumHeight`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_minimum_height(&self) -> SetMinimumHeight {
      SetMinimumHeight(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::Qt3DWindow::setMaximumHeight`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_maximum_height(&self) -> SetMaximumHeight {
      SetMaximumHeight(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::Qt3DWindow::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::Qt3DWindow::setMinimumWidth`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_minimum_width(&self) -> SetMinimumWidth {
      SetMinimumWidth(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::Qt3DWindow::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::Qt3DWindow::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::Qt3DWindow::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::Qt3DWindow::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::Qt3DWindow::requestActivate`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn request_activate(&self) -> RequestActivate {
      RequestActivate(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::Qt3DWindow::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::Qt3DWindow::setTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_title(&self) -> SetTitle {
      SetTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::Qt3DWindow::setMaximumWidth`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_maximum_width(&self) -> SetMaximumWidth {
      SetMaximumWidth(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::Qt3DWindow::alert`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn alert(&self) -> Alert {
      Alert(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::Qt3DWindow::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::Qt3DWindow::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::Qt3DWindow::setWidth`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_width(&self) -> SetWidth {
      SetWidth(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::Qt3DWindow::setHeight`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_height(&self) -> SetHeight {
      SetHeight(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::Qt3DWindow::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::Qt3DWindow::setX`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_x(&self) -> SetX {
      SetX(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::Qt3DWindow::setY`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_y(&self) -> SetY {
      SetY(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::Qt3DWindow::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
  }
  impl ::qt_3d_window::Qt3DWindow {
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

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::qt_3d_window::Qt3DWindow {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_extras_c_Qt3DWindow_G_static_cast_QObject_ptr(self as *mut ::qt_3d_window::Qt3DWindow) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DWindow_G_static_cast_QObject_ptr(self as *const ::qt_3d_window::Qt3DWindow as *mut ::qt_3d_window::Qt3DWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::surface::Surface> for ::qt_3d_window::Qt3DWindow {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::surface::Surface {
    let ffi_result =
      unsafe { ::ffi::qt_3d_extras_c_Qt3DWindow_G_static_cast_QSurface_ptr(self as *mut ::qt_3d_window::Qt3DWindow) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::surface::Surface {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DWindow_G_static_cast_QSurface_ptr(self as *const ::qt_3d_window::Qt3DWindow as *mut ::qt_3d_window::Qt3DWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::window::Window> for ::qt_3d_window::Qt3DWindow {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::window::Window {
    let ffi_result =
      unsafe { ::ffi::qt_3d_extras_c_Qt3DWindow_G_static_cast_QWindow_ptr(self as *mut ::qt_3d_window::Qt3DWindow) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::window::Window {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DWindow_G_static_cast_QWindow_ptr(self as *const ::qt_3d_window::Qt3DWindow as *mut ::qt_3d_window::Qt3DWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::qt_3d_window::Qt3DWindow> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::qt_3d_window::Qt3DWindow {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DWindow_G_static_cast_Qt3DExtras_Qt3DWindow_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::qt_3d_window::Qt3DWindow {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DWindow_G_static_cast_Qt3DExtras_Qt3DWindow_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::qt_3d_window::Qt3DWindow> for ::qt_gui::surface::Surface {
  unsafe fn static_cast_mut(&mut self) -> &mut ::qt_3d_window::Qt3DWindow {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DWindow_G_static_cast_Qt3DExtras_Qt3DWindow_ptr_QSurface(self as *mut ::qt_gui::surface::Surface);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::qt_3d_window::Qt3DWindow {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DWindow_G_static_cast_Qt3DExtras_Qt3DWindow_ptr_QSurface(self as *const ::qt_gui::surface::Surface as *mut ::qt_gui::surface::Surface);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::qt_3d_window::Qt3DWindow> for ::qt_gui::window::Window {
  unsafe fn static_cast_mut(&mut self) -> &mut ::qt_3d_window::Qt3DWindow {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DWindow_G_static_cast_Qt3DExtras_Qt3DWindow_ptr_QWindow(self as *mut ::qt_gui::window::Window);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::qt_3d_window::Qt3DWindow {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DWindow_G_static_cast_Qt3DExtras_Qt3DWindow_ptr_QWindow(self as *const ::qt_gui::window::Window as *mut ::qt_gui::window::Window);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::qt_3d_window::Qt3DWindow {
  type Target = ::qt_gui::window::Window;
  fn deref(&self) -> &::qt_gui::window::Window {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DWindow_G_static_cast_QWindow_ptr(self as *const ::qt_3d_window::Qt3DWindow as *mut ::qt_3d_window::Qt3DWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::qt_3d_window::Qt3DWindow {
  fn deref_mut(&mut self) -> &mut ::qt_gui::window::Window {
    let ffi_result =
      unsafe { ::ffi::qt_3d_extras_c_Qt3DWindow_G_static_cast_QWindow_ptr(self as *mut ::qt_3d_window::Qt3DWindow) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

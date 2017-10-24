/// C++ type: <span style='color: green;'>```QScroller::Input```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Input {
  /// C++ enum variant: <span style='color: green;'>```InputPress = 1```</span>
  Press = 1,
  /// C++ enum variant: <span style='color: green;'>```InputMove = 2```</span>
  Move = 2,
  /// C++ enum variant: <span style='color: green;'>```InputRelease = 3```</span>
  Release = 3,
}

/// C++ type: <span style='color: green;'>```QScroller```</span>
#[repr(C)]
pub struct Scroller(u8);

impl Scroller {
  /// C++ method: <span style='color: green;'>```static QList<QScroller*> QScroller::activeScrollers()```</span>
  ///
  ///
  pub fn active_scrollers() -> ::list::ListScrollerMutPtr {
    {
      let mut object: ::list::ListScrollerMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QScroller_activeScrollers_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QScroller::ensureVisible```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn ensure_visible(&mut self, (&::qt_core::rect_f::RectF, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QScroller::ensureVisible(const QRectF& rect, double xmargin, double ymargin)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn ensure_visible(&mut self, (&::qt_core::rect_f::RectF, ::libc::c_double, ::libc::c_double, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QScroller::ensureVisible(const QRectF& rect, double xmargin, double ymargin, int scrollTime)```</span>
  ///
  ///
  pub fn ensure_visible<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ScrollerEnsureVisibleArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPointF QScroller::finalPosition() const```</span>
  ///
  ///
  pub fn final_position(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QScroller_finalPosition_to_output(self as *const ::scroller::Scroller, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QScroller::handleInput```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn handle_input(&mut self, (::scroller::Input, &::qt_core::point_f::PointF)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QScroller::handleInput(QScroller::Input input, const QPointF& position)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn handle_input(&mut self, (::scroller::Input, &::qt_core::point_f::PointF, i64)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QScroller::handleInput(QScroller::Input input, const QPointF& position, qint64 timestamp = ?)```</span>
  ///
  ///
  pub fn handle_input<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::ScrollerHandleInputArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static bool QScroller::hasScroller(QObject* target)```</span>
  ///
  ///
  pub unsafe fn has_scroller(target: *mut ::qt_core::object::Object) -> bool {
    ::ffi::qt_widgets_c_QScroller_hasScroller(target)
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QScroller::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QScroller_metaObject(self as *const ::scroller::Scroller) }
  }

  /// C++ method: <span style='color: green;'>```QPointF QScroller::pixelPerMeter() const```</span>
  ///
  ///
  pub fn pixel_per_meter(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QScroller_pixelPerMeter_to_output(self as *const ::scroller::Scroller, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QScroller::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QScroller_qt_metacall(self as *mut ::scroller::Scroller,
                                              arg1 as *const ::qt_core::meta_object::Call,
                                              arg2,
                                              arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QScroller::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QScroller_qt_metacast(self as *mut ::scroller::Scroller, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QScroller::resendPrepareEvent()```</span>
  ///
  ///
  pub fn resend_prepare_event(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QScroller_resendPrepareEvent(self as *mut ::scroller::Scroller) }
  }

  /// C++ method: <span style='color: green;'>```QScroller::scrollTo```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn scroll_to(&mut self, &::qt_core::point_f::PointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QScroller::scrollTo(const QPointF& pos)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn scroll_to(&mut self, (&::qt_core::point_f::PointF, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QScroller::scrollTo(const QPointF& pos, int scrollTime)```</span>
  ///
  ///
  pub fn scroll_to<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ScrollerScrollToArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QScroller::scroller```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn scroller(*mut ::qt_core::object::Object) -> *mut ::scroller::Scroller```<br>
  /// C++ method: <span style='color: green;'>```static QScroller* QScroller::scroller(QObject* target)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn scroller(*const ::qt_core::object::Object) -> *const ::scroller::Scroller```<br>
  /// C++ method: <span style='color: green;'>```static const QScroller* QScroller::scroller(const QObject* target)```</span>
  ///
  ///
  pub unsafe fn scroller<Args>(args: Args) -> Args::ReturnType
    where Args: overloading::ScrollerScrollerArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QScrollerProperties QScroller::scrollerProperties() const```</span>
  ///
  ///
  pub fn scroller_properties(&self) -> ::cpp_utils::CppBox<::scroller_properties::ScrollerProperties> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QScroller_scrollerProperties_as_ptr(self as *const ::scroller::Scroller) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QScroller::setScrollerProperties(const QScrollerProperties& prop)```</span>
  ///
  ///
  pub fn set_scroller_properties(&mut self, prop: &::scroller_properties::ScrollerProperties) {
    unsafe {
      ::ffi::qt_widgets_c_QScroller_setScrollerProperties(self as *mut ::scroller::Scroller,
                                                          prop as *const ::scroller_properties::ScrollerProperties)
    }
  }

  /// C++ method: <span style='color: green;'>```QScroller::setSnapPositionsX```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_snap_positions_x(&mut self, &::qt_gui::list::ListCDouble) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QScroller::setSnapPositionsX(const QList<double>& positions)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_snap_positions_x(&mut self, (::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QScroller::setSnapPositionsX(double first, double interval)```</span>
  ///
  ///
  pub fn set_snap_positions_x<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ScrollerSetSnapPositionsXArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QScroller::setSnapPositionsY```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_snap_positions_y(&mut self, &::qt_gui::list::ListCDouble) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QScroller::setSnapPositionsY(const QList<double>& positions)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_snap_positions_y(&mut self, (::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QScroller::setSnapPositionsY(double first, double interval)```</span>
  ///
  ///
  pub fn set_snap_positions_y<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ScrollerSetSnapPositionsYArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QScroller::State QScroller::state() const```</span>
  ///
  ///
  pub fn state(&self) -> ::scroller::State {
    unsafe { ::ffi::qt_widgets_c_QScroller_state(self as *const ::scroller::Scroller) }
  }

  /// C++ method: <span style='color: green;'>```void QScroller::stop()```</span>
  ///
  ///
  pub fn stop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QScroller_stop(self as *mut ::scroller::Scroller) }
  }

  /// C++ method: <span style='color: green;'>```QObject* QScroller::target() const```</span>
  ///
  ///
  pub fn target(&self) -> *mut ::qt_core::object::Object {
    unsafe { ::ffi::qt_widgets_c_QScroller_target(self as *const ::scroller::Scroller) }
  }

  /// C++ method: <span style='color: green;'>```static QString QScroller::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QScroller_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QScroller::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QScroller_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static void QScroller::ungrabGesture(QObject* target)```</span>
  ///
  ///
  pub unsafe fn ungrab_gesture(target: *mut ::qt_core::object::Object) {
    ::ffi::qt_widgets_c_QScroller_ungrabGesture(target)
  }

  /// C++ method: <span style='color: green;'>```QPointF QScroller::velocity() const```</span>
  ///
  ///
  pub fn velocity(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QScroller_velocity_to_output(self as *const ::scroller::Scroller, &mut object);
      }
      object
    }
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Scroller`.
  pub struct Signals<'a>(&'a ::scroller::Scroller);
  /// Represents a built-in Qt signal `QScroller::scrollerPropertiesChanged`.
  ///
  /// An object of this type can be created from `Scroller` with `object.signals().scroller_properties_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Scroller` object.
  pub struct ScrollerPropertiesChanged<'a>(&'a ::scroller::Scroller);
  impl<'a> ::qt_core::connection::Receiver for ScrollerPropertiesChanged<'a> {
    type Arguments = (&'static ::scroller_properties::ScrollerProperties,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2scrollerPropertiesChanged(const QScrollerProperties&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ScrollerPropertiesChanged<'a> {}
  /// Represents a built-in Qt signal `QScroller::stateChanged`.
  ///
  /// An object of this type can be created from `Scroller` with `object.signals().state_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Scroller` object.
  pub struct StateChanged<'a>(&'a ::scroller::Scroller);
  impl<'a> ::qt_core::connection::Receiver for StateChanged<'a> {
    type Arguments = (&'static ::scroller::State,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2stateChanged(QScroller::State)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for StateChanged<'a> {}
  /// Represents a built-in Qt signal `QScroller::objectNameChanged`.
  ///
  /// An object of this type can be created from `Scroller` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Scroller` object.
  pub struct ObjectNameChanged<'a>(&'a ::scroller::Scroller);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QScroller::scrollerPropertiesChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scroller_properties_changed(&self) -> ScrollerPropertiesChanged {
      ScrollerPropertiesChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QScroller::stateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn state_changed(&self) -> StateChanged {
      StateChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QScroller::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Scroller`.
  pub struct Slots<'a>(&'a ::scroller::Scroller);
  /// Represents a built-in Qt slot `QScroller::setScrollerProperties`.
  ///
  /// An object of this type can be created from `Scroller` with `object.slots().set_scroller_properties()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Scroller` object.
  pub struct SetScrollerProperties<'a>(&'a ::scroller::Scroller);
  impl<'a> ::qt_core::connection::Receiver for SetScrollerProperties<'a> {
    type Arguments = (&'static ::scroller_properties::ScrollerProperties,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setScrollerProperties(const QScrollerProperties&)\0"
    }
  }
  /// Represents a built-in Qt slot `QScroller::ensureVisible`.
  ///
  /// An object of this type can be created from `Scroller` with `object.slots().ensure_visible_qt_core_rect_f_ref_c_double_c_double()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Scroller` object.
  pub struct EnsureVisibleQtCoreRectFRefCDoubleCDouble<'a>(&'a ::scroller::Scroller);
  impl<'a> ::qt_core::connection::Receiver for EnsureVisibleQtCoreRectFRefCDoubleCDouble<'a> {
    type Arguments = (&'static ::qt_core::rect_f::RectF, ::libc::c_double, ::libc::c_double);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1ensureVisible(const QRectF&,double,double)\0"
    }
  }
  /// Represents a built-in Qt slot `QScroller::ensureVisible`.
  ///
  /// An object of this type can be created from `Scroller` with `object.slots().ensure_visible_qt_core_rect_f_ref_c_double_c_double_c_int()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Scroller` object.
  pub struct EnsureVisibleQtCoreRectFRefCDoubleCDoubleCInt<'a>(&'a ::scroller::Scroller);
  impl<'a> ::qt_core::connection::Receiver for EnsureVisibleQtCoreRectFRefCDoubleCDoubleCInt<'a> {
    type Arguments = (&'static ::qt_core::rect_f::RectF, ::libc::c_double, ::libc::c_double, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1ensureVisible(const QRectF&,double,double,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QScroller::scrollTo`.
  ///
  /// An object of this type can be created from `Scroller` with `object.slots().scroll_to_qt_core_point_f_ref()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Scroller` object.
  pub struct ScrollToQtCorePointFRef<'a>(&'a ::scroller::Scroller);
  impl<'a> ::qt_core::connection::Receiver for ScrollToQtCorePointFRef<'a> {
    type Arguments = (&'static ::qt_core::point_f::PointF,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1scrollTo(const QPointF&)\0"
    }
  }
  /// Represents a built-in Qt slot `QScroller::scrollTo`.
  ///
  /// An object of this type can be created from `Scroller` with `object.slots().scroll_to_qt_core_point_f_ref_c_int()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Scroller` object.
  pub struct ScrollToQtCorePointFRefCInt<'a>(&'a ::scroller::Scroller);
  impl<'a> ::qt_core::connection::Receiver for ScrollToQtCorePointFRefCInt<'a> {
    type Arguments = (&'static ::qt_core::point_f::PointF, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1scrollTo(const QPointF&,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QScroller::resendPrepareEvent`.
  ///
  /// An object of this type can be created from `Scroller` with `object.slots().resend_prepare_event()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Scroller` object.
  pub struct ResendPrepareEvent<'a>(&'a ::scroller::Scroller);
  impl<'a> ::qt_core::connection::Receiver for ResendPrepareEvent<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1resendPrepareEvent()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QScroller::setScrollerProperties`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_scroller_properties(&self) -> SetScrollerProperties {
      SetScrollerProperties(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QScroller::ensureVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn ensure_visible_qt_core_rect_f_ref_c_double_c_double(&self) -> EnsureVisibleQtCoreRectFRefCDoubleCDouble {
      EnsureVisibleQtCoreRectFRefCDoubleCDouble(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QScroller::ensureVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn ensure_visible_qt_core_rect_f_ref_c_double_c_double_c_int
      (&self)
       -> EnsureVisibleQtCoreRectFRefCDoubleCDoubleCInt {
      EnsureVisibleQtCoreRectFRefCDoubleCDoubleCInt(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QScroller::scrollTo`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scroll_to_qt_core_point_f_ref(&self) -> ScrollToQtCorePointFRef {
      ScrollToQtCorePointFRef(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QScroller::scrollTo`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scroll_to_qt_core_point_f_ref_c_int(&self) -> ScrollToQtCorePointFRefCInt {
      ScrollToQtCorePointFRefCInt(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QScroller::resendPrepareEvent`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn resend_prepare_event(&self) -> ResendPrepareEvent {
      ResendPrepareEvent(self.0)
    }
  }
  impl ::scroller::Scroller {
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

/// C++ type: <span style='color: green;'>```QScroller::ScrollerGestureType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ScrollerGestureType {
  /// C++ enum variant: <span style='color: green;'>```TouchGesture = 0```</span>
  Touch = 0,
  /// C++ enum variant: <span style='color: green;'>```LeftMouseButtonGesture = 1```</span>
  LeftMouseButton = 1,
  /// C++ enum variant: <span style='color: green;'>```RightMouseButtonGesture = 2```</span>
  RightMouseButton = 2,
  /// C++ enum variant: <span style='color: green;'>```MiddleMouseButtonGesture = 3```</span>
  MiddleMouseButton = 3,
}

/// C++ type: <span style='color: green;'>```QScroller::State```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum State {
  /// C++ enum variant: <span style='color: green;'>```Inactive = 0```</span>
  Inactive = 0,
  /// C++ enum variant: <span style='color: green;'>```Pressed = 1```</span>
  Pressed = 1,
  /// C++ enum variant: <span style='color: green;'>```Dragging = 2```</span>
  Dragging = 2,
  /// C++ enum variant: <span style='color: green;'>```Scrolling = 3```</span>
  Scrolling = 3,
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::scroller::Scroller {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QScroller_G_static_cast_QObject_ptr(self as *mut ::scroller::Scroller) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QScroller_G_static_cast_QObject_ptr(self as *const ::scroller::Scroller as *mut ::scroller::Scroller) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::scroller::Scroller> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::scroller::Scroller {
    let ffi_result =
      ::ffi::qt_widgets_c_QScroller_G_static_cast_QScroller_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::scroller::Scroller {
    let ffi_result = ::ffi::qt_widgets_c_QScroller_G_static_cast_QScroller_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::scroller::Scroller {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QScroller_G_static_cast_QObject_ptr(self as *const ::scroller::Scroller as *mut ::scroller::Scroller) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::scroller::Scroller {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QScroller_G_static_cast_QObject_ptr(self as *mut ::scroller::Scroller) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Scroller::ensure_visible](../struct.Scroller.html#method.ensure_visible) method.
  pub trait ScrollerEnsureVisibleArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::scroller::Scroller) -> ();
  }
  impl<'largs> ScrollerEnsureVisibleArgs<'largs>
    for (&'largs ::qt_core::rect_f::RectF, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::scroller::Scroller) -> () {
      let rect = self.0;
      let xmargin = self.1;
      let ymargin = self.2;
      unsafe {
        ::ffi::qt_widgets_c_QScroller_ensureVisible_rect_xmargin_ymargin(original_self as *mut ::scroller::Scroller,
                                                                         rect as *const ::qt_core::rect_f::RectF,
                                                                         xmargin,
                                                                         ymargin)
      }
    }
  }
  impl<'largs> ScrollerEnsureVisibleArgs<'largs>
    for (&'largs ::qt_core::rect_f::RectF, ::libc::c_double, ::libc::c_double, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::scroller::Scroller) -> () {
      let rect = self.0;
      let xmargin = self.1;
      let ymargin = self.2;
      let scroll_time = self.3;
      unsafe { ::ffi::qt_widgets_c_QScroller_ensureVisible_rect_xmargin_ymargin_scrollTime(original_self as *mut ::scroller::Scroller, rect as *const ::qt_core::rect_f::RectF, xmargin, ymargin, scroll_time) }
    }
  }
  /// This trait represents a set of arguments accepted by [Scroller::handle_input](../struct.Scroller.html#method.handle_input) method.
  pub trait ScrollerHandleInputArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::scroller::Scroller) -> bool;
  }
  impl<'largs> ScrollerHandleInputArgs<'largs> for (::scroller::Input, &'largs ::qt_core::point_f::PointF) {
    fn exec(self, original_self: &'largs mut ::scroller::Scroller) -> bool {
      let input = self.0;
      let position = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QScroller_handleInput_input_position(original_self as *mut ::scroller::Scroller,
                                                                 input,
                                                                 position as *const ::qt_core::point_f::PointF)
      }
    }
  }
  impl<'largs> ScrollerHandleInputArgs<'largs> for (::scroller::Input, &'largs ::qt_core::point_f::PointF, i64) {
    fn exec(self, original_self: &'largs mut ::scroller::Scroller) -> bool {
      let input = self.0;
      let position = self.1;
      let timestamp = self.2;
      unsafe { ::ffi::qt_widgets_c_QScroller_handleInput_input_position_timestamp(original_self as *mut ::scroller::Scroller, input, position as *const ::qt_core::point_f::PointF, timestamp) }
    }
  }
  /// This trait represents a set of arguments accepted by [Scroller::scroll_to](../struct.Scroller.html#method.scroll_to) method.
  pub trait ScrollerScrollToArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::scroller::Scroller) -> ();
  }
  impl<'largs> ScrollerScrollToArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    fn exec(self, original_self: &'largs mut ::scroller::Scroller) -> () {
      let pos = self;
      unsafe {
        ::ffi::qt_widgets_c_QScroller_scrollTo_pos(original_self as *mut ::scroller::Scroller,
                                                   pos as *const ::qt_core::point_f::PointF)
      }
    }
  }
  impl<'largs> ScrollerScrollToArgs<'largs> for (&'largs ::qt_core::point_f::PointF, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::scroller::Scroller) -> () {
      let pos = self.0;
      let scroll_time = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QScroller_scrollTo_pos_scrollTime(original_self as *mut ::scroller::Scroller,
                                                              pos as *const ::qt_core::point_f::PointF,
                                                              scroll_time)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Scroller::scroller](../struct.Scroller.html#method.scroller) method.
  pub trait ScrollerScrollerArgs {
    type ReturnType;
    unsafe fn exec(self) -> Self::ReturnType;
  }
  impl ScrollerScrollerArgs for *mut ::qt_core::object::Object {
    type ReturnType = *mut ::scroller::Scroller;
    unsafe fn exec(self) -> *mut ::scroller::Scroller {
      let target = self;
      ::ffi::qt_widgets_c_QScroller_scroller_QObject_ptr(target)
    }
  }
  impl ScrollerScrollerArgs for *const ::qt_core::object::Object {
    type ReturnType = *const ::scroller::Scroller;
    unsafe fn exec(self) -> *const ::scroller::Scroller {
      let target = self;
      ::ffi::qt_widgets_c_QScroller_scroller_const_QObject_ptr(target)
    }
  }
  /// This trait represents a set of arguments accepted by [Scroller::set_snap_positions_x](../struct.Scroller.html#method.set_snap_positions_x) method.
  pub trait ScrollerSetSnapPositionsXArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::scroller::Scroller) -> ();
  }
  impl<'largs> ScrollerSetSnapPositionsXArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::scroller::Scroller) -> () {
      let first = self.0;
      let interval = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QScroller_setSnapPositionsX_first_interval(original_self as *mut ::scroller::Scroller,
                                                                       first,
                                                                       interval)
      }
    }
  }
  impl<'largs> ScrollerSetSnapPositionsXArgs<'largs> for &'largs ::qt_gui::list::ListCDouble {
    fn exec(self, original_self: &'largs mut ::scroller::Scroller) -> () {
      let positions = self;
      unsafe {
        ::ffi::qt_widgets_c_QScroller_setSnapPositionsX_positions(original_self as *mut ::scroller::Scroller,
                                                                  positions as *const ::qt_gui::list::ListCDouble)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Scroller::set_snap_positions_y](../struct.Scroller.html#method.set_snap_positions_y) method.
  pub trait ScrollerSetSnapPositionsYArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::scroller::Scroller) -> ();
  }
  impl<'largs> ScrollerSetSnapPositionsYArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::scroller::Scroller) -> () {
      let first = self.0;
      let interval = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QScroller_setSnapPositionsY_first_interval(original_self as *mut ::scroller::Scroller,
                                                                       first,
                                                                       interval)
      }
    }
  }
  impl<'largs> ScrollerSetSnapPositionsYArgs<'largs> for &'largs ::qt_gui::list::ListCDouble {
    fn exec(self, original_self: &'largs mut ::scroller::Scroller) -> () {
      let positions = self;
      unsafe {
        ::ffi::qt_widgets_c_QScroller_setSnapPositionsY_positions(original_self as *mut ::scroller::Scroller,
                                                                  positions as *const ::qt_gui::list::ListCDouble)
      }
    }
  }
}

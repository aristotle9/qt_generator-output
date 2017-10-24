/// C++ type: <span style='color: green;'>```QGraphicsWidget```</span>
#[repr(C)]
pub struct GraphicsWidget(u8);

impl GraphicsWidget {
  /// C++ method: <span style='color: green;'>```QList<QAction*> QGraphicsWidget::actions() const```</span>
  ///
  ///
  pub fn actions(&self) -> ::list::ListActionMutPtr {
    {
      let mut object: ::list::ListActionMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsWidget_actions_to_output(self as *const ::graphics_widget::GraphicsWidget,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::addAction(QAction* action)```</span>
  ///
  ///
  pub unsafe fn add_action(&mut self, action: *mut ::action::Action) {
    ::ffi::qt_widgets_c_QGraphicsWidget_addAction(self as *mut ::graphics_widget::GraphicsWidget, action)
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::addActions(QList<QAction*> actions)```</span>
  ///
  ///
  pub fn add_actions(&mut self, actions: &::list::ListActionMutPtr) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsWidget_addActions(self as *mut ::graphics_widget::GraphicsWidget,
                                                     actions as *const ::list::ListActionMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::adjustSize()```</span>
  ///
  ///
  pub fn adjust_size(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_adjustSize(self as *mut ::graphics_widget::GraphicsWidget) }
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsWidget::autoFillBackground() const```</span>
  ///
  ///
  pub fn auto_fill_background(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_autoFillBackground(self as *const ::graphics_widget::GraphicsWidget) }
  }

  /// C++ method: <span style='color: green;'>```virtual QRectF QGraphicsWidget::boundingRect() const```</span>
  ///
  ///
  pub fn bounding_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsWidget_boundingRect_to_output(self as *const ::graphics_widget::GraphicsWidget,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] bool QGraphicsWidget::close()```</span>
  ///
  ///
  pub fn close(&mut self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_close(self as *mut ::graphics_widget::GraphicsWidget) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsWidget* QGraphicsWidget::focusWidget() const```</span>
  ///
  ///
  pub fn focus_widget(&self) -> *mut ::graphics_widget::GraphicsWidget {
    unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_focusWidget(self as *const ::graphics_widget::GraphicsWidget) }
  }

  /// C++ method: <span style='color: green;'>```QFont QGraphicsWidget::font() const```</span>
  ///
  ///
  pub fn font(&self) -> ::qt_gui::font::Font {
    {
      let mut object: ::qt_gui::font::Font =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsWidget_font_to_output(self as *const ::graphics_widget::GraphicsWidget,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QGraphicsWidget::getContentsMargins(double* left, double* top, double* right, double* bottom) const```</span>
  ///
  ///
  pub unsafe fn get_contents_margins(&self,
                                     left: *mut ::libc::c_double,
                                     top: *mut ::libc::c_double,
                                     right: *mut ::libc::c_double,
                                     bottom: *mut ::libc::c_double) {
    ::ffi::qt_widgets_c_QGraphicsWidget_getContentsMargins(self as *const ::graphics_widget::GraphicsWidget,
                                                           left,
                                                           top,
                                                           right,
                                                           bottom)
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::getWindowFrameMargins(double* left, double* top, double* right, double* bottom) const```</span>
  ///
  ///
  pub unsafe fn get_window_frame_margins(&self,
                                         left: *mut ::libc::c_double,
                                         top: *mut ::libc::c_double,
                                         right: *mut ::libc::c_double,
                                         bottom: *mut ::libc::c_double) {
    ::ffi::qt_widgets_c_QGraphicsWidget_getWindowFrameMargins(self as *const ::graphics_widget::GraphicsWidget,
                                                              left,
                                                              top,
                                                              right,
                                                              bottom)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsWidget::grabShortcut```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn grab_shortcut(&mut self, &::qt_gui::key_sequence::KeySequence) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QGraphicsWidget::grabShortcut(const QKeySequence& sequence)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn grab_shortcut(&mut self, (&::qt_gui::key_sequence::KeySequence, &::qt_core::qt::ShortcutContext)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QGraphicsWidget::grabShortcut(const QKeySequence& sequence, Qt::ShortcutContext context = ?)```</span>
  ///
  ///
  pub fn grab_shortcut<'largs, Args>(&'largs mut self, args: Args) -> ::libc::c_int
    where Args: overloading::GraphicsWidgetGrabShortcutArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::insertAction(QAction* before, QAction* action)```</span>
  ///
  ///
  pub unsafe fn insert_action(&mut self, before: *mut ::action::Action, action: *mut ::action::Action) {
    ::ffi::qt_widgets_c_QGraphicsWidget_insertAction(self as *mut ::graphics_widget::GraphicsWidget,
                                                     before,
                                                     action)
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::insertActions(QAction* before, QList<QAction*> actions)```</span>
  ///
  ///
  pub unsafe fn insert_actions(&mut self, before: *mut ::action::Action, actions: &::list::ListActionMutPtr) {
    ::ffi::qt_widgets_c_QGraphicsWidget_insertActions(self as *mut ::graphics_widget::GraphicsWidget,
                                                      before,
                                                      actions as *const ::list::ListActionMutPtr)
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsWidget::isActiveWindow() const```</span>
  ///
  ///
  pub fn is_active_window(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_isActiveWindow(self as *const ::graphics_widget::GraphicsWidget) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsLayout* QGraphicsWidget::layout() const```</span>
  ///
  ///
  pub fn layout(&self) -> *mut ::graphics_layout::GraphicsLayout {
    unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_layout(self as *const ::graphics_widget::GraphicsWidget) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QGraphicsWidget::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_metaObject(self as *const ::graphics_widget::GraphicsWidget) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsWidget::paint```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn paint(&mut self, (*mut ::qt_gui::painter::Painter, *const ::style_option_graphics_item::StyleOptionGraphicsItem)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QGraphicsWidget::paint(QPainter* painter, const QStyleOptionGraphicsItem* option)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn paint(&mut self, (*mut ::qt_gui::painter::Painter, *const ::style_option_graphics_item::StyleOptionGraphicsItem, *mut ::widget::Widget)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QGraphicsWidget::paint(QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget = ?)```</span>
  ///
  ///
  pub unsafe fn paint<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsWidgetPaintArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsWidget::paintWindowFrame```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn paint_window_frame(&mut self, (*mut ::qt_gui::painter::Painter, *const ::style_option_graphics_item::StyleOptionGraphicsItem)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QGraphicsWidget::paintWindowFrame(QPainter* painter, const QStyleOptionGraphicsItem* option)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn paint_window_frame(&mut self, (*mut ::qt_gui::painter::Painter, *const ::style_option_graphics_item::StyleOptionGraphicsItem, *mut ::widget::Widget)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QGraphicsWidget::paintWindowFrame(QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget = ?)```</span>
  ///
  ///
  pub unsafe fn paint_window_frame<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsWidgetPaintWindowFrameArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPalette QGraphicsWidget::palette() const```</span>
  ///
  ///
  pub fn palette(&self) -> ::qt_gui::palette::Palette {
    {
      let mut object: ::qt_gui::palette::Palette =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsWidget_palette_to_output(self as *const ::graphics_widget::GraphicsWidget,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsWidget::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QGraphicsWidget_qt_metacall(self as *mut ::graphics_widget::GraphicsWidget,
                                                    arg1 as *const ::qt_core::meta_object::Call,
                                                    arg2,
                                                    arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QGraphicsWidget::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QGraphicsWidget_qt_metacast(self as *mut ::graphics_widget::GraphicsWidget, arg1)
  }

  /// C++ method: <span style='color: green;'>```QRectF QGraphicsWidget::rect() const```</span>
  ///
  ///
  pub fn rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsWidget_rect_to_output(self as *const ::graphics_widget::GraphicsWidget,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::releaseShortcut(int id)```</span>
  ///
  ///
  pub fn release_shortcut(&mut self, id: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_releaseShortcut(self as *mut ::graphics_widget::GraphicsWidget, id) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::removeAction(QAction* action)```</span>
  ///
  ///
  pub unsafe fn remove_action(&mut self, action: *mut ::action::Action) {
    ::ffi::qt_widgets_c_QGraphicsWidget_removeAction(self as *mut ::graphics_widget::GraphicsWidget, action)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsWidget::resize```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn resize(&mut self, &::qt_core::size_f::SizeF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::resize(const QSizeF& size)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn resize(&mut self, (::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::resize(double w, double h)```</span>
  ///
  ///
  pub fn resize<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsWidgetResizeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsWidget::setAttribute```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_attribute(&mut self, &::qt_core::qt::WidgetAttribute) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::setAttribute(Qt::WidgetAttribute attribute)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_attribute(&mut self, (&::qt_core::qt::WidgetAttribute, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::setAttribute(Qt::WidgetAttribute attribute, bool on = ?)```</span>
  ///
  ///
  pub fn set_attribute<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsWidgetSetAttributeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::setAutoFillBackground(bool enabled)```</span>
  ///
  ///
  pub fn set_auto_fill_background(&mut self, enabled: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsWidget_setAutoFillBackground(self as *mut ::graphics_widget::GraphicsWidget, enabled)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::setContentsMargins(double left, double top, double right, double bottom)```</span>
  ///
  ///
  pub fn set_contents_margins(&mut self,
                              left: ::libc::c_double,
                              top: ::libc::c_double,
                              right: ::libc::c_double,
                              bottom: ::libc::c_double) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsWidget_setContentsMargins(self as *mut ::graphics_widget::GraphicsWidget,
                                                             left,
                                                             top,
                                                             right,
                                                             bottom)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::setFocusPolicy(Qt::FocusPolicy policy)```</span>
  ///
  ///
  pub fn set_focus_policy(&mut self, policy: &::qt_core::qt::FocusPolicy) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsWidget_setFocusPolicy(self as *mut ::graphics_widget::GraphicsWidget,
                                                         policy as *const ::qt_core::qt::FocusPolicy)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::setFont(const QFont& font)```</span>
  ///
  ///
  pub fn set_font(&mut self, font: &::qt_gui::font::Font) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsWidget_setFont(self as *mut ::graphics_widget::GraphicsWidget,
                                                  font as *const ::qt_gui::font::Font)
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsWidget::setGeometry```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_geometry(&mut self, &::qt_core::rect_f::RectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QGraphicsWidget::setGeometry(const QRectF& rect)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_geometry(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::setGeometry(double x, double y, double w, double h)```</span>
  ///
  ///
  pub fn set_geometry<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsWidgetSetGeometryArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::setLayout(QGraphicsLayout* layout)```</span>
  ///
  ///
  pub unsafe fn set_layout(&mut self, layout: *mut ::graphics_layout::GraphicsLayout) {
    ::ffi::qt_widgets_c_QGraphicsWidget_setLayout(self as *mut ::graphics_widget::GraphicsWidget, layout)
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::setLayoutDirection(Qt::LayoutDirection direction)```</span>
  ///
  ///
  pub fn set_layout_direction(&mut self, direction: &::qt_core::qt::LayoutDirection) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsWidget_setLayoutDirection(self as *mut ::graphics_widget::GraphicsWidget,
                                                             direction as *const ::qt_core::qt::LayoutDirection)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::setPalette(const QPalette& palette)```</span>
  ///
  ///
  pub fn set_palette(&mut self, palette: &::qt_gui::palette::Palette) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsWidget_setPalette(self as *mut ::graphics_widget::GraphicsWidget,
                                                     palette as *const ::qt_gui::palette::Palette)
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsWidget::setShortcutAutoRepeat```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_shortcut_auto_repeat(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::setShortcutAutoRepeat(int id)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_shortcut_auto_repeat(&mut self, (::libc::c_int, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::setShortcutAutoRepeat(int id, bool enabled = ?)```</span>
  ///
  ///
  pub fn set_shortcut_auto_repeat<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsWidgetSetShortcutAutoRepeatArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsWidget::setShortcutEnabled```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_shortcut_enabled(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::setShortcutEnabled(int id)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_shortcut_enabled(&mut self, (::libc::c_int, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::setShortcutEnabled(int id, bool enabled = ?)```</span>
  ///
  ///
  pub fn set_shortcut_enabled<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsWidgetSetShortcutEnabledArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::setStyle(QStyle* style)```</span>
  ///
  ///
  pub unsafe fn set_style(&mut self, style: *mut ::style::Style) {
    ::ffi::qt_widgets_c_QGraphicsWidget_setStyle(self as *mut ::graphics_widget::GraphicsWidget, style)
  }

  /// C++ method: <span style='color: green;'>```static void QGraphicsWidget::setTabOrder(QGraphicsWidget* first, QGraphicsWidget* second)```</span>
  ///
  ///
  pub unsafe fn set_tab_order(first: *mut ::graphics_widget::GraphicsWidget,
                              second: *mut ::graphics_widget::GraphicsWidget) {
    ::ffi::qt_widgets_c_QGraphicsWidget_setTabOrder(first, second)
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::setWindowFrameMargins(double left, double top, double right, double bottom)```</span>
  ///
  ///
  pub fn set_window_frame_margins(&mut self,
                                  left: ::libc::c_double,
                                  top: ::libc::c_double,
                                  right: ::libc::c_double,
                                  bottom: ::libc::c_double) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsWidget_setWindowFrameMargins(self as *mut ::graphics_widget::GraphicsWidget,
                                                                left,
                                                                top,
                                                                right,
                                                                bottom)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::setWindowTitle(const QString& title)```</span>
  ///
  ///
  pub fn set_window_title(&mut self, title: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsWidget_setWindowTitle(self as *mut ::graphics_widget::GraphicsWidget,
                                                         title as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QPainterPath QGraphicsWidget::shape() const```</span>
  ///
  ///
  pub fn shape(&self) -> ::qt_gui::painter_path::PainterPath {
    {
      let mut object: ::qt_gui::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsWidget_shape_to_output(self as *const ::graphics_widget::GraphicsWidget,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSizeF QGraphicsWidget::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::qt_core::size_f::SizeF {
    {
      let mut object: ::qt_core::size_f::SizeF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsWidget_size_to_output(self as *const ::graphics_widget::GraphicsWidget,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStyle* QGraphicsWidget::style() const```</span>
  ///
  ///
  pub fn style(&self) -> *mut ::style::Style {
    unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_style(self as *const ::graphics_widget::GraphicsWidget) }
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsWidget::testAttribute(Qt::WidgetAttribute attribute) const```</span>
  ///
  ///
  pub fn test_attribute(&self, attribute: &::qt_core::qt::WidgetAttribute) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsWidget_testAttribute(self as *const ::graphics_widget::GraphicsWidget,
                                                        attribute as *const ::qt_core::qt::WidgetAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsWidget::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsWidget_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsWidget::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsWidget_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsWidget::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_type(self as *const ::graphics_widget::GraphicsWidget) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::unsetLayoutDirection()```</span>
  ///
  ///
  pub fn unset_layout_direction(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_unsetLayoutDirection(self as *mut ::graphics_widget::GraphicsWidget) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsWidget::unsetWindowFrameMargins()```</span>
  ///
  ///
  pub fn unset_window_frame_margins(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsWidget_unsetWindowFrameMargins(self as *mut ::graphics_widget::GraphicsWidget)
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF QGraphicsWidget::windowFrameGeometry() const```</span>
  ///
  ///
  pub fn window_frame_geometry(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsWidget_windowFrameGeometry_to_output(self as *const ::graphics_widget::GraphicsWidget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF QGraphicsWidget::windowFrameRect() const```</span>
  ///
  ///
  pub fn window_frame_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsWidget_windowFrameRect_to_output(self as *const ::graphics_widget::GraphicsWidget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QGraphicsWidget::windowTitle() const```</span>
  ///
  ///
  pub fn window_title(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsWidget_windowTitle_to_output(self as *const ::graphics_widget::GraphicsWidget,
                                                                  &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_widget::GraphicsWidget {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsWidget_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `GraphicsWidget`.
  pub struct Signals<'a>(&'a ::graphics_widget::GraphicsWidget);
  /// Represents a built-in Qt signal `QGraphicsWidget::parentChanged`.
  ///
  /// An object of this type can be created from `GraphicsWidget` with `object.signals().parent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsWidget` object.
  pub struct ParentChanged<'a>(&'a ::graphics_widget::GraphicsWidget);
  impl<'a> ::qt_core::connection::Receiver for ParentChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2parentChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ParentChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsWidget::visibleChanged`.
  ///
  /// An object of this type can be created from `GraphicsWidget` with `object.signals().visible_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsWidget` object.
  pub struct VisibleChanged<'a>(&'a ::graphics_widget::GraphicsWidget);
  impl<'a> ::qt_core::connection::Receiver for VisibleChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2visibleChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for VisibleChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsWidget::scaleChanged`.
  ///
  /// An object of this type can be created from `GraphicsWidget` with `object.signals().scale_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsWidget` object.
  pub struct ScaleChanged<'a>(&'a ::graphics_widget::GraphicsWidget);
  impl<'a> ::qt_core::connection::Receiver for ScaleChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2scaleChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ScaleChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsWidget::geometryChanged`.
  ///
  /// An object of this type can be created from `GraphicsWidget` with `object.signals().geometry_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsWidget` object.
  pub struct GeometryChanged<'a>(&'a ::graphics_widget::GraphicsWidget);
  impl<'a> ::qt_core::connection::Receiver for GeometryChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2geometryChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for GeometryChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsWidget::layoutChanged`.
  ///
  /// An object of this type can be created from `GraphicsWidget` with `object.signals().layout_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsWidget` object.
  pub struct LayoutChanged<'a>(&'a ::graphics_widget::GraphicsWidget);
  impl<'a> ::qt_core::connection::Receiver for LayoutChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2layoutChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for LayoutChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsWidget::xChanged`.
  ///
  /// An object of this type can be created from `GraphicsWidget` with `object.signals().x_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsWidget` object.
  pub struct XChanged<'a>(&'a ::graphics_widget::GraphicsWidget);
  impl<'a> ::qt_core::connection::Receiver for XChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2xChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for XChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsWidget::widthChanged`.
  ///
  /// An object of this type can be created from `GraphicsWidget` with `object.signals().width_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsWidget` object.
  pub struct WidthChanged<'a>(&'a ::graphics_widget::GraphicsWidget);
  impl<'a> ::qt_core::connection::Receiver for WidthChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2widthChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WidthChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsWidget::yChanged`.
  ///
  /// An object of this type can be created from `GraphicsWidget` with `object.signals().y_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsWidget` object.
  pub struct YChanged<'a>(&'a ::graphics_widget::GraphicsWidget);
  impl<'a> ::qt_core::connection::Receiver for YChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2yChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for YChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsWidget::childrenChanged`.
  ///
  /// An object of this type can be created from `GraphicsWidget` with `object.signals().children_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsWidget` object.
  pub struct ChildrenChanged<'a>(&'a ::graphics_widget::GraphicsWidget);
  impl<'a> ::qt_core::connection::Receiver for ChildrenChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2childrenChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ChildrenChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsWidget::opacityChanged`.
  ///
  /// An object of this type can be created from `GraphicsWidget` with `object.signals().opacity_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsWidget` object.
  pub struct OpacityChanged<'a>(&'a ::graphics_widget::GraphicsWidget);
  impl<'a> ::qt_core::connection::Receiver for OpacityChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2opacityChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for OpacityChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsWidget::enabledChanged`.
  ///
  /// An object of this type can be created from `GraphicsWidget` with `object.signals().enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsWidget` object.
  pub struct EnabledChanged<'a>(&'a ::graphics_widget::GraphicsWidget);
  impl<'a> ::qt_core::connection::Receiver for EnabledChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2enabledChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for EnabledChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsWidget::zChanged`.
  ///
  /// An object of this type can be created from `GraphicsWidget` with `object.signals().z_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsWidget` object.
  pub struct ZChanged<'a>(&'a ::graphics_widget::GraphicsWidget);
  impl<'a> ::qt_core::connection::Receiver for ZChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2zChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ZChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsWidget::heightChanged`.
  ///
  /// An object of this type can be created from `GraphicsWidget` with `object.signals().height_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsWidget` object.
  pub struct HeightChanged<'a>(&'a ::graphics_widget::GraphicsWidget);
  impl<'a> ::qt_core::connection::Receiver for HeightChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2heightChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for HeightChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsWidget::rotationChanged`.
  ///
  /// An object of this type can be created from `GraphicsWidget` with `object.signals().rotation_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsWidget` object.
  pub struct RotationChanged<'a>(&'a ::graphics_widget::GraphicsWidget);
  impl<'a> ::qt_core::connection::Receiver for RotationChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rotationChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RotationChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QGraphicsWidget::parentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn parent_changed(&self) -> ParentChanged {
      ParentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsWidget::visibleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn visible_changed(&self) -> VisibleChanged {
      VisibleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsWidget::scaleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scale_changed(&self) -> ScaleChanged {
      ScaleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsWidget::geometryChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn geometry_changed(&self) -> GeometryChanged {
      GeometryChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsWidget::layoutChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn layout_changed(&self) -> LayoutChanged {
      LayoutChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsWidget::xChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn x_changed(&self) -> XChanged {
      XChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsWidget::widthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn width_changed(&self) -> WidthChanged {
      WidthChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsWidget::yChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn y_changed(&self) -> YChanged {
      YChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsWidget::childrenChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn children_changed(&self) -> ChildrenChanged {
      ChildrenChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsWidget::opacityChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn opacity_changed(&self) -> OpacityChanged {
      OpacityChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsWidget::enabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enabled_changed(&self) -> EnabledChanged {
      EnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsWidget::zChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn z_changed(&self) -> ZChanged {
      ZChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsWidget::heightChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn height_changed(&self) -> HeightChanged {
      HeightChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsWidget::rotationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rotation_changed(&self) -> RotationChanged {
      RotationChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `GraphicsWidget`.
  pub struct Slots<'a>(&'a ::graphics_widget::GraphicsWidget);
  /// Represents a built-in Qt slot `QGraphicsWidget::close`.
  ///
  /// An object of this type can be created from `GraphicsWidget` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsWidget` object.
  pub struct Close<'a>(&'a ::graphics_widget::GraphicsWidget);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsWidget::updateMicroFocus`.
  ///
  /// An object of this type can be created from `GraphicsWidget` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsWidget` object.
  pub struct UpdateMicroFocus<'a>(&'a ::graphics_widget::GraphicsWidget);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QGraphicsWidget::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsWidget::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
  }
  impl ::graphics_widget::GraphicsWidget {
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

impl ::cpp_utils::DynamicCast<::graphics_widget::GraphicsWidget> for ::graphics_item::GraphicsItem {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_widget::GraphicsWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_G_dynamic_cast_QGraphicsWidget_ptr_QGraphicsItem(self as *mut ::graphics_item::GraphicsItem) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_widget::GraphicsWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_G_dynamic_cast_QGraphicsWidget_ptr_QGraphicsItem(self as *const ::graphics_item::GraphicsItem as *mut ::graphics_item::GraphicsItem) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::graphics_widget::GraphicsWidget> for ::graphics_layout_item::GraphicsLayoutItem {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_widget::GraphicsWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_G_dynamic_cast_QGraphicsWidget_ptr_QGraphicsLayoutItem(self as *mut ::graphics_layout_item::GraphicsLayoutItem) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_widget::GraphicsWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_G_dynamic_cast_QGraphicsWidget_ptr_QGraphicsLayoutItem(self as *const ::graphics_layout_item::GraphicsLayoutItem as *mut ::graphics_layout_item::GraphicsLayoutItem) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::graphics_widget::GraphicsWidget> for ::graphics_object::GraphicsObject {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_widget::GraphicsWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_G_dynamic_cast_QGraphicsWidget_ptr_QGraphicsObject(self as *mut ::graphics_object::GraphicsObject) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_widget::GraphicsWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_G_dynamic_cast_QGraphicsWidget_ptr_QGraphicsObject(self as *const ::graphics_object::GraphicsObject as *mut ::graphics_object::GraphicsObject) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::graphics_widget::GraphicsWidget {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsWidget_G_static_cast_QObject_ptr(self as *mut ::graphics_widget::GraphicsWidget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_G_static_cast_QObject_ptr(self as *const ::graphics_widget::GraphicsWidget as *mut ::graphics_widget::GraphicsWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::graphics_item::GraphicsItem> for ::graphics_widget::GraphicsWidget {
  fn static_cast_mut(&mut self) -> &mut ::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsItem_ptr(self as *mut ::graphics_widget::GraphicsWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsItem_ptr(self as *const ::graphics_widget::GraphicsWidget as *mut ::graphics_widget::GraphicsWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::graphics_layout_item::GraphicsLayoutItem> for ::graphics_widget::GraphicsWidget {
  fn static_cast_mut(&mut self) -> &mut ::graphics_layout_item::GraphicsLayoutItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsLayoutItem_ptr(self as *mut ::graphics_widget::GraphicsWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::graphics_layout_item::GraphicsLayoutItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsLayoutItem_ptr(self as *const ::graphics_widget::GraphicsWidget as *mut ::graphics_widget::GraphicsWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::graphics_object::GraphicsObject> for ::graphics_widget::GraphicsWidget {
  fn static_cast_mut(&mut self) -> &mut ::graphics_object::GraphicsObject {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsObject_ptr(self as *mut ::graphics_widget::GraphicsWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::graphics_object::GraphicsObject {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsObject_ptr(self as *const ::graphics_widget::GraphicsWidget as *mut ::graphics_widget::GraphicsWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_widget::GraphicsWidget> for ::graphics_item::GraphicsItem {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_widget::GraphicsWidget {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsWidget_ptr_QGraphicsItem(self as *mut ::graphics_item::GraphicsItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_widget::GraphicsWidget {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsWidget_ptr_QGraphicsItem(self as *const ::graphics_item::GraphicsItem as *mut ::graphics_item::GraphicsItem);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_widget::GraphicsWidget> for ::graphics_layout_item::GraphicsLayoutItem {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_widget::GraphicsWidget {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsWidget_ptr_QGraphicsLayoutItem(self as *mut ::graphics_layout_item::GraphicsLayoutItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_widget::GraphicsWidget {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsWidget_ptr_QGraphicsLayoutItem(self as *const ::graphics_layout_item::GraphicsLayoutItem as *mut ::graphics_layout_item::GraphicsLayoutItem);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_widget::GraphicsWidget> for ::graphics_object::GraphicsObject {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_widget::GraphicsWidget {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsWidget_ptr_QGraphicsObject(self as *mut ::graphics_object::GraphicsObject);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_widget::GraphicsWidget {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsWidget_ptr_QGraphicsObject(self as *const ::graphics_object::GraphicsObject as *mut ::graphics_object::GraphicsObject);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_widget::GraphicsWidget> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_widget::GraphicsWidget {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsWidget_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_widget::GraphicsWidget {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsWidget_G_static_cast_QGraphicsWidget_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GraphicsWidget::grab_shortcut](../struct.GraphicsWidget.html#method.grab_shortcut) method.
  pub trait GraphicsWidgetGrabShortcutArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_widget::GraphicsWidget) -> ::libc::c_int;
  }
  impl<'largs> GraphicsWidgetGrabShortcutArgs<'largs> for &'largs ::qt_gui::key_sequence::KeySequence {
    fn exec(self, original_self: &'largs mut ::graphics_widget::GraphicsWidget) -> ::libc::c_int {
      let sequence = self;
      unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_grabShortcut_sequence(original_self as *mut ::graphics_widget::GraphicsWidget, sequence as *const ::qt_gui::key_sequence::KeySequence) }
    }
  }
  impl<'largs> GraphicsWidgetGrabShortcutArgs<'largs>
    for (&'largs ::qt_gui::key_sequence::KeySequence, &'largs ::qt_core::qt::ShortcutContext) {
    fn exec(self, original_self: &'largs mut ::graphics_widget::GraphicsWidget) -> ::libc::c_int {
      let sequence = self.0;
      let context = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_grabShortcut_sequence_context(original_self as *mut ::graphics_widget::GraphicsWidget, sequence as *const ::qt_gui::key_sequence::KeySequence, context as *const ::qt_core::qt::ShortcutContext) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsWidget::paint](../struct.GraphicsWidget.html#method.paint) method.
  pub trait GraphicsWidgetPaintArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_widget::GraphicsWidget) -> ();
  }
  impl<'largs> GraphicsWidgetPaintArgs<'largs>
    for (*mut ::qt_gui::painter::Painter, *const ::style_option_graphics_item::StyleOptionGraphicsItem) {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_widget::GraphicsWidget) -> () {
      let painter = self.0;
      let option = self.1;
      ::ffi::qt_widgets_c_QGraphicsWidget_paint_painter_option(original_self as *mut ::graphics_widget::GraphicsWidget, painter, option)
    }
  }
  impl<'largs> GraphicsWidgetPaintArgs<'largs>
    for (*mut ::qt_gui::painter::Painter,
                                                        *const ::style_option_graphics_item::StyleOptionGraphicsItem,
                                                        *mut ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_widget::GraphicsWidget) -> () {
      let painter = self.0;
      let option = self.1;
      let widget = self.2;
      ::ffi::qt_widgets_c_QGraphicsWidget_paint_painter_option_widget(original_self as *mut ::graphics_widget::GraphicsWidget, painter, option, widget)
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsWidget::paint_window_frame](../struct.GraphicsWidget.html#method.paint_window_frame) method.
  pub trait GraphicsWidgetPaintWindowFrameArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_widget::GraphicsWidget) -> ();
  }
  impl<'largs> GraphicsWidgetPaintWindowFrameArgs<'largs> for (*mut ::qt_gui::painter::Painter,*const ::style_option_graphics_item::StyleOptionGraphicsItem) {

  unsafe fn exec(self, original_self: &'largs mut ::graphics_widget::GraphicsWidget) -> () {
    let painter = self.0;
let option = self.1;
    ::ffi::qt_widgets_c_QGraphicsWidget_paintWindowFrame_painter_option(original_self as *mut ::graphics_widget::GraphicsWidget, painter, option)
  }
}
  impl<'largs> GraphicsWidgetPaintWindowFrameArgs<'largs> for (*mut ::qt_gui::painter::Painter,*const ::style_option_graphics_item::StyleOptionGraphicsItem,*mut ::widget::Widget) {

  unsafe fn exec(self, original_self: &'largs mut ::graphics_widget::GraphicsWidget) -> () {
    let painter = self.0;
let option = self.1;
let widget = self.2;
    ::ffi::qt_widgets_c_QGraphicsWidget_paintWindowFrame_painter_option_widget(original_self as *mut ::graphics_widget::GraphicsWidget, painter, option, widget)
  }
}
  /// This trait represents a set of arguments accepted by [GraphicsWidget::resize](../struct.GraphicsWidget.html#method.resize) method.
  pub trait GraphicsWidgetResizeArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_widget::GraphicsWidget) -> ();
  }
  impl<'largs> GraphicsWidgetResizeArgs<'largs> for &'largs ::qt_core::size_f::SizeF {
    fn exec(self, original_self: &'largs mut ::graphics_widget::GraphicsWidget) -> () {
      let size = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsWidget_resize_size(original_self as *mut ::graphics_widget::GraphicsWidget,
                                                        size as *const ::qt_core::size_f::SizeF)
      }
    }
  }
  impl<'largs> GraphicsWidgetResizeArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::graphics_widget::GraphicsWidget) -> () {
      let w = self.0;
      let h = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsWidget_resize_w_h(original_self as *mut ::graphics_widget::GraphicsWidget,
                                                       w,
                                                       h)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsWidget::set_attribute](../struct.GraphicsWidget.html#method.set_attribute) method.
  pub trait GraphicsWidgetSetAttributeArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_widget::GraphicsWidget) -> ();
  }
  impl<'largs> GraphicsWidgetSetAttributeArgs<'largs> for &'largs ::qt_core::qt::WidgetAttribute {
    fn exec(self, original_self: &'largs mut ::graphics_widget::GraphicsWidget) -> () {
      let attribute = self;
      unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_setAttribute_attribute(original_self as *mut ::graphics_widget::GraphicsWidget, attribute as *const ::qt_core::qt::WidgetAttribute) }
    }
  }
  impl<'largs> GraphicsWidgetSetAttributeArgs<'largs> for (&'largs ::qt_core::qt::WidgetAttribute, bool) {
    fn exec(self, original_self: &'largs mut ::graphics_widget::GraphicsWidget) -> () {
      let attribute = self.0;
      let on = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_setAttribute_attribute_on(original_self as *mut ::graphics_widget::GraphicsWidget, attribute as *const ::qt_core::qt::WidgetAttribute, on) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsWidget::set_geometry](../struct.GraphicsWidget.html#method.set_geometry) method.
  pub trait GraphicsWidgetSetGeometryArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_widget::GraphicsWidget) -> ();
  }
  impl<'largs> GraphicsWidgetSetGeometryArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::graphics_widget::GraphicsWidget) -> () {
      let rect = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsWidget_setGeometry_rect(original_self as *mut ::graphics_widget::GraphicsWidget,
                                                             rect as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> GraphicsWidgetSetGeometryArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::graphics_widget::GraphicsWidget) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsWidget_setGeometry_x_y_w_h(original_self as *mut ::graphics_widget::GraphicsWidget, x, y, w, h)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsWidget::set_shortcut_auto_repeat](../struct.GraphicsWidget.html#method.set_shortcut_auto_repeat) method.
  pub trait GraphicsWidgetSetShortcutAutoRepeatArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_widget::GraphicsWidget) -> ();
  }
  impl<'largs> GraphicsWidgetSetShortcutAutoRepeatArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::graphics_widget::GraphicsWidget) -> () {
      let id = self;
      unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_setShortcutAutoRepeat_id(original_self as *mut ::graphics_widget::GraphicsWidget, id) }
    }
  }
  impl<'largs> GraphicsWidgetSetShortcutAutoRepeatArgs<'largs> for (::libc::c_int, bool) {
    fn exec(self, original_self: &'largs mut ::graphics_widget::GraphicsWidget) -> () {
      let id = self.0;
      let enabled = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_setShortcutAutoRepeat_id_enabled(original_self as *mut ::graphics_widget::GraphicsWidget, id, enabled) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsWidget::set_shortcut_enabled](../struct.GraphicsWidget.html#method.set_shortcut_enabled) method.
  pub trait GraphicsWidgetSetShortcutEnabledArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_widget::GraphicsWidget) -> ();
  }
  impl<'largs> GraphicsWidgetSetShortcutEnabledArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::graphics_widget::GraphicsWidget) -> () {
      let id = self;
      unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_setShortcutEnabled_id(original_self as *mut ::graphics_widget::GraphicsWidget, id) }
    }
  }
  impl<'largs> GraphicsWidgetSetShortcutEnabledArgs<'largs> for (::libc::c_int, bool) {
    fn exec(self, original_self: &'largs mut ::graphics_widget::GraphicsWidget) -> () {
      let id = self.0;
      let enabled = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsWidget_setShortcutEnabled_id_enabled(original_self as *mut ::graphics_widget::GraphicsWidget, id, enabled) }
    }
  }
}

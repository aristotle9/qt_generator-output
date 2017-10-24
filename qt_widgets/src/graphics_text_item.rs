/// C++ type: <span style='color: green;'>```QGraphicsTextItem```</span>
#[repr(C)]
pub struct GraphicsTextItem(u8);

impl GraphicsTextItem {
  /// C++ method: <span style='color: green;'>```void QGraphicsTextItem::adjustSize()```</span>
  ///
  ///
  pub fn adjust_size(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsTextItem_adjustSize(self as *mut ::graphics_text_item::GraphicsTextItem) }
  }

  /// C++ method: <span style='color: green;'>```virtual QRectF QGraphicsTextItem::boundingRect() const```</span>
  ///
  ///
  pub fn bounding_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsTextItem_boundingRect_to_output(self as *const ::graphics_text_item::GraphicsTextItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QGraphicsTextItem::contains(const QPointF& point) const```</span>
  ///
  ///
  pub fn contains(&self, point: &::qt_core::point_f::PointF) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsTextItem_contains(self as *const ::graphics_text_item::GraphicsTextItem,
                                                     point as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```QColor QGraphicsTextItem::defaultTextColor() const```</span>
  ///
  ///
  pub fn default_text_color(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsTextItem_defaultTextColor_to_output(self as *const ::graphics_text_item::GraphicsTextItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextDocument* QGraphicsTextItem::document() const```</span>
  ///
  ///
  pub fn document(&self) -> *mut ::qt_gui::text_document::TextDocument {
    unsafe { ::ffi::qt_widgets_c_QGraphicsTextItem_document(self as *const ::graphics_text_item::GraphicsTextItem) }
  }

  /// C++ method: <span style='color: green;'>```QFont QGraphicsTextItem::font() const```</span>
  ///
  ///
  pub fn font(&self) -> ::qt_gui::font::Font {
    {
      let mut object: ::qt_gui::font::Font =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsTextItem_font_to_output(self as *const ::graphics_text_item::GraphicsTextItem,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QGraphicsTextItem::isObscuredBy(const QGraphicsItem* item) const```</span>
  ///
  ///
  pub unsafe fn is_obscured_by(&self, item: *const ::graphics_item::GraphicsItem) -> bool {
    ::ffi::qt_widgets_c_QGraphicsTextItem_isObscuredBy(self as *const ::graphics_text_item::GraphicsTextItem, item)
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QGraphicsTextItem::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QGraphicsTextItem_metaObject(self as *const ::graphics_text_item::GraphicsTextItem) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsTextItem::QGraphicsTextItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::graphics_text_item::GraphicsTextItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsTextItem::QGraphicsTextItem()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::cpp_utils::CppBox<::graphics_text_item::GraphicsTextItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsTextItem::QGraphicsTextItem(const QString& text)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::graphics_text_item::GraphicsTextItem>
    where Args: overloading::GraphicsTextItemNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QGraphicsTextItem::QGraphicsTextItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::graphics_item::GraphicsItem) -> ::cpp_utils::CppBox<::graphics_text_item::GraphicsTextItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsTextItem::QGraphicsTextItem(QGraphicsItem* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::string::String, *mut ::graphics_item::GraphicsItem)) -> ::cpp_utils::CppBox<::graphics_text_item::GraphicsTextItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsTextItem::QGraphicsTextItem(const QString& text, QGraphicsItem* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::graphics_text_item::GraphicsTextItem>
    where Args: overloading::GraphicsTextItemNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual QPainterPath QGraphicsTextItem::opaqueArea() const```</span>
  ///
  ///
  pub fn opaque_area(&self) -> ::qt_gui::painter_path::PainterPath {
    {
      let mut object: ::qt_gui::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsTextItem_opaqueArea_to_output(self as *const ::graphics_text_item::GraphicsTextItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsTextItem::openExternalLinks() const```</span>
  ///
  ///
  pub fn open_external_links(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsTextItem_openExternalLinks(self as *const ::graphics_text_item::GraphicsTextItem)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QGraphicsTextItem::paint(QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn paint(&mut self,
                      painter: *mut ::qt_gui::painter::Painter,
                      option: *const ::style_option_graphics_item::StyleOptionGraphicsItem,
                      widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QGraphicsTextItem_paint(self as *mut ::graphics_text_item::GraphicsTextItem,
                                                painter,
                                                option,
                                                widget)
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsTextItem::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QGraphicsTextItem_qt_metacall(self as *mut ::graphics_text_item::GraphicsTextItem,
                                                      arg1 as *const ::qt_core::meta_object::Call,
                                                      arg2,
                                                      arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QGraphicsTextItem::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QGraphicsTextItem_qt_metacast(self as *mut ::graphics_text_item::GraphicsTextItem, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsTextItem::setDefaultTextColor(const QColor& c)```</span>
  ///
  ///
  pub fn set_default_text_color(&mut self, c: &::qt_gui::color::Color) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsTextItem_setDefaultTextColor(self as *mut ::graphics_text_item::GraphicsTextItem,
                                                                c as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsTextItem::setDocument(QTextDocument* document)```</span>
  ///
  ///
  pub unsafe fn set_document(&mut self, document: *mut ::qt_gui::text_document::TextDocument) {
    ::ffi::qt_widgets_c_QGraphicsTextItem_setDocument(self as *mut ::graphics_text_item::GraphicsTextItem,
                                                      document)
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsTextItem::setFont(const QFont& font)```</span>
  ///
  ///
  pub fn set_font(&mut self, font: &::qt_gui::font::Font) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsTextItem_setFont(self as *mut ::graphics_text_item::GraphicsTextItem,
                                                    font as *const ::qt_gui::font::Font)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsTextItem::setHtml(const QString& html)```</span>
  ///
  ///
  pub fn set_html(&mut self, html: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsTextItem_setHtml(self as *mut ::graphics_text_item::GraphicsTextItem,
                                                    html as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsTextItem::setOpenExternalLinks(bool open)```</span>
  ///
  ///
  pub fn set_open_external_links(&mut self, open: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsTextItem_setOpenExternalLinks(self as *mut ::graphics_text_item::GraphicsTextItem,
                                                                 open)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsTextItem::setPlainText(const QString& text)```</span>
  ///
  ///
  pub fn set_plain_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsTextItem_setPlainText(self as *mut ::graphics_text_item::GraphicsTextItem,
                                                         text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsTextItem::setTabChangesFocus(bool b)```</span>
  ///
  ///
  pub fn set_tab_changes_focus(&mut self, b: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsTextItem_setTabChangesFocus(self as *mut ::graphics_text_item::GraphicsTextItem, b)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsTextItem::setTextCursor(const QTextCursor& cursor)```</span>
  ///
  ///
  pub fn set_text_cursor(&mut self, cursor: &::qt_gui::text_cursor::TextCursor) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsTextItem_setTextCursor(self as *mut ::graphics_text_item::GraphicsTextItem,
                                                          cursor as *const ::qt_gui::text_cursor::TextCursor)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsTextItem::setTextWidth(double width)```</span>
  ///
  ///
  pub fn set_text_width(&mut self, width: ::libc::c_double) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsTextItem_setTextWidth(self as *mut ::graphics_text_item::GraphicsTextItem, width)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QPainterPath QGraphicsTextItem::shape() const```</span>
  ///
  ///
  pub fn shape(&self) -> ::qt_gui::painter_path::PainterPath {
    {
      let mut object: ::qt_gui::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsTextItem_shape_to_output(self as *const ::graphics_text_item::GraphicsTextItem,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsTextItem::tabChangesFocus() const```</span>
  ///
  ///
  pub fn tab_changes_focus(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsTextItem_tabChangesFocus(self as *const ::graphics_text_item::GraphicsTextItem)
    }
  }

  /// C++ method: <span style='color: green;'>```QTextCursor QGraphicsTextItem::textCursor() const```</span>
  ///
  ///
  pub fn text_cursor(&self) -> ::cpp_utils::CppBox<::qt_gui::text_cursor::TextCursor> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsTextItem_textCursor_as_ptr(self as *const ::graphics_text_item::GraphicsTextItem)
      };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsTextItem::textWidth() const```</span>
  ///
  ///
  pub fn text_width(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsTextItem_textWidth(self as *const ::graphics_text_item::GraphicsTextItem) }
  }

  /// C++ method: <span style='color: green;'>```QString QGraphicsTextItem::toHtml() const```</span>
  ///
  ///
  pub fn to_html(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsTextItem_toHtml_to_output(self as *const ::graphics_text_item::GraphicsTextItem,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QGraphicsTextItem::toPlainText() const```</span>
  ///
  ///
  pub fn to_plain_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsTextItem_toPlainText_to_output(self as *const ::graphics_text_item::GraphicsTextItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsTextItem::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsTextItem_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsTextItem::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsTextItem_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsTextItem::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QGraphicsTextItem_type(self as *const ::graphics_text_item::GraphicsTextItem) }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_text_item::GraphicsTextItem {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsTextItem_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `GraphicsTextItem`.
  pub struct Signals<'a>(&'a ::graphics_text_item::GraphicsTextItem);
  /// Represents a built-in Qt signal `QGraphicsTextItem::xChanged`.
  ///
  /// An object of this type can be created from `GraphicsTextItem` with `object.signals().x_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsTextItem` object.
  pub struct XChanged<'a>(&'a ::graphics_text_item::GraphicsTextItem);
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
  /// Represents a built-in Qt signal `QGraphicsTextItem::linkHovered`.
  ///
  /// An object of this type can be created from `GraphicsTextItem` with `object.signals().link_hovered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsTextItem` object.
  pub struct LinkHovered<'a>(&'a ::graphics_text_item::GraphicsTextItem);
  impl<'a> ::qt_core::connection::Receiver for LinkHovered<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2linkHovered(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for LinkHovered<'a> {}
  /// Represents a built-in Qt signal `QGraphicsTextItem::linkActivated`.
  ///
  /// An object of this type can be created from `GraphicsTextItem` with `object.signals().link_activated()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsTextItem` object.
  pub struct LinkActivated<'a>(&'a ::graphics_text_item::GraphicsTextItem);
  impl<'a> ::qt_core::connection::Receiver for LinkActivated<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2linkActivated(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for LinkActivated<'a> {}
  /// Represents a built-in Qt signal `QGraphicsTextItem::zChanged`.
  ///
  /// An object of this type can be created from `GraphicsTextItem` with `object.signals().z_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsTextItem` object.
  pub struct ZChanged<'a>(&'a ::graphics_text_item::GraphicsTextItem);
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
  /// Represents a built-in Qt signal `QGraphicsTextItem::heightChanged`.
  ///
  /// An object of this type can be created from `GraphicsTextItem` with `object.signals().height_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsTextItem` object.
  pub struct HeightChanged<'a>(&'a ::graphics_text_item::GraphicsTextItem);
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
  /// Represents a built-in Qt signal `QGraphicsTextItem::visibleChanged`.
  ///
  /// An object of this type can be created from `GraphicsTextItem` with `object.signals().visible_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsTextItem` object.
  pub struct VisibleChanged<'a>(&'a ::graphics_text_item::GraphicsTextItem);
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
  /// Represents a built-in Qt signal `QGraphicsTextItem::enabledChanged`.
  ///
  /// An object of this type can be created from `GraphicsTextItem` with `object.signals().enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsTextItem` object.
  pub struct EnabledChanged<'a>(&'a ::graphics_text_item::GraphicsTextItem);
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
  /// Represents a built-in Qt signal `QGraphicsTextItem::childrenChanged`.
  ///
  /// An object of this type can be created from `GraphicsTextItem` with `object.signals().children_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsTextItem` object.
  pub struct ChildrenChanged<'a>(&'a ::graphics_text_item::GraphicsTextItem);
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
  /// Represents a built-in Qt signal `QGraphicsTextItem::rotationChanged`.
  ///
  /// An object of this type can be created from `GraphicsTextItem` with `object.signals().rotation_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsTextItem` object.
  pub struct RotationChanged<'a>(&'a ::graphics_text_item::GraphicsTextItem);
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
  /// Represents a built-in Qt signal `QGraphicsTextItem::yChanged`.
  ///
  /// An object of this type can be created from `GraphicsTextItem` with `object.signals().y_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsTextItem` object.
  pub struct YChanged<'a>(&'a ::graphics_text_item::GraphicsTextItem);
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
  /// Represents a built-in Qt signal `QGraphicsTextItem::parentChanged`.
  ///
  /// An object of this type can be created from `GraphicsTextItem` with `object.signals().parent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsTextItem` object.
  pub struct ParentChanged<'a>(&'a ::graphics_text_item::GraphicsTextItem);
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
  /// Represents a built-in Qt signal `QGraphicsTextItem::opacityChanged`.
  ///
  /// An object of this type can be created from `GraphicsTextItem` with `object.signals().opacity_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsTextItem` object.
  pub struct OpacityChanged<'a>(&'a ::graphics_text_item::GraphicsTextItem);
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
  /// Represents a built-in Qt signal `QGraphicsTextItem::scaleChanged`.
  ///
  /// An object of this type can be created from `GraphicsTextItem` with `object.signals().scale_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsTextItem` object.
  pub struct ScaleChanged<'a>(&'a ::graphics_text_item::GraphicsTextItem);
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
  /// Represents a built-in Qt signal `QGraphicsTextItem::widthChanged`.
  ///
  /// An object of this type can be created from `GraphicsTextItem` with `object.signals().width_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsTextItem` object.
  pub struct WidthChanged<'a>(&'a ::graphics_text_item::GraphicsTextItem);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QGraphicsTextItem::xChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn x_changed(&self) -> XChanged {
      XChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsTextItem::linkHovered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn link_hovered(&self) -> LinkHovered {
      LinkHovered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsTextItem::linkActivated`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn link_activated(&self) -> LinkActivated {
      LinkActivated(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsTextItem::zChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn z_changed(&self) -> ZChanged {
      ZChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsTextItem::heightChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn height_changed(&self) -> HeightChanged {
      HeightChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsTextItem::visibleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn visible_changed(&self) -> VisibleChanged {
      VisibleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsTextItem::enabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enabled_changed(&self) -> EnabledChanged {
      EnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsTextItem::childrenChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn children_changed(&self) -> ChildrenChanged {
      ChildrenChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsTextItem::rotationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rotation_changed(&self) -> RotationChanged {
      RotationChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsTextItem::yChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn y_changed(&self) -> YChanged {
      YChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsTextItem::parentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn parent_changed(&self) -> ParentChanged {
      ParentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsTextItem::opacityChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn opacity_changed(&self) -> OpacityChanged {
      OpacityChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsTextItem::scaleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scale_changed(&self) -> ScaleChanged {
      ScaleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsTextItem::widthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn width_changed(&self) -> WidthChanged {
      WidthChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `GraphicsTextItem`.
  pub struct Slots<'a>(&'a ::graphics_text_item::GraphicsTextItem);
  /// Represents a built-in Qt slot `QGraphicsTextItem::updateMicroFocus`.
  ///
  /// An object of this type can be created from `GraphicsTextItem` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsTextItem` object.
  pub struct UpdateMicroFocus<'a>(&'a ::graphics_text_item::GraphicsTextItem);
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
    /// Returns an object representing a built-in Qt slot `QGraphicsTextItem::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
  }
  impl ::graphics_text_item::GraphicsTextItem {
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

impl ::cpp_utils::DynamicCast<::graphics_text_item::GraphicsTextItem> for ::graphics_item::GraphicsItem {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_text_item::GraphicsTextItem> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsTextItem_G_dynamic_cast_QGraphicsTextItem_ptr_QGraphicsItem(self as *mut ::graphics_item::GraphicsItem) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_text_item::GraphicsTextItem> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsTextItem_G_dynamic_cast_QGraphicsTextItem_ptr_QGraphicsItem(self as *const ::graphics_item::GraphicsItem as *mut ::graphics_item::GraphicsItem) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::graphics_text_item::GraphicsTextItem> for ::graphics_object::GraphicsObject {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_text_item::GraphicsTextItem> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsTextItem_G_dynamic_cast_QGraphicsTextItem_ptr_QGraphicsObject(self as *mut ::graphics_object::GraphicsObject) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_text_item::GraphicsTextItem> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsTextItem_G_dynamic_cast_QGraphicsTextItem_ptr_QGraphicsObject(self as *const ::graphics_object::GraphicsObject as *mut ::graphics_object::GraphicsObject) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::graphics_text_item::GraphicsTextItem {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsTextItem_G_static_cast_QObject_ptr(self as *mut ::graphics_text_item::GraphicsTextItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsTextItem_G_static_cast_QObject_ptr(self as *const ::graphics_text_item::GraphicsTextItem as *mut ::graphics_text_item::GraphicsTextItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::graphics_item::GraphicsItem> for ::graphics_text_item::GraphicsTextItem {
  fn static_cast_mut(&mut self) -> &mut ::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsTextItem_G_static_cast_QGraphicsItem_ptr(self as *mut ::graphics_text_item::GraphicsTextItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsTextItem_G_static_cast_QGraphicsItem_ptr(self as *const ::graphics_text_item::GraphicsTextItem as *mut ::graphics_text_item::GraphicsTextItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::graphics_object::GraphicsObject> for ::graphics_text_item::GraphicsTextItem {
  fn static_cast_mut(&mut self) -> &mut ::graphics_object::GraphicsObject {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsTextItem_G_static_cast_QGraphicsObject_ptr(self as *mut ::graphics_text_item::GraphicsTextItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::graphics_object::GraphicsObject {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsTextItem_G_static_cast_QGraphicsObject_ptr(self as *const ::graphics_text_item::GraphicsTextItem as *mut ::graphics_text_item::GraphicsTextItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_text_item::GraphicsTextItem> for ::graphics_item::GraphicsItem {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_text_item::GraphicsTextItem {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsTextItem_G_static_cast_QGraphicsTextItem_ptr_QGraphicsItem(self as *mut ::graphics_item::GraphicsItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_text_item::GraphicsTextItem {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsTextItem_G_static_cast_QGraphicsTextItem_ptr_QGraphicsItem(self as *const ::graphics_item::GraphicsItem as *mut ::graphics_item::GraphicsItem);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_text_item::GraphicsTextItem> for ::graphics_object::GraphicsObject {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_text_item::GraphicsTextItem {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsTextItem_G_static_cast_QGraphicsTextItem_ptr_QGraphicsObject(self as *mut ::graphics_object::GraphicsObject);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_text_item::GraphicsTextItem {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsTextItem_G_static_cast_QGraphicsTextItem_ptr_QGraphicsObject(self as *const ::graphics_object::GraphicsObject as *mut ::graphics_object::GraphicsObject);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_text_item::GraphicsTextItem> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_text_item::GraphicsTextItem {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsTextItem_G_static_cast_QGraphicsTextItem_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_text_item::GraphicsTextItem {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsTextItem_G_static_cast_QGraphicsTextItem_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::graphics_text_item::GraphicsTextItem {
  type Target = ::graphics_object::GraphicsObject;
  fn deref(&self) -> &::graphics_object::GraphicsObject {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsTextItem_G_static_cast_QGraphicsObject_ptr(self as *const ::graphics_text_item::GraphicsTextItem as *mut ::graphics_text_item::GraphicsTextItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_text_item::GraphicsTextItem {
  fn deref_mut(&mut self) -> &mut ::graphics_object::GraphicsObject {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsTextItem_G_static_cast_QGraphicsObject_ptr(self as *mut ::graphics_text_item::GraphicsTextItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GraphicsTextItem::new](../struct.GraphicsTextItem.html#method.new) method.
  pub trait GraphicsTextItemNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_text_item::GraphicsTextItem>;
  }
  impl GraphicsTextItemNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_text_item::GraphicsTextItem> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsTextItem_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> GraphicsTextItemNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_text_item::GraphicsTextItem> {
      let text = self;
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_QGraphicsTextItem_new_text(text as *const ::qt_core::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsTextItem::new_unsafe](../struct.GraphicsTextItem.html#method.new_unsafe) method.
  pub trait GraphicsTextItemNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_text_item::GraphicsTextItem>;
  }
  impl GraphicsTextItemNewUnsafeArgs for *mut ::graphics_item::GraphicsItem {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_text_item::GraphicsTextItem> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QGraphicsTextItem_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> GraphicsTextItemNewUnsafeArgs for (&'a ::qt_core::string::String, *mut ::graphics_item::GraphicsItem) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_text_item::GraphicsTextItem> {
      let text = self.0;
      let parent = self.1;
      let ffi_result =
        ::ffi::qt_widgets_c_QGraphicsTextItem_new_text_parent(text as *const ::qt_core::string::String, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}

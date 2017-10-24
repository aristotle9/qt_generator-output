/// C++ type: <span style='color: green;'>```QAbstractTextDocumentLayout```</span>
#[repr(C)]
pub struct AbstractTextDocumentLayout(u8);

impl AbstractTextDocumentLayout {
  /// C++ method: <span style='color: green;'>```QString QAbstractTextDocumentLayout::anchorAt(const QPointF& pos) const```</span>
  ///
  ///
  pub fn anchor_at(&self, pos: &::qt_core::point_f::PointF) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAbstractTextDocumentLayout_anchorAt_to_output(self as *const ::abstract_text_document_layout::AbstractTextDocumentLayout, pos as *const ::qt_core::point_f::PointF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QRectF QAbstractTextDocumentLayout::blockBoundingRect(const QTextBlock& block) const```</span>
  ///
  ///
  pub fn block_bounding_rect(&self, block: &::text_block::TextBlock) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAbstractTextDocumentLayout_blockBoundingRect_to_output(self as *const ::abstract_text_document_layout::AbstractTextDocumentLayout, block as *const ::text_block::TextBlock, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextDocument* QAbstractTextDocumentLayout::document() const```</span>
  ///
  ///
  pub fn document(&self) -> *mut ::text_document::TextDocument {
    unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_document(self as *const ::abstract_text_document_layout::AbstractTextDocumentLayout) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QSizeF QAbstractTextDocumentLayout::documentSize() const```</span>
  ///
  ///
  pub fn document_size(&self) -> ::qt_core::size_f::SizeF {
    {
      let mut object: ::qt_core::size_f::SizeF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAbstractTextDocumentLayout_documentSize_to_output(self as *const ::abstract_text_document_layout::AbstractTextDocumentLayout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual void QAbstractTextDocumentLayout::draw(QPainter* painter, const QAbstractTextDocumentLayout::PaintContext& context)```</span>
  ///
  ///
  pub unsafe fn draw(&mut self,
                     painter: *mut ::painter::Painter,
                     context: &::abstract_text_document_layout::PaintContext) {
    ::ffi::qt_gui_c_QAbstractTextDocumentLayout_draw(self as *mut ::abstract_text_document_layout::AbstractTextDocumentLayout, painter, context as *const ::abstract_text_document_layout::PaintContext)
  }

  /// C++ method: <span style='color: green;'>```QTextFormat QAbstractTextDocumentLayout::formatAt(const QPointF& pos) const```</span>
  ///
  ///
  pub fn format_at(&self, pos: &::qt_core::point_f::PointF) -> ::text_format::TextFormat {
    {
      let mut object: ::text_format::TextFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAbstractTextDocumentLayout_formatAt_to_output(self as *const ::abstract_text_document_layout::AbstractTextDocumentLayout, pos as *const ::qt_core::point_f::PointF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QRectF QAbstractTextDocumentLayout::frameBoundingRect(QTextFrame* frame) const```</span>
  ///
  ///
  pub unsafe fn frame_bounding_rect(&self, frame: *mut ::text_frame::TextFrame) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QAbstractTextDocumentLayout_frameBoundingRect_to_output(self as *const ::abstract_text_document_layout::AbstractTextDocumentLayout, frame, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextObjectInterface* QAbstractTextDocumentLayout::handlerForObject(int objectType) const```</span>
  ///
  ///
  pub fn handler_for_object(&self, object_type: ::libc::c_int) -> *mut ::text_object_interface::TextObjectInterface {
    unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_handlerForObject(self as *const ::abstract_text_document_layout::AbstractTextDocumentLayout, object_type) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual int QAbstractTextDocumentLayout::hitTest(const QPointF& point, Qt::HitTestAccuracy accuracy) const```</span>
  ///
  ///
  pub fn hit_test(&self,
                  point: &::qt_core::point_f::PointF,
                  accuracy: &::qt_core::qt::HitTestAccuracy)
                  -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_hitTest(self as *const ::abstract_text_document_layout::AbstractTextDocumentLayout, point as *const ::qt_core::point_f::PointF, accuracy as *const ::qt_core::qt::HitTestAccuracy) }
  }

  /// C++ method: <span style='color: green;'>```QString QAbstractTextDocumentLayout::imageAt(const QPointF& pos) const```</span>
  ///
  ///
  pub fn image_at(&self, pos: &::qt_core::point_f::PointF) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAbstractTextDocumentLayout_imageAt_to_output(self as *const ::abstract_text_document_layout::AbstractTextDocumentLayout, pos as *const ::qt_core::point_f::PointF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QAbstractTextDocumentLayout::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_metaObject(self as *const ::abstract_text_document_layout::AbstractTextDocumentLayout) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual int QAbstractTextDocumentLayout::pageCount() const```</span>
  ///
  ///
  pub fn page_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_pageCount(self as *const ::abstract_text_document_layout::AbstractTextDocumentLayout) }
  }

  /// C++ method: <span style='color: green;'>```QPaintDevice* QAbstractTextDocumentLayout::paintDevice() const```</span>
  ///
  ///
  pub fn paint_device(&self) -> *mut ::paint_device::PaintDevice {
    unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_paintDevice(self as *const ::abstract_text_document_layout::AbstractTextDocumentLayout) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QAbstractTextDocumentLayout::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QAbstractTextDocumentLayout_qt_metacall(self as *mut ::abstract_text_document_layout::AbstractTextDocumentLayout, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QAbstractTextDocumentLayout::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QAbstractTextDocumentLayout_qt_metacast(self as *mut ::abstract_text_document_layout::AbstractTextDocumentLayout, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QAbstractTextDocumentLayout::registerHandler(int objectType, QObject* component)```</span>
  ///
  ///
  pub unsafe fn register_handler(&mut self, object_type: ::libc::c_int, component: *mut ::qt_core::object::Object) {
    ::ffi::qt_gui_c_QAbstractTextDocumentLayout_registerHandler(self as *mut ::abstract_text_document_layout::AbstractTextDocumentLayout, object_type, component)
  }

  /// C++ method: <span style='color: green;'>```void QAbstractTextDocumentLayout::setPaintDevice(QPaintDevice* device)```</span>
  ///
  ///
  pub unsafe fn set_paint_device(&mut self, device: *mut ::paint_device::PaintDevice) {
    ::ffi::qt_gui_c_QAbstractTextDocumentLayout_setPaintDevice(self as *mut ::abstract_text_document_layout::AbstractTextDocumentLayout, device)
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractTextDocumentLayout::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QAbstractTextDocumentLayout_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractTextDocumentLayout::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QAbstractTextDocumentLayout_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractTextDocumentLayout::unregisterHandler(int objectType)```</span>
  ///
  ///
  pub fn unregister_handler(&mut self, object_type: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_unregisterHandler_objectType(self as *mut ::abstract_text_document_layout::AbstractTextDocumentLayout, object_type) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractTextDocumentLayout::unregisterHandler(int objectType, QObject* component = ?)```</span>
  ///
  ///
  pub unsafe fn unregister_handler_unsafe(&mut self,
                                          object_type: ::libc::c_int,
                                          component: *mut ::qt_core::object::Object) {
    ::ffi::qt_gui_c_QAbstractTextDocumentLayout_unregisterHandler_objectType_component(self as *mut ::abstract_text_document_layout::AbstractTextDocumentLayout, object_type, component)
  }
}

impl ::cpp_utils::CppDeletable for ::abstract_text_document_layout::AbstractTextDocumentLayout {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QAbstractTextDocumentLayout_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `AbstractTextDocumentLayout`.
  pub struct Signals<'a>(&'a ::abstract_text_document_layout::AbstractTextDocumentLayout);
  /// Represents a built-in Qt signal `QAbstractTextDocumentLayout::pageCountChanged`.
  ///
  /// An object of this type can be created from `AbstractTextDocumentLayout` with `object.signals().page_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTextDocumentLayout` object.
  pub struct PageCountChanged<'a>(&'a ::abstract_text_document_layout::AbstractTextDocumentLayout);
  impl<'a> ::qt_core::connection::Receiver for PageCountChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2pageCountChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for PageCountChanged<'a> {}
  /// Represents a built-in Qt signal `QAbstractTextDocumentLayout::documentSizeChanged`.
  ///
  /// An object of this type can be created from `AbstractTextDocumentLayout` with `object.signals().document_size_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTextDocumentLayout` object.
  pub struct DocumentSizeChanged<'a>(&'a ::abstract_text_document_layout::AbstractTextDocumentLayout);
  impl<'a> ::qt_core::connection::Receiver for DocumentSizeChanged<'a> {
    type Arguments = (&'static ::qt_core::size_f::SizeF,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2documentSizeChanged(const QSizeF&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DocumentSizeChanged<'a> {}
  /// Represents a built-in Qt signal `QAbstractTextDocumentLayout::objectNameChanged`.
  ///
  /// An object of this type can be created from `AbstractTextDocumentLayout` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTextDocumentLayout` object.
  pub struct ObjectNameChanged<'a>(&'a ::abstract_text_document_layout::AbstractTextDocumentLayout);
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
  /// Represents a built-in Qt signal `QAbstractTextDocumentLayout::update`.
  ///
  /// An object of this type can be created from `AbstractTextDocumentLayout` with `object.signals().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTextDocumentLayout` object.
  pub struct Update<'a>(&'a ::abstract_text_document_layout::AbstractTextDocumentLayout);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = (&'static ::qt_core::rect_f::RectF,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2update(const QRectF&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Update<'a> {}
  /// Represents a built-in Qt signal `QAbstractTextDocumentLayout::updateBlock`.
  ///
  /// An object of this type can be created from `AbstractTextDocumentLayout` with `object.signals().update_block()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTextDocumentLayout` object.
  pub struct UpdateBlock<'a>(&'a ::abstract_text_document_layout::AbstractTextDocumentLayout);
  impl<'a> ::qt_core::connection::Receiver for UpdateBlock<'a> {
    type Arguments = (&'static ::text_block::TextBlock,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2updateBlock(const QTextBlock&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for UpdateBlock<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QAbstractTextDocumentLayout::pageCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn page_count_changed(&self) -> PageCountChanged {
      PageCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractTextDocumentLayout::documentSizeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn document_size_changed(&self) -> DocumentSizeChanged {
      DocumentSizeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractTextDocumentLayout::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractTextDocumentLayout::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractTextDocumentLayout::updateBlock`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_block(&self) -> UpdateBlock {
      UpdateBlock(self.0)
    }
  }
  impl ::abstract_text_document_layout::AbstractTextDocumentLayout {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

/// C++ type: <span style='color: green;'>```QAbstractTextDocumentLayout::PaintContext```</span>
#[repr(C)]
pub struct PaintContext([u8; ::type_sizes::QT_GUI_ABSTRACT_TEXT_DOCUMENT_LAYOUT_PAINT_CONTEXT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for PaintContext {
  unsafe fn new_uninitialized() -> PaintContext {
    PaintContext(::std::mem::uninitialized())
  }
}

impl PaintContext {
  /// C++ method: <span style='color: green;'>```const QRectF& QAbstractTextDocumentLayout::PaintContext::clip() const```</span>
  ///
  ///
  pub fn clip<'l0>(&'l0 self) -> &'l0 ::qt_core::rect_f::RectF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_PaintContext_clip(self as *const ::abstract_text_document_layout::PaintContext) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRectF& QAbstractTextDocumentLayout::PaintContext::clip_mut()```</span>
  ///
  ///
  pub fn clip_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::rect_f::RectF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_PaintContext_clip_mut(self as *mut ::abstract_text_document_layout::PaintContext) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QAbstractTextDocumentLayout::PaintContext::cursorPosition() const```</span>
  ///
  ///
  pub fn cursor_position(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_PaintContext_cursorPosition(self as *const ::abstract_text_document_layout::PaintContext) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QAbstractTextDocumentLayout::PaintContext::PaintContext()```</span>
  ///
  ///
  pub fn new() -> ::abstract_text_document_layout::PaintContext {
    {
      let mut object: ::abstract_text_document_layout::PaintContext =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAbstractTextDocumentLayout_PaintContext_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QPalette& QAbstractTextDocumentLayout::PaintContext::palette() const```</span>
  ///
  ///
  pub fn palette<'l0>(&'l0 self) -> &'l0 ::palette::Palette {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_PaintContext_palette(self as *const ::abstract_text_document_layout::PaintContext) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPalette& QAbstractTextDocumentLayout::PaintContext::palette_mut()```</span>
  ///
  ///
  pub fn palette_mut<'l0>(&'l0 mut self) -> &'l0 mut ::palette::Palette {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_PaintContext_palette_mut(self as *mut ::abstract_text_document_layout::PaintContext) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QVector<QAbstractTextDocumentLayout::Selection>& QAbstractTextDocumentLayout::PaintContext::selections() const```</span>
  ///
  ///
  pub fn selections<'l0>(&'l0 self) -> &'l0 ::vector::VectorAbstractTextDocumentLayoutSelection {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_PaintContext_selections(self as *const ::abstract_text_document_layout::PaintContext) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QAbstractTextDocumentLayout::Selection>& QAbstractTextDocumentLayout::PaintContext::selections_mut()```</span>
  ///
  ///
  pub fn selections_mut<'l0>(&'l0 mut self) -> &'l0 mut ::vector::VectorAbstractTextDocumentLayoutSelection {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_PaintContext_selections_mut(self as *mut ::abstract_text_document_layout::PaintContext) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QAbstractTextDocumentLayout::PaintContext::set_clip(QRectF value)```</span>
  ///
  ///
  pub fn set_clip(&mut self, value: &::qt_core::rect_f::RectF) {
    unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_PaintContext_set_clip(self as *mut ::abstract_text_document_layout::PaintContext, value as *const ::qt_core::rect_f::RectF) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractTextDocumentLayout::PaintContext::set_cursorPosition(int value)```</span>
  ///
  ///
  pub fn set_cursor_position(&mut self, value: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_PaintContext_set_cursorPosition(self as *mut ::abstract_text_document_layout::PaintContext, value) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractTextDocumentLayout::PaintContext::set_palette(QPalette value)```</span>
  ///
  ///
  pub fn set_palette(&mut self, value: &::palette::Palette) {
    unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_PaintContext_set_palette(self as *mut ::abstract_text_document_layout::PaintContext, value as *const ::palette::Palette) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractTextDocumentLayout::PaintContext::set_selections(QVector<QAbstractTextDocumentLayout::Selection> value)```</span>
  ///
  ///
  pub fn set_selections(&mut self, value: &::vector::VectorAbstractTextDocumentLayoutSelection) {
    unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_PaintContext_set_selections(self as *mut ::abstract_text_document_layout::PaintContext, value as *const ::vector::VectorAbstractTextDocumentLayoutSelection) }
  }
}

impl Drop for ::abstract_text_document_layout::PaintContext {
  /// C++ method: <span style='color: green;'>```[destructor] void QAbstractTextDocumentLayout::PaintContext::~QAbstractTextDocumentLayout::PaintContext()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_PaintContext_destructor(self as *mut ::abstract_text_document_layout::PaintContext) }
  }
}

/// C++ type: <span style='color: green;'>```QAbstractTextDocumentLayout::Selection```</span>
#[repr(C)]
pub struct Selection(u8);

impl Selection {
  /// C++ method: <span style='color: green;'>```const QTextCursor& QAbstractTextDocumentLayout::Selection::cursor() const```</span>
  ///
  ///
  pub fn cursor<'l0>(&'l0 self) -> &'l0 ::text_cursor::TextCursor {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_Selection_cursor(self as *const ::abstract_text_document_layout::Selection) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextCursor& QAbstractTextDocumentLayout::Selection::cursor_mut()```</span>
  ///
  ///
  pub fn cursor_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_cursor::TextCursor {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_Selection_cursor_mut(self as *mut ::abstract_text_document_layout::Selection) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTextCharFormat& QAbstractTextDocumentLayout::Selection::format() const```</span>
  ///
  ///
  pub fn format<'l0>(&'l0 self) -> &'l0 ::text_char_format::TextCharFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_Selection_format(self as *const ::abstract_text_document_layout::Selection) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextCharFormat& QAbstractTextDocumentLayout::Selection::format_mut()```</span>
  ///
  ///
  pub fn format_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_char_format::TextCharFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_Selection_format_mut(self as *mut ::abstract_text_document_layout::Selection) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QAbstractTextDocumentLayout::Selection::set_cursor(QTextCursor value)```</span>
  ///
  ///
  pub fn set_cursor(&mut self, value: &::text_cursor::TextCursor) {
    unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_Selection_set_cursor(self as *mut ::abstract_text_document_layout::Selection, value as *const ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractTextDocumentLayout::Selection::set_format(QTextCharFormat value)```</span>
  ///
  ///
  pub fn set_format(&mut self, value: &::text_char_format::TextCharFormat) {
    unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_Selection_set_format(self as *mut ::abstract_text_document_layout::Selection, value as *const ::text_char_format::TextCharFormat) }
  }
}

impl ::cpp_utils::CppDeletable for ::abstract_text_document_layout::Selection {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QAbstractTextDocumentLayout_Selection_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::abstract_text_document_layout::AbstractTextDocumentLayout {
fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_G_static_cast_QObject_ptr(self as *mut ::abstract_text_document_layout::AbstractTextDocumentLayout) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_G_static_cast_QObject_ptr(self as *const ::abstract_text_document_layout::AbstractTextDocumentLayout as *mut ::abstract_text_document_layout::AbstractTextDocumentLayout) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::abstract_text_document_layout::AbstractTextDocumentLayout> for ::qt_core::object::Object {
unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_text_document_layout::AbstractTextDocumentLayout {
let ffi_result = ::ffi::qt_gui_c_QAbstractTextDocumentLayout_G_static_cast_QAbstractTextDocumentLayout_ptr(self as *mut ::qt_core::object::Object);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::abstract_text_document_layout::AbstractTextDocumentLayout {
let ffi_result = ::ffi::qt_gui_c_QAbstractTextDocumentLayout_G_static_cast_QAbstractTextDocumentLayout_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::abstract_text_document_layout::AbstractTextDocumentLayout {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_G_static_cast_QObject_ptr(self as *const ::abstract_text_document_layout::AbstractTextDocumentLayout as *mut ::abstract_text_document_layout::AbstractTextDocumentLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::abstract_text_document_layout::AbstractTextDocumentLayout {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAbstractTextDocumentLayout_G_static_cast_QObject_ptr(self as *mut ::abstract_text_document_layout::AbstractTextDocumentLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

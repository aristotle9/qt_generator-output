/// C++ type: <span style='color: green;'>```QPlainTextDocumentLayout```</span>
#[repr(C)]
pub struct PlainTextDocumentLayout(u8);

impl PlainTextDocumentLayout {
  /// C++ method: <span style='color: green;'>```virtual QRectF QPlainTextDocumentLayout::blockBoundingRect(const QTextBlock& block) const```</span>
  ///
  ///
  pub fn block_bounding_rect(&self, block: &::qt_gui::text_block::TextBlock) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QPlainTextDocumentLayout_blockBoundingRect_to_output(self as *const ::plain_text_document_layout::PlainTextDocumentLayout, block as *const ::qt_gui::text_block::TextBlock, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QPlainTextDocumentLayout::cursorWidth() const```</span>
  ///
  ///
  pub fn cursor_width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QPlainTextDocumentLayout_cursorWidth(self as *const ::plain_text_document_layout::PlainTextDocumentLayout) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSizeF QPlainTextDocumentLayout::documentSize() const```</span>
  ///
  ///
  pub fn document_size(&self) -> ::qt_core::size_f::SizeF {
    {
      let mut object: ::qt_core::size_f::SizeF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QPlainTextDocumentLayout_documentSize_to_output(self as *const ::plain_text_document_layout::PlainTextDocumentLayout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QPlainTextDocumentLayout::draw(QPainter* arg1, const QAbstractTextDocumentLayout::PaintContext& arg2)```</span>
  ///
  ///
  pub unsafe fn draw(&mut self,
                     arg1: *mut ::qt_gui::painter::Painter,
                     arg2: &::qt_gui::abstract_text_document_layout::PaintContext) {
    ::ffi::qt_widgets_c_QPlainTextDocumentLayout_draw(self as *mut ::plain_text_document_layout::PlainTextDocumentLayout, arg1, arg2 as *const ::qt_gui::abstract_text_document_layout::PaintContext)
  }

  /// C++ method: <span style='color: green;'>```void QPlainTextDocumentLayout::ensureBlockLayout(const QTextBlock& block) const```</span>
  ///
  ///
  pub fn ensure_block_layout(&self, block: &::qt_gui::text_block::TextBlock) {
    unsafe { ::ffi::qt_widgets_c_QPlainTextDocumentLayout_ensureBlockLayout(self as *const ::plain_text_document_layout::PlainTextDocumentLayout, block as *const ::qt_gui::text_block::TextBlock) }
  }

  /// C++ method: <span style='color: green;'>```virtual QRectF QPlainTextDocumentLayout::frameBoundingRect(QTextFrame* arg1) const```</span>
  ///
  ///
  pub unsafe fn frame_bounding_rect(&self, arg1: *mut ::qt_gui::text_frame::TextFrame) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QPlainTextDocumentLayout_frameBoundingRect_to_output(self as *const ::plain_text_document_layout::PlainTextDocumentLayout, arg1, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QPlainTextDocumentLayout::hitTest(const QPointF& arg1, Qt::HitTestAccuracy arg2) const```</span>
  ///
  ///
  pub fn hit_test(&self, arg1: &::qt_core::point_f::PointF, arg2: &::qt_core::qt::HitTestAccuracy) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QPlainTextDocumentLayout_hitTest(self as *const ::plain_text_document_layout::PlainTextDocumentLayout, arg1 as *const ::qt_core::point_f::PointF, arg2 as *const ::qt_core::qt::HitTestAccuracy) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QPlainTextDocumentLayout::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QPlainTextDocumentLayout_metaObject(self as *const ::plain_text_document_layout::PlainTextDocumentLayout) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QPlainTextDocumentLayout::QPlainTextDocumentLayout(QTextDocument* document)```</span>
  ///
  ///
  pub unsafe fn new(document: *mut ::qt_gui::text_document::TextDocument)
                    -> ::cpp_utils::CppBox<::plain_text_document_layout::PlainTextDocumentLayout> {
    let ffi_result = ::ffi::qt_widgets_c_QPlainTextDocumentLayout_new(document);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QPlainTextDocumentLayout::pageCount() const```</span>
  ///
  ///
  pub fn page_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QPlainTextDocumentLayout_pageCount(self as *const ::plain_text_document_layout::PlainTextDocumentLayout) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QPlainTextDocumentLayout::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QPlainTextDocumentLayout_qt_metacall(self as *mut ::plain_text_document_layout::PlainTextDocumentLayout, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QPlainTextDocumentLayout::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QPlainTextDocumentLayout_qt_metacast(self as *mut ::plain_text_document_layout::PlainTextDocumentLayout, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QPlainTextDocumentLayout::requestUpdate()```</span>
  ///
  ///
  pub fn request_update(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QPlainTextDocumentLayout_requestUpdate(self as *mut ::plain_text_document_layout::PlainTextDocumentLayout) }
  }

  /// C++ method: <span style='color: green;'>```void QPlainTextDocumentLayout::setCursorWidth(int width)```</span>
  ///
  ///
  pub fn set_cursor_width(&mut self, width: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QPlainTextDocumentLayout_setCursorWidth(self as *mut ::plain_text_document_layout::PlainTextDocumentLayout, width) }
  }

  /// C++ method: <span style='color: green;'>```static QString QPlainTextDocumentLayout::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QPlainTextDocumentLayout_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QPlainTextDocumentLayout::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QPlainTextDocumentLayout_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::plain_text_document_layout::PlainTextDocumentLayout {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QPlainTextDocumentLayout_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `PlainTextDocumentLayout`.
  pub struct Signals<'a>(&'a ::plain_text_document_layout::PlainTextDocumentLayout);
  /// Represents a built-in Qt signal `QPlainTextDocumentLayout::update`.
  ///
  /// An object of this type can be created from `PlainTextDocumentLayout` with `object.signals().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextDocumentLayout` object.
  pub struct Update<'a>(&'a ::plain_text_document_layout::PlainTextDocumentLayout);
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
  /// Represents a built-in Qt signal `QPlainTextDocumentLayout::pageCountChanged`.
  ///
  /// An object of this type can be created from `PlainTextDocumentLayout` with `object.signals().page_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextDocumentLayout` object.
  pub struct PageCountChanged<'a>(&'a ::plain_text_document_layout::PlainTextDocumentLayout);
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
  /// Represents a built-in Qt signal `QPlainTextDocumentLayout::documentSizeChanged`.
  ///
  /// An object of this type can be created from `PlainTextDocumentLayout` with `object.signals().document_size_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextDocumentLayout` object.
  pub struct DocumentSizeChanged<'a>(&'a ::plain_text_document_layout::PlainTextDocumentLayout);
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
  /// Represents a built-in Qt signal `QPlainTextDocumentLayout::updateBlock`.
  ///
  /// An object of this type can be created from `PlainTextDocumentLayout` with `object.signals().update_block()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextDocumentLayout` object.
  pub struct UpdateBlock<'a>(&'a ::plain_text_document_layout::PlainTextDocumentLayout);
  impl<'a> ::qt_core::connection::Receiver for UpdateBlock<'a> {
    type Arguments = (&'static ::qt_gui::text_block::TextBlock,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2updateBlock(const QTextBlock&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for UpdateBlock<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QPlainTextDocumentLayout::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPlainTextDocumentLayout::pageCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn page_count_changed(&self) -> PageCountChanged {
      PageCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPlainTextDocumentLayout::documentSizeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn document_size_changed(&self) -> DocumentSizeChanged {
      DocumentSizeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPlainTextDocumentLayout::updateBlock`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_block(&self) -> UpdateBlock {
      UpdateBlock(self.0)
    }
  }
  impl ::plain_text_document_layout::PlainTextDocumentLayout {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::plain_text_document_layout::PlainTextDocumentLayout {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPlainTextDocumentLayout_G_static_cast_QObject_ptr(self as *mut ::plain_text_document_layout::PlainTextDocumentLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPlainTextDocumentLayout_G_static_cast_QObject_ptr(self as *const ::plain_text_document_layout::PlainTextDocumentLayout as *mut ::plain_text_document_layout::PlainTextDocumentLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::abstract_text_document_layout::AbstractTextDocumentLayout> for ::plain_text_document_layout::PlainTextDocumentLayout {
fn static_cast_mut(&mut self) -> &mut ::qt_gui::abstract_text_document_layout::AbstractTextDocumentLayout {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QPlainTextDocumentLayout_G_static_cast_QAbstractTextDocumentLayout_ptr(self as *mut ::plain_text_document_layout::PlainTextDocumentLayout) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_gui::abstract_text_document_layout::AbstractTextDocumentLayout {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QPlainTextDocumentLayout_G_static_cast_QAbstractTextDocumentLayout_ptr(self as *const ::plain_text_document_layout::PlainTextDocumentLayout as *mut ::plain_text_document_layout::PlainTextDocumentLayout) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::plain_text_document_layout::PlainTextDocumentLayout> for ::qt_gui::abstract_text_document_layout::AbstractTextDocumentLayout {
unsafe fn static_cast_mut(&mut self) -> &mut ::plain_text_document_layout::PlainTextDocumentLayout {
let ffi_result = ::ffi::qt_widgets_c_QPlainTextDocumentLayout_G_static_cast_QPlainTextDocumentLayout_ptr_QAbstractTextDocumentLayout(self as *mut ::qt_gui::abstract_text_document_layout::AbstractTextDocumentLayout);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::plain_text_document_layout::PlainTextDocumentLayout {
let ffi_result = ::ffi::qt_widgets_c_QPlainTextDocumentLayout_G_static_cast_QPlainTextDocumentLayout_ptr_QAbstractTextDocumentLayout(self as *const ::qt_gui::abstract_text_document_layout::AbstractTextDocumentLayout as *mut ::qt_gui::abstract_text_document_layout::AbstractTextDocumentLayout);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::plain_text_document_layout::PlainTextDocumentLayout> for ::qt_core::object::Object {
unsafe fn static_cast_mut(&mut self) -> &mut ::plain_text_document_layout::PlainTextDocumentLayout {
let ffi_result = ::ffi::qt_widgets_c_QPlainTextDocumentLayout_G_static_cast_QPlainTextDocumentLayout_ptr_QObject(self as *mut ::qt_core::object::Object);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::plain_text_document_layout::PlainTextDocumentLayout {
let ffi_result = ::ffi::qt_widgets_c_QPlainTextDocumentLayout_G_static_cast_QPlainTextDocumentLayout_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::plain_text_document_layout::PlainTextDocumentLayout {
  type Target = ::qt_gui::abstract_text_document_layout::AbstractTextDocumentLayout;
  fn deref(&self) -> &::qt_gui::abstract_text_document_layout::AbstractTextDocumentLayout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPlainTextDocumentLayout_G_static_cast_QAbstractTextDocumentLayout_ptr(self as *const ::plain_text_document_layout::PlainTextDocumentLayout as *mut ::plain_text_document_layout::PlainTextDocumentLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::plain_text_document_layout::PlainTextDocumentLayout {
  fn deref_mut(&mut self) -> &mut ::qt_gui::abstract_text_document_layout::AbstractTextDocumentLayout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPlainTextDocumentLayout_G_static_cast_QAbstractTextDocumentLayout_ptr(self as *mut ::plain_text_document_layout::PlainTextDocumentLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

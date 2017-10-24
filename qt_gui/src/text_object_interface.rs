/// C++ type: <span style='color: green;'>```QTextObjectInterface```</span>
#[repr(C)]
pub struct TextObjectInterface(u8);

impl TextObjectInterface {
  /// C++ method: <span style='color: green;'>```pure virtual void QTextObjectInterface::drawObject(QPainter* painter, const QRectF& rect, QTextDocument* doc, int posInDocument, const QTextFormat& format)```</span>
  ///
  ///
  pub unsafe fn draw_object(&mut self,
                            painter: *mut ::painter::Painter,
                            rect: &::qt_core::rect_f::RectF,
                            doc: *mut ::text_document::TextDocument,
                            pos_in_document: ::libc::c_int,
                            format: &::text_format::TextFormat) {
    ::ffi::qt_gui_c_QTextObjectInterface_drawObject(self as *mut ::text_object_interface::TextObjectInterface,
                                                    painter,
                                                    rect as *const ::qt_core::rect_f::RectF,
                                                    doc,
                                                    pos_in_document,
                                                    format as *const ::text_format::TextFormat)
  }

  /// C++ method: <span style='color: green;'>```pure virtual QSizeF QTextObjectInterface::intrinsicSize(QTextDocument* doc, int posInDocument, const QTextFormat& format)```</span>
  ///
  ///
  pub unsafe fn intrinsic_size(&mut self,
                               doc: *mut ::text_document::TextDocument,
                               pos_in_document: ::libc::c_int,
                               format: &::text_format::TextFormat)
                               -> ::qt_core::size_f::SizeF {
    {
      let mut object: ::qt_core::size_f::SizeF =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QTextObjectInterface_intrinsicSize_to_output(self as *mut ::text_object_interface::TextObjectInterface, doc, pos_in_document, format as *const ::text_format::TextFormat, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::text_object_interface::TextObjectInterface {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QTextObjectInterface_delete
  }
}

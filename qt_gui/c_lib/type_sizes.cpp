#include <QtGui>
#include <iostream>

int main() {
  std::cout << "pub const QT_GUI_RGBA_64_RGBA_64: usize = " << sizeof(QRgba64) << ";\n";
  std::cout << "pub const QT_GUI_COLOR_COLOR: usize = " << sizeof(QColor) << ";\n";
  std::cout << "pub const QT_GUI_KEY_SEQUENCE_KEY_SEQUENCE: usize = " << sizeof(QKeySequence) << ";\n";
  std::cout << "pub const QT_GUI_INPUT_METHOD_EVENT_ATTRIBUTE: usize = " << sizeof(QInputMethodEvent::Attribute) << ";\n";
  std::cout << "pub const QT_GUI_POINTING_DEVICE_UNIQUE_ID_POINTING_DEVICE_UNIQUE_ID: usize = " << sizeof(QPointingDeviceUniqueId) << ";\n";
  std::cout << "pub const QT_GUI_TOUCH_EVENT_TOUCH_POINT: usize = " << sizeof(QTouchEvent::TouchPoint) << ";\n";
  std::cout << "pub const QT_GUI_FONT_FONT: usize = " << sizeof(QFont) << ";\n";
  std::cout << "pub const QT_GUI_POLYGON_POLYGON: usize = " << sizeof(QPolygon) << ";\n";
  std::cout << "pub const QT_GUI_POLYGON_F_POLYGON_F: usize = " << sizeof(QPolygonF) << ";\n";
  std::cout << "pub const QT_GUI_MATRIX_MATRIX: usize = " << sizeof(QMatrix) << ";\n";
  std::cout << "pub const QT_GUI_PAINTER_PATH_PAINTER_PATH: usize = " << sizeof(QPainterPath) << ";\n";
  std::cout << "pub const QT_GUI_PAINTER_PATH_ELEMENT: usize = " << sizeof(QPainterPath::Element) << ";\n";
  std::cout << "pub const QT_GUI_PAINTER_PATH_STROKER_PAINTER_PATH_STROKER: usize = " << sizeof(QPainterPathStroker) << ";\n";
  std::cout << "pub const QT_GUI_TRANSFORM_TRANSFORM: usize = " << sizeof(QTransform) << ";\n";
  std::cout << "pub const QT_GUI_PIXEL_FORMAT_PIXEL_FORMAT: usize = " << sizeof(QPixelFormat) << ";\n";
  std::cout << "pub const QT_GUI_BRUSH_BRUSH: usize = " << sizeof(QBrush) << ";\n";
  std::cout << "pub const QT_GUI_PEN_PEN: usize = " << sizeof(QPen) << ";\n";
  std::cout << "pub const QT_GUI_TEXT_OPTION_TEXT_OPTION: usize = " << sizeof(QTextOption) << ";\n";
  std::cout << "pub const QT_GUI_TEXT_OPTION_TAB: usize = " << sizeof(QTextOption::Tab) << ";\n";
  std::cout << "pub const QT_GUI_TEXT_LENGTH_TEXT_LENGTH: usize = " << sizeof(QTextLength) << ";\n";
  std::cout << "pub const QT_GUI_TEXT_FORMAT_TEXT_FORMAT: usize = " << sizeof(QTextFormat) << ";\n";
  std::cout << "pub const QT_GUI_TEXT_CHAR_FORMAT_TEXT_CHAR_FORMAT: usize = " << sizeof(QTextCharFormat) << ";\n";
  std::cout << "pub const QT_GUI_TEXT_BLOCK_FORMAT_TEXT_BLOCK_FORMAT: usize = " << sizeof(QTextBlockFormat) << ";\n";
  std::cout << "pub const QT_GUI_TEXT_LIST_FORMAT_TEXT_LIST_FORMAT: usize = " << sizeof(QTextListFormat) << ";\n";
  std::cout << "pub const QT_GUI_TEXT_IMAGE_FORMAT_TEXT_IMAGE_FORMAT: usize = " << sizeof(QTextImageFormat) << ";\n";
  std::cout << "pub const QT_GUI_TEXT_FRAME_FORMAT_TEXT_FRAME_FORMAT: usize = " << sizeof(QTextFrameFormat) << ";\n";
  std::cout << "pub const QT_GUI_TEXT_TABLE_FORMAT_TEXT_TABLE_FORMAT: usize = " << sizeof(QTextTableFormat) << ";\n";
  std::cout << "pub const QT_GUI_TEXT_TABLE_CELL_FORMAT_TEXT_TABLE_CELL_FORMAT: usize = " << sizeof(QTextTableCellFormat) << ";\n";
  std::cout << "pub const QT_GUI_RAW_FONT_RAW_FONT: usize = " << sizeof(QRawFont) << ";\n";
  std::cout << "pub const QT_GUI_GLYPH_RUN_GLYPH_RUN: usize = " << sizeof(QGlyphRun) << ";\n";
  std::cout << "pub const QT_GUI_TEXT_INLINE_OBJECT_TEXT_INLINE_OBJECT: usize = " << sizeof(QTextInlineObject) << ";\n";
  std::cout << "pub const QT_GUI_TEXT_LINE_TEXT_LINE: usize = " << sizeof(QTextLine) << ";\n";
  std::cout << "pub const QT_GUI_PALETTE_PALETTE: usize = " << sizeof(QPalette) << ";\n";
  std::cout << "pub const QT_GUI_ABSTRACT_TEXT_DOCUMENT_LAYOUT_PAINT_CONTEXT: usize = " << sizeof(QAbstractTextDocumentLayout::PaintContext) << ";\n";
  std::cout << "pub const QT_GUI_ACCESSIBLE_STATE: usize = " << sizeof(QAccessible::State) << ";\n";
  std::cout << "pub const QT_GUI_SURFACE_FORMAT_SURFACE_FORMAT: usize = " << sizeof(QSurfaceFormat) << ";\n";
  std::cout << "pub const QT_GUI_ICON_ICON: usize = " << sizeof(QIcon) << ";\n";
  std::cout << "pub const QT_GUI_FONT_INFO_FONT_INFO: usize = " << sizeof(QFontInfo) << ";\n";
  std::cout << "pub const QT_GUI_FONT_METRICS_FONT_METRICS: usize = " << sizeof(QFontMetrics) << ";\n";
  std::cout << "pub const QT_GUI_FONT_METRICS_F_FONT_METRICS_F: usize = " << sizeof(QFontMetricsF) << ";\n";
  std::cout << "pub const QT_GUI_IMAGE_READER_IMAGE_READER: usize = " << sizeof(QImageReader) << ";\n";
  std::cout << "pub const QT_GUI_QUATERNION_QUATERNION: usize = " << sizeof(QQuaternion) << ";\n";
  std::cout << "pub const QT_GUI_OPENGL_BUFFER_OPEN_G_L_BUFFER: usize = " << sizeof(QOpenGLBuffer) << ";\n";
  std::cout << "pub const QT_GUI_OPENGL_VERSION_PROFILE_OPEN_G_L_VERSION_PROFILE: usize = " << sizeof(QOpenGLVersionProfile) << ";\n";
  std::cout << "pub const QT_GUI_OPENGL_DEBUG_MESSAGE_OPEN_G_L_DEBUG_MESSAGE: usize = " << sizeof(QOpenGLDebugMessage) << ";\n";
  std::cout << "pub const QT_GUI_OPENGL_FRAMEBUFFER_OBJECT_FORMAT_OPEN_G_L_FRAMEBUFFER_OBJECT_FORMAT: usize = " << sizeof(QOpenGLFramebufferObjectFormat) << ";\n";
  std::cout << "pub const QT_GUI_OPENGL_TEXTURE_BLITTER_OPEN_G_L_TEXTURE_BLITTER: usize = " << sizeof(QOpenGLTextureBlitter) << ";\n";
  std::cout << "pub const QT_GUI_OPENGL_VERTEX_ARRAY_OBJECT_BINDER: usize = " << sizeof(QOpenGLVertexArrayObject::Binder) << ";\n";
  std::cout << "pub const QT_GUI_PAGE_SIZE_PAGE_SIZE: usize = " << sizeof(QPageSize) << ";\n";
  std::cout << "pub const QT_GUI_PAGE_LAYOUT_PAGE_LAYOUT: usize = " << sizeof(QPageLayout) << ";\n";
  std::cout << "pub const QT_GUI_PAGED_PAINT_DEVICE_MARGINS: usize = " << sizeof(QPagedPaintDevice::Margins) << ";\n";
  std::cout << "pub const QT_GUI_TEXT_ITEM_TEXT_ITEM: usize = " << sizeof(QTextItem) << ";\n";
  std::cout << "pub const QT_GUI_PAINT_ENGINE_STATE_PAINT_ENGINE_STATE: usize = " << sizeof(QPaintEngineState) << ";\n";
  std::cout << "pub const QT_GUI_PICTURE_IO_PICTURE_I_O: usize = " << sizeof(QPictureIO) << ";\n";
  std::cout << "pub const QT_GUI_PIXMAP_CACHE_KEY: usize = " << sizeof(QPixmapCache::Key) << ";\n";
  std::cout << "pub const QT_GUI_STATIC_TEXT_STATIC_TEXT: usize = " << sizeof(QStaticText) << ";\n";
  std::cout << "pub const QT_GUI_TEXT_FRAME_ITERATOR: usize = " << sizeof(QTextFrame::iterator) << ";\n";
  std::cout << "pub const QT_GUI_TEXT_BLOCK_TEXT_BLOCK: usize = " << sizeof(QTextBlock) << ";\n";
  std::cout << "pub const QT_GUI_TEXT_BLOCK_ITERATOR: usize = " << sizeof(QTextBlock::iterator) << ";\n";
  std::cout << "pub const QT_GUI_TEXT_FRAGMENT_TEXT_FRAGMENT: usize = " << sizeof(QTextFragment) << ";\n";
  std::cout << "pub const QT_GUI_TEXT_DOCUMENT_FRAGMENT_TEXT_DOCUMENT_FRAGMENT: usize = " << sizeof(QTextDocumentFragment) << ";\n";
  std::cout << "pub const QT_GUI_TEXT_DOCUMENT_WRITER_TEXT_DOCUMENT_WRITER: usize = " << sizeof(QTextDocumentWriter) << ";\n";
  std::cout << "pub const QT_GUI_TEXT_TABLE_CELL_TEXT_TABLE_CELL: usize = " << sizeof(QTextTableCell) << ";\n";
  std::cout << "pub const QT_GUI_VECTOR_VECTOR_QT_CORE_RECT: usize = " << sizeof(QVector< QRect >) << ";\n";
  std::cout << "pub const QT_GUI_VECTOR_VECTOR_QT_CORE_POINT: usize = " << sizeof(QVector< QPoint >) << ";\n";
  std::cout << "pub const QT_GUI_VECTOR_VECTOR_C_DOUBLE: usize = " << sizeof(QVector< double >) << ";\n";
  std::cout << "pub const QT_GUI_VECTOR_VECTOR_TEXT_LENGTH: usize = " << sizeof(QVector< QTextLength >) << ";\n";
  std::cout << "pub const QT_GUI_VECTOR_VECTOR_U32: usize = " << sizeof(QVector< quint32 >) << ";\n";
  std::cout << "pub const QT_GUI_VECTOR_VECTOR_TEXT_LAYOUT_FORMAT_RANGE: usize = " << sizeof(QVector< QTextLayout::FormatRange >) << ";\n";
  std::cout << "pub const QT_GUI_VECTOR_VECTOR_TEXT_FORMAT: usize = " << sizeof(QVector< QTextFormat >) << ";\n";
  std::cout << "pub const QT_GUI_VECTOR_VECTOR_QT_CORE_SIZE: usize = " << sizeof(QVector< QSize >) << ";\n";
  std::cout << "pub const QT_GUI_VECTOR_VECTOR_C_FLOAT: usize = " << sizeof(QVector< float >) << ";\n";
  std::cout << "pub const QT_GUI_VECTOR_VECTOR_U64: usize = " << sizeof(QVector< GLuint64 >) << ";\n";
  std::cout << "pub const QT_GUI_VECTOR_VECTOR_QT_CORE_LINE_F: usize = " << sizeof(QVector< QLineF >) << ";\n";
  std::cout << "pub const QT_GUI_VECTOR_VECTOR_QT_CORE_LINE: usize = " << sizeof(QVector< QLine >) << ";\n";
  std::cout << "pub const QT_GUI_VECTOR_VECTOR_QT_CORE_RECT_F: usize = " << sizeof(QVector< QRectF >) << ";\n";
  std::cout << "pub const QT_GUI_VECTOR_VECTOR_ABSTRACT_TEXT_DOCUMENT_LAYOUT_SELECTION: usize = " << sizeof(QVector< QAbstractTextDocumentLayout::Selection >) << ";\n";
  std::cout << "pub const QT_GUI_LIST_LIST_KEY_SEQUENCE: usize = " << sizeof(QList< QKeySequence >) << ";\n";
  std::cout << "pub const QT_GUI_LIST_LIST_TOUCH_DEVICE_PTR: usize = " << sizeof(QList< const QTouchDevice* >) << ";\n";
  std::cout << "pub const QT_GUI_LIST_LIST_INPUT_METHOD_EVENT_ATTRIBUTE: usize = " << sizeof(QList< QInputMethodEvent::Attribute >) << ";\n";
  std::cout << "pub const QT_GUI_LIST_LIST_TOUCH_EVENT_TOUCH_POINT: usize = " << sizeof(QList< QTouchEvent::TouchPoint >) << ";\n";
  std::cout << "pub const QT_GUI_LIST_LIST_POLYGON_F: usize = " << sizeof(QList< QPolygonF >) << ";\n";
  std::cout << "pub const QT_GUI_LIST_LIST_C_DOUBLE: usize = " << sizeof(QList< double >) << ";\n";
  std::cout << "pub const QT_GUI_LIST_LIST_TEXT_OPTION_TAB: usize = " << sizeof(QList< QTextOption::Tab >) << ";\n";
  std::cout << "pub const QT_GUI_LIST_LIST_FONT_DATABASE_WRITING_SYSTEM: usize = " << sizeof(QList< QFontDatabase::WritingSystem >) << ";\n";
  std::cout << "pub const QT_GUI_LIST_LIST_TEXT_LAYOUT_FORMAT_RANGE: usize = " << sizeof(QList< QTextLayout::FormatRange >) << ";\n";
  std::cout << "pub const QT_GUI_LIST_LIST_GLYPH_RUN: usize = " << sizeof(QList< QGlyphRun >) << ";\n";
  std::cout << "pub const QT_GUI_LIST_LIST_ACCESSIBLE_INTERFACE_MUT_PTR: usize = " << sizeof(QList< QAccessibleInterface* >) << ";\n";
  std::cout << "pub const QT_GUI_LIST_LIST_QT_CORE_SIZE: usize = " << sizeof(QList< QSize >) << ";\n";
  std::cout << "pub const QT_GUI_LIST_LIST_WINDOW_MUT_PTR: usize = " << sizeof(QList< QWindow* >) << ";\n";
  std::cout << "pub const QT_GUI_LIST_LIST_SCREEN_MUT_PTR: usize = " << sizeof(QList< QScreen* >) << ";\n";
  std::cout << "pub const QT_GUI_LIST_LIST_OPENGL_CONTEXT_MUT_PTR: usize = " << sizeof(QList< QOpenGLContext* >) << ";\n";
  std::cout << "pub const QT_GUI_LIST_LIST_OPENGL_DEBUG_MESSAGE: usize = " << sizeof(QList< QOpenGLDebugMessage >) << ";\n";
  std::cout << "pub const QT_GUI_LIST_LIST_OPENGL_SHADER_MUT_PTR: usize = " << sizeof(QList< QOpenGLShader* >) << ";\n";
  std::cout << "pub const QT_GUI_LIST_LIST_STANDARD_ITEM_MUT_PTR: usize = " << sizeof(QList< QStandardItem* >) << ";\n";
  std::cout << "pub const QT_GUI_LIST_LIST_TEXT_BLOCK: usize = " << sizeof(QList< QTextBlock >) << ";\n";
  std::cout << "pub const QT_GUI_LIST_LIST_TEXT_FRAME_MUT_PTR: usize = " << sizeof(QList< QTextFrame* >) << ";\n";
  std::cout << "pub const QT_GUI_PAIR_PAIR_C_DOUBLE_COLOR: usize = " << sizeof(QPair< double, QColor >) << ";\n";
  std::cout << "pub const QT_GUI_PAIR_PAIR_C_INT_C_INT: usize = " << sizeof(QPair< int, int >) << ";\n";
  std::cout << "pub const QT_GUI_PAIR_PAIR_OPENGL_TEXTURE_FILTER_OPENGL_TEXTURE_FILTER: usize = " << sizeof(QPair< QOpenGLTexture::Filter, QOpenGLTexture::Filter >) << ";\n";
  std::cout << "pub const QT_GUI_PAIR_PAIR_C_FLOAT_C_FLOAT: usize = " << sizeof(QPair< float, float >) << ";\n";
  std::cout << "pub const QT_GUI_SET_SET_QT_CORE_BYTE_ARRAY: usize = " << sizeof(QSet< QByteArray >) << ";\n";
  std::cout << "pub const QT_GUI_VECTOR_VECTOR_PAIR_PAIR_C_DOUBLE_COLOR: usize = " << sizeof(QVector< QPair< double, QColor > >) << ";\n";
}

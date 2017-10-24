#include "qt_gui_c_QAbstractTextDocumentLayout.h"

QAbstractTextDocumentLayout* qt_gui_c_QAbstractTextDocumentLayout_G_static_cast_QAbstractTextDocumentLayout_ptr(QObject* ptr) {
  return static_cast<QAbstractTextDocumentLayout*>(ptr);
}

QObject* qt_gui_c_QAbstractTextDocumentLayout_G_static_cast_QObject_ptr(QAbstractTextDocumentLayout* ptr) {
  return static_cast<QObject*>(ptr);
}

const QRectF* qt_gui_c_QAbstractTextDocumentLayout_PaintContext_clip(const QAbstractTextDocumentLayout::PaintContext* this_ptr) {
  return &this_ptr->clip;
}

QRectF* qt_gui_c_QAbstractTextDocumentLayout_PaintContext_clip_mut(QAbstractTextDocumentLayout::PaintContext* this_ptr) {
  return &this_ptr->clip;
}

void qt_gui_c_QAbstractTextDocumentLayout_PaintContext_constructor(QAbstractTextDocumentLayout::PaintContext* output) {
  new(output) QAbstractTextDocumentLayout::PaintContext();
}

int qt_gui_c_QAbstractTextDocumentLayout_PaintContext_cursorPosition(const QAbstractTextDocumentLayout::PaintContext* this_ptr) {
  return this_ptr->cursorPosition;
}

void qt_gui_c_QAbstractTextDocumentLayout_PaintContext_destructor(QAbstractTextDocumentLayout::PaintContext* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

const QPalette* qt_gui_c_QAbstractTextDocumentLayout_PaintContext_palette(const QAbstractTextDocumentLayout::PaintContext* this_ptr) {
  return &this_ptr->palette;
}

QPalette* qt_gui_c_QAbstractTextDocumentLayout_PaintContext_palette_mut(QAbstractTextDocumentLayout::PaintContext* this_ptr) {
  return &this_ptr->palette;
}

const QVector< QAbstractTextDocumentLayout::Selection >* qt_gui_c_QAbstractTextDocumentLayout_PaintContext_selections(const QAbstractTextDocumentLayout::PaintContext* this_ptr) {
  return &this_ptr->selections;
}

QVector< QAbstractTextDocumentLayout::Selection >* qt_gui_c_QAbstractTextDocumentLayout_PaintContext_selections_mut(QAbstractTextDocumentLayout::PaintContext* this_ptr) {
  return &this_ptr->selections;
}

void qt_gui_c_QAbstractTextDocumentLayout_PaintContext_set_clip(QAbstractTextDocumentLayout::PaintContext* this_ptr, const QRectF* value) {
  this_ptr->clip = *value;
}

void qt_gui_c_QAbstractTextDocumentLayout_PaintContext_set_cursorPosition(QAbstractTextDocumentLayout::PaintContext* this_ptr, int value) {
  this_ptr->cursorPosition = value;
}

void qt_gui_c_QAbstractTextDocumentLayout_PaintContext_set_palette(QAbstractTextDocumentLayout::PaintContext* this_ptr, const QPalette* value) {
  this_ptr->palette = *value;
}

void qt_gui_c_QAbstractTextDocumentLayout_PaintContext_set_selections(QAbstractTextDocumentLayout::PaintContext* this_ptr, const QVector< QAbstractTextDocumentLayout::Selection >* value) {
  this_ptr->selections = *value;
}

const QTextCursor* qt_gui_c_QAbstractTextDocumentLayout_Selection_cursor(const QAbstractTextDocumentLayout::Selection* this_ptr) {
  return &this_ptr->cursor;
}

QTextCursor* qt_gui_c_QAbstractTextDocumentLayout_Selection_cursor_mut(QAbstractTextDocumentLayout::Selection* this_ptr) {
  return &this_ptr->cursor;
}

void qt_gui_c_QAbstractTextDocumentLayout_Selection_delete(QAbstractTextDocumentLayout::Selection* this_ptr) {
  delete this_ptr;
}

const QTextCharFormat* qt_gui_c_QAbstractTextDocumentLayout_Selection_format(const QAbstractTextDocumentLayout::Selection* this_ptr) {
  return &this_ptr->format;
}

QTextCharFormat* qt_gui_c_QAbstractTextDocumentLayout_Selection_format_mut(QAbstractTextDocumentLayout::Selection* this_ptr) {
  return &this_ptr->format;
}

void qt_gui_c_QAbstractTextDocumentLayout_Selection_set_cursor(QAbstractTextDocumentLayout::Selection* this_ptr, const QTextCursor* value) {
  this_ptr->cursor = *value;
}

void qt_gui_c_QAbstractTextDocumentLayout_Selection_set_format(QAbstractTextDocumentLayout::Selection* this_ptr, const QTextCharFormat* value) {
  this_ptr->format = *value;
}

void qt_gui_c_QAbstractTextDocumentLayout_anchorAt_to_output(const QAbstractTextDocumentLayout* this_ptr, const QPointF* pos, QString* output) {
  new(output) QString(this_ptr->anchorAt(*pos));
}

void qt_gui_c_QAbstractTextDocumentLayout_blockBoundingRect_to_output(const QAbstractTextDocumentLayout* this_ptr, const QTextBlock* block, QRectF* output) {
  new(output) QRectF(this_ptr->blockBoundingRect(*block));
}

void qt_gui_c_QAbstractTextDocumentLayout_delete(QAbstractTextDocumentLayout* this_ptr) {
  delete this_ptr;
}

QTextDocument* qt_gui_c_QAbstractTextDocumentLayout_document(const QAbstractTextDocumentLayout* this_ptr) {
  return this_ptr->document();
}

void qt_gui_c_QAbstractTextDocumentLayout_documentSize_to_output(const QAbstractTextDocumentLayout* this_ptr, QSizeF* output) {
  new(output) QSizeF(this_ptr->documentSize());
}

void qt_gui_c_QAbstractTextDocumentLayout_draw(QAbstractTextDocumentLayout* this_ptr, QPainter* painter, const QAbstractTextDocumentLayout::PaintContext* context) {
  this_ptr->draw(painter, *context);
}

void qt_gui_c_QAbstractTextDocumentLayout_formatAt_to_output(const QAbstractTextDocumentLayout* this_ptr, const QPointF* pos, QTextFormat* output) {
  new(output) QTextFormat(this_ptr->formatAt(*pos));
}

void qt_gui_c_QAbstractTextDocumentLayout_frameBoundingRect_to_output(const QAbstractTextDocumentLayout* this_ptr, QTextFrame* frame, QRectF* output) {
  new(output) QRectF(this_ptr->frameBoundingRect(frame));
}

QTextObjectInterface* qt_gui_c_QAbstractTextDocumentLayout_handlerForObject(const QAbstractTextDocumentLayout* this_ptr, int objectType) {
  return this_ptr->handlerForObject(objectType);
}

int qt_gui_c_QAbstractTextDocumentLayout_hitTest(const QAbstractTextDocumentLayout* this_ptr, const QPointF* point, const Qt::HitTestAccuracy* accuracy) {
  return this_ptr->hitTest(*point, *accuracy);
}

void qt_gui_c_QAbstractTextDocumentLayout_imageAt_to_output(const QAbstractTextDocumentLayout* this_ptr, const QPointF* pos, QString* output) {
  new(output) QString(this_ptr->imageAt(*pos));
}

const QMetaObject* qt_gui_c_QAbstractTextDocumentLayout_metaObject(const QAbstractTextDocumentLayout* this_ptr) {
  return this_ptr->metaObject();
}

int qt_gui_c_QAbstractTextDocumentLayout_pageCount(const QAbstractTextDocumentLayout* this_ptr) {
  return this_ptr->pageCount();
}

QPaintDevice* qt_gui_c_QAbstractTextDocumentLayout_paintDevice(const QAbstractTextDocumentLayout* this_ptr) {
  return this_ptr->paintDevice();
}

int qt_gui_c_QAbstractTextDocumentLayout_qt_metacall(QAbstractTextDocumentLayout* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QAbstractTextDocumentLayout_qt_metacast(QAbstractTextDocumentLayout* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QAbstractTextDocumentLayout_registerHandler(QAbstractTextDocumentLayout* this_ptr, int objectType, QObject* component) {
  this_ptr->registerHandler(objectType, component);
}

void qt_gui_c_QAbstractTextDocumentLayout_setPaintDevice(QAbstractTextDocumentLayout* this_ptr, QPaintDevice* device) {
  this_ptr->setPaintDevice(device);
}

void qt_gui_c_QAbstractTextDocumentLayout_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractTextDocumentLayout::trUtf8(s, c, n));
}

void qt_gui_c_QAbstractTextDocumentLayout_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractTextDocumentLayout::tr(s, c, n));
}

void qt_gui_c_QAbstractTextDocumentLayout_unregisterHandler_objectType(QAbstractTextDocumentLayout* this_ptr, int objectType) {
  this_ptr->unregisterHandler(objectType);
}

void qt_gui_c_QAbstractTextDocumentLayout_unregisterHandler_objectType_component(QAbstractTextDocumentLayout* this_ptr, int objectType, QObject* component) {
  this_ptr->unregisterHandler(objectType, component);
}


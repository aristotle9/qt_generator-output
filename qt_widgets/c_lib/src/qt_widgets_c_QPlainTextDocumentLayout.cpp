#include "qt_widgets_c_QPlainTextDocumentLayout.h"

QAbstractTextDocumentLayout* qt_widgets_c_QPlainTextDocumentLayout_G_static_cast_QAbstractTextDocumentLayout_ptr(QPlainTextDocumentLayout* ptr) {
  return static_cast<QAbstractTextDocumentLayout*>(ptr);
}

QObject* qt_widgets_c_QPlainTextDocumentLayout_G_static_cast_QObject_ptr(QPlainTextDocumentLayout* ptr) {
  return static_cast<QObject*>(ptr);
}

QPlainTextDocumentLayout* qt_widgets_c_QPlainTextDocumentLayout_G_static_cast_QPlainTextDocumentLayout_ptr_QAbstractTextDocumentLayout(QAbstractTextDocumentLayout* ptr) {
  return static_cast<QPlainTextDocumentLayout*>(ptr);
}

QPlainTextDocumentLayout* qt_widgets_c_QPlainTextDocumentLayout_G_static_cast_QPlainTextDocumentLayout_ptr_QObject(QObject* ptr) {
  return static_cast<QPlainTextDocumentLayout*>(ptr);
}

void qt_widgets_c_QPlainTextDocumentLayout_blockBoundingRect_to_output(const QPlainTextDocumentLayout* this_ptr, const QTextBlock* block, QRectF* output) {
  new(output) QRectF(this_ptr->blockBoundingRect(*block));
}

int qt_widgets_c_QPlainTextDocumentLayout_cursorWidth(const QPlainTextDocumentLayout* this_ptr) {
  return this_ptr->cursorWidth();
}

void qt_widgets_c_QPlainTextDocumentLayout_delete(QPlainTextDocumentLayout* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QPlainTextDocumentLayout_documentSize_to_output(const QPlainTextDocumentLayout* this_ptr, QSizeF* output) {
  new(output) QSizeF(this_ptr->documentSize());
}

void qt_widgets_c_QPlainTextDocumentLayout_draw(QPlainTextDocumentLayout* this_ptr, QPainter* arg1, const QAbstractTextDocumentLayout::PaintContext* arg2) {
  this_ptr->draw(arg1, *arg2);
}

void qt_widgets_c_QPlainTextDocumentLayout_ensureBlockLayout(const QPlainTextDocumentLayout* this_ptr, const QTextBlock* block) {
  this_ptr->ensureBlockLayout(*block);
}

void qt_widgets_c_QPlainTextDocumentLayout_frameBoundingRect_to_output(const QPlainTextDocumentLayout* this_ptr, QTextFrame* arg1, QRectF* output) {
  new(output) QRectF(this_ptr->frameBoundingRect(arg1));
}

int qt_widgets_c_QPlainTextDocumentLayout_hitTest(const QPlainTextDocumentLayout* this_ptr, const QPointF* arg1, const Qt::HitTestAccuracy* arg2) {
  return this_ptr->hitTest(*arg1, *arg2);
}

const QMetaObject* qt_widgets_c_QPlainTextDocumentLayout_metaObject(const QPlainTextDocumentLayout* this_ptr) {
  return this_ptr->metaObject();
}

QPlainTextDocumentLayout* qt_widgets_c_QPlainTextDocumentLayout_new(QTextDocument* document) {
  return new QPlainTextDocumentLayout(document);
}

int qt_widgets_c_QPlainTextDocumentLayout_pageCount(const QPlainTextDocumentLayout* this_ptr) {
  return this_ptr->pageCount();
}

int qt_widgets_c_QPlainTextDocumentLayout_qt_metacall(QPlainTextDocumentLayout* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QPlainTextDocumentLayout_qt_metacast(QPlainTextDocumentLayout* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QPlainTextDocumentLayout_requestUpdate(QPlainTextDocumentLayout* this_ptr) {
  this_ptr->requestUpdate();
}

void qt_widgets_c_QPlainTextDocumentLayout_setCursorWidth(QPlainTextDocumentLayout* this_ptr, int width) {
  this_ptr->setCursorWidth(width);
}

void qt_widgets_c_QPlainTextDocumentLayout_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QPlainTextDocumentLayout::trUtf8(s, c, n));
}

void qt_widgets_c_QPlainTextDocumentLayout_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QPlainTextDocumentLayout::tr(s, c, n));
}


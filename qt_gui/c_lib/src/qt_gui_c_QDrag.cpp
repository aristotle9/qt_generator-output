#include "qt_gui_c_QDrag.h"

QDrag* qt_gui_c_QDrag_G_static_cast_QDrag_ptr(QObject* ptr) {
  return static_cast<QDrag*>(ptr);
}

QObject* qt_gui_c_QDrag_G_static_cast_QObject_ptr(QDrag* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_gui_c_QDrag_cancel() {
  QDrag::cancel();
}

void qt_gui_c_QDrag_delete(QDrag* this_ptr) {
  delete this_ptr;
}

QPixmap* qt_gui_c_QDrag_dragCursor_as_ptr(const QDrag* this_ptr, const Qt::DropAction* action) {
  return new QPixmap(this_ptr->dragCursor(*action));
}

void qt_gui_c_QDrag_hotSpot_to_output(const QDrag* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->hotSpot());
}

const QMetaObject* qt_gui_c_QDrag_metaObject(const QDrag* this_ptr) {
  return this_ptr->metaObject();
}

QMimeData* qt_gui_c_QDrag_mimeData(const QDrag* this_ptr) {
  return this_ptr->mimeData();
}

QDrag* qt_gui_c_QDrag_new(QObject* dragSource) {
  return new QDrag(dragSource);
}

QPixmap* qt_gui_c_QDrag_pixmap_as_ptr(const QDrag* this_ptr) {
  return new QPixmap(this_ptr->pixmap());
}

int qt_gui_c_QDrag_qt_metacall(QDrag* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QDrag_qt_metacast(QDrag* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QDrag_setDragCursor(QDrag* this_ptr, const QPixmap* cursor, const Qt::DropAction* action) {
  this_ptr->setDragCursor(*cursor, *action);
}

void qt_gui_c_QDrag_setHotSpot(QDrag* this_ptr, const QPoint* hotspot) {
  this_ptr->setHotSpot(*hotspot);
}

void qt_gui_c_QDrag_setMimeData(QDrag* this_ptr, QMimeData* data) {
  this_ptr->setMimeData(data);
}

void qt_gui_c_QDrag_setPixmap(QDrag* this_ptr, const QPixmap* arg1) {
  this_ptr->setPixmap(*arg1);
}

QObject* qt_gui_c_QDrag_source(const QDrag* this_ptr) {
  return this_ptr->source();
}

QObject* qt_gui_c_QDrag_target(const QDrag* this_ptr) {
  return this_ptr->target();
}

void qt_gui_c_QDrag_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QDrag::trUtf8(s, c, n));
}

void qt_gui_c_QDrag_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QDrag::tr(s, c, n));
}


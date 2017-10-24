#include "qt_widgets_c_QGraphicsObject.h"

QGraphicsObject* qt_widgets_c_QGraphicsObject_G_dynamic_cast_QGraphicsObject_ptr(QGraphicsItem* ptr) {
  return dynamic_cast<QGraphicsObject*>(ptr);
}

QGraphicsItem* qt_widgets_c_QGraphicsObject_G_static_cast_QGraphicsItem_ptr(QGraphicsObject* ptr) {
  return static_cast<QGraphicsItem*>(ptr);
}

QGraphicsObject* qt_widgets_c_QGraphicsObject_G_static_cast_QGraphicsObject_ptr_QGraphicsItem(QGraphicsItem* ptr) {
  return static_cast<QGraphicsObject*>(ptr);
}

QGraphicsObject* qt_widgets_c_QGraphicsObject_G_static_cast_QGraphicsObject_ptr_QObject(QObject* ptr) {
  return static_cast<QGraphicsObject*>(ptr);
}

QObject* qt_widgets_c_QGraphicsObject_G_static_cast_QObject_ptr(QGraphicsObject* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_widgets_c_QGraphicsObject_delete(QGraphicsObject* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_widgets_c_QGraphicsObject_metaObject(const QGraphicsObject* this_ptr) {
  return this_ptr->metaObject();
}

int qt_widgets_c_QGraphicsObject_qt_metacall(QGraphicsObject* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QGraphicsObject_qt_metacast(QGraphicsObject* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QGraphicsObject_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsObject::trUtf8(s, c, n));
}

void qt_widgets_c_QGraphicsObject_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsObject::tr(s, c, n));
}

void qt_widgets_c_QGraphicsObject_ungrabGesture(QGraphicsObject* this_ptr, const Qt::GestureType* type) {
  this_ptr->ungrabGesture(*type);
}


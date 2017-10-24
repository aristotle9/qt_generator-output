#include "qt_widgets_c_QGraphicsAnchor.h"

QGraphicsAnchor* qt_widgets_c_QGraphicsAnchor_G_static_cast_QGraphicsAnchor_ptr(QObject* ptr) {
  return static_cast<QGraphicsAnchor*>(ptr);
}

QObject* qt_widgets_c_QGraphicsAnchor_G_static_cast_QObject_ptr(QGraphicsAnchor* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_widgets_c_QGraphicsAnchor_delete(QGraphicsAnchor* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_widgets_c_QGraphicsAnchor_metaObject(const QGraphicsAnchor* this_ptr) {
  return this_ptr->metaObject();
}

int qt_widgets_c_QGraphicsAnchor_qt_metacall(QGraphicsAnchor* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QGraphicsAnchor_qt_metacast(QGraphicsAnchor* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QGraphicsAnchor_setSizePolicy(QGraphicsAnchor* this_ptr, const QSizePolicy::Policy* policy) {
  this_ptr->setSizePolicy(*policy);
}

void qt_widgets_c_QGraphicsAnchor_setSpacing(QGraphicsAnchor* this_ptr, double spacing) {
  this_ptr->setSpacing(spacing);
}

double qt_widgets_c_QGraphicsAnchor_spacing(const QGraphicsAnchor* this_ptr) {
  return this_ptr->spacing();
}

void qt_widgets_c_QGraphicsAnchor_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsAnchor::trUtf8(s, c, n));
}

void qt_widgets_c_QGraphicsAnchor_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsAnchor::tr(s, c, n));
}

void qt_widgets_c_QGraphicsAnchor_unsetSpacing(QGraphicsAnchor* this_ptr) {
  this_ptr->unsetSpacing();
}


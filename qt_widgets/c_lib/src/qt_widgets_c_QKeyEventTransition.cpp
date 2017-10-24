#include "qt_widgets_c_QKeyEventTransition.h"

QAbstractTransition* qt_widgets_c_QKeyEventTransition_G_static_cast_QAbstractTransition_ptr(QKeyEventTransition* ptr) {
  return static_cast<QAbstractTransition*>(ptr);
}

QEventTransition* qt_widgets_c_QKeyEventTransition_G_static_cast_QEventTransition_ptr(QKeyEventTransition* ptr) {
  return static_cast<QEventTransition*>(ptr);
}

QKeyEventTransition* qt_widgets_c_QKeyEventTransition_G_static_cast_QKeyEventTransition_ptr_QAbstractTransition(QAbstractTransition* ptr) {
  return static_cast<QKeyEventTransition*>(ptr);
}

QKeyEventTransition* qt_widgets_c_QKeyEventTransition_G_static_cast_QKeyEventTransition_ptr_QEventTransition(QEventTransition* ptr) {
  return static_cast<QKeyEventTransition*>(ptr);
}

QKeyEventTransition* qt_widgets_c_QKeyEventTransition_G_static_cast_QKeyEventTransition_ptr_QObject(QObject* ptr) {
  return static_cast<QKeyEventTransition*>(ptr);
}

QObject* qt_widgets_c_QKeyEventTransition_G_static_cast_QObject_ptr(QKeyEventTransition* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_widgets_c_QKeyEventTransition_delete(QKeyEventTransition* this_ptr) {
  delete this_ptr;
}

int qt_widgets_c_QKeyEventTransition_key(const QKeyEventTransition* this_ptr) {
  return this_ptr->key();
}

const QMetaObject* qt_widgets_c_QKeyEventTransition_metaObject(const QKeyEventTransition* this_ptr) {
  return this_ptr->metaObject();
}

QKeyEventTransition* qt_widgets_c_QKeyEventTransition_new_no_args() {
  return new QKeyEventTransition();
}

QKeyEventTransition* qt_widgets_c_QKeyEventTransition_new_object_type_key(QObject* object, const QEvent::Type* type, int key) {
  return new QKeyEventTransition(object, *type, key);
}

QKeyEventTransition* qt_widgets_c_QKeyEventTransition_new_object_type_key_sourceState(QObject* object, const QEvent::Type* type, int key, QState* sourceState) {
  return new QKeyEventTransition(object, *type, key, sourceState);
}

QKeyEventTransition* qt_widgets_c_QKeyEventTransition_new_sourceState(QState* sourceState) {
  return new QKeyEventTransition(sourceState);
}

int qt_widgets_c_QKeyEventTransition_qt_metacall(QKeyEventTransition* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QKeyEventTransition_qt_metacast(QKeyEventTransition* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QKeyEventTransition_setKey(QKeyEventTransition* this_ptr, int key) {
  this_ptr->setKey(key);
}

void qt_widgets_c_QKeyEventTransition_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QKeyEventTransition::trUtf8(s, c, n));
}

void qt_widgets_c_QKeyEventTransition_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QKeyEventTransition::tr(s, c, n));
}


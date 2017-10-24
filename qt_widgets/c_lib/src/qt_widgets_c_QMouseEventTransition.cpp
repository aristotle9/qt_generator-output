#include "qt_widgets_c_QMouseEventTransition.h"

QAbstractTransition* qt_widgets_c_QMouseEventTransition_G_static_cast_QAbstractTransition_ptr(QMouseEventTransition* ptr) {
  return static_cast<QAbstractTransition*>(ptr);
}

QEventTransition* qt_widgets_c_QMouseEventTransition_G_static_cast_QEventTransition_ptr(QMouseEventTransition* ptr) {
  return static_cast<QEventTransition*>(ptr);
}

QMouseEventTransition* qt_widgets_c_QMouseEventTransition_G_static_cast_QMouseEventTransition_ptr_QAbstractTransition(QAbstractTransition* ptr) {
  return static_cast<QMouseEventTransition*>(ptr);
}

QMouseEventTransition* qt_widgets_c_QMouseEventTransition_G_static_cast_QMouseEventTransition_ptr_QEventTransition(QEventTransition* ptr) {
  return static_cast<QMouseEventTransition*>(ptr);
}

QMouseEventTransition* qt_widgets_c_QMouseEventTransition_G_static_cast_QMouseEventTransition_ptr_QObject(QObject* ptr) {
  return static_cast<QMouseEventTransition*>(ptr);
}

QObject* qt_widgets_c_QMouseEventTransition_G_static_cast_QObject_ptr(QMouseEventTransition* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_widgets_c_QMouseEventTransition_delete(QMouseEventTransition* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QMouseEventTransition_hitTestPath_to_output(const QMouseEventTransition* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->hitTestPath());
}

const QMetaObject* qt_widgets_c_QMouseEventTransition_metaObject(const QMouseEventTransition* this_ptr) {
  return this_ptr->metaObject();
}

QMouseEventTransition* qt_widgets_c_QMouseEventTransition_new_no_args() {
  return new QMouseEventTransition();
}

QMouseEventTransition* qt_widgets_c_QMouseEventTransition_new_object_type_button(QObject* object, const QEvent::Type* type, const Qt::MouseButton* button) {
  return new QMouseEventTransition(object, *type, *button);
}

QMouseEventTransition* qt_widgets_c_QMouseEventTransition_new_object_type_button_sourceState(QObject* object, const QEvent::Type* type, const Qt::MouseButton* button, QState* sourceState) {
  return new QMouseEventTransition(object, *type, *button, sourceState);
}

QMouseEventTransition* qt_widgets_c_QMouseEventTransition_new_sourceState(QState* sourceState) {
  return new QMouseEventTransition(sourceState);
}

int qt_widgets_c_QMouseEventTransition_qt_metacall(QMouseEventTransition* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QMouseEventTransition_qt_metacast(QMouseEventTransition* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QMouseEventTransition_setButton(QMouseEventTransition* this_ptr, const Qt::MouseButton* button) {
  this_ptr->setButton(*button);
}

void qt_widgets_c_QMouseEventTransition_setHitTestPath(QMouseEventTransition* this_ptr, const QPainterPath* path) {
  this_ptr->setHitTestPath(*path);
}

void qt_widgets_c_QMouseEventTransition_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QMouseEventTransition::trUtf8(s, c, n));
}

void qt_widgets_c_QMouseEventTransition_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QMouseEventTransition::tr(s, c, n));
}


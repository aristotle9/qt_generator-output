#include "qt_widgets_c_QWidgetAction.h"

QWidgetAction* qt_widgets_c_QWidgetAction_G_dynamic_cast_QWidgetAction_ptr(QAction* ptr) {
  return dynamic_cast<QWidgetAction*>(ptr);
}

QAction* qt_widgets_c_QWidgetAction_G_static_cast_QAction_ptr(QWidgetAction* ptr) {
  return static_cast<QAction*>(ptr);
}

QObject* qt_widgets_c_QWidgetAction_G_static_cast_QObject_ptr(QWidgetAction* ptr) {
  return static_cast<QObject*>(ptr);
}

QWidgetAction* qt_widgets_c_QWidgetAction_G_static_cast_QWidgetAction_ptr_QAction(QAction* ptr) {
  return static_cast<QWidgetAction*>(ptr);
}

QWidgetAction* qt_widgets_c_QWidgetAction_G_static_cast_QWidgetAction_ptr_QObject(QObject* ptr) {
  return static_cast<QWidgetAction*>(ptr);
}

QWidget* qt_widgets_c_QWidgetAction_defaultWidget(const QWidgetAction* this_ptr) {
  return this_ptr->defaultWidget();
}

void qt_widgets_c_QWidgetAction_delete(QWidgetAction* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_widgets_c_QWidgetAction_metaObject(const QWidgetAction* this_ptr) {
  return this_ptr->metaObject();
}

QWidgetAction* qt_widgets_c_QWidgetAction_new(QObject* parent) {
  return new QWidgetAction(parent);
}

int qt_widgets_c_QWidgetAction_qt_metacall(QWidgetAction* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QWidgetAction_qt_metacast(QWidgetAction* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QWidgetAction_releaseWidget(QWidgetAction* this_ptr, QWidget* widget) {
  this_ptr->releaseWidget(widget);
}

QWidget* qt_widgets_c_QWidgetAction_requestWidget(QWidgetAction* this_ptr, QWidget* parent) {
  return this_ptr->requestWidget(parent);
}

void qt_widgets_c_QWidgetAction_setDefaultWidget(QWidgetAction* this_ptr, QWidget* w) {
  this_ptr->setDefaultWidget(w);
}

void qt_widgets_c_QWidgetAction_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QWidgetAction::trUtf8(s, c, n));
}

void qt_widgets_c_QWidgetAction_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QWidgetAction::tr(s, c, n));
}


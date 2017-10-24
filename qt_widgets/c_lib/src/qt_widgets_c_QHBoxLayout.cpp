#include "qt_widgets_c_QHBoxLayout.h"

QHBoxLayout* qt_widgets_c_QHBoxLayout_G_dynamic_cast_QHBoxLayout_ptr_QBoxLayout(QBoxLayout* ptr) {
  return dynamic_cast<QHBoxLayout*>(ptr);
}

QHBoxLayout* qt_widgets_c_QHBoxLayout_G_dynamic_cast_QHBoxLayout_ptr_QLayout(QLayout* ptr) {
  return dynamic_cast<QHBoxLayout*>(ptr);
}

QHBoxLayout* qt_widgets_c_QHBoxLayout_G_dynamic_cast_QHBoxLayout_ptr_QLayoutItem(QLayoutItem* ptr) {
  return dynamic_cast<QHBoxLayout*>(ptr);
}

QBoxLayout* qt_widgets_c_QHBoxLayout_G_static_cast_QBoxLayout_ptr(QHBoxLayout* ptr) {
  return static_cast<QBoxLayout*>(ptr);
}

QHBoxLayout* qt_widgets_c_QHBoxLayout_G_static_cast_QHBoxLayout_ptr_QBoxLayout(QBoxLayout* ptr) {
  return static_cast<QHBoxLayout*>(ptr);
}

QHBoxLayout* qt_widgets_c_QHBoxLayout_G_static_cast_QHBoxLayout_ptr_QLayout(QLayout* ptr) {
  return static_cast<QHBoxLayout*>(ptr);
}

QHBoxLayout* qt_widgets_c_QHBoxLayout_G_static_cast_QHBoxLayout_ptr_QLayoutItem(QLayoutItem* ptr) {
  return static_cast<QHBoxLayout*>(ptr);
}

QHBoxLayout* qt_widgets_c_QHBoxLayout_G_static_cast_QHBoxLayout_ptr_QObject(QObject* ptr) {
  return static_cast<QHBoxLayout*>(ptr);
}

QLayoutItem* qt_widgets_c_QHBoxLayout_G_static_cast_QLayoutItem_ptr(QHBoxLayout* ptr) {
  return static_cast<QLayoutItem*>(ptr);
}

QLayout* qt_widgets_c_QHBoxLayout_G_static_cast_QLayout_ptr(QHBoxLayout* ptr) {
  return static_cast<QLayout*>(ptr);
}

QObject* qt_widgets_c_QHBoxLayout_G_static_cast_QObject_ptr(QHBoxLayout* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_widgets_c_QHBoxLayout_delete(QHBoxLayout* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_widgets_c_QHBoxLayout_metaObject(const QHBoxLayout* this_ptr) {
  return this_ptr->metaObject();
}

QHBoxLayout* qt_widgets_c_QHBoxLayout_new_no_args() {
  return new QHBoxLayout();
}

QHBoxLayout* qt_widgets_c_QHBoxLayout_new_parent(QWidget* parent) {
  return new QHBoxLayout(parent);
}

int qt_widgets_c_QHBoxLayout_qt_metacall(QHBoxLayout* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QHBoxLayout_qt_metacast(QHBoxLayout* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QHBoxLayout_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QHBoxLayout::trUtf8(s, c, n));
}

void qt_widgets_c_QHBoxLayout_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QHBoxLayout::tr(s, c, n));
}


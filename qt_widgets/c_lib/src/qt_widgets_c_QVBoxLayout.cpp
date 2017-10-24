#include "qt_widgets_c_QVBoxLayout.h"

QVBoxLayout* qt_widgets_c_QVBoxLayout_G_dynamic_cast_QVBoxLayout_ptr_QBoxLayout(QBoxLayout* ptr) {
  return dynamic_cast<QVBoxLayout*>(ptr);
}

QVBoxLayout* qt_widgets_c_QVBoxLayout_G_dynamic_cast_QVBoxLayout_ptr_QLayout(QLayout* ptr) {
  return dynamic_cast<QVBoxLayout*>(ptr);
}

QVBoxLayout* qt_widgets_c_QVBoxLayout_G_dynamic_cast_QVBoxLayout_ptr_QLayoutItem(QLayoutItem* ptr) {
  return dynamic_cast<QVBoxLayout*>(ptr);
}

QBoxLayout* qt_widgets_c_QVBoxLayout_G_static_cast_QBoxLayout_ptr(QVBoxLayout* ptr) {
  return static_cast<QBoxLayout*>(ptr);
}

QLayoutItem* qt_widgets_c_QVBoxLayout_G_static_cast_QLayoutItem_ptr(QVBoxLayout* ptr) {
  return static_cast<QLayoutItem*>(ptr);
}

QLayout* qt_widgets_c_QVBoxLayout_G_static_cast_QLayout_ptr(QVBoxLayout* ptr) {
  return static_cast<QLayout*>(ptr);
}

QObject* qt_widgets_c_QVBoxLayout_G_static_cast_QObject_ptr(QVBoxLayout* ptr) {
  return static_cast<QObject*>(ptr);
}

QVBoxLayout* qt_widgets_c_QVBoxLayout_G_static_cast_QVBoxLayout_ptr_QBoxLayout(QBoxLayout* ptr) {
  return static_cast<QVBoxLayout*>(ptr);
}

QVBoxLayout* qt_widgets_c_QVBoxLayout_G_static_cast_QVBoxLayout_ptr_QLayout(QLayout* ptr) {
  return static_cast<QVBoxLayout*>(ptr);
}

QVBoxLayout* qt_widgets_c_QVBoxLayout_G_static_cast_QVBoxLayout_ptr_QLayoutItem(QLayoutItem* ptr) {
  return static_cast<QVBoxLayout*>(ptr);
}

QVBoxLayout* qt_widgets_c_QVBoxLayout_G_static_cast_QVBoxLayout_ptr_QObject(QObject* ptr) {
  return static_cast<QVBoxLayout*>(ptr);
}

void qt_widgets_c_QVBoxLayout_delete(QVBoxLayout* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_widgets_c_QVBoxLayout_metaObject(const QVBoxLayout* this_ptr) {
  return this_ptr->metaObject();
}

QVBoxLayout* qt_widgets_c_QVBoxLayout_new_no_args() {
  return new QVBoxLayout();
}

QVBoxLayout* qt_widgets_c_QVBoxLayout_new_parent(QWidget* parent) {
  return new QVBoxLayout(parent);
}

int qt_widgets_c_QVBoxLayout_qt_metacall(QVBoxLayout* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QVBoxLayout_qt_metacast(QVBoxLayout* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QVBoxLayout_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QVBoxLayout::trUtf8(s, c, n));
}

void qt_widgets_c_QVBoxLayout_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QVBoxLayout::tr(s, c, n));
}


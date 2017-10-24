#include "qt_widgets_c_QTimeEdit.h"

QTimeEdit* qt_widgets_c_QTimeEdit_G_dynamic_cast_QTimeEdit_ptr_QAbstractSpinBox(QAbstractSpinBox* ptr) {
  return dynamic_cast<QTimeEdit*>(ptr);
}

QTimeEdit* qt_widgets_c_QTimeEdit_G_dynamic_cast_QTimeEdit_ptr_QDateTimeEdit(QDateTimeEdit* ptr) {
  return dynamic_cast<QTimeEdit*>(ptr);
}

QTimeEdit* qt_widgets_c_QTimeEdit_G_dynamic_cast_QTimeEdit_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QTimeEdit*>(ptr);
}

QAbstractSpinBox* qt_widgets_c_QTimeEdit_G_static_cast_QAbstractSpinBox_ptr(QTimeEdit* ptr) {
  return static_cast<QAbstractSpinBox*>(ptr);
}

QDateTimeEdit* qt_widgets_c_QTimeEdit_G_static_cast_QDateTimeEdit_ptr(QTimeEdit* ptr) {
  return static_cast<QDateTimeEdit*>(ptr);
}

QObject* qt_widgets_c_QTimeEdit_G_static_cast_QObject_ptr(QTimeEdit* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QTimeEdit_G_static_cast_QPaintDevice_ptr(QTimeEdit* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QTimeEdit* qt_widgets_c_QTimeEdit_G_static_cast_QTimeEdit_ptr_QAbstractSpinBox(QAbstractSpinBox* ptr) {
  return static_cast<QTimeEdit*>(ptr);
}

QTimeEdit* qt_widgets_c_QTimeEdit_G_static_cast_QTimeEdit_ptr_QDateTimeEdit(QDateTimeEdit* ptr) {
  return static_cast<QTimeEdit*>(ptr);
}

QTimeEdit* qt_widgets_c_QTimeEdit_G_static_cast_QTimeEdit_ptr_QObject(QObject* ptr) {
  return static_cast<QTimeEdit*>(ptr);
}

QTimeEdit* qt_widgets_c_QTimeEdit_G_static_cast_QTimeEdit_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QTimeEdit*>(ptr);
}

QTimeEdit* qt_widgets_c_QTimeEdit_G_static_cast_QTimeEdit_ptr_QWidget(QWidget* ptr) {
  return static_cast<QTimeEdit*>(ptr);
}

QWidget* qt_widgets_c_QTimeEdit_G_static_cast_QWidget_ptr(QTimeEdit* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QTimeEdit_delete(QTimeEdit* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_widgets_c_QTimeEdit_metaObject(const QTimeEdit* this_ptr) {
  return this_ptr->metaObject();
}

QTimeEdit* qt_widgets_c_QTimeEdit_new_no_args() {
  return new QTimeEdit();
}

QTimeEdit* qt_widgets_c_QTimeEdit_new_parent(QWidget* parent) {
  return new QTimeEdit(parent);
}

QTimeEdit* qt_widgets_c_QTimeEdit_new_time(const QTime* time) {
  return new QTimeEdit(*time);
}

QTimeEdit* qt_widgets_c_QTimeEdit_new_time_parent(const QTime* time, QWidget* parent) {
  return new QTimeEdit(*time, parent);
}

int qt_widgets_c_QTimeEdit_qt_metacall(QTimeEdit* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QTimeEdit_qt_metacast(QTimeEdit* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QTimeEdit_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTimeEdit::trUtf8(s, c, n));
}

void qt_widgets_c_QTimeEdit_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTimeEdit::tr(s, c, n));
}


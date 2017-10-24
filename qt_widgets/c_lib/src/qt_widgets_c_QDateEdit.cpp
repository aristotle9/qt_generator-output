#include "qt_widgets_c_QDateEdit.h"

QDateEdit* qt_widgets_c_QDateEdit_G_dynamic_cast_QDateEdit_ptr_QAbstractSpinBox(QAbstractSpinBox* ptr) {
  return dynamic_cast<QDateEdit*>(ptr);
}

QDateEdit* qt_widgets_c_QDateEdit_G_dynamic_cast_QDateEdit_ptr_QDateTimeEdit(QDateTimeEdit* ptr) {
  return dynamic_cast<QDateEdit*>(ptr);
}

QDateEdit* qt_widgets_c_QDateEdit_G_dynamic_cast_QDateEdit_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QDateEdit*>(ptr);
}

QAbstractSpinBox* qt_widgets_c_QDateEdit_G_static_cast_QAbstractSpinBox_ptr(QDateEdit* ptr) {
  return static_cast<QAbstractSpinBox*>(ptr);
}

QDateEdit* qt_widgets_c_QDateEdit_G_static_cast_QDateEdit_ptr_QAbstractSpinBox(QAbstractSpinBox* ptr) {
  return static_cast<QDateEdit*>(ptr);
}

QDateEdit* qt_widgets_c_QDateEdit_G_static_cast_QDateEdit_ptr_QDateTimeEdit(QDateTimeEdit* ptr) {
  return static_cast<QDateEdit*>(ptr);
}

QDateEdit* qt_widgets_c_QDateEdit_G_static_cast_QDateEdit_ptr_QObject(QObject* ptr) {
  return static_cast<QDateEdit*>(ptr);
}

QDateEdit* qt_widgets_c_QDateEdit_G_static_cast_QDateEdit_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QDateEdit*>(ptr);
}

QDateEdit* qt_widgets_c_QDateEdit_G_static_cast_QDateEdit_ptr_QWidget(QWidget* ptr) {
  return static_cast<QDateEdit*>(ptr);
}

QDateTimeEdit* qt_widgets_c_QDateEdit_G_static_cast_QDateTimeEdit_ptr(QDateEdit* ptr) {
  return static_cast<QDateTimeEdit*>(ptr);
}

QObject* qt_widgets_c_QDateEdit_G_static_cast_QObject_ptr(QDateEdit* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QDateEdit_G_static_cast_QPaintDevice_ptr(QDateEdit* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QDateEdit_G_static_cast_QWidget_ptr(QDateEdit* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QDateEdit_delete(QDateEdit* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_widgets_c_QDateEdit_metaObject(const QDateEdit* this_ptr) {
  return this_ptr->metaObject();
}

QDateEdit* qt_widgets_c_QDateEdit_new_date(const QDate* date) {
  return new QDateEdit(*date);
}

QDateEdit* qt_widgets_c_QDateEdit_new_date_parent(const QDate* date, QWidget* parent) {
  return new QDateEdit(*date, parent);
}

QDateEdit* qt_widgets_c_QDateEdit_new_no_args() {
  return new QDateEdit();
}

QDateEdit* qt_widgets_c_QDateEdit_new_parent(QWidget* parent) {
  return new QDateEdit(parent);
}

int qt_widgets_c_QDateEdit_qt_metacall(QDateEdit* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QDateEdit_qt_metacast(QDateEdit* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QDateEdit_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QDateEdit::trUtf8(s, c, n));
}

void qt_widgets_c_QDateEdit_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QDateEdit::tr(s, c, n));
}


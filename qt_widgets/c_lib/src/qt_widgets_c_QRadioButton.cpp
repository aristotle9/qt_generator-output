#include "qt_widgets_c_QRadioButton.h"

QRadioButton* qt_widgets_c_QRadioButton_G_dynamic_cast_QRadioButton_ptr_QAbstractButton(QAbstractButton* ptr) {
  return dynamic_cast<QRadioButton*>(ptr);
}

QRadioButton* qt_widgets_c_QRadioButton_G_dynamic_cast_QRadioButton_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QRadioButton*>(ptr);
}

QAbstractButton* qt_widgets_c_QRadioButton_G_static_cast_QAbstractButton_ptr(QRadioButton* ptr) {
  return static_cast<QAbstractButton*>(ptr);
}

QObject* qt_widgets_c_QRadioButton_G_static_cast_QObject_ptr(QRadioButton* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QRadioButton_G_static_cast_QPaintDevice_ptr(QRadioButton* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QRadioButton* qt_widgets_c_QRadioButton_G_static_cast_QRadioButton_ptr_QAbstractButton(QAbstractButton* ptr) {
  return static_cast<QRadioButton*>(ptr);
}

QRadioButton* qt_widgets_c_QRadioButton_G_static_cast_QRadioButton_ptr_QObject(QObject* ptr) {
  return static_cast<QRadioButton*>(ptr);
}

QRadioButton* qt_widgets_c_QRadioButton_G_static_cast_QRadioButton_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QRadioButton*>(ptr);
}

QRadioButton* qt_widgets_c_QRadioButton_G_static_cast_QRadioButton_ptr_QWidget(QWidget* ptr) {
  return static_cast<QRadioButton*>(ptr);
}

QWidget* qt_widgets_c_QRadioButton_G_static_cast_QWidget_ptr(QRadioButton* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QRadioButton_delete(QRadioButton* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_widgets_c_QRadioButton_metaObject(const QRadioButton* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QRadioButton_minimumSizeHint_to_output(const QRadioButton* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSizeHint());
}

QRadioButton* qt_widgets_c_QRadioButton_new_no_args() {
  return new QRadioButton();
}

QRadioButton* qt_widgets_c_QRadioButton_new_parent(QWidget* parent) {
  return new QRadioButton(parent);
}

QRadioButton* qt_widgets_c_QRadioButton_new_text(const QString* text) {
  return new QRadioButton(*text);
}

QRadioButton* qt_widgets_c_QRadioButton_new_text_parent(const QString* text, QWidget* parent) {
  return new QRadioButton(*text, parent);
}

int qt_widgets_c_QRadioButton_qt_metacall(QRadioButton* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QRadioButton_qt_metacast(QRadioButton* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QRadioButton_sizeHint_to_output(const QRadioButton* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QRadioButton_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QRadioButton::trUtf8(s, c, n));
}

void qt_widgets_c_QRadioButton_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QRadioButton::tr(s, c, n));
}


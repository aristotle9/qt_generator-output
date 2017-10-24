#include "qt_widgets_c_QCheckBox.h"

QCheckBox* qt_widgets_c_QCheckBox_G_dynamic_cast_QCheckBox_ptr_QAbstractButton(QAbstractButton* ptr) {
  return dynamic_cast<QCheckBox*>(ptr);
}

QCheckBox* qt_widgets_c_QCheckBox_G_dynamic_cast_QCheckBox_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QCheckBox*>(ptr);
}

QAbstractButton* qt_widgets_c_QCheckBox_G_static_cast_QAbstractButton_ptr(QCheckBox* ptr) {
  return static_cast<QAbstractButton*>(ptr);
}

QCheckBox* qt_widgets_c_QCheckBox_G_static_cast_QCheckBox_ptr_QAbstractButton(QAbstractButton* ptr) {
  return static_cast<QCheckBox*>(ptr);
}

QCheckBox* qt_widgets_c_QCheckBox_G_static_cast_QCheckBox_ptr_QObject(QObject* ptr) {
  return static_cast<QCheckBox*>(ptr);
}

QCheckBox* qt_widgets_c_QCheckBox_G_static_cast_QCheckBox_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QCheckBox*>(ptr);
}

QCheckBox* qt_widgets_c_QCheckBox_G_static_cast_QCheckBox_ptr_QWidget(QWidget* ptr) {
  return static_cast<QCheckBox*>(ptr);
}

QObject* qt_widgets_c_QCheckBox_G_static_cast_QObject_ptr(QCheckBox* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QCheckBox_G_static_cast_QPaintDevice_ptr(QCheckBox* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QCheckBox_G_static_cast_QWidget_ptr(QCheckBox* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QCheckBox_delete(QCheckBox* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QCheckBox_isTristate(const QCheckBox* this_ptr) {
  return this_ptr->isTristate();
}

const QMetaObject* qt_widgets_c_QCheckBox_metaObject(const QCheckBox* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QCheckBox_minimumSizeHint_to_output(const QCheckBox* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSizeHint());
}

QCheckBox* qt_widgets_c_QCheckBox_new_no_args() {
  return new QCheckBox();
}

QCheckBox* qt_widgets_c_QCheckBox_new_parent(QWidget* parent) {
  return new QCheckBox(parent);
}

QCheckBox* qt_widgets_c_QCheckBox_new_text(const QString* text) {
  return new QCheckBox(*text);
}

QCheckBox* qt_widgets_c_QCheckBox_new_text_parent(const QString* text, QWidget* parent) {
  return new QCheckBox(*text, parent);
}

int qt_widgets_c_QCheckBox_qt_metacall(QCheckBox* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QCheckBox_qt_metacast(QCheckBox* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QCheckBox_setCheckState(QCheckBox* this_ptr, const Qt::CheckState* state) {
  this_ptr->setCheckState(*state);
}

void qt_widgets_c_QCheckBox_setTristate_no_args(QCheckBox* this_ptr) {
  this_ptr->setTristate();
}

void qt_widgets_c_QCheckBox_setTristate_y(QCheckBox* this_ptr, bool y) {
  this_ptr->setTristate(y);
}

void qt_widgets_c_QCheckBox_sizeHint_to_output(const QCheckBox* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QCheckBox_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QCheckBox::trUtf8(s, c, n));
}

void qt_widgets_c_QCheckBox_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QCheckBox::tr(s, c, n));
}


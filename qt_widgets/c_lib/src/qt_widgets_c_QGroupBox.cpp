#include "qt_widgets_c_QGroupBox.h"

QGroupBox* qt_widgets_c_QGroupBox_G_dynamic_cast_QGroupBox_ptr(QWidget* ptr) {
  return dynamic_cast<QGroupBox*>(ptr);
}

QGroupBox* qt_widgets_c_QGroupBox_G_static_cast_QGroupBox_ptr_QObject(QObject* ptr) {
  return static_cast<QGroupBox*>(ptr);
}

QGroupBox* qt_widgets_c_QGroupBox_G_static_cast_QGroupBox_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QGroupBox*>(ptr);
}

QGroupBox* qt_widgets_c_QGroupBox_G_static_cast_QGroupBox_ptr_QWidget(QWidget* ptr) {
  return static_cast<QGroupBox*>(ptr);
}

QObject* qt_widgets_c_QGroupBox_G_static_cast_QObject_ptr(QGroupBox* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QGroupBox_G_static_cast_QPaintDevice_ptr(QGroupBox* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QGroupBox_G_static_cast_QWidget_ptr(QGroupBox* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QGroupBox_delete(QGroupBox* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QGroupBox_isCheckable(const QGroupBox* this_ptr) {
  return this_ptr->isCheckable();
}

bool qt_widgets_c_QGroupBox_isChecked(const QGroupBox* this_ptr) {
  return this_ptr->isChecked();
}

bool qt_widgets_c_QGroupBox_isFlat(const QGroupBox* this_ptr) {
  return this_ptr->isFlat();
}

const QMetaObject* qt_widgets_c_QGroupBox_metaObject(const QGroupBox* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QGroupBox_minimumSizeHint_to_output(const QGroupBox* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSizeHint());
}

QGroupBox* qt_widgets_c_QGroupBox_new_no_args() {
  return new QGroupBox();
}

QGroupBox* qt_widgets_c_QGroupBox_new_parent(QWidget* parent) {
  return new QGroupBox(parent);
}

QGroupBox* qt_widgets_c_QGroupBox_new_title(const QString* title) {
  return new QGroupBox(*title);
}

QGroupBox* qt_widgets_c_QGroupBox_new_title_parent(const QString* title, QWidget* parent) {
  return new QGroupBox(*title, parent);
}

int qt_widgets_c_QGroupBox_qt_metacall(QGroupBox* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QGroupBox_qt_metacast(QGroupBox* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QGroupBox_setAlignment(QGroupBox* this_ptr, int alignment) {
  this_ptr->setAlignment(alignment);
}

void qt_widgets_c_QGroupBox_setCheckable(QGroupBox* this_ptr, bool checkable) {
  this_ptr->setCheckable(checkable);
}

void qt_widgets_c_QGroupBox_setChecked(QGroupBox* this_ptr, bool checked) {
  this_ptr->setChecked(checked);
}

void qt_widgets_c_QGroupBox_setFlat(QGroupBox* this_ptr, bool flat) {
  this_ptr->setFlat(flat);
}

void qt_widgets_c_QGroupBox_setTitle(QGroupBox* this_ptr, const QString* title) {
  this_ptr->setTitle(*title);
}

void qt_widgets_c_QGroupBox_title_to_output(const QGroupBox* this_ptr, QString* output) {
  new(output) QString(this_ptr->title());
}

void qt_widgets_c_QGroupBox_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGroupBox::trUtf8(s, c, n));
}

void qt_widgets_c_QGroupBox_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGroupBox::tr(s, c, n));
}


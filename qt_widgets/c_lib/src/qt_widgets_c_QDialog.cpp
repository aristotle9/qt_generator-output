#include "qt_widgets_c_QDialog.h"

QDialog* qt_widgets_c_QDialog_G_dynamic_cast_QDialog_ptr(QWidget* ptr) {
  return dynamic_cast<QDialog*>(ptr);
}

QDialog* qt_widgets_c_QDialog_G_static_cast_QDialog_ptr_QObject(QObject* ptr) {
  return static_cast<QDialog*>(ptr);
}

QDialog* qt_widgets_c_QDialog_G_static_cast_QDialog_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QDialog*>(ptr);
}

QDialog* qt_widgets_c_QDialog_G_static_cast_QDialog_ptr_QWidget(QWidget* ptr) {
  return static_cast<QDialog*>(ptr);
}

QObject* qt_widgets_c_QDialog_G_static_cast_QObject_ptr(QDialog* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QDialog_G_static_cast_QPaintDevice_ptr(QDialog* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QDialog_G_static_cast_QWidget_ptr(QDialog* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QDialog_accept(QDialog* this_ptr) {
  this_ptr->accept();
}

void qt_widgets_c_QDialog_delete(QDialog* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QDialog_done(QDialog* this_ptr, int arg1) {
  this_ptr->done(arg1);
}

int qt_widgets_c_QDialog_exec(QDialog* this_ptr) {
  return this_ptr->exec();
}

QWidget* qt_widgets_c_QDialog_extension(const QDialog* this_ptr) {
  return this_ptr->extension();
}

bool qt_widgets_c_QDialog_isSizeGripEnabled(const QDialog* this_ptr) {
  return this_ptr->isSizeGripEnabled();
}

const QMetaObject* qt_widgets_c_QDialog_metaObject(const QDialog* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QDialog_minimumSizeHint_to_output(const QDialog* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSizeHint());
}

void qt_widgets_c_QDialog_open(QDialog* this_ptr) {
  this_ptr->open();
}

int qt_widgets_c_QDialog_qt_metacall(QDialog* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QDialog_qt_metacast(QDialog* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QDialog_reject(QDialog* this_ptr) {
  this_ptr->reject();
}

int qt_widgets_c_QDialog_result(const QDialog* this_ptr) {
  return this_ptr->result();
}

void qt_widgets_c_QDialog_setExtension(QDialog* this_ptr, QWidget* extension) {
  this_ptr->setExtension(extension);
}

void qt_widgets_c_QDialog_setModal(QDialog* this_ptr, bool modal) {
  this_ptr->setModal(modal);
}

void qt_widgets_c_QDialog_setOrientation(QDialog* this_ptr, const Qt::Orientation* orientation) {
  this_ptr->setOrientation(*orientation);
}

void qt_widgets_c_QDialog_setResult(QDialog* this_ptr, int r) {
  this_ptr->setResult(r);
}

void qt_widgets_c_QDialog_setSizeGripEnabled(QDialog* this_ptr, bool arg1) {
  this_ptr->setSizeGripEnabled(arg1);
}

void qt_widgets_c_QDialog_setVisible(QDialog* this_ptr, bool visible) {
  this_ptr->setVisible(visible);
}

void qt_widgets_c_QDialog_showExtension(QDialog* this_ptr, bool arg1) {
  this_ptr->showExtension(arg1);
}

void qt_widgets_c_QDialog_sizeHint_to_output(const QDialog* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QDialog_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QDialog::trUtf8(s, c, n));
}

void qt_widgets_c_QDialog_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QDialog::tr(s, c, n));
}


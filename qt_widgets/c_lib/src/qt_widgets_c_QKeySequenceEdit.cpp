#include "qt_widgets_c_QKeySequenceEdit.h"

QKeySequenceEdit* qt_widgets_c_QKeySequenceEdit_G_dynamic_cast_QKeySequenceEdit_ptr(QWidget* ptr) {
  return dynamic_cast<QKeySequenceEdit*>(ptr);
}

QKeySequenceEdit* qt_widgets_c_QKeySequenceEdit_G_static_cast_QKeySequenceEdit_ptr_QObject(QObject* ptr) {
  return static_cast<QKeySequenceEdit*>(ptr);
}

QKeySequenceEdit* qt_widgets_c_QKeySequenceEdit_G_static_cast_QKeySequenceEdit_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QKeySequenceEdit*>(ptr);
}

QKeySequenceEdit* qt_widgets_c_QKeySequenceEdit_G_static_cast_QKeySequenceEdit_ptr_QWidget(QWidget* ptr) {
  return static_cast<QKeySequenceEdit*>(ptr);
}

QObject* qt_widgets_c_QKeySequenceEdit_G_static_cast_QObject_ptr(QKeySequenceEdit* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QKeySequenceEdit_G_static_cast_QPaintDevice_ptr(QKeySequenceEdit* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QKeySequenceEdit_G_static_cast_QWidget_ptr(QKeySequenceEdit* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QKeySequenceEdit_clear(QKeySequenceEdit* this_ptr) {
  this_ptr->clear();
}

void qt_widgets_c_QKeySequenceEdit_delete(QKeySequenceEdit* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QKeySequenceEdit_keySequence_to_output(const QKeySequenceEdit* this_ptr, QKeySequence* output) {
  new(output) QKeySequence(this_ptr->keySequence());
}

const QMetaObject* qt_widgets_c_QKeySequenceEdit_metaObject(const QKeySequenceEdit* this_ptr) {
  return this_ptr->metaObject();
}

QKeySequenceEdit* qt_widgets_c_QKeySequenceEdit_new_keySequence(const QKeySequence* keySequence) {
  return new QKeySequenceEdit(*keySequence);
}

QKeySequenceEdit* qt_widgets_c_QKeySequenceEdit_new_keySequence_parent(const QKeySequence* keySequence, QWidget* parent) {
  return new QKeySequenceEdit(*keySequence, parent);
}

QKeySequenceEdit* qt_widgets_c_QKeySequenceEdit_new_no_args() {
  return new QKeySequenceEdit();
}

QKeySequenceEdit* qt_widgets_c_QKeySequenceEdit_new_parent(QWidget* parent) {
  return new QKeySequenceEdit(parent);
}

int qt_widgets_c_QKeySequenceEdit_qt_metacall(QKeySequenceEdit* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QKeySequenceEdit_qt_metacast(QKeySequenceEdit* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QKeySequenceEdit_setKeySequence(QKeySequenceEdit* this_ptr, const QKeySequence* keySequence) {
  this_ptr->setKeySequence(*keySequence);
}

void qt_widgets_c_QKeySequenceEdit_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QKeySequenceEdit::trUtf8(s, c, n));
}

void qt_widgets_c_QKeySequenceEdit_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QKeySequenceEdit::tr(s, c, n));
}


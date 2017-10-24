#include "qt_widgets_c_QErrorMessage.h"

QErrorMessage* qt_widgets_c_QErrorMessage_G_dynamic_cast_QErrorMessage_ptr_QDialog(QDialog* ptr) {
  return dynamic_cast<QErrorMessage*>(ptr);
}

QErrorMessage* qt_widgets_c_QErrorMessage_G_dynamic_cast_QErrorMessage_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QErrorMessage*>(ptr);
}

QDialog* qt_widgets_c_QErrorMessage_G_static_cast_QDialog_ptr(QErrorMessage* ptr) {
  return static_cast<QDialog*>(ptr);
}

QErrorMessage* qt_widgets_c_QErrorMessage_G_static_cast_QErrorMessage_ptr_QDialog(QDialog* ptr) {
  return static_cast<QErrorMessage*>(ptr);
}

QErrorMessage* qt_widgets_c_QErrorMessage_G_static_cast_QErrorMessage_ptr_QObject(QObject* ptr) {
  return static_cast<QErrorMessage*>(ptr);
}

QErrorMessage* qt_widgets_c_QErrorMessage_G_static_cast_QErrorMessage_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QErrorMessage*>(ptr);
}

QErrorMessage* qt_widgets_c_QErrorMessage_G_static_cast_QErrorMessage_ptr_QWidget(QWidget* ptr) {
  return static_cast<QErrorMessage*>(ptr);
}

QObject* qt_widgets_c_QErrorMessage_G_static_cast_QObject_ptr(QErrorMessage* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QErrorMessage_G_static_cast_QPaintDevice_ptr(QErrorMessage* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QErrorMessage_G_static_cast_QWidget_ptr(QErrorMessage* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QErrorMessage_delete(QErrorMessage* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_widgets_c_QErrorMessage_metaObject(const QErrorMessage* this_ptr) {
  return this_ptr->metaObject();
}

QErrorMessage* qt_widgets_c_QErrorMessage_new_no_args() {
  return new QErrorMessage();
}

QErrorMessage* qt_widgets_c_QErrorMessage_new_parent(QWidget* parent) {
  return new QErrorMessage(parent);
}

QErrorMessage* qt_widgets_c_QErrorMessage_qtHandler() {
  return QErrorMessage::qtHandler();
}

int qt_widgets_c_QErrorMessage_qt_metacall(QErrorMessage* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QErrorMessage_qt_metacast(QErrorMessage* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QErrorMessage_showMessage_message(QErrorMessage* this_ptr, const QString* message) {
  this_ptr->showMessage(*message);
}

void qt_widgets_c_QErrorMessage_showMessage_message_type(QErrorMessage* this_ptr, const QString* message, const QString* type) {
  this_ptr->showMessage(*message, *type);
}

void qt_widgets_c_QErrorMessage_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QErrorMessage::trUtf8(s, c, n));
}

void qt_widgets_c_QErrorMessage_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QErrorMessage::tr(s, c, n));
}


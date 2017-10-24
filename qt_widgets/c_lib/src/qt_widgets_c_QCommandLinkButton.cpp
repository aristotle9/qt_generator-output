#include "qt_widgets_c_QCommandLinkButton.h"

QCommandLinkButton* qt_widgets_c_QCommandLinkButton_G_dynamic_cast_QCommandLinkButton_ptr_QAbstractButton(QAbstractButton* ptr) {
  return dynamic_cast<QCommandLinkButton*>(ptr);
}

QCommandLinkButton* qt_widgets_c_QCommandLinkButton_G_dynamic_cast_QCommandLinkButton_ptr_QPushButton(QPushButton* ptr) {
  return dynamic_cast<QCommandLinkButton*>(ptr);
}

QCommandLinkButton* qt_widgets_c_QCommandLinkButton_G_dynamic_cast_QCommandLinkButton_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QCommandLinkButton*>(ptr);
}

QAbstractButton* qt_widgets_c_QCommandLinkButton_G_static_cast_QAbstractButton_ptr(QCommandLinkButton* ptr) {
  return static_cast<QAbstractButton*>(ptr);
}

QCommandLinkButton* qt_widgets_c_QCommandLinkButton_G_static_cast_QCommandLinkButton_ptr_QAbstractButton(QAbstractButton* ptr) {
  return static_cast<QCommandLinkButton*>(ptr);
}

QCommandLinkButton* qt_widgets_c_QCommandLinkButton_G_static_cast_QCommandLinkButton_ptr_QObject(QObject* ptr) {
  return static_cast<QCommandLinkButton*>(ptr);
}

QCommandLinkButton* qt_widgets_c_QCommandLinkButton_G_static_cast_QCommandLinkButton_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QCommandLinkButton*>(ptr);
}

QCommandLinkButton* qt_widgets_c_QCommandLinkButton_G_static_cast_QCommandLinkButton_ptr_QPushButton(QPushButton* ptr) {
  return static_cast<QCommandLinkButton*>(ptr);
}

QCommandLinkButton* qt_widgets_c_QCommandLinkButton_G_static_cast_QCommandLinkButton_ptr_QWidget(QWidget* ptr) {
  return static_cast<QCommandLinkButton*>(ptr);
}

QObject* qt_widgets_c_QCommandLinkButton_G_static_cast_QObject_ptr(QCommandLinkButton* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QCommandLinkButton_G_static_cast_QPaintDevice_ptr(QCommandLinkButton* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QPushButton* qt_widgets_c_QCommandLinkButton_G_static_cast_QPushButton_ptr(QCommandLinkButton* ptr) {
  return static_cast<QPushButton*>(ptr);
}

QWidget* qt_widgets_c_QCommandLinkButton_G_static_cast_QWidget_ptr(QCommandLinkButton* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QCommandLinkButton_delete(QCommandLinkButton* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QCommandLinkButton_description_to_output(const QCommandLinkButton* this_ptr, QString* output) {
  new(output) QString(this_ptr->description());
}

const QMetaObject* qt_widgets_c_QCommandLinkButton_metaObject(const QCommandLinkButton* this_ptr) {
  return this_ptr->metaObject();
}

QCommandLinkButton* qt_widgets_c_QCommandLinkButton_new_no_args() {
  return new QCommandLinkButton();
}

QCommandLinkButton* qt_widgets_c_QCommandLinkButton_new_parent(QWidget* parent) {
  return new QCommandLinkButton(parent);
}

QCommandLinkButton* qt_widgets_c_QCommandLinkButton_new_text(const QString* text) {
  return new QCommandLinkButton(*text);
}

QCommandLinkButton* qt_widgets_c_QCommandLinkButton_new_text_description(const QString* text, const QString* description) {
  return new QCommandLinkButton(*text, *description);
}

QCommandLinkButton* qt_widgets_c_QCommandLinkButton_new_text_description_parent(const QString* text, const QString* description, QWidget* parent) {
  return new QCommandLinkButton(*text, *description, parent);
}

QCommandLinkButton* qt_widgets_c_QCommandLinkButton_new_text_parent(const QString* text, QWidget* parent) {
  return new QCommandLinkButton(*text, parent);
}

int qt_widgets_c_QCommandLinkButton_qt_metacall(QCommandLinkButton* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QCommandLinkButton_qt_metacast(QCommandLinkButton* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QCommandLinkButton_setDescription(QCommandLinkButton* this_ptr, const QString* description) {
  this_ptr->setDescription(*description);
}

void qt_widgets_c_QCommandLinkButton_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QCommandLinkButton::trUtf8(s, c, n));
}

void qt_widgets_c_QCommandLinkButton_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QCommandLinkButton::tr(s, c, n));
}


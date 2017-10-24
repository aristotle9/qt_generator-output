#include "qt_widgets_c_QDialogButtonBox.h"

QDialogButtonBox* qt_widgets_c_QDialogButtonBox_G_dynamic_cast_QDialogButtonBox_ptr(QWidget* ptr) {
  return dynamic_cast<QDialogButtonBox*>(ptr);
}

QDialogButtonBox* qt_widgets_c_QDialogButtonBox_G_static_cast_QDialogButtonBox_ptr_QObject(QObject* ptr) {
  return static_cast<QDialogButtonBox*>(ptr);
}

QDialogButtonBox* qt_widgets_c_QDialogButtonBox_G_static_cast_QDialogButtonBox_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QDialogButtonBox*>(ptr);
}

QDialogButtonBox* qt_widgets_c_QDialogButtonBox_G_static_cast_QDialogButtonBox_ptr_QWidget(QWidget* ptr) {
  return static_cast<QDialogButtonBox*>(ptr);
}

QObject* qt_widgets_c_QDialogButtonBox_G_static_cast_QObject_ptr(QDialogButtonBox* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QDialogButtonBox_G_static_cast_QPaintDevice_ptr(QDialogButtonBox* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QDialogButtonBox_G_static_cast_QWidget_ptr(QDialogButtonBox* ptr) {
  return static_cast<QWidget*>(ptr);
}

QPushButton* qt_widgets_c_QDialogButtonBox_addButton_button(QDialogButtonBox* this_ptr, QDialogButtonBox::StandardButton button) {
  return this_ptr->addButton(button);
}

void qt_widgets_c_QDialogButtonBox_addButton_button_role(QDialogButtonBox* this_ptr, QAbstractButton* button, QDialogButtonBox::ButtonRole role) {
  this_ptr->addButton(button, role);
}

QPushButton* qt_widgets_c_QDialogButtonBox_addButton_text_role(QDialogButtonBox* this_ptr, const QString* text, QDialogButtonBox::ButtonRole role) {
  return this_ptr->addButton(*text, role);
}

QPushButton* qt_widgets_c_QDialogButtonBox_button(const QDialogButtonBox* this_ptr, QDialogButtonBox::StandardButton which) {
  return this_ptr->button(which);
}

QDialogButtonBox::ButtonRole qt_widgets_c_QDialogButtonBox_buttonRole(const QDialogButtonBox* this_ptr, QAbstractButton* button) {
  return this_ptr->buttonRole(button);
}

void qt_widgets_c_QDialogButtonBox_buttons_to_output(const QDialogButtonBox* this_ptr, QList< QAbstractButton* >* output) {
  new(output) QList< QAbstractButton* >(this_ptr->buttons());
}

bool qt_widgets_c_QDialogButtonBox_centerButtons(const QDialogButtonBox* this_ptr) {
  return this_ptr->centerButtons();
}

void qt_widgets_c_QDialogButtonBox_clear(QDialogButtonBox* this_ptr) {
  this_ptr->clear();
}

void qt_widgets_c_QDialogButtonBox_delete(QDialogButtonBox* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_widgets_c_QDialogButtonBox_metaObject(const QDialogButtonBox* this_ptr) {
  return this_ptr->metaObject();
}

QDialogButtonBox* qt_widgets_c_QDialogButtonBox_new_buttons(unsigned int buttons) {
  return new QDialogButtonBox(QFlags< QDialogButtonBox::StandardButton >(buttons));
}

QDialogButtonBox* qt_widgets_c_QDialogButtonBox_new_buttons_orientation(unsigned int buttons, const Qt::Orientation* orientation) {
  return new QDialogButtonBox(QFlags< QDialogButtonBox::StandardButton >(buttons), *orientation);
}

QDialogButtonBox* qt_widgets_c_QDialogButtonBox_new_buttons_orientation_parent(unsigned int buttons, const Qt::Orientation* orientation, QWidget* parent) {
  return new QDialogButtonBox(QFlags< QDialogButtonBox::StandardButton >(buttons), *orientation, parent);
}

QDialogButtonBox* qt_widgets_c_QDialogButtonBox_new_buttons_parent(unsigned int buttons, QWidget* parent) {
  return new QDialogButtonBox(QFlags< QDialogButtonBox::StandardButton >(buttons), parent);
}

QDialogButtonBox* qt_widgets_c_QDialogButtonBox_new_no_args() {
  return new QDialogButtonBox();
}

QDialogButtonBox* qt_widgets_c_QDialogButtonBox_new_orientation(const Qt::Orientation* orientation) {
  return new QDialogButtonBox(*orientation);
}

QDialogButtonBox* qt_widgets_c_QDialogButtonBox_new_orientation_parent(const Qt::Orientation* orientation, QWidget* parent) {
  return new QDialogButtonBox(*orientation, parent);
}

QDialogButtonBox* qt_widgets_c_QDialogButtonBox_new_parent(QWidget* parent) {
  return new QDialogButtonBox(parent);
}

int qt_widgets_c_QDialogButtonBox_qt_metacall(QDialogButtonBox* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QDialogButtonBox_qt_metacast(QDialogButtonBox* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QDialogButtonBox_removeButton(QDialogButtonBox* this_ptr, QAbstractButton* button) {
  this_ptr->removeButton(button);
}

void qt_widgets_c_QDialogButtonBox_setCenterButtons(QDialogButtonBox* this_ptr, bool center) {
  this_ptr->setCenterButtons(center);
}

void qt_widgets_c_QDialogButtonBox_setOrientation(QDialogButtonBox* this_ptr, const Qt::Orientation* orientation) {
  this_ptr->setOrientation(*orientation);
}

void qt_widgets_c_QDialogButtonBox_setStandardButtons(QDialogButtonBox* this_ptr, unsigned int buttons) {
  this_ptr->setStandardButtons(QFlags< QDialogButtonBox::StandardButton >(buttons));
}

QDialogButtonBox::StandardButton qt_widgets_c_QDialogButtonBox_standardButton(const QDialogButtonBox* this_ptr, QAbstractButton* button) {
  return this_ptr->standardButton(button);
}

unsigned int qt_widgets_c_QDialogButtonBox_standardButtons(const QDialogButtonBox* this_ptr) {
  return uint(this_ptr->standardButtons());
}

void qt_widgets_c_QDialogButtonBox_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QDialogButtonBox::trUtf8(s, c, n));
}

void qt_widgets_c_QDialogButtonBox_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QDialogButtonBox::tr(s, c, n));
}


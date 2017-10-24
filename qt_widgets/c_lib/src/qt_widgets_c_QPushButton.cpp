#include "qt_widgets_c_QPushButton.h"

QPushButton* qt_widgets_c_QPushButton_G_dynamic_cast_QPushButton_ptr_QAbstractButton(QAbstractButton* ptr) {
  return dynamic_cast<QPushButton*>(ptr);
}

QPushButton* qt_widgets_c_QPushButton_G_dynamic_cast_QPushButton_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QPushButton*>(ptr);
}

QAbstractButton* qt_widgets_c_QPushButton_G_static_cast_QAbstractButton_ptr(QPushButton* ptr) {
  return static_cast<QAbstractButton*>(ptr);
}

QObject* qt_widgets_c_QPushButton_G_static_cast_QObject_ptr(QPushButton* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QPushButton_G_static_cast_QPaintDevice_ptr(QPushButton* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QPushButton* qt_widgets_c_QPushButton_G_static_cast_QPushButton_ptr_QAbstractButton(QAbstractButton* ptr) {
  return static_cast<QPushButton*>(ptr);
}

QPushButton* qt_widgets_c_QPushButton_G_static_cast_QPushButton_ptr_QObject(QObject* ptr) {
  return static_cast<QPushButton*>(ptr);
}

QPushButton* qt_widgets_c_QPushButton_G_static_cast_QPushButton_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QPushButton*>(ptr);
}

QPushButton* qt_widgets_c_QPushButton_G_static_cast_QPushButton_ptr_QWidget(QWidget* ptr) {
  return static_cast<QPushButton*>(ptr);
}

QWidget* qt_widgets_c_QPushButton_G_static_cast_QWidget_ptr(QPushButton* ptr) {
  return static_cast<QWidget*>(ptr);
}

bool qt_widgets_c_QPushButton_autoDefault(const QPushButton* this_ptr) {
  return this_ptr->autoDefault();
}

void qt_widgets_c_QPushButton_delete(QPushButton* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QPushButton_isDefault(const QPushButton* this_ptr) {
  return this_ptr->isDefault();
}

bool qt_widgets_c_QPushButton_isFlat(const QPushButton* this_ptr) {
  return this_ptr->isFlat();
}

QMenu* qt_widgets_c_QPushButton_menu(const QPushButton* this_ptr) {
  return this_ptr->menu();
}

const QMetaObject* qt_widgets_c_QPushButton_metaObject(const QPushButton* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QPushButton_minimumSizeHint_to_output(const QPushButton* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSizeHint());
}

QPushButton* qt_widgets_c_QPushButton_new_icon_text(const QIcon* icon, const QString* text) {
  return new QPushButton(*icon, *text);
}

QPushButton* qt_widgets_c_QPushButton_new_icon_text_parent(const QIcon* icon, const QString* text, QWidget* parent) {
  return new QPushButton(*icon, *text, parent);
}

QPushButton* qt_widgets_c_QPushButton_new_no_args() {
  return new QPushButton();
}

QPushButton* qt_widgets_c_QPushButton_new_parent(QWidget* parent) {
  return new QPushButton(parent);
}

QPushButton* qt_widgets_c_QPushButton_new_text(const QString* text) {
  return new QPushButton(*text);
}

QPushButton* qt_widgets_c_QPushButton_new_text_parent(const QString* text, QWidget* parent) {
  return new QPushButton(*text, parent);
}

int qt_widgets_c_QPushButton_qt_metacall(QPushButton* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QPushButton_qt_metacast(QPushButton* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QPushButton_setAutoDefault(QPushButton* this_ptr, bool arg1) {
  this_ptr->setAutoDefault(arg1);
}

void qt_widgets_c_QPushButton_setDefault(QPushButton* this_ptr, bool arg1) {
  this_ptr->setDefault(arg1);
}

void qt_widgets_c_QPushButton_setFlat(QPushButton* this_ptr, bool arg1) {
  this_ptr->setFlat(arg1);
}

void qt_widgets_c_QPushButton_setMenu(QPushButton* this_ptr, QMenu* menu) {
  this_ptr->setMenu(menu);
}

void qt_widgets_c_QPushButton_showMenu(QPushButton* this_ptr) {
  this_ptr->showMenu();
}

void qt_widgets_c_QPushButton_sizeHint_to_output(const QPushButton* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QPushButton_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QPushButton::trUtf8(s, c, n));
}

void qt_widgets_c_QPushButton_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QPushButton::tr(s, c, n));
}


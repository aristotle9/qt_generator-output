#include "qt_widgets_c_QToolButton.h"

QToolButton* qt_widgets_c_QToolButton_G_dynamic_cast_QToolButton_ptr_QAbstractButton(QAbstractButton* ptr) {
  return dynamic_cast<QToolButton*>(ptr);
}

QToolButton* qt_widgets_c_QToolButton_G_dynamic_cast_QToolButton_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QToolButton*>(ptr);
}

QAbstractButton* qt_widgets_c_QToolButton_G_static_cast_QAbstractButton_ptr(QToolButton* ptr) {
  return static_cast<QAbstractButton*>(ptr);
}

QObject* qt_widgets_c_QToolButton_G_static_cast_QObject_ptr(QToolButton* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QToolButton_G_static_cast_QPaintDevice_ptr(QToolButton* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QToolButton* qt_widgets_c_QToolButton_G_static_cast_QToolButton_ptr_QAbstractButton(QAbstractButton* ptr) {
  return static_cast<QToolButton*>(ptr);
}

QToolButton* qt_widgets_c_QToolButton_G_static_cast_QToolButton_ptr_QObject(QObject* ptr) {
  return static_cast<QToolButton*>(ptr);
}

QToolButton* qt_widgets_c_QToolButton_G_static_cast_QToolButton_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QToolButton*>(ptr);
}

QToolButton* qt_widgets_c_QToolButton_G_static_cast_QToolButton_ptr_QWidget(QWidget* ptr) {
  return static_cast<QToolButton*>(ptr);
}

QWidget* qt_widgets_c_QToolButton_G_static_cast_QWidget_ptr(QToolButton* ptr) {
  return static_cast<QWidget*>(ptr);
}

bool qt_widgets_c_QToolButton_autoRaise(const QToolButton* this_ptr) {
  return this_ptr->autoRaise();
}

QAction* qt_widgets_c_QToolButton_defaultAction(const QToolButton* this_ptr) {
  return this_ptr->defaultAction();
}

void qt_widgets_c_QToolButton_delete(QToolButton* this_ptr) {
  delete this_ptr;
}

QMenu* qt_widgets_c_QToolButton_menu(const QToolButton* this_ptr) {
  return this_ptr->menu();
}

const QMetaObject* qt_widgets_c_QToolButton_metaObject(const QToolButton* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QToolButton_minimumSizeHint_to_output(const QToolButton* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSizeHint());
}

QToolButton* qt_widgets_c_QToolButton_new_no_args() {
  return new QToolButton();
}

QToolButton* qt_widgets_c_QToolButton_new_parent(QWidget* parent) {
  return new QToolButton(parent);
}

QToolButton::ToolButtonPopupMode qt_widgets_c_QToolButton_popupMode(const QToolButton* this_ptr) {
  return this_ptr->popupMode();
}

int qt_widgets_c_QToolButton_qt_metacall(QToolButton* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QToolButton_qt_metacast(QToolButton* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QToolButton_setArrowType(QToolButton* this_ptr, const Qt::ArrowType* type) {
  this_ptr->setArrowType(*type);
}

void qt_widgets_c_QToolButton_setAutoRaise(QToolButton* this_ptr, bool enable) {
  this_ptr->setAutoRaise(enable);
}

void qt_widgets_c_QToolButton_setDefaultAction(QToolButton* this_ptr, QAction* arg1) {
  this_ptr->setDefaultAction(arg1);
}

void qt_widgets_c_QToolButton_setMenu(QToolButton* this_ptr, QMenu* menu) {
  this_ptr->setMenu(menu);
}

void qt_widgets_c_QToolButton_setPopupMode(QToolButton* this_ptr, QToolButton::ToolButtonPopupMode mode) {
  this_ptr->setPopupMode(mode);
}

void qt_widgets_c_QToolButton_setToolButtonStyle(QToolButton* this_ptr, const Qt::ToolButtonStyle* style) {
  this_ptr->setToolButtonStyle(*style);
}

void qt_widgets_c_QToolButton_showMenu(QToolButton* this_ptr) {
  this_ptr->showMenu();
}

void qt_widgets_c_QToolButton_sizeHint_to_output(const QToolButton* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QToolButton_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QToolButton::trUtf8(s, c, n));
}

void qt_widgets_c_QToolButton_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QToolButton::tr(s, c, n));
}


#include "qt_widgets_c_QMdiSubWindow.h"

QMdiSubWindow* qt_widgets_c_QMdiSubWindow_G_dynamic_cast_QMdiSubWindow_ptr(QWidget* ptr) {
  return dynamic_cast<QMdiSubWindow*>(ptr);
}

QMdiSubWindow* qt_widgets_c_QMdiSubWindow_G_static_cast_QMdiSubWindow_ptr_QObject(QObject* ptr) {
  return static_cast<QMdiSubWindow*>(ptr);
}

QMdiSubWindow* qt_widgets_c_QMdiSubWindow_G_static_cast_QMdiSubWindow_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QMdiSubWindow*>(ptr);
}

QMdiSubWindow* qt_widgets_c_QMdiSubWindow_G_static_cast_QMdiSubWindow_ptr_QWidget(QWidget* ptr) {
  return static_cast<QMdiSubWindow*>(ptr);
}

QObject* qt_widgets_c_QMdiSubWindow_G_static_cast_QObject_ptr(QMdiSubWindow* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QMdiSubWindow_G_static_cast_QPaintDevice_ptr(QMdiSubWindow* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QMdiSubWindow_G_static_cast_QWidget_ptr(QMdiSubWindow* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QMdiSubWindow_delete(QMdiSubWindow* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QMdiSubWindow_isShaded(const QMdiSubWindow* this_ptr) {
  return this_ptr->isShaded();
}

int qt_widgets_c_QMdiSubWindow_keyboardPageStep(const QMdiSubWindow* this_ptr) {
  return this_ptr->keyboardPageStep();
}

int qt_widgets_c_QMdiSubWindow_keyboardSingleStep(const QMdiSubWindow* this_ptr) {
  return this_ptr->keyboardSingleStep();
}

QWidget* qt_widgets_c_QMdiSubWindow_maximizedButtonsWidget(const QMdiSubWindow* this_ptr) {
  return this_ptr->maximizedButtonsWidget();
}

QWidget* qt_widgets_c_QMdiSubWindow_maximizedSystemMenuIconWidget(const QMdiSubWindow* this_ptr) {
  return this_ptr->maximizedSystemMenuIconWidget();
}

QMdiArea* qt_widgets_c_QMdiSubWindow_mdiArea(const QMdiSubWindow* this_ptr) {
  return this_ptr->mdiArea();
}

const QMetaObject* qt_widgets_c_QMdiSubWindow_metaObject(const QMdiSubWindow* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QMdiSubWindow_minimumSizeHint_to_output(const QMdiSubWindow* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSizeHint());
}

int qt_widgets_c_QMdiSubWindow_qt_metacall(QMdiSubWindow* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QMdiSubWindow_qt_metacast(QMdiSubWindow* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QMdiSubWindow_setKeyboardPageStep(QMdiSubWindow* this_ptr, int step) {
  this_ptr->setKeyboardPageStep(step);
}

void qt_widgets_c_QMdiSubWindow_setKeyboardSingleStep(QMdiSubWindow* this_ptr, int step) {
  this_ptr->setKeyboardSingleStep(step);
}

void qt_widgets_c_QMdiSubWindow_setOption_option(QMdiSubWindow* this_ptr, QMdiSubWindow::SubWindowOption option) {
  this_ptr->setOption(option);
}

void qt_widgets_c_QMdiSubWindow_setOption_option_on(QMdiSubWindow* this_ptr, QMdiSubWindow::SubWindowOption option, bool on) {
  this_ptr->setOption(option, on);
}

void qt_widgets_c_QMdiSubWindow_setSystemMenu(QMdiSubWindow* this_ptr, QMenu* systemMenu) {
  this_ptr->setSystemMenu(systemMenu);
}

void qt_widgets_c_QMdiSubWindow_setWidget(QMdiSubWindow* this_ptr, QWidget* widget) {
  this_ptr->setWidget(widget);
}

void qt_widgets_c_QMdiSubWindow_showShaded(QMdiSubWindow* this_ptr) {
  this_ptr->showShaded();
}

void qt_widgets_c_QMdiSubWindow_showSystemMenu(QMdiSubWindow* this_ptr) {
  this_ptr->showSystemMenu();
}

void qt_widgets_c_QMdiSubWindow_sizeHint_to_output(const QMdiSubWindow* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

QMenu* qt_widgets_c_QMdiSubWindow_systemMenu(const QMdiSubWindow* this_ptr) {
  return this_ptr->systemMenu();
}

bool qt_widgets_c_QMdiSubWindow_testOption(const QMdiSubWindow* this_ptr, QMdiSubWindow::SubWindowOption arg1) {
  return this_ptr->testOption(arg1);
}

void qt_widgets_c_QMdiSubWindow_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QMdiSubWindow::trUtf8(s, c, n));
}

void qt_widgets_c_QMdiSubWindow_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QMdiSubWindow::tr(s, c, n));
}

QWidget* qt_widgets_c_QMdiSubWindow_widget(const QMdiSubWindow* this_ptr) {
  return this_ptr->widget();
}


#include "qt_widgets_c_QDockWidget.h"

QDockWidget* qt_widgets_c_QDockWidget_G_dynamic_cast_QDockWidget_ptr(QWidget* ptr) {
  return dynamic_cast<QDockWidget*>(ptr);
}

QDockWidget* qt_widgets_c_QDockWidget_G_static_cast_QDockWidget_ptr_QObject(QObject* ptr) {
  return static_cast<QDockWidget*>(ptr);
}

QDockWidget* qt_widgets_c_QDockWidget_G_static_cast_QDockWidget_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QDockWidget*>(ptr);
}

QDockWidget* qt_widgets_c_QDockWidget_G_static_cast_QDockWidget_ptr_QWidget(QWidget* ptr) {
  return static_cast<QDockWidget*>(ptr);
}

QObject* qt_widgets_c_QDockWidget_G_static_cast_QObject_ptr(QDockWidget* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QDockWidget_G_static_cast_QPaintDevice_ptr(QDockWidget* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QDockWidget_G_static_cast_QWidget_ptr(QDockWidget* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QDockWidget_delete(QDockWidget* this_ptr) {
  delete this_ptr;
}

unsigned int qt_widgets_c_QDockWidget_features(const QDockWidget* this_ptr) {
  return uint(this_ptr->features());
}

bool qt_widgets_c_QDockWidget_isAreaAllowed(const QDockWidget* this_ptr, const Qt::DockWidgetArea* area) {
  return this_ptr->isAreaAllowed(*area);
}

bool qt_widgets_c_QDockWidget_isFloating(const QDockWidget* this_ptr) {
  return this_ptr->isFloating();
}

const QMetaObject* qt_widgets_c_QDockWidget_metaObject(const QDockWidget* this_ptr) {
  return this_ptr->metaObject();
}

int qt_widgets_c_QDockWidget_qt_metacall(QDockWidget* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QDockWidget_qt_metacast(QDockWidget* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QDockWidget_setFeatures(QDockWidget* this_ptr, unsigned int features) {
  this_ptr->setFeatures(QFlags< QDockWidget::DockWidgetFeature >(features));
}

void qt_widgets_c_QDockWidget_setFloating(QDockWidget* this_ptr, bool floating) {
  this_ptr->setFloating(floating);
}

void qt_widgets_c_QDockWidget_setTitleBarWidget(QDockWidget* this_ptr, QWidget* widget) {
  this_ptr->setTitleBarWidget(widget);
}

void qt_widgets_c_QDockWidget_setWidget(QDockWidget* this_ptr, QWidget* widget) {
  this_ptr->setWidget(widget);
}

QWidget* qt_widgets_c_QDockWidget_titleBarWidget(const QDockWidget* this_ptr) {
  return this_ptr->titleBarWidget();
}

QAction* qt_widgets_c_QDockWidget_toggleViewAction(const QDockWidget* this_ptr) {
  return this_ptr->toggleViewAction();
}

void qt_widgets_c_QDockWidget_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QDockWidget::trUtf8(s, c, n));
}

void qt_widgets_c_QDockWidget_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QDockWidget::tr(s, c, n));
}

QWidget* qt_widgets_c_QDockWidget_widget(const QDockWidget* this_ptr) {
  return this_ptr->widget();
}


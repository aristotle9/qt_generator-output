#include "qt_widgets_c_QDesktopWidget.h"

QDesktopWidget* qt_widgets_c_QDesktopWidget_G_dynamic_cast_QDesktopWidget_ptr(QWidget* ptr) {
  return dynamic_cast<QDesktopWidget*>(ptr);
}

QDesktopWidget* qt_widgets_c_QDesktopWidget_G_static_cast_QDesktopWidget_ptr_QObject(QObject* ptr) {
  return static_cast<QDesktopWidget*>(ptr);
}

QDesktopWidget* qt_widgets_c_QDesktopWidget_G_static_cast_QDesktopWidget_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QDesktopWidget*>(ptr);
}

QDesktopWidget* qt_widgets_c_QDesktopWidget_G_static_cast_QDesktopWidget_ptr_QWidget(QWidget* ptr) {
  return static_cast<QDesktopWidget*>(ptr);
}

QObject* qt_widgets_c_QDesktopWidget_G_static_cast_QObject_ptr(QDesktopWidget* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QDesktopWidget_G_static_cast_QPaintDevice_ptr(QDesktopWidget* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QDesktopWidget_G_static_cast_QWidget_ptr(QDesktopWidget* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QDesktopWidget_availableGeometry_to_output_no_args(const QDesktopWidget* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->availableGeometry());
}

void qt_widgets_c_QDesktopWidget_availableGeometry_to_output_point(const QDesktopWidget* this_ptr, const QPoint* point, QRect* output) {
  new(output) QRect(this_ptr->availableGeometry(*point));
}

void qt_widgets_c_QDesktopWidget_availableGeometry_to_output_screen(const QDesktopWidget* this_ptr, int screen, QRect* output) {
  new(output) QRect(this_ptr->availableGeometry(screen));
}

void qt_widgets_c_QDesktopWidget_availableGeometry_to_output_widget(const QDesktopWidget* this_ptr, const QWidget* widget, QRect* output) {
  new(output) QRect(this_ptr->availableGeometry(widget));
}

void qt_widgets_c_QDesktopWidget_delete(QDesktopWidget* this_ptr) {
  delete this_ptr;
}

bool qt_widgets_c_QDesktopWidget_isVirtualDesktop(const QDesktopWidget* this_ptr) {
  return this_ptr->isVirtualDesktop();
}

const QMetaObject* qt_widgets_c_QDesktopWidget_metaObject(const QDesktopWidget* this_ptr) {
  return this_ptr->metaObject();
}

QDesktopWidget* qt_widgets_c_QDesktopWidget_new() {
  return new QDesktopWidget();
}

int qt_widgets_c_QDesktopWidget_numScreens(const QDesktopWidget* this_ptr) {
  return this_ptr->numScreens();
}

int qt_widgets_c_QDesktopWidget_primaryScreen(const QDesktopWidget* this_ptr) {
  return this_ptr->primaryScreen();
}

int qt_widgets_c_QDesktopWidget_qt_metacall(QDesktopWidget* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QDesktopWidget_qt_metacast(QDesktopWidget* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

int qt_widgets_c_QDesktopWidget_screenCount(const QDesktopWidget* this_ptr) {
  return this_ptr->screenCount();
}

void qt_widgets_c_QDesktopWidget_screenGeometry_to_output_no_args(const QDesktopWidget* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->screenGeometry());
}

void qt_widgets_c_QDesktopWidget_screenGeometry_to_output_point(const QDesktopWidget* this_ptr, const QPoint* point, QRect* output) {
  new(output) QRect(this_ptr->screenGeometry(*point));
}

void qt_widgets_c_QDesktopWidget_screenGeometry_to_output_screen(const QDesktopWidget* this_ptr, int screen, QRect* output) {
  new(output) QRect(this_ptr->screenGeometry(screen));
}

void qt_widgets_c_QDesktopWidget_screenGeometry_to_output_widget(const QDesktopWidget* this_ptr, const QWidget* widget, QRect* output) {
  new(output) QRect(this_ptr->screenGeometry(widget));
}

int qt_widgets_c_QDesktopWidget_screenNumber_arg1(const QDesktopWidget* this_ptr, const QPoint* arg1) {
  return this_ptr->screenNumber(*arg1);
}

int qt_widgets_c_QDesktopWidget_screenNumber_no_args(const QDesktopWidget* this_ptr) {
  return this_ptr->screenNumber();
}

int qt_widgets_c_QDesktopWidget_screenNumber_widget(const QDesktopWidget* this_ptr, const QWidget* widget) {
  return this_ptr->screenNumber(widget);
}

QWidget* qt_widgets_c_QDesktopWidget_screen_no_args(QDesktopWidget* this_ptr) {
  return this_ptr->screen();
}

QWidget* qt_widgets_c_QDesktopWidget_screen_screen(QDesktopWidget* this_ptr, int screen) {
  return this_ptr->screen(screen);
}

void qt_widgets_c_QDesktopWidget_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QDesktopWidget::trUtf8(s, c, n));
}

void qt_widgets_c_QDesktopWidget_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QDesktopWidget::tr(s, c, n));
}


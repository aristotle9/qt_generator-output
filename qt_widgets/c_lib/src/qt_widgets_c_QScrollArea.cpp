#include "qt_widgets_c_QScrollArea.h"

QScrollArea* qt_widgets_c_QScrollArea_G_dynamic_cast_QScrollArea_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return dynamic_cast<QScrollArea*>(ptr);
}

QScrollArea* qt_widgets_c_QScrollArea_G_dynamic_cast_QScrollArea_ptr_QFrame(QFrame* ptr) {
  return dynamic_cast<QScrollArea*>(ptr);
}

QScrollArea* qt_widgets_c_QScrollArea_G_dynamic_cast_QScrollArea_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QScrollArea*>(ptr);
}

QAbstractScrollArea* qt_widgets_c_QScrollArea_G_static_cast_QAbstractScrollArea_ptr(QScrollArea* ptr) {
  return static_cast<QAbstractScrollArea*>(ptr);
}

QFrame* qt_widgets_c_QScrollArea_G_static_cast_QFrame_ptr(QScrollArea* ptr) {
  return static_cast<QFrame*>(ptr);
}

QObject* qt_widgets_c_QScrollArea_G_static_cast_QObject_ptr(QScrollArea* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QScrollArea_G_static_cast_QPaintDevice_ptr(QScrollArea* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QScrollArea* qt_widgets_c_QScrollArea_G_static_cast_QScrollArea_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return static_cast<QScrollArea*>(ptr);
}

QScrollArea* qt_widgets_c_QScrollArea_G_static_cast_QScrollArea_ptr_QFrame(QFrame* ptr) {
  return static_cast<QScrollArea*>(ptr);
}

QScrollArea* qt_widgets_c_QScrollArea_G_static_cast_QScrollArea_ptr_QObject(QObject* ptr) {
  return static_cast<QScrollArea*>(ptr);
}

QScrollArea* qt_widgets_c_QScrollArea_G_static_cast_QScrollArea_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QScrollArea*>(ptr);
}

QScrollArea* qt_widgets_c_QScrollArea_G_static_cast_QScrollArea_ptr_QWidget(QWidget* ptr) {
  return static_cast<QScrollArea*>(ptr);
}

QWidget* qt_widgets_c_QScrollArea_G_static_cast_QWidget_ptr(QScrollArea* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QScrollArea_delete(QScrollArea* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QScrollArea_ensureVisible_x_y(QScrollArea* this_ptr, int x, int y) {
  this_ptr->ensureVisible(x, y);
}

void qt_widgets_c_QScrollArea_ensureVisible_x_y_xmargin(QScrollArea* this_ptr, int x, int y, int xmargin) {
  this_ptr->ensureVisible(x, y, xmargin);
}

void qt_widgets_c_QScrollArea_ensureVisible_x_y_xmargin_ymargin(QScrollArea* this_ptr, int x, int y, int xmargin, int ymargin) {
  this_ptr->ensureVisible(x, y, xmargin, ymargin);
}

void qt_widgets_c_QScrollArea_ensureWidgetVisible_childWidget(QScrollArea* this_ptr, QWidget* childWidget) {
  this_ptr->ensureWidgetVisible(childWidget);
}

void qt_widgets_c_QScrollArea_ensureWidgetVisible_childWidget_xmargin(QScrollArea* this_ptr, QWidget* childWidget, int xmargin) {
  this_ptr->ensureWidgetVisible(childWidget, xmargin);
}

void qt_widgets_c_QScrollArea_ensureWidgetVisible_childWidget_xmargin_ymargin(QScrollArea* this_ptr, QWidget* childWidget, int xmargin, int ymargin) {
  this_ptr->ensureWidgetVisible(childWidget, xmargin, ymargin);
}

bool qt_widgets_c_QScrollArea_focusNextPrevChild(QScrollArea* this_ptr, bool next) {
  return this_ptr->focusNextPrevChild(next);
}

const QMetaObject* qt_widgets_c_QScrollArea_metaObject(const QScrollArea* this_ptr) {
  return this_ptr->metaObject();
}

QScrollArea* qt_widgets_c_QScrollArea_new_no_args() {
  return new QScrollArea();
}

QScrollArea* qt_widgets_c_QScrollArea_new_parent(QWidget* parent) {
  return new QScrollArea(parent);
}

int qt_widgets_c_QScrollArea_qt_metacall(QScrollArea* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QScrollArea_qt_metacast(QScrollArea* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QScrollArea_setWidget(QScrollArea* this_ptr, QWidget* widget) {
  this_ptr->setWidget(widget);
}

void qt_widgets_c_QScrollArea_setWidgetResizable(QScrollArea* this_ptr, bool resizable) {
  this_ptr->setWidgetResizable(resizable);
}

void qt_widgets_c_QScrollArea_sizeHint_to_output(const QScrollArea* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

QWidget* qt_widgets_c_QScrollArea_takeWidget(QScrollArea* this_ptr) {
  return this_ptr->takeWidget();
}

void qt_widgets_c_QScrollArea_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QScrollArea::trUtf8(s, c, n));
}

void qt_widgets_c_QScrollArea_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QScrollArea::tr(s, c, n));
}

QWidget* qt_widgets_c_QScrollArea_widget(const QScrollArea* this_ptr) {
  return this_ptr->widget();
}

bool qt_widgets_c_QScrollArea_widgetResizable(const QScrollArea* this_ptr) {
  return this_ptr->widgetResizable();
}


#include "qt_widgets_c_QStackedWidget.h"

QStackedWidget* qt_widgets_c_QStackedWidget_G_dynamic_cast_QStackedWidget_ptr_QFrame(QFrame* ptr) {
  return dynamic_cast<QStackedWidget*>(ptr);
}

QStackedWidget* qt_widgets_c_QStackedWidget_G_dynamic_cast_QStackedWidget_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QStackedWidget*>(ptr);
}

QFrame* qt_widgets_c_QStackedWidget_G_static_cast_QFrame_ptr(QStackedWidget* ptr) {
  return static_cast<QFrame*>(ptr);
}

QObject* qt_widgets_c_QStackedWidget_G_static_cast_QObject_ptr(QStackedWidget* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QStackedWidget_G_static_cast_QPaintDevice_ptr(QStackedWidget* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QStackedWidget* qt_widgets_c_QStackedWidget_G_static_cast_QStackedWidget_ptr_QFrame(QFrame* ptr) {
  return static_cast<QStackedWidget*>(ptr);
}

QStackedWidget* qt_widgets_c_QStackedWidget_G_static_cast_QStackedWidget_ptr_QObject(QObject* ptr) {
  return static_cast<QStackedWidget*>(ptr);
}

QStackedWidget* qt_widgets_c_QStackedWidget_G_static_cast_QStackedWidget_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QStackedWidget*>(ptr);
}

QStackedWidget* qt_widgets_c_QStackedWidget_G_static_cast_QStackedWidget_ptr_QWidget(QWidget* ptr) {
  return static_cast<QStackedWidget*>(ptr);
}

QWidget* qt_widgets_c_QStackedWidget_G_static_cast_QWidget_ptr(QStackedWidget* ptr) {
  return static_cast<QWidget*>(ptr);
}

int qt_widgets_c_QStackedWidget_addWidget(QStackedWidget* this_ptr, QWidget* w) {
  return this_ptr->addWidget(w);
}

int qt_widgets_c_QStackedWidget_count(const QStackedWidget* this_ptr) {
  return this_ptr->count();
}

int qt_widgets_c_QStackedWidget_currentIndex(const QStackedWidget* this_ptr) {
  return this_ptr->currentIndex();
}

QWidget* qt_widgets_c_QStackedWidget_currentWidget(const QStackedWidget* this_ptr) {
  return this_ptr->currentWidget();
}

void qt_widgets_c_QStackedWidget_delete(QStackedWidget* this_ptr) {
  delete this_ptr;
}

int qt_widgets_c_QStackedWidget_indexOf(const QStackedWidget* this_ptr, QWidget* arg1) {
  return this_ptr->indexOf(arg1);
}

int qt_widgets_c_QStackedWidget_insertWidget(QStackedWidget* this_ptr, int index, QWidget* w) {
  return this_ptr->insertWidget(index, w);
}

const QMetaObject* qt_widgets_c_QStackedWidget_metaObject(const QStackedWidget* this_ptr) {
  return this_ptr->metaObject();
}

QStackedWidget* qt_widgets_c_QStackedWidget_new_no_args() {
  return new QStackedWidget();
}

QStackedWidget* qt_widgets_c_QStackedWidget_new_parent(QWidget* parent) {
  return new QStackedWidget(parent);
}

int qt_widgets_c_QStackedWidget_qt_metacall(QStackedWidget* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QStackedWidget_qt_metacast(QStackedWidget* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QStackedWidget_removeWidget(QStackedWidget* this_ptr, QWidget* w) {
  this_ptr->removeWidget(w);
}

void qt_widgets_c_QStackedWidget_setCurrentIndex(QStackedWidget* this_ptr, int index) {
  this_ptr->setCurrentIndex(index);
}

void qt_widgets_c_QStackedWidget_setCurrentWidget(QStackedWidget* this_ptr, QWidget* w) {
  this_ptr->setCurrentWidget(w);
}

void qt_widgets_c_QStackedWidget_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QStackedWidget::trUtf8(s, c, n));
}

void qt_widgets_c_QStackedWidget_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QStackedWidget::tr(s, c, n));
}

QWidget* qt_widgets_c_QStackedWidget_widget(const QStackedWidget* this_ptr, int arg1) {
  return this_ptr->widget(arg1);
}


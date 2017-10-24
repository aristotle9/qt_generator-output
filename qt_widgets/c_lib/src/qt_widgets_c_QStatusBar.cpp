#include "qt_widgets_c_QStatusBar.h"

QStatusBar* qt_widgets_c_QStatusBar_G_dynamic_cast_QStatusBar_ptr(QWidget* ptr) {
  return dynamic_cast<QStatusBar*>(ptr);
}

QObject* qt_widgets_c_QStatusBar_G_static_cast_QObject_ptr(QStatusBar* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QStatusBar_G_static_cast_QPaintDevice_ptr(QStatusBar* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QStatusBar* qt_widgets_c_QStatusBar_G_static_cast_QStatusBar_ptr_QObject(QObject* ptr) {
  return static_cast<QStatusBar*>(ptr);
}

QStatusBar* qt_widgets_c_QStatusBar_G_static_cast_QStatusBar_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QStatusBar*>(ptr);
}

QStatusBar* qt_widgets_c_QStatusBar_G_static_cast_QStatusBar_ptr_QWidget(QWidget* ptr) {
  return static_cast<QStatusBar*>(ptr);
}

QWidget* qt_widgets_c_QStatusBar_G_static_cast_QWidget_ptr(QStatusBar* ptr) {
  return static_cast<QWidget*>(ptr);
}

void qt_widgets_c_QStatusBar_addPermanentWidget_widget(QStatusBar* this_ptr, QWidget* widget) {
  this_ptr->addPermanentWidget(widget);
}

void qt_widgets_c_QStatusBar_addPermanentWidget_widget_stretch(QStatusBar* this_ptr, QWidget* widget, int stretch) {
  this_ptr->addPermanentWidget(widget, stretch);
}

void qt_widgets_c_QStatusBar_addWidget_widget(QStatusBar* this_ptr, QWidget* widget) {
  this_ptr->addWidget(widget);
}

void qt_widgets_c_QStatusBar_addWidget_widget_stretch(QStatusBar* this_ptr, QWidget* widget, int stretch) {
  this_ptr->addWidget(widget, stretch);
}

void qt_widgets_c_QStatusBar_clearMessage(QStatusBar* this_ptr) {
  this_ptr->clearMessage();
}

void qt_widgets_c_QStatusBar_currentMessage_to_output(const QStatusBar* this_ptr, QString* output) {
  new(output) QString(this_ptr->currentMessage());
}

void qt_widgets_c_QStatusBar_delete(QStatusBar* this_ptr) {
  delete this_ptr;
}

int qt_widgets_c_QStatusBar_insertPermanentWidget_index_widget(QStatusBar* this_ptr, int index, QWidget* widget) {
  return this_ptr->insertPermanentWidget(index, widget);
}

int qt_widgets_c_QStatusBar_insertPermanentWidget_index_widget_stretch(QStatusBar* this_ptr, int index, QWidget* widget, int stretch) {
  return this_ptr->insertPermanentWidget(index, widget, stretch);
}

int qt_widgets_c_QStatusBar_insertWidget_index_widget(QStatusBar* this_ptr, int index, QWidget* widget) {
  return this_ptr->insertWidget(index, widget);
}

int qt_widgets_c_QStatusBar_insertWidget_index_widget_stretch(QStatusBar* this_ptr, int index, QWidget* widget, int stretch) {
  return this_ptr->insertWidget(index, widget, stretch);
}

bool qt_widgets_c_QStatusBar_isSizeGripEnabled(const QStatusBar* this_ptr) {
  return this_ptr->isSizeGripEnabled();
}

const QMetaObject* qt_widgets_c_QStatusBar_metaObject(const QStatusBar* this_ptr) {
  return this_ptr->metaObject();
}

QStatusBar* qt_widgets_c_QStatusBar_new_no_args() {
  return new QStatusBar();
}

QStatusBar* qt_widgets_c_QStatusBar_new_parent(QWidget* parent) {
  return new QStatusBar(parent);
}

int qt_widgets_c_QStatusBar_qt_metacall(QStatusBar* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QStatusBar_qt_metacast(QStatusBar* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QStatusBar_removeWidget(QStatusBar* this_ptr, QWidget* widget) {
  this_ptr->removeWidget(widget);
}

void qt_widgets_c_QStatusBar_setSizeGripEnabled(QStatusBar* this_ptr, bool arg1) {
  this_ptr->setSizeGripEnabled(arg1);
}

void qt_widgets_c_QStatusBar_showMessage_text(QStatusBar* this_ptr, const QString* text) {
  this_ptr->showMessage(*text);
}

void qt_widgets_c_QStatusBar_showMessage_text_timeout(QStatusBar* this_ptr, const QString* text, int timeout) {
  this_ptr->showMessage(*text, timeout);
}

void qt_widgets_c_QStatusBar_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QStatusBar::trUtf8(s, c, n));
}

void qt_widgets_c_QStatusBar_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QStatusBar::tr(s, c, n));
}


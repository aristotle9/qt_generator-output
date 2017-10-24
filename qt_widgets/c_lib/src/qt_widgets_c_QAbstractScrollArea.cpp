#include "qt_widgets_c_QAbstractScrollArea.h"

QAbstractScrollArea* qt_widgets_c_QAbstractScrollArea_G_dynamic_cast_QAbstractScrollArea_ptr_QFrame(QFrame* ptr) {
  return dynamic_cast<QAbstractScrollArea*>(ptr);
}

QAbstractScrollArea* qt_widgets_c_QAbstractScrollArea_G_dynamic_cast_QAbstractScrollArea_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QAbstractScrollArea*>(ptr);
}

QAbstractScrollArea* qt_widgets_c_QAbstractScrollArea_G_static_cast_QAbstractScrollArea_ptr_QFrame(QFrame* ptr) {
  return static_cast<QAbstractScrollArea*>(ptr);
}

QAbstractScrollArea* qt_widgets_c_QAbstractScrollArea_G_static_cast_QAbstractScrollArea_ptr_QObject(QObject* ptr) {
  return static_cast<QAbstractScrollArea*>(ptr);
}

QAbstractScrollArea* qt_widgets_c_QAbstractScrollArea_G_static_cast_QAbstractScrollArea_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QAbstractScrollArea*>(ptr);
}

QAbstractScrollArea* qt_widgets_c_QAbstractScrollArea_G_static_cast_QAbstractScrollArea_ptr_QWidget(QWidget* ptr) {
  return static_cast<QAbstractScrollArea*>(ptr);
}

QFrame* qt_widgets_c_QAbstractScrollArea_G_static_cast_QFrame_ptr(QAbstractScrollArea* ptr) {
  return static_cast<QFrame*>(ptr);
}

QObject* qt_widgets_c_QAbstractScrollArea_G_static_cast_QObject_ptr(QAbstractScrollArea* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QAbstractScrollArea_G_static_cast_QPaintDevice_ptr(QAbstractScrollArea* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QAbstractScrollArea_G_static_cast_QWidget_ptr(QAbstractScrollArea* ptr) {
  return static_cast<QWidget*>(ptr);
}

QWidget* qt_widgets_c_QAbstractScrollArea_cornerWidget(const QAbstractScrollArea* this_ptr) {
  return this_ptr->cornerWidget();
}

void qt_widgets_c_QAbstractScrollArea_delete(QAbstractScrollArea* this_ptr) {
  delete this_ptr;
}

QScrollBar* qt_widgets_c_QAbstractScrollArea_horizontalScrollBar(const QAbstractScrollArea* this_ptr) {
  return this_ptr->horizontalScrollBar();
}

void qt_widgets_c_QAbstractScrollArea_maximumViewportSize_to_output(const QAbstractScrollArea* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->maximumViewportSize());
}

const QMetaObject* qt_widgets_c_QAbstractScrollArea_metaObject(const QAbstractScrollArea* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QAbstractScrollArea_minimumSizeHint_to_output(const QAbstractScrollArea* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSizeHint());
}

QAbstractScrollArea* qt_widgets_c_QAbstractScrollArea_new_no_args() {
  return new QAbstractScrollArea();
}

QAbstractScrollArea* qt_widgets_c_QAbstractScrollArea_new_parent(QWidget* parent) {
  return new QAbstractScrollArea(parent);
}

int qt_widgets_c_QAbstractScrollArea_qt_metacall(QAbstractScrollArea* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QAbstractScrollArea_qt_metacast(QAbstractScrollArea* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QAbstractScrollArea_setCornerWidget(QAbstractScrollArea* this_ptr, QWidget* widget) {
  this_ptr->setCornerWidget(widget);
}

void qt_widgets_c_QAbstractScrollArea_setHorizontalScrollBar(QAbstractScrollArea* this_ptr, QScrollBar* scrollbar) {
  this_ptr->setHorizontalScrollBar(scrollbar);
}

void qt_widgets_c_QAbstractScrollArea_setHorizontalScrollBarPolicy(QAbstractScrollArea* this_ptr, const Qt::ScrollBarPolicy* arg1) {
  this_ptr->setHorizontalScrollBarPolicy(*arg1);
}

void qt_widgets_c_QAbstractScrollArea_setSizeAdjustPolicy(QAbstractScrollArea* this_ptr, QAbstractScrollArea::SizeAdjustPolicy policy) {
  this_ptr->setSizeAdjustPolicy(policy);
}

void qt_widgets_c_QAbstractScrollArea_setVerticalScrollBar(QAbstractScrollArea* this_ptr, QScrollBar* scrollbar) {
  this_ptr->setVerticalScrollBar(scrollbar);
}

void qt_widgets_c_QAbstractScrollArea_setVerticalScrollBarPolicy(QAbstractScrollArea* this_ptr, const Qt::ScrollBarPolicy* arg1) {
  this_ptr->setVerticalScrollBarPolicy(*arg1);
}

void qt_widgets_c_QAbstractScrollArea_setViewport(QAbstractScrollArea* this_ptr, QWidget* widget) {
  this_ptr->setViewport(widget);
}

void qt_widgets_c_QAbstractScrollArea_setupViewport(QAbstractScrollArea* this_ptr, QWidget* viewport) {
  this_ptr->setupViewport(viewport);
}

QAbstractScrollArea::SizeAdjustPolicy qt_widgets_c_QAbstractScrollArea_sizeAdjustPolicy(const QAbstractScrollArea* this_ptr) {
  return this_ptr->sizeAdjustPolicy();
}

void qt_widgets_c_QAbstractScrollArea_sizeHint_to_output(const QAbstractScrollArea* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QAbstractScrollArea_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractScrollArea::trUtf8(s, c, n));
}

void qt_widgets_c_QAbstractScrollArea_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QAbstractScrollArea::tr(s, c, n));
}

QScrollBar* qt_widgets_c_QAbstractScrollArea_verticalScrollBar(const QAbstractScrollArea* this_ptr) {
  return this_ptr->verticalScrollBar();
}

QWidget* qt_widgets_c_QAbstractScrollArea_viewport(const QAbstractScrollArea* this_ptr) {
  return this_ptr->viewport();
}


#include "qt_widgets_c_QGraphicsProxyWidget.h"

QGraphicsProxyWidget* qt_widgets_c_QGraphicsProxyWidget_G_dynamic_cast_QGraphicsProxyWidget_ptr_QGraphicsItem(QGraphicsItem* ptr) {
  return dynamic_cast<QGraphicsProxyWidget*>(ptr);
}

QGraphicsProxyWidget* qt_widgets_c_QGraphicsProxyWidget_G_dynamic_cast_QGraphicsProxyWidget_ptr_QGraphicsLayoutItem(QGraphicsLayoutItem* ptr) {
  return dynamic_cast<QGraphicsProxyWidget*>(ptr);
}

QGraphicsProxyWidget* qt_widgets_c_QGraphicsProxyWidget_G_dynamic_cast_QGraphicsProxyWidget_ptr_QGraphicsObject(QGraphicsObject* ptr) {
  return dynamic_cast<QGraphicsProxyWidget*>(ptr);
}

QGraphicsProxyWidget* qt_widgets_c_QGraphicsProxyWidget_G_dynamic_cast_QGraphicsProxyWidget_ptr_QGraphicsWidget(QGraphicsWidget* ptr) {
  return dynamic_cast<QGraphicsProxyWidget*>(ptr);
}

QGraphicsItem* qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsItem_ptr(QGraphicsProxyWidget* ptr) {
  return static_cast<QGraphicsItem*>(ptr);
}

QGraphicsLayoutItem* qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsLayoutItem_ptr(QGraphicsProxyWidget* ptr) {
  return static_cast<QGraphicsLayoutItem*>(ptr);
}

QGraphicsObject* qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsObject_ptr(QGraphicsProxyWidget* ptr) {
  return static_cast<QGraphicsObject*>(ptr);
}

QGraphicsProxyWidget* qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsProxyWidget_ptr_QGraphicsItem(QGraphicsItem* ptr) {
  return static_cast<QGraphicsProxyWidget*>(ptr);
}

QGraphicsProxyWidget* qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsProxyWidget_ptr_QGraphicsLayoutItem(QGraphicsLayoutItem* ptr) {
  return static_cast<QGraphicsProxyWidget*>(ptr);
}

QGraphicsProxyWidget* qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsProxyWidget_ptr_QGraphicsObject(QGraphicsObject* ptr) {
  return static_cast<QGraphicsProxyWidget*>(ptr);
}

QGraphicsProxyWidget* qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsProxyWidget_ptr_QGraphicsWidget(QGraphicsWidget* ptr) {
  return static_cast<QGraphicsProxyWidget*>(ptr);
}

QGraphicsProxyWidget* qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsProxyWidget_ptr_QObject(QObject* ptr) {
  return static_cast<QGraphicsProxyWidget*>(ptr);
}

QGraphicsWidget* qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QGraphicsWidget_ptr(QGraphicsProxyWidget* ptr) {
  return static_cast<QGraphicsWidget*>(ptr);
}

QObject* qt_widgets_c_QGraphicsProxyWidget_G_static_cast_QObject_ptr(QGraphicsProxyWidget* ptr) {
  return static_cast<QObject*>(ptr);
}

QGraphicsProxyWidget* qt_widgets_c_QGraphicsProxyWidget_createProxyForChildWidget(QGraphicsProxyWidget* this_ptr, QWidget* child) {
  return this_ptr->createProxyForChildWidget(child);
}

void qt_widgets_c_QGraphicsProxyWidget_delete(QGraphicsProxyWidget* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_widgets_c_QGraphicsProxyWidget_metaObject(const QGraphicsProxyWidget* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QGraphicsProxyWidget_paint(QGraphicsProxyWidget* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget) {
  this_ptr->paint(painter, option, widget);
}

int qt_widgets_c_QGraphicsProxyWidget_qt_metacall(QGraphicsProxyWidget* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QGraphicsProxyWidget_qt_metacast(QGraphicsProxyWidget* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QGraphicsProxyWidget_setGeometry(QGraphicsProxyWidget* this_ptr, const QRectF* rect) {
  this_ptr->setGeometry(*rect);
}

void qt_widgets_c_QGraphicsProxyWidget_setWidget(QGraphicsProxyWidget* this_ptr, QWidget* widget) {
  this_ptr->setWidget(widget);
}

void qt_widgets_c_QGraphicsProxyWidget_subWidgetRect_to_output(const QGraphicsProxyWidget* this_ptr, const QWidget* widget, QRectF* output) {
  new(output) QRectF(this_ptr->subWidgetRect(widget));
}

void qt_widgets_c_QGraphicsProxyWidget_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsProxyWidget::trUtf8(s, c, n));
}

void qt_widgets_c_QGraphicsProxyWidget_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsProxyWidget::tr(s, c, n));
}

int qt_widgets_c_QGraphicsProxyWidget_type(const QGraphicsProxyWidget* this_ptr) {
  return this_ptr->type();
}

QWidget* qt_widgets_c_QGraphicsProxyWidget_widget(const QGraphicsProxyWidget* this_ptr) {
  return this_ptr->widget();
}


#include "qt_widgets_c_QWidgetItem.h"

QWidgetItem* qt_widgets_c_QWidgetItem_G_dynamic_cast_QWidgetItem_ptr(QLayoutItem* ptr) {
  return dynamic_cast<QWidgetItem*>(ptr);
}

QLayoutItem* qt_widgets_c_QWidgetItem_G_static_cast_QLayoutItem_ptr(QWidgetItem* ptr) {
  return static_cast<QLayoutItem*>(ptr);
}

QWidgetItem* qt_widgets_c_QWidgetItem_G_static_cast_QWidgetItem_ptr(QLayoutItem* ptr) {
  return static_cast<QWidgetItem*>(ptr);
}

void qt_widgets_c_QWidgetItem_delete(QWidgetItem* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QWidgetItem_geometry_to_output(const QWidgetItem* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->geometry());
}

bool qt_widgets_c_QWidgetItem_hasHeightForWidth(const QWidgetItem* this_ptr) {
  return this_ptr->hasHeightForWidth();
}

int qt_widgets_c_QWidgetItem_heightForWidth(const QWidgetItem* this_ptr, int arg1) {
  return this_ptr->heightForWidth(arg1);
}

bool qt_widgets_c_QWidgetItem_isEmpty(const QWidgetItem* this_ptr) {
  return this_ptr->isEmpty();
}

void qt_widgets_c_QWidgetItem_maximumSize_to_output(const QWidgetItem* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->maximumSize());
}

void qt_widgets_c_QWidgetItem_minimumSize_to_output(const QWidgetItem* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSize());
}

QWidgetItem* qt_widgets_c_QWidgetItem_new(QWidget* w) {
  return new QWidgetItem(w);
}

void qt_widgets_c_QWidgetItem_setGeometry(QWidgetItem* this_ptr, const QRect* arg1) {
  this_ptr->setGeometry(*arg1);
}

void qt_widgets_c_QWidgetItem_sizeHint_to_output(const QWidgetItem* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

QWidget* qt_widgets_c_QWidgetItem_widget(QWidgetItem* this_ptr) {
  return this_ptr->widget();
}


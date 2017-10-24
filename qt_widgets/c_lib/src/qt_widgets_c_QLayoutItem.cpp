#include "qt_widgets_c_QLayoutItem.h"

void qt_widgets_c_QLayoutItem_delete(QLayoutItem* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QLayoutItem_geometry_to_output(const QLayoutItem* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->geometry());
}

bool qt_widgets_c_QLayoutItem_hasHeightForWidth(const QLayoutItem* this_ptr) {
  return this_ptr->hasHeightForWidth();
}

int qt_widgets_c_QLayoutItem_heightForWidth(const QLayoutItem* this_ptr, int arg1) {
  return this_ptr->heightForWidth(arg1);
}

void qt_widgets_c_QLayoutItem_invalidate(QLayoutItem* this_ptr) {
  this_ptr->invalidate();
}

bool qt_widgets_c_QLayoutItem_isEmpty(const QLayoutItem* this_ptr) {
  return this_ptr->isEmpty();
}

QLayout* qt_widgets_c_QLayoutItem_layout(QLayoutItem* this_ptr) {
  return this_ptr->layout();
}

void qt_widgets_c_QLayoutItem_maximumSize_to_output(const QLayoutItem* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->maximumSize());
}

int qt_widgets_c_QLayoutItem_minimumHeightForWidth(const QLayoutItem* this_ptr, int arg1) {
  return this_ptr->minimumHeightForWidth(arg1);
}

void qt_widgets_c_QLayoutItem_minimumSize_to_output(const QLayoutItem* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSize());
}

void qt_widgets_c_QLayoutItem_setGeometry(QLayoutItem* this_ptr, const QRect* arg1) {
  this_ptr->setGeometry(*arg1);
}

void qt_widgets_c_QLayoutItem_sizeHint_to_output(const QLayoutItem* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

QSpacerItem* qt_widgets_c_QLayoutItem_spacerItem(QLayoutItem* this_ptr) {
  return this_ptr->spacerItem();
}

QWidget* qt_widgets_c_QLayoutItem_widget(QLayoutItem* this_ptr) {
  return this_ptr->widget();
}


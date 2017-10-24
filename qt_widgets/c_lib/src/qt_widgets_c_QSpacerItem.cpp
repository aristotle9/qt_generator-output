#include "qt_widgets_c_QSpacerItem.h"

QSpacerItem* qt_widgets_c_QSpacerItem_G_dynamic_cast_QSpacerItem_ptr(QLayoutItem* ptr) {
  return dynamic_cast<QSpacerItem*>(ptr);
}

QLayoutItem* qt_widgets_c_QSpacerItem_G_static_cast_QLayoutItem_ptr(QSpacerItem* ptr) {
  return static_cast<QLayoutItem*>(ptr);
}

QSpacerItem* qt_widgets_c_QSpacerItem_G_static_cast_QSpacerItem_ptr(QLayoutItem* ptr) {
  return static_cast<QSpacerItem*>(ptr);
}

void qt_widgets_c_QSpacerItem_changeSize_w_h(QSpacerItem* this_ptr, int w, int h) {
  this_ptr->changeSize(w, h);
}

void qt_widgets_c_QSpacerItem_changeSize_w_h_hData(QSpacerItem* this_ptr, int w, int h, const QSizePolicy::Policy* hData) {
  this_ptr->changeSize(w, h, *hData);
}

void qt_widgets_c_QSpacerItem_changeSize_w_h_hData_vData(QSpacerItem* this_ptr, int w, int h, const QSizePolicy::Policy* hData, const QSizePolicy::Policy* vData) {
  this_ptr->changeSize(w, h, *hData, *vData);
}

void qt_widgets_c_QSpacerItem_delete(QSpacerItem* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QSpacerItem_geometry_to_output(const QSpacerItem* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->geometry());
}

bool qt_widgets_c_QSpacerItem_isEmpty(const QSpacerItem* this_ptr) {
  return this_ptr->isEmpty();
}

void qt_widgets_c_QSpacerItem_maximumSize_to_output(const QSpacerItem* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->maximumSize());
}

void qt_widgets_c_QSpacerItem_minimumSize_to_output(const QSpacerItem* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSize());
}

QSpacerItem* qt_widgets_c_QSpacerItem_new_w_h(int w, int h) {
  return new QSpacerItem(w, h);
}

QSpacerItem* qt_widgets_c_QSpacerItem_new_w_h_hData(int w, int h, const QSizePolicy::Policy* hData) {
  return new QSpacerItem(w, h, *hData);
}

QSpacerItem* qt_widgets_c_QSpacerItem_new_w_h_hData_vData(int w, int h, const QSizePolicy::Policy* hData, const QSizePolicy::Policy* vData) {
  return new QSpacerItem(w, h, *hData, *vData);
}

void qt_widgets_c_QSpacerItem_setGeometry(QSpacerItem* this_ptr, const QRect* arg1) {
  this_ptr->setGeometry(*arg1);
}

void qt_widgets_c_QSpacerItem_sizeHint_to_output(const QSpacerItem* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QSpacerItem_sizePolicy_to_output(const QSpacerItem* this_ptr, QSizePolicy* output) {
  new(output) QSizePolicy(this_ptr->sizePolicy());
}

QSpacerItem* qt_widgets_c_QSpacerItem_spacerItem(QSpacerItem* this_ptr) {
  return this_ptr->spacerItem();
}


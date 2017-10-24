#include "qt_gui_c_QPagedPaintDevice.h"

QPagedPaintDevice* qt_gui_c_QPagedPaintDevice_G_dynamic_cast_QPagedPaintDevice_ptr(QPaintDevice* ptr) {
  return dynamic_cast<QPagedPaintDevice*>(ptr);
}

QPagedPaintDevice* qt_gui_c_QPagedPaintDevice_G_static_cast_QPagedPaintDevice_ptr(QPaintDevice* ptr) {
  return static_cast<QPagedPaintDevice*>(ptr);
}

QPaintDevice* qt_gui_c_QPagedPaintDevice_G_static_cast_QPaintDevice_ptr(QPagedPaintDevice* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

double qt_gui_c_QPagedPaintDevice_Margins_bottom(const QPagedPaintDevice::Margins* this_ptr) {
  return this_ptr->bottom;
}

void qt_gui_c_QPagedPaintDevice_Margins_destructor(QPagedPaintDevice::Margins* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

double qt_gui_c_QPagedPaintDevice_Margins_left(const QPagedPaintDevice::Margins* this_ptr) {
  return this_ptr->left;
}

double qt_gui_c_QPagedPaintDevice_Margins_right(const QPagedPaintDevice::Margins* this_ptr) {
  return this_ptr->right;
}

void qt_gui_c_QPagedPaintDevice_Margins_set_bottom(QPagedPaintDevice::Margins* this_ptr, double value) {
  this_ptr->bottom = value;
}

void qt_gui_c_QPagedPaintDevice_Margins_set_left(QPagedPaintDevice::Margins* this_ptr, double value) {
  this_ptr->left = value;
}

void qt_gui_c_QPagedPaintDevice_Margins_set_right(QPagedPaintDevice::Margins* this_ptr, double value) {
  this_ptr->right = value;
}

void qt_gui_c_QPagedPaintDevice_Margins_set_top(QPagedPaintDevice::Margins* this_ptr, double value) {
  this_ptr->top = value;
}

double qt_gui_c_QPagedPaintDevice_Margins_top(const QPagedPaintDevice::Margins* this_ptr) {
  return this_ptr->top;
}

void qt_gui_c_QPagedPaintDevice_delete(QPagedPaintDevice* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QPagedPaintDevice_margins_to_output(const QPagedPaintDevice* this_ptr, QPagedPaintDevice::Margins* output) {
  new(output) QPagedPaintDevice::Margins(this_ptr->margins());
}

bool qt_gui_c_QPagedPaintDevice_newPage(QPagedPaintDevice* this_ptr) {
  return this_ptr->newPage();
}

void qt_gui_c_QPagedPaintDevice_pageLayout_to_output(const QPagedPaintDevice* this_ptr, QPageLayout* output) {
  new(output) QPageLayout(this_ptr->pageLayout());
}

QPagedPaintDevice::PageSize qt_gui_c_QPagedPaintDevice_pageSize(const QPagedPaintDevice* this_ptr) {
  return this_ptr->pageSize();
}

void qt_gui_c_QPagedPaintDevice_pageSizeMM_to_output(const QPagedPaintDevice* this_ptr, QSizeF* output) {
  new(output) QSizeF(this_ptr->pageSizeMM());
}

void qt_gui_c_QPagedPaintDevice_setMargins(QPagedPaintDevice* this_ptr, const QPagedPaintDevice::Margins* margins) {
  this_ptr->setMargins(*margins);
}

bool qt_gui_c_QPagedPaintDevice_setPageLayout(QPagedPaintDevice* this_ptr, const QPageLayout* pageLayout) {
  return this_ptr->setPageLayout(*pageLayout);
}

bool qt_gui_c_QPagedPaintDevice_setPageMargins_margins(QPagedPaintDevice* this_ptr, const QMarginsF* margins) {
  return this_ptr->setPageMargins(*margins);
}

bool qt_gui_c_QPagedPaintDevice_setPageMargins_margins_units(QPagedPaintDevice* this_ptr, const QMarginsF* margins, const QPageLayout::Unit* units) {
  return this_ptr->setPageMargins(*margins, *units);
}

bool qt_gui_c_QPagedPaintDevice_setPageOrientation(QPagedPaintDevice* this_ptr, const QPageLayout::Orientation* orientation) {
  return this_ptr->setPageOrientation(*orientation);
}

void qt_gui_c_QPagedPaintDevice_setPageSizeMM(QPagedPaintDevice* this_ptr, const QSizeF* size) {
  this_ptr->setPageSizeMM(*size);
}

bool qt_gui_c_QPagedPaintDevice_setPageSize_pageSize(QPagedPaintDevice* this_ptr, const QPageSize* pageSize) {
  return this_ptr->setPageSize(*pageSize);
}

void qt_gui_c_QPagedPaintDevice_setPageSize_size(QPagedPaintDevice* this_ptr, QPagedPaintDevice::PageSize size) {
  this_ptr->setPageSize(size);
}


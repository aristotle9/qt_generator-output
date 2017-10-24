#include "qt_gui_c_QPageSize.h"

bool qt_gui_c_QPageSize_G_operator_neq(const QPageSize* lhs, const QPageSize* rhs) {
  return operator!=(*lhs, *rhs);
}

void qt_gui_c_QPageSize_G_operator_shl_to_output(const QDebug* dbg, const QPageSize* pageSize, QDebug* output) {
  new(output) QDebug(operator<<(*dbg, *pageSize));
}

void qt_gui_c_QPageSize_G_swap(QPageSize* value1, QPageSize* value2) {
  swap(*value1, *value2);
}

void qt_gui_c_QPageSize_constructor_no_args(QPageSize* output) {
  new(output) QPageSize();
}

void qt_gui_c_QPageSize_constructor_other(const QPageSize* other, QPageSize* output) {
  new(output) QPageSize(*other);
}

void qt_gui_c_QPageSize_constructor_pageSizeId(QPageSize::PageSizeId pageSizeId, QPageSize* output) {
  new(output) QPageSize(pageSizeId);
}

void qt_gui_c_QPageSize_constructor_pointSize(const QSize* pointSize, QPageSize* output) {
  new(output) QPageSize(*pointSize);
}

void qt_gui_c_QPageSize_constructor_pointSize_name(const QSize* pointSize, const QString* name, QPageSize* output) {
  new(output) QPageSize(*pointSize, *name);
}

void qt_gui_c_QPageSize_constructor_pointSize_name_matchPolicy(const QSize* pointSize, const QString* name, QPageSize::SizeMatchPolicy matchPolicy, QPageSize* output) {
  new(output) QPageSize(*pointSize, *name, matchPolicy);
}

void qt_gui_c_QPageSize_constructor_size_units(const QSizeF* size, QPageSize::Unit units, QPageSize* output) {
  new(output) QPageSize(*size, units);
}

void qt_gui_c_QPageSize_constructor_size_units_name(const QSizeF* size, QPageSize::Unit units, const QString* name, QPageSize* output) {
  new(output) QPageSize(*size, units, *name);
}

void qt_gui_c_QPageSize_constructor_size_units_name_matchPolicy(const QSizeF* size, QPageSize::Unit units, const QString* name, QPageSize::SizeMatchPolicy matchPolicy, QPageSize* output) {
  new(output) QPageSize(*size, units, *name, matchPolicy);
}

void qt_gui_c_QPageSize_definitionSize_to_output_no_args(const QPageSize* this_ptr, QSizeF* output) {
  new(output) QSizeF(this_ptr->definitionSize());
}

void qt_gui_c_QPageSize_definitionSize_to_output_pageSizeId(QPageSize::PageSizeId pageSizeId, QSizeF* output) {
  new(output) QSizeF(QPageSize::definitionSize(pageSizeId));
}

QPageSize::Unit qt_gui_c_QPageSize_definitionUnits_no_args(const QPageSize* this_ptr) {
  return this_ptr->definitionUnits();
}

QPageSize::Unit qt_gui_c_QPageSize_definitionUnits_pageSizeId(QPageSize::PageSizeId pageSizeId) {
  return QPageSize::definitionUnits(pageSizeId);
}

void qt_gui_c_QPageSize_destructor(QPageSize* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

QPageSize::PageSizeId qt_gui_c_QPageSize_id_no_args(const QPageSize* this_ptr) {
  return this_ptr->id();
}

QPageSize::PageSizeId qt_gui_c_QPageSize_id_pointSize(const QSize* pointSize) {
  return QPageSize::id(*pointSize);
}

QPageSize::PageSizeId qt_gui_c_QPageSize_id_pointSize_matchPolicy(const QSize* pointSize, QPageSize::SizeMatchPolicy matchPolicy) {
  return QPageSize::id(*pointSize, matchPolicy);
}

QPageSize::PageSizeId qt_gui_c_QPageSize_id_size_units(const QSizeF* size, QPageSize::Unit units) {
  return QPageSize::id(*size, units);
}

QPageSize::PageSizeId qt_gui_c_QPageSize_id_size_units_matchPolicy(const QSizeF* size, QPageSize::Unit units, QPageSize::SizeMatchPolicy matchPolicy) {
  return QPageSize::id(*size, units, matchPolicy);
}

QPageSize::PageSizeId qt_gui_c_QPageSize_id_windowsId(int windowsId) {
  return QPageSize::id(windowsId);
}

bool qt_gui_c_QPageSize_isEquivalentTo(const QPageSize* this_ptr, const QPageSize* other) {
  return this_ptr->isEquivalentTo(*other);
}

bool qt_gui_c_QPageSize_isValid(const QPageSize* this_ptr) {
  return this_ptr->isValid();
}

void qt_gui_c_QPageSize_key_to_output_no_args(const QPageSize* this_ptr, QString* output) {
  new(output) QString(this_ptr->key());
}

void qt_gui_c_QPageSize_key_to_output_pageSizeId(QPageSize::PageSizeId pageSizeId, QString* output) {
  new(output) QString(QPageSize::key(pageSizeId));
}

void qt_gui_c_QPageSize_name_to_output_no_args(const QPageSize* this_ptr, QString* output) {
  new(output) QString(this_ptr->name());
}

void qt_gui_c_QPageSize_name_to_output_pageSizeId(QPageSize::PageSizeId pageSizeId, QString* output) {
  new(output) QString(QPageSize::name(pageSizeId));
}

QPageSize* qt_gui_c_QPageSize_operator_assign(QPageSize* this_ptr, const QPageSize* other) {
  return &this_ptr->operator=(*other);
}

void qt_gui_c_QPageSize_rectPixels_to_output(const QPageSize* this_ptr, int resolution, QRect* output) {
  new(output) QRect(this_ptr->rectPixels(resolution));
}

void qt_gui_c_QPageSize_rectPoints_to_output(const QPageSize* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->rectPoints());
}

void qt_gui_c_QPageSize_rect_to_output(const QPageSize* this_ptr, QPageSize::Unit units, QRectF* output) {
  new(output) QRectF(this_ptr->rect(units));
}

void qt_gui_c_QPageSize_sizePixels_to_output_pageSizeId_resolution(QPageSize::PageSizeId pageSizeId, int resolution, QSize* output) {
  new(output) QSize(QPageSize::sizePixels(pageSizeId, resolution));
}

void qt_gui_c_QPageSize_sizePixels_to_output_resolution(const QPageSize* this_ptr, int resolution, QSize* output) {
  new(output) QSize(this_ptr->sizePixels(resolution));
}

void qt_gui_c_QPageSize_sizePoints_to_output_no_args(const QPageSize* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizePoints());
}

void qt_gui_c_QPageSize_sizePoints_to_output_pageSizeId(QPageSize::PageSizeId pageSizeId, QSize* output) {
  new(output) QSize(QPageSize::sizePoints(pageSizeId));
}

void qt_gui_c_QPageSize_size_to_output_pageSizeId_units(QPageSize::PageSizeId pageSizeId, QPageSize::Unit units, QSizeF* output) {
  new(output) QSizeF(QPageSize::size(pageSizeId, units));
}

void qt_gui_c_QPageSize_size_to_output_units(const QPageSize* this_ptr, QPageSize::Unit units, QSizeF* output) {
  new(output) QSizeF(this_ptr->size(units));
}

void qt_gui_c_QPageSize_swap(QPageSize* this_ptr, QPageSize* other) {
  this_ptr->swap(*other);
}

int qt_gui_c_QPageSize_windowsId_no_args(const QPageSize* this_ptr) {
  return this_ptr->windowsId();
}

int qt_gui_c_QPageSize_windowsId_pageSizeId(QPageSize::PageSizeId pageSizeId) {
  return QPageSize::windowsId(pageSizeId);
}


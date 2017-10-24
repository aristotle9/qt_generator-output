#include "qt_gui_c_QPageLayout.h"

bool qt_gui_c_QPageLayout_G_operator_neq(const QPageLayout* lhs, const QPageLayout* rhs) {
  return operator!=(*lhs, *rhs);
}

void qt_gui_c_QPageLayout_G_operator_shl_to_output(const QDebug* dbg, const QPageLayout* pageLayout, QDebug* output) {
  new(output) QDebug(operator<<(*dbg, *pageLayout));
}

void qt_gui_c_QPageLayout_G_swap(QPageLayout* value1, QPageLayout* value2) {
  swap(*value1, *value2);
}

void qt_gui_c_QPageLayout_constructor_no_args(QPageLayout* output) {
  new(output) QPageLayout();
}

void qt_gui_c_QPageLayout_constructor_other(const QPageLayout* other, QPageLayout* output) {
  new(output) QPageLayout(*other);
}

void qt_gui_c_QPageLayout_constructor_pageSize_orientation_margins(const QPageSize* pageSize, QPageLayout::Orientation orientation, const QMarginsF* margins, QPageLayout* output) {
  new(output) QPageLayout(*pageSize, orientation, *margins);
}

void qt_gui_c_QPageLayout_constructor_pageSize_orientation_margins_units(const QPageSize* pageSize, QPageLayout::Orientation orientation, const QMarginsF* margins, QPageLayout::Unit units, QPageLayout* output) {
  new(output) QPageLayout(*pageSize, orientation, *margins, units);
}

void qt_gui_c_QPageLayout_constructor_pageSize_orientation_margins_units_minMargins(const QPageSize* pageSize, QPageLayout::Orientation orientation, const QMarginsF* margins, QPageLayout::Unit units, const QMarginsF* minMargins, QPageLayout* output) {
  new(output) QPageLayout(*pageSize, orientation, *margins, units, *minMargins);
}

void qt_gui_c_QPageLayout_destructor(QPageLayout* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

void qt_gui_c_QPageLayout_fullRectPixels_to_output(const QPageLayout* this_ptr, int resolution, QRect* output) {
  new(output) QRect(this_ptr->fullRectPixels(resolution));
}

void qt_gui_c_QPageLayout_fullRectPoints_to_output(const QPageLayout* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->fullRectPoints());
}

void qt_gui_c_QPageLayout_fullRect_to_output_no_args(const QPageLayout* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->fullRect());
}

void qt_gui_c_QPageLayout_fullRect_to_output_units(const QPageLayout* this_ptr, QPageLayout::Unit units, QRectF* output) {
  new(output) QRectF(this_ptr->fullRect(units));
}

bool qt_gui_c_QPageLayout_isEquivalentTo(const QPageLayout* this_ptr, const QPageLayout* other) {
  return this_ptr->isEquivalentTo(*other);
}

bool qt_gui_c_QPageLayout_isValid(const QPageLayout* this_ptr) {
  return this_ptr->isValid();
}

void qt_gui_c_QPageLayout_marginsPixels_to_output(const QPageLayout* this_ptr, int resolution, QMargins* output) {
  new(output) QMargins(this_ptr->marginsPixels(resolution));
}

void qt_gui_c_QPageLayout_marginsPoints_to_output(const QPageLayout* this_ptr, QMargins* output) {
  new(output) QMargins(this_ptr->marginsPoints());
}

void qt_gui_c_QPageLayout_margins_to_output_no_args(const QPageLayout* this_ptr, QMarginsF* output) {
  new(output) QMarginsF(this_ptr->margins());
}

void qt_gui_c_QPageLayout_margins_to_output_units(const QPageLayout* this_ptr, QPageLayout::Unit units, QMarginsF* output) {
  new(output) QMarginsF(this_ptr->margins(units));
}

void qt_gui_c_QPageLayout_maximumMargins_to_output(const QPageLayout* this_ptr, QMarginsF* output) {
  new(output) QMarginsF(this_ptr->maximumMargins());
}

void qt_gui_c_QPageLayout_minimumMargins_to_output(const QPageLayout* this_ptr, QMarginsF* output) {
  new(output) QMarginsF(this_ptr->minimumMargins());
}

QPageLayout::Mode qt_gui_c_QPageLayout_mode(const QPageLayout* this_ptr) {
  return this_ptr->mode();
}

QPageLayout* qt_gui_c_QPageLayout_operator_assign(QPageLayout* this_ptr, const QPageLayout* other) {
  return &this_ptr->operator=(*other);
}

QPageLayout::Orientation qt_gui_c_QPageLayout_orientation(const QPageLayout* this_ptr) {
  return this_ptr->orientation();
}

void qt_gui_c_QPageLayout_pageSize_to_output(const QPageLayout* this_ptr, QPageSize* output) {
  new(output) QPageSize(this_ptr->pageSize());
}

void qt_gui_c_QPageLayout_paintRectPixels_to_output(const QPageLayout* this_ptr, int resolution, QRect* output) {
  new(output) QRect(this_ptr->paintRectPixels(resolution));
}

void qt_gui_c_QPageLayout_paintRectPoints_to_output(const QPageLayout* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->paintRectPoints());
}

void qt_gui_c_QPageLayout_paintRect_to_output_no_args(const QPageLayout* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->paintRect());
}

void qt_gui_c_QPageLayout_paintRect_to_output_units(const QPageLayout* this_ptr, QPageLayout::Unit units, QRectF* output) {
  new(output) QRectF(this_ptr->paintRect(units));
}

bool qt_gui_c_QPageLayout_setBottomMargin(QPageLayout* this_ptr, double bottomMargin) {
  return this_ptr->setBottomMargin(bottomMargin);
}

bool qt_gui_c_QPageLayout_setLeftMargin(QPageLayout* this_ptr, double leftMargin) {
  return this_ptr->setLeftMargin(leftMargin);
}

bool qt_gui_c_QPageLayout_setMargins(QPageLayout* this_ptr, const QMarginsF* margins) {
  return this_ptr->setMargins(*margins);
}

void qt_gui_c_QPageLayout_setMinimumMargins(QPageLayout* this_ptr, const QMarginsF* minMargins) {
  this_ptr->setMinimumMargins(*minMargins);
}

void qt_gui_c_QPageLayout_setMode(QPageLayout* this_ptr, QPageLayout::Mode mode) {
  this_ptr->setMode(mode);
}

void qt_gui_c_QPageLayout_setOrientation(QPageLayout* this_ptr, QPageLayout::Orientation orientation) {
  this_ptr->setOrientation(orientation);
}

void qt_gui_c_QPageLayout_setPageSize_pageSize(QPageLayout* this_ptr, const QPageSize* pageSize) {
  this_ptr->setPageSize(*pageSize);
}

void qt_gui_c_QPageLayout_setPageSize_pageSize_minMargins(QPageLayout* this_ptr, const QPageSize* pageSize, const QMarginsF* minMargins) {
  this_ptr->setPageSize(*pageSize, *minMargins);
}

bool qt_gui_c_QPageLayout_setRightMargin(QPageLayout* this_ptr, double rightMargin) {
  return this_ptr->setRightMargin(rightMargin);
}

bool qt_gui_c_QPageLayout_setTopMargin(QPageLayout* this_ptr, double topMargin) {
  return this_ptr->setTopMargin(topMargin);
}

void qt_gui_c_QPageLayout_setUnits(QPageLayout* this_ptr, QPageLayout::Unit units) {
  this_ptr->setUnits(units);
}

void qt_gui_c_QPageLayout_swap(QPageLayout* this_ptr, QPageLayout* other) {
  this_ptr->swap(*other);
}

QPageLayout::Unit qt_gui_c_QPageLayout_units(const QPageLayout* this_ptr) {
  return this_ptr->units();
}


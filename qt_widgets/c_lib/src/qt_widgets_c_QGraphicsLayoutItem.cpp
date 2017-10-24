#include "qt_widgets_c_QGraphicsLayoutItem.h"

void qt_widgets_c_QGraphicsLayoutItem_contentsRect_to_output(const QGraphicsLayoutItem* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->contentsRect());
}

void qt_widgets_c_QGraphicsLayoutItem_delete(QGraphicsLayoutItem* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QGraphicsLayoutItem_effectiveSizeHint_to_output_which(const QGraphicsLayoutItem* this_ptr, const Qt::SizeHint* which, QSizeF* output) {
  new(output) QSizeF(this_ptr->effectiveSizeHint(*which));
}

void qt_widgets_c_QGraphicsLayoutItem_effectiveSizeHint_to_output_which_constraint(const QGraphicsLayoutItem* this_ptr, const Qt::SizeHint* which, const QSizeF* constraint, QSizeF* output) {
  new(output) QSizeF(this_ptr->effectiveSizeHint(*which, *constraint));
}

void qt_widgets_c_QGraphicsLayoutItem_geometry_to_output(const QGraphicsLayoutItem* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->geometry());
}

void qt_widgets_c_QGraphicsLayoutItem_getContentsMargins(const QGraphicsLayoutItem* this_ptr, double* left, double* top, double* right, double* bottom) {
  this_ptr->getContentsMargins(left, top, right, bottom);
}

QGraphicsItem* qt_widgets_c_QGraphicsLayoutItem_graphicsItem(const QGraphicsLayoutItem* this_ptr) {
  return this_ptr->graphicsItem();
}

bool qt_widgets_c_QGraphicsLayoutItem_isLayout(const QGraphicsLayoutItem* this_ptr) {
  return this_ptr->isLayout();
}

double qt_widgets_c_QGraphicsLayoutItem_maximumHeight(const QGraphicsLayoutItem* this_ptr) {
  return this_ptr->maximumHeight();
}

void qt_widgets_c_QGraphicsLayoutItem_maximumSize_to_output(const QGraphicsLayoutItem* this_ptr, QSizeF* output) {
  new(output) QSizeF(this_ptr->maximumSize());
}

double qt_widgets_c_QGraphicsLayoutItem_maximumWidth(const QGraphicsLayoutItem* this_ptr) {
  return this_ptr->maximumWidth();
}

double qt_widgets_c_QGraphicsLayoutItem_minimumHeight(const QGraphicsLayoutItem* this_ptr) {
  return this_ptr->minimumHeight();
}

void qt_widgets_c_QGraphicsLayoutItem_minimumSize_to_output(const QGraphicsLayoutItem* this_ptr, QSizeF* output) {
  new(output) QSizeF(this_ptr->minimumSize());
}

double qt_widgets_c_QGraphicsLayoutItem_minimumWidth(const QGraphicsLayoutItem* this_ptr) {
  return this_ptr->minimumWidth();
}

bool qt_widgets_c_QGraphicsLayoutItem_ownedByLayout(const QGraphicsLayoutItem* this_ptr) {
  return this_ptr->ownedByLayout();
}

QGraphicsLayoutItem* qt_widgets_c_QGraphicsLayoutItem_parentLayoutItem(const QGraphicsLayoutItem* this_ptr) {
  return this_ptr->parentLayoutItem();
}

double qt_widgets_c_QGraphicsLayoutItem_preferredHeight(const QGraphicsLayoutItem* this_ptr) {
  return this_ptr->preferredHeight();
}

void qt_widgets_c_QGraphicsLayoutItem_preferredSize_to_output(const QGraphicsLayoutItem* this_ptr, QSizeF* output) {
  new(output) QSizeF(this_ptr->preferredSize());
}

double qt_widgets_c_QGraphicsLayoutItem_preferredWidth(const QGraphicsLayoutItem* this_ptr) {
  return this_ptr->preferredWidth();
}

void qt_widgets_c_QGraphicsLayoutItem_setGeometry(QGraphicsLayoutItem* this_ptr, const QRectF* rect) {
  this_ptr->setGeometry(*rect);
}

void qt_widgets_c_QGraphicsLayoutItem_setMaximumHeight(QGraphicsLayoutItem* this_ptr, double height) {
  this_ptr->setMaximumHeight(height);
}

void qt_widgets_c_QGraphicsLayoutItem_setMaximumSize_size(QGraphicsLayoutItem* this_ptr, const QSizeF* size) {
  this_ptr->setMaximumSize(*size);
}

void qt_widgets_c_QGraphicsLayoutItem_setMaximumSize_w_h(QGraphicsLayoutItem* this_ptr, double w, double h) {
  this_ptr->setMaximumSize(w, h);
}

void qt_widgets_c_QGraphicsLayoutItem_setMaximumWidth(QGraphicsLayoutItem* this_ptr, double width) {
  this_ptr->setMaximumWidth(width);
}

void qt_widgets_c_QGraphicsLayoutItem_setMinimumHeight(QGraphicsLayoutItem* this_ptr, double height) {
  this_ptr->setMinimumHeight(height);
}

void qt_widgets_c_QGraphicsLayoutItem_setMinimumSize_size(QGraphicsLayoutItem* this_ptr, const QSizeF* size) {
  this_ptr->setMinimumSize(*size);
}

void qt_widgets_c_QGraphicsLayoutItem_setMinimumSize_w_h(QGraphicsLayoutItem* this_ptr, double w, double h) {
  this_ptr->setMinimumSize(w, h);
}

void qt_widgets_c_QGraphicsLayoutItem_setMinimumWidth(QGraphicsLayoutItem* this_ptr, double width) {
  this_ptr->setMinimumWidth(width);
}

void qt_widgets_c_QGraphicsLayoutItem_setParentLayoutItem(QGraphicsLayoutItem* this_ptr, QGraphicsLayoutItem* parent) {
  this_ptr->setParentLayoutItem(parent);
}

void qt_widgets_c_QGraphicsLayoutItem_setPreferredHeight(QGraphicsLayoutItem* this_ptr, double height) {
  this_ptr->setPreferredHeight(height);
}

void qt_widgets_c_QGraphicsLayoutItem_setPreferredSize_size(QGraphicsLayoutItem* this_ptr, const QSizeF* size) {
  this_ptr->setPreferredSize(*size);
}

void qt_widgets_c_QGraphicsLayoutItem_setPreferredSize_w_h(QGraphicsLayoutItem* this_ptr, double w, double h) {
  this_ptr->setPreferredSize(w, h);
}

void qt_widgets_c_QGraphicsLayoutItem_setPreferredWidth(QGraphicsLayoutItem* this_ptr, double width) {
  this_ptr->setPreferredWidth(width);
}

void qt_widgets_c_QGraphicsLayoutItem_setSizePolicy_hPolicy_vPolicy(QGraphicsLayoutItem* this_ptr, const QSizePolicy::Policy* hPolicy, const QSizePolicy::Policy* vPolicy) {
  this_ptr->setSizePolicy(*hPolicy, *vPolicy);
}

void qt_widgets_c_QGraphicsLayoutItem_setSizePolicy_hPolicy_vPolicy_controlType(QGraphicsLayoutItem* this_ptr, const QSizePolicy::Policy* hPolicy, const QSizePolicy::Policy* vPolicy, const QSizePolicy::ControlType* controlType) {
  this_ptr->setSizePolicy(*hPolicy, *vPolicy, *controlType);
}

void qt_widgets_c_QGraphicsLayoutItem_setSizePolicy_policy(QGraphicsLayoutItem* this_ptr, const QSizePolicy* policy) {
  this_ptr->setSizePolicy(*policy);
}

void qt_widgets_c_QGraphicsLayoutItem_sizePolicy_to_output(const QGraphicsLayoutItem* this_ptr, QSizePolicy* output) {
  new(output) QSizePolicy(this_ptr->sizePolicy());
}

void qt_widgets_c_QGraphicsLayoutItem_updateGeometry(QGraphicsLayoutItem* this_ptr) {
  this_ptr->updateGeometry();
}


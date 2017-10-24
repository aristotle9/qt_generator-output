#include "qt_widgets_c_QGraphicsLinearLayout.h"

QGraphicsLinearLayout* qt_widgets_c_QGraphicsLinearLayout_G_dynamic_cast_QGraphicsLinearLayout_ptr_QGraphicsLayout(QGraphicsLayout* ptr) {
  return dynamic_cast<QGraphicsLinearLayout*>(ptr);
}

QGraphicsLinearLayout* qt_widgets_c_QGraphicsLinearLayout_G_dynamic_cast_QGraphicsLinearLayout_ptr_QGraphicsLayoutItem(QGraphicsLayoutItem* ptr) {
  return dynamic_cast<QGraphicsLinearLayout*>(ptr);
}

QGraphicsLayoutItem* qt_widgets_c_QGraphicsLinearLayout_G_static_cast_QGraphicsLayoutItem_ptr(QGraphicsLinearLayout* ptr) {
  return static_cast<QGraphicsLayoutItem*>(ptr);
}

QGraphicsLayout* qt_widgets_c_QGraphicsLinearLayout_G_static_cast_QGraphicsLayout_ptr(QGraphicsLinearLayout* ptr) {
  return static_cast<QGraphicsLayout*>(ptr);
}

QGraphicsLinearLayout* qt_widgets_c_QGraphicsLinearLayout_G_static_cast_QGraphicsLinearLayout_ptr_QGraphicsLayout(QGraphicsLayout* ptr) {
  return static_cast<QGraphicsLinearLayout*>(ptr);
}

QGraphicsLinearLayout* qt_widgets_c_QGraphicsLinearLayout_G_static_cast_QGraphicsLinearLayout_ptr_QGraphicsLayoutItem(QGraphicsLayoutItem* ptr) {
  return static_cast<QGraphicsLinearLayout*>(ptr);
}

void qt_widgets_c_QGraphicsLinearLayout_addItem(QGraphicsLinearLayout* this_ptr, QGraphicsLayoutItem* item) {
  this_ptr->addItem(item);
}

void qt_widgets_c_QGraphicsLinearLayout_addStretch_no_args(QGraphicsLinearLayout* this_ptr) {
  this_ptr->addStretch();
}

void qt_widgets_c_QGraphicsLinearLayout_addStretch_stretch(QGraphicsLinearLayout* this_ptr, int stretch) {
  this_ptr->addStretch(stretch);
}

int qt_widgets_c_QGraphicsLinearLayout_count(const QGraphicsLinearLayout* this_ptr) {
  return this_ptr->count();
}

void qt_widgets_c_QGraphicsLinearLayout_delete(QGraphicsLinearLayout* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QGraphicsLinearLayout_dump_indent(const QGraphicsLinearLayout* this_ptr, int indent) {
  this_ptr->dump(indent);
}

void qt_widgets_c_QGraphicsLinearLayout_dump_no_args(const QGraphicsLinearLayout* this_ptr) {
  this_ptr->dump();
}

void qt_widgets_c_QGraphicsLinearLayout_insertItem(QGraphicsLinearLayout* this_ptr, int index, QGraphicsLayoutItem* item) {
  this_ptr->insertItem(index, item);
}

void qt_widgets_c_QGraphicsLinearLayout_insertStretch_index(QGraphicsLinearLayout* this_ptr, int index) {
  this_ptr->insertStretch(index);
}

void qt_widgets_c_QGraphicsLinearLayout_insertStretch_index_stretch(QGraphicsLinearLayout* this_ptr, int index, int stretch) {
  this_ptr->insertStretch(index, stretch);
}

void qt_widgets_c_QGraphicsLinearLayout_invalidate(QGraphicsLinearLayout* this_ptr) {
  this_ptr->invalidate();
}

QGraphicsLayoutItem* qt_widgets_c_QGraphicsLinearLayout_itemAt(const QGraphicsLinearLayout* this_ptr, int index) {
  return this_ptr->itemAt(index);
}

double qt_widgets_c_QGraphicsLinearLayout_itemSpacing(const QGraphicsLinearLayout* this_ptr, int index) {
  return this_ptr->itemSpacing(index);
}

QGraphicsLinearLayout* qt_widgets_c_QGraphicsLinearLayout_new_no_args() {
  return new QGraphicsLinearLayout();
}

QGraphicsLinearLayout* qt_widgets_c_QGraphicsLinearLayout_new_orientation(const Qt::Orientation* orientation) {
  return new QGraphicsLinearLayout(*orientation);
}

QGraphicsLinearLayout* qt_widgets_c_QGraphicsLinearLayout_new_orientation_parent(const Qt::Orientation* orientation, QGraphicsLayoutItem* parent) {
  return new QGraphicsLinearLayout(*orientation, parent);
}

QGraphicsLinearLayout* qt_widgets_c_QGraphicsLinearLayout_new_parent(QGraphicsLayoutItem* parent) {
  return new QGraphicsLinearLayout(parent);
}

void qt_widgets_c_QGraphicsLinearLayout_removeAt(QGraphicsLinearLayout* this_ptr, int index) {
  this_ptr->removeAt(index);
}

void qt_widgets_c_QGraphicsLinearLayout_removeItem(QGraphicsLinearLayout* this_ptr, QGraphicsLayoutItem* item) {
  this_ptr->removeItem(item);
}

void qt_widgets_c_QGraphicsLinearLayout_setGeometry(QGraphicsLinearLayout* this_ptr, const QRectF* rect) {
  this_ptr->setGeometry(*rect);
}

void qt_widgets_c_QGraphicsLinearLayout_setItemSpacing(QGraphicsLinearLayout* this_ptr, int index, double spacing) {
  this_ptr->setItemSpacing(index, spacing);
}

void qt_widgets_c_QGraphicsLinearLayout_setOrientation(QGraphicsLinearLayout* this_ptr, const Qt::Orientation* orientation) {
  this_ptr->setOrientation(*orientation);
}

void qt_widgets_c_QGraphicsLinearLayout_setSpacing(QGraphicsLinearLayout* this_ptr, double spacing) {
  this_ptr->setSpacing(spacing);
}

void qt_widgets_c_QGraphicsLinearLayout_setStretchFactor(QGraphicsLinearLayout* this_ptr, QGraphicsLayoutItem* item, int stretch) {
  this_ptr->setStretchFactor(item, stretch);
}

void qt_widgets_c_QGraphicsLinearLayout_sizeHint_to_output_which(const QGraphicsLinearLayout* this_ptr, const Qt::SizeHint* which, QSizeF* output) {
  new(output) QSizeF(this_ptr->sizeHint(*which));
}

void qt_widgets_c_QGraphicsLinearLayout_sizeHint_to_output_which_constraint(const QGraphicsLinearLayout* this_ptr, const Qt::SizeHint* which, const QSizeF* constraint, QSizeF* output) {
  new(output) QSizeF(this_ptr->sizeHint(*which, *constraint));
}

double qt_widgets_c_QGraphicsLinearLayout_spacing(const QGraphicsLinearLayout* this_ptr) {
  return this_ptr->spacing();
}

int qt_widgets_c_QGraphicsLinearLayout_stretchFactor(const QGraphicsLinearLayout* this_ptr, QGraphicsLayoutItem* item) {
  return this_ptr->stretchFactor(item);
}


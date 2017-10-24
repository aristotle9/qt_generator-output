#include "qt_gui_c_QBrush.h"

QDataStream* qt_gui_c_QBrush_G_operator_shl(QDataStream* arg1, const QBrush* arg2) {
  return &operator<<(*arg1, *arg2);
}

void qt_gui_c_QBrush_G_operator_shl_to_output(const QDebug* arg1, const QBrush* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

QDataStream* qt_gui_c_QBrush_G_operator_shr(QDataStream* arg1, QBrush* arg2) {
  return &operator>>(*arg1, *arg2);
}

void qt_gui_c_QBrush_G_swap(QBrush* value1, QBrush* value2) {
  swap(*value1, *value2);
}

const QColor* qt_gui_c_QBrush_color(const QBrush* this_ptr) {
  return &this_ptr->color();
}

void qt_gui_c_QBrush_constructor_QBrush(const QBrush* brush, QBrush* output) {
  new(output) QBrush(*brush);
}

void qt_gui_c_QBrush_constructor_QColor(const QColor* color, QBrush* output) {
  new(output) QBrush(*color);
}

void qt_gui_c_QBrush_constructor_QColor_QPixmap(const QColor* color, const QPixmap* pixmap, QBrush* output) {
  new(output) QBrush(*color, *pixmap);
}

void qt_gui_c_QBrush_constructor_QColor_Qt_BrushStyle(const QColor* color, const Qt::BrushStyle* bs, QBrush* output) {
  new(output) QBrush(*color, *bs);
}

void qt_gui_c_QBrush_constructor_QGradient(const QGradient* gradient, QBrush* output) {
  new(output) QBrush(*gradient);
}

void qt_gui_c_QBrush_constructor_QImage(const QImage* image, QBrush* output) {
  new(output) QBrush(*image);
}

void qt_gui_c_QBrush_constructor_QPixmap(const QPixmap* pixmap, QBrush* output) {
  new(output) QBrush(*pixmap);
}

void qt_gui_c_QBrush_constructor_Qt_BrushStyle(const Qt::BrushStyle* bs, QBrush* output) {
  new(output) QBrush(*bs);
}

void qt_gui_c_QBrush_constructor_Qt_GlobalColor(const Qt::GlobalColor* color, QBrush* output) {
  new(output) QBrush(*color);
}

void qt_gui_c_QBrush_constructor_Qt_GlobalColor_QPixmap(const Qt::GlobalColor* color, const QPixmap* pixmap, QBrush* output) {
  new(output) QBrush(*color, *pixmap);
}

void qt_gui_c_QBrush_constructor_Qt_GlobalColor_Qt_BrushStyle(const Qt::GlobalColor* color, const Qt::BrushStyle* bs, QBrush* output) {
  new(output) QBrush(*color, *bs);
}

void qt_gui_c_QBrush_constructor_no_args(QBrush* output) {
  new(output) QBrush();
}

void qt_gui_c_QBrush_convert_to_QVariant_to_output(const QBrush* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->operator QVariant());
}

void qt_gui_c_QBrush_destructor(QBrush* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

const QGradient* qt_gui_c_QBrush_gradient(const QBrush* this_ptr) {
  return this_ptr->gradient();
}

bool qt_gui_c_QBrush_isDetached(const QBrush* this_ptr) {
  return this_ptr->isDetached();
}

bool qt_gui_c_QBrush_isOpaque(const QBrush* this_ptr) {
  return this_ptr->isOpaque();
}

const QMatrix* qt_gui_c_QBrush_matrix(const QBrush* this_ptr) {
  return &this_ptr->matrix();
}

QBrush* qt_gui_c_QBrush_operator_assign(QBrush* this_ptr, const QBrush* brush) {
  return &this_ptr->operator=(*brush);
}

bool qt_gui_c_QBrush_operator_eq(const QBrush* this_ptr, const QBrush* b) {
  return this_ptr->operator==(*b);
}

bool qt_gui_c_QBrush_operator_neq(const QBrush* this_ptr, const QBrush* b) {
  return this_ptr->operator!=(*b);
}

void qt_gui_c_QBrush_setColor_QColor(QBrush* this_ptr, const QColor* color) {
  this_ptr->setColor(*color);
}

void qt_gui_c_QBrush_setColor_Qt_GlobalColor(QBrush* this_ptr, const Qt::GlobalColor* color) {
  this_ptr->setColor(*color);
}

void qt_gui_c_QBrush_setMatrix(QBrush* this_ptr, const QMatrix* mat) {
  this_ptr->setMatrix(*mat);
}

void qt_gui_c_QBrush_setStyle(QBrush* this_ptr, const Qt::BrushStyle* arg1) {
  this_ptr->setStyle(*arg1);
}

void qt_gui_c_QBrush_setTexture(QBrush* this_ptr, const QPixmap* pixmap) {
  this_ptr->setTexture(*pixmap);
}

void qt_gui_c_QBrush_setTextureImage(QBrush* this_ptr, const QImage* image) {
  this_ptr->setTextureImage(*image);
}

void qt_gui_c_QBrush_setTransform(QBrush* this_ptr, const QTransform* arg1) {
  this_ptr->setTransform(*arg1);
}

void qt_gui_c_QBrush_swap(QBrush* this_ptr, QBrush* other) {
  this_ptr->swap(*other);
}

QImage* qt_gui_c_QBrush_textureImage_as_ptr(const QBrush* this_ptr) {
  return new QImage(this_ptr->textureImage());
}

QPixmap* qt_gui_c_QBrush_texture_as_ptr(const QBrush* this_ptr) {
  return new QPixmap(this_ptr->texture());
}

void qt_gui_c_QBrush_transform_to_output(const QBrush* this_ptr, QTransform* output) {
  new(output) QTransform(this_ptr->transform());
}


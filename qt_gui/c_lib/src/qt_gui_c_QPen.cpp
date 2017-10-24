#include "qt_gui_c_QPen.h"

QDataStream* qt_gui_c_QPen_G_operator_shl(QDataStream* arg1, const QPen* arg2) {
  return &operator<<(*arg1, *arg2);
}

void qt_gui_c_QPen_G_operator_shl_to_output(const QDebug* arg1, const QPen* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

QDataStream* qt_gui_c_QPen_G_operator_shr(QDataStream* arg1, QPen* arg2) {
  return &operator>>(*arg1, *arg2);
}

void qt_gui_c_QPen_G_swap(QPen* value1, QPen* value2) {
  swap(*value1, *value2);
}

void qt_gui_c_QPen_brush_to_output(const QPen* this_ptr, QBrush* output) {
  new(output) QBrush(this_ptr->brush());
}

void qt_gui_c_QPen_color_to_output(const QPen* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->color());
}

void qt_gui_c_QPen_constructor_arg1(const Qt::PenStyle* arg1, QPen* output) {
  new(output) QPen(*arg1);
}

void qt_gui_c_QPen_constructor_brush_width(const QBrush* brush, double width, QPen* output) {
  new(output) QPen(*brush, width);
}

void qt_gui_c_QPen_constructor_brush_width_s(const QBrush* brush, double width, const Qt::PenStyle* s, QPen* output) {
  new(output) QPen(*brush, width, *s);
}

void qt_gui_c_QPen_constructor_brush_width_s_c(const QBrush* brush, double width, const Qt::PenStyle* s, const Qt::PenCapStyle* c, QPen* output) {
  new(output) QPen(*brush, width, *s, *c);
}

void qt_gui_c_QPen_constructor_brush_width_s_c_j(const QBrush* brush, double width, const Qt::PenStyle* s, const Qt::PenCapStyle* c, const Qt::PenJoinStyle* j, QPen* output) {
  new(output) QPen(*brush, width, *s, *c, *j);
}

void qt_gui_c_QPen_constructor_color(const QColor* color, QPen* output) {
  new(output) QPen(*color);
}

void qt_gui_c_QPen_constructor_no_args(QPen* output) {
  new(output) QPen();
}

void qt_gui_c_QPen_constructor_pen(const QPen* pen, QPen* output) {
  new(output) QPen(*pen);
}

void qt_gui_c_QPen_convert_to_QVariant_to_output(const QPen* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->operator QVariant());
}

double qt_gui_c_QPen_dashOffset(const QPen* this_ptr) {
  return this_ptr->dashOffset();
}

void qt_gui_c_QPen_dashPattern_to_output(const QPen* this_ptr, QVector< double >* output) {
  new(output) QVector< double >(this_ptr->dashPattern());
}

void qt_gui_c_QPen_destructor(QPen* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QPen_isCosmetic(const QPen* this_ptr) {
  return this_ptr->isCosmetic();
}

bool qt_gui_c_QPen_isDetached(QPen* this_ptr) {
  return this_ptr->isDetached();
}

bool qt_gui_c_QPen_isSolid(const QPen* this_ptr) {
  return this_ptr->isSolid();
}

double qt_gui_c_QPen_miterLimit(const QPen* this_ptr) {
  return this_ptr->miterLimit();
}

QPen* qt_gui_c_QPen_operator_assign(QPen* this_ptr, const QPen* pen) {
  return &this_ptr->operator=(*pen);
}

bool qt_gui_c_QPen_operator_eq(const QPen* this_ptr, const QPen* p) {
  return this_ptr->operator==(*p);
}

bool qt_gui_c_QPen_operator_neq(const QPen* this_ptr, const QPen* p) {
  return this_ptr->operator!=(*p);
}

void qt_gui_c_QPen_setBrush(QPen* this_ptr, const QBrush* brush) {
  this_ptr->setBrush(*brush);
}

void qt_gui_c_QPen_setCapStyle(QPen* this_ptr, const Qt::PenCapStyle* pcs) {
  this_ptr->setCapStyle(*pcs);
}

void qt_gui_c_QPen_setColor(QPen* this_ptr, const QColor* color) {
  this_ptr->setColor(*color);
}

void qt_gui_c_QPen_setCosmetic(QPen* this_ptr, bool cosmetic) {
  this_ptr->setCosmetic(cosmetic);
}

void qt_gui_c_QPen_setDashOffset(QPen* this_ptr, double doffset) {
  this_ptr->setDashOffset(doffset);
}

void qt_gui_c_QPen_setDashPattern(QPen* this_ptr, const QVector< double >* pattern) {
  this_ptr->setDashPattern(*pattern);
}

void qt_gui_c_QPen_setJoinStyle(QPen* this_ptr, const Qt::PenJoinStyle* pcs) {
  this_ptr->setJoinStyle(*pcs);
}

void qt_gui_c_QPen_setMiterLimit(QPen* this_ptr, double limit) {
  this_ptr->setMiterLimit(limit);
}

void qt_gui_c_QPen_setStyle(QPen* this_ptr, const Qt::PenStyle* arg1) {
  this_ptr->setStyle(*arg1);
}

void qt_gui_c_QPen_setWidth(QPen* this_ptr, int width) {
  this_ptr->setWidth(width);
}

void qt_gui_c_QPen_setWidthF(QPen* this_ptr, double width) {
  this_ptr->setWidthF(width);
}

void qt_gui_c_QPen_swap(QPen* this_ptr, QPen* other) {
  this_ptr->swap(*other);
}

int qt_gui_c_QPen_width(const QPen* this_ptr) {
  return this_ptr->width();
}

double qt_gui_c_QPen_widthF(const QPen* this_ptr) {
  return this_ptr->widthF();
}


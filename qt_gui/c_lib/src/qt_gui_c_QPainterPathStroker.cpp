#include "qt_gui_c_QPainterPathStroker.h"

void qt_gui_c_QPainterPathStroker_constructor_no_args(QPainterPathStroker* output) {
  new(output) QPainterPathStroker();
}

void qt_gui_c_QPainterPathStroker_constructor_pen(const QPen* pen, QPainterPathStroker* output) {
  new(output) QPainterPathStroker(*pen);
}

void qt_gui_c_QPainterPathStroker_createStroke_to_output(const QPainterPathStroker* this_ptr, const QPainterPath* path, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->createStroke(*path));
}

double qt_gui_c_QPainterPathStroker_curveThreshold(const QPainterPathStroker* this_ptr) {
  return this_ptr->curveThreshold();
}

double qt_gui_c_QPainterPathStroker_dashOffset(const QPainterPathStroker* this_ptr) {
  return this_ptr->dashOffset();
}

void qt_gui_c_QPainterPathStroker_dashPattern_to_output(const QPainterPathStroker* this_ptr, QVector< double >* output) {
  new(output) QVector< double >(this_ptr->dashPattern());
}

void qt_gui_c_QPainterPathStroker_destructor(QPainterPathStroker* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

double qt_gui_c_QPainterPathStroker_miterLimit(const QPainterPathStroker* this_ptr) {
  return this_ptr->miterLimit();
}

void qt_gui_c_QPainterPathStroker_setCapStyle(QPainterPathStroker* this_ptr, const Qt::PenCapStyle* style) {
  this_ptr->setCapStyle(*style);
}

void qt_gui_c_QPainterPathStroker_setCurveThreshold(QPainterPathStroker* this_ptr, double threshold) {
  this_ptr->setCurveThreshold(threshold);
}

void qt_gui_c_QPainterPathStroker_setDashOffset(QPainterPathStroker* this_ptr, double offset) {
  this_ptr->setDashOffset(offset);
}

void qt_gui_c_QPainterPathStroker_setDashPattern_arg1(QPainterPathStroker* this_ptr, const Qt::PenStyle* arg1) {
  this_ptr->setDashPattern(*arg1);
}

void qt_gui_c_QPainterPathStroker_setDashPattern_dashPattern(QPainterPathStroker* this_ptr, const QVector< double >* dashPattern) {
  this_ptr->setDashPattern(*dashPattern);
}

void qt_gui_c_QPainterPathStroker_setJoinStyle(QPainterPathStroker* this_ptr, const Qt::PenJoinStyle* style) {
  this_ptr->setJoinStyle(*style);
}

void qt_gui_c_QPainterPathStroker_setMiterLimit(QPainterPathStroker* this_ptr, double length) {
  this_ptr->setMiterLimit(length);
}

void qt_gui_c_QPainterPathStroker_setWidth(QPainterPathStroker* this_ptr, double width) {
  this_ptr->setWidth(width);
}

double qt_gui_c_QPainterPathStroker_width(const QPainterPathStroker* this_ptr) {
  return this_ptr->width();
}


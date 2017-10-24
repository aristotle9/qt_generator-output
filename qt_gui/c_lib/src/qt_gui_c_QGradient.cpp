#include "qt_gui_c_QGradient.h"

QGradient::CoordinateMode qt_gui_c_QGradient_coordinateMode(const QGradient* this_ptr) {
  return this_ptr->coordinateMode();
}

void qt_gui_c_QGradient_delete(QGradient* this_ptr) {
  delete this_ptr;
}

QGradient::InterpolationMode qt_gui_c_QGradient_interpolationMode(const QGradient* this_ptr) {
  return this_ptr->interpolationMode();
}

QGradient* qt_gui_c_QGradient_new() {
  return new QGradient();
}

bool qt_gui_c_QGradient_operator_eq(const QGradient* this_ptr, const QGradient* gradient) {
  return this_ptr->operator==(*gradient);
}

bool qt_gui_c_QGradient_operator_neq(const QGradient* this_ptr, const QGradient* other) {
  return this_ptr->operator!=(*other);
}

void qt_gui_c_QGradient_setColorAt(QGradient* this_ptr, double pos, const QColor* color) {
  this_ptr->setColorAt(pos, *color);
}

void qt_gui_c_QGradient_setCoordinateMode(QGradient* this_ptr, QGradient::CoordinateMode mode) {
  this_ptr->setCoordinateMode(mode);
}

void qt_gui_c_QGradient_setInterpolationMode(QGradient* this_ptr, QGradient::InterpolationMode mode) {
  this_ptr->setInterpolationMode(mode);
}

void qt_gui_c_QGradient_setSpread(QGradient* this_ptr, QGradient::Spread spread) {
  this_ptr->setSpread(spread);
}

void qt_gui_c_QGradient_setStops(QGradient* this_ptr, const QVector< QPair< double, QColor > >* stops) {
  this_ptr->setStops(*stops);
}

QGradient::Spread qt_gui_c_QGradient_spread(const QGradient* this_ptr) {
  return this_ptr->spread();
}

void qt_gui_c_QGradient_stops_to_output(const QGradient* this_ptr, QVector< QPair< double, QColor > >* output) {
  new(output) QVector< QPair< double, QColor > >(this_ptr->stops());
}

QGradient::Type qt_gui_c_QGradient_type(const QGradient* this_ptr) {
  return this_ptr->type();
}


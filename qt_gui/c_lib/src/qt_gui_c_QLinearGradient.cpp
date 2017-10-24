#include "qt_gui_c_QLinearGradient.h"

QGradient* qt_gui_c_QLinearGradient_G_static_cast_QGradient_ptr(QLinearGradient* ptr) {
  return static_cast<QGradient*>(ptr);
}

QLinearGradient* qt_gui_c_QLinearGradient_G_static_cast_QLinearGradient_ptr(QGradient* ptr) {
  return static_cast<QLinearGradient*>(ptr);
}

void qt_gui_c_QLinearGradient_delete(QLinearGradient* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QLinearGradient_finalStop_to_output(const QLinearGradient* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->finalStop());
}

QLinearGradient* qt_gui_c_QLinearGradient_new_no_args() {
  return new QLinearGradient();
}

QLinearGradient* qt_gui_c_QLinearGradient_new_start_finalStop(const QPointF* start, const QPointF* finalStop) {
  return new QLinearGradient(*start, *finalStop);
}

QLinearGradient* qt_gui_c_QLinearGradient_new_xStart_yStart_xFinalStop_yFinalStop(double xStart, double yStart, double xFinalStop, double yFinalStop) {
  return new QLinearGradient(xStart, yStart, xFinalStop, yFinalStop);
}

void qt_gui_c_QLinearGradient_setFinalStop_stop(QLinearGradient* this_ptr, const QPointF* stop) {
  this_ptr->setFinalStop(*stop);
}

void qt_gui_c_QLinearGradient_setFinalStop_x_y(QLinearGradient* this_ptr, double x, double y) {
  this_ptr->setFinalStop(x, y);
}

void qt_gui_c_QLinearGradient_setStart_start(QLinearGradient* this_ptr, const QPointF* start) {
  this_ptr->setStart(*start);
}

void qt_gui_c_QLinearGradient_setStart_x_y(QLinearGradient* this_ptr, double x, double y) {
  this_ptr->setStart(x, y);
}

void qt_gui_c_QLinearGradient_start_to_output(const QLinearGradient* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->start());
}


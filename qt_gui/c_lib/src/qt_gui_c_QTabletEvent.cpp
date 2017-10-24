#include "qt_gui_c_QTabletEvent.h"

QTabletEvent* qt_gui_c_QTabletEvent_G_dynamic_cast_QTabletEvent_ptr(QInputEvent* ptr) {
  return dynamic_cast<QTabletEvent*>(ptr);
}

QEvent* qt_gui_c_QTabletEvent_G_static_cast_QEvent_ptr(QTabletEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QInputEvent* qt_gui_c_QTabletEvent_G_static_cast_QInputEvent_ptr(QTabletEvent* ptr) {
  return static_cast<QInputEvent*>(ptr);
}

QTabletEvent* qt_gui_c_QTabletEvent_G_static_cast_QTabletEvent_ptr_QEvent(QEvent* ptr) {
  return static_cast<QTabletEvent*>(ptr);
}

QTabletEvent* qt_gui_c_QTabletEvent_G_static_cast_QTabletEvent_ptr_QInputEvent(QInputEvent* ptr) {
  return static_cast<QTabletEvent*>(ptr);
}

void qt_gui_c_QTabletEvent_delete(QTabletEvent* this_ptr) {
  delete this_ptr;
}

QTabletEvent::TabletDevice qt_gui_c_QTabletEvent_device(const QTabletEvent* this_ptr) {
  return this_ptr->device();
}

const QPointF* qt_gui_c_QTabletEvent_globalPosF(const QTabletEvent* this_ptr) {
  return &this_ptr->globalPosF();
}

void qt_gui_c_QTabletEvent_globalPos_to_output(const QTabletEvent* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->globalPos());
}

int qt_gui_c_QTabletEvent_globalX(const QTabletEvent* this_ptr) {
  return this_ptr->globalX();
}

int qt_gui_c_QTabletEvent_globalY(const QTabletEvent* this_ptr) {
  return this_ptr->globalY();
}

double qt_gui_c_QTabletEvent_hiResGlobalX(const QTabletEvent* this_ptr) {
  return this_ptr->hiResGlobalX();
}

double qt_gui_c_QTabletEvent_hiResGlobalY(const QTabletEvent* this_ptr) {
  return this_ptr->hiResGlobalY();
}

QTabletEvent::PointerType qt_gui_c_QTabletEvent_pointerType(const QTabletEvent* this_ptr) {
  return this_ptr->pointerType();
}

const QPointF* qt_gui_c_QTabletEvent_posF(const QTabletEvent* this_ptr) {
  return &this_ptr->posF();
}

void qt_gui_c_QTabletEvent_pos_to_output(const QTabletEvent* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->pos());
}

double qt_gui_c_QTabletEvent_pressure(const QTabletEvent* this_ptr) {
  return this_ptr->pressure();
}

double qt_gui_c_QTabletEvent_rotation(const QTabletEvent* this_ptr) {
  return this_ptr->rotation();
}

double qt_gui_c_QTabletEvent_tangentialPressure(const QTabletEvent* this_ptr) {
  return this_ptr->tangentialPressure();
}

qint64 qt_gui_c_QTabletEvent_uniqueId(const QTabletEvent* this_ptr) {
  return this_ptr->uniqueId();
}

int qt_gui_c_QTabletEvent_x(const QTabletEvent* this_ptr) {
  return this_ptr->x();
}

int qt_gui_c_QTabletEvent_xTilt(const QTabletEvent* this_ptr) {
  return this_ptr->xTilt();
}

int qt_gui_c_QTabletEvent_y(const QTabletEvent* this_ptr) {
  return this_ptr->y();
}

int qt_gui_c_QTabletEvent_yTilt(const QTabletEvent* this_ptr) {
  return this_ptr->yTilt();
}

int qt_gui_c_QTabletEvent_z(const QTabletEvent* this_ptr) {
  return this_ptr->z();
}


#include "qt_gui_c_QTouchEvent.h"

QTouchEvent* qt_gui_c_QTouchEvent_G_dynamic_cast_QTouchEvent_ptr(QInputEvent* ptr) {
  return dynamic_cast<QTouchEvent*>(ptr);
}

QEvent* qt_gui_c_QTouchEvent_G_static_cast_QEvent_ptr(QTouchEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QInputEvent* qt_gui_c_QTouchEvent_G_static_cast_QInputEvent_ptr(QTouchEvent* ptr) {
  return static_cast<QInputEvent*>(ptr);
}

QTouchEvent* qt_gui_c_QTouchEvent_G_static_cast_QTouchEvent_ptr_QEvent(QEvent* ptr) {
  return static_cast<QTouchEvent*>(ptr);
}

QTouchEvent* qt_gui_c_QTouchEvent_G_static_cast_QTouchEvent_ptr_QInputEvent(QInputEvent* ptr) {
  return static_cast<QTouchEvent*>(ptr);
}

void qt_gui_c_QTouchEvent_TouchPoint_constructor_id(int id, QTouchEvent::TouchPoint* output) {
  new(output) QTouchEvent::TouchPoint(id);
}

void qt_gui_c_QTouchEvent_TouchPoint_constructor_no_args(QTouchEvent::TouchPoint* output) {
  new(output) QTouchEvent::TouchPoint();
}

void qt_gui_c_QTouchEvent_TouchPoint_constructor_other(const QTouchEvent::TouchPoint* other, QTouchEvent::TouchPoint* output) {
  new(output) QTouchEvent::TouchPoint(*other);
}

void qt_gui_c_QTouchEvent_TouchPoint_destructor(QTouchEvent::TouchPoint* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

void qt_gui_c_QTouchEvent_TouchPoint_ellipseDiameters_to_output(const QTouchEvent::TouchPoint* this_ptr, QSizeF* output) {
  new(output) QSizeF(this_ptr->ellipseDiameters());
}

unsigned int qt_gui_c_QTouchEvent_TouchPoint_flags(const QTouchEvent::TouchPoint* this_ptr) {
  return uint(this_ptr->flags());
}

int qt_gui_c_QTouchEvent_TouchPoint_id(const QTouchEvent::TouchPoint* this_ptr) {
  return this_ptr->id();
}

void qt_gui_c_QTouchEvent_TouchPoint_lastNormalizedPos_to_output(const QTouchEvent::TouchPoint* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->lastNormalizedPos());
}

void qt_gui_c_QTouchEvent_TouchPoint_lastPos_to_output(const QTouchEvent::TouchPoint* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->lastPos());
}

void qt_gui_c_QTouchEvent_TouchPoint_lastScenePos_to_output(const QTouchEvent::TouchPoint* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->lastScenePos());
}

void qt_gui_c_QTouchEvent_TouchPoint_lastScreenPos_to_output(const QTouchEvent::TouchPoint* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->lastScreenPos());
}

void qt_gui_c_QTouchEvent_TouchPoint_normalizedPos_to_output(const QTouchEvent::TouchPoint* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->normalizedPos());
}

QTouchEvent::TouchPoint* qt_gui_c_QTouchEvent_TouchPoint_operator_assign(QTouchEvent::TouchPoint* this_ptr, const QTouchEvent::TouchPoint* other) {
  return &this_ptr->operator=(*other);
}

void qt_gui_c_QTouchEvent_TouchPoint_pos_to_output(const QTouchEvent::TouchPoint* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->pos());
}

double qt_gui_c_QTouchEvent_TouchPoint_pressure(const QTouchEvent::TouchPoint* this_ptr) {
  return this_ptr->pressure();
}

void qt_gui_c_QTouchEvent_TouchPoint_rawScreenPositions_to_output(const QTouchEvent::TouchPoint* this_ptr, QVector< QPointF >* output) {
  new(output) QVector< QPointF >(this_ptr->rawScreenPositions());
}

void qt_gui_c_QTouchEvent_TouchPoint_rect_to_output(const QTouchEvent::TouchPoint* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->rect());
}

double qt_gui_c_QTouchEvent_TouchPoint_rotation(const QTouchEvent::TouchPoint* this_ptr) {
  return this_ptr->rotation();
}

void qt_gui_c_QTouchEvent_TouchPoint_scenePos_to_output(const QTouchEvent::TouchPoint* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->scenePos());
}

void qt_gui_c_QTouchEvent_TouchPoint_sceneRect_to_output(const QTouchEvent::TouchPoint* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->sceneRect());
}

void qt_gui_c_QTouchEvent_TouchPoint_screenPos_to_output(const QTouchEvent::TouchPoint* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->screenPos());
}

void qt_gui_c_QTouchEvent_TouchPoint_screenRect_to_output(const QTouchEvent::TouchPoint* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->screenRect());
}

void qt_gui_c_QTouchEvent_TouchPoint_setEllipseDiameters(QTouchEvent::TouchPoint* this_ptr, const QSizeF* dia) {
  this_ptr->setEllipseDiameters(*dia);
}

void qt_gui_c_QTouchEvent_TouchPoint_setFlags(QTouchEvent::TouchPoint* this_ptr, unsigned int flags) {
  this_ptr->setFlags(QFlags< QTouchEvent::TouchPoint::InfoFlag >(flags));
}

void qt_gui_c_QTouchEvent_TouchPoint_setId(QTouchEvent::TouchPoint* this_ptr, int id) {
  this_ptr->setId(id);
}

void qt_gui_c_QTouchEvent_TouchPoint_setLastNormalizedPos(QTouchEvent::TouchPoint* this_ptr, const QPointF* lastNormalizedPos) {
  this_ptr->setLastNormalizedPos(*lastNormalizedPos);
}

void qt_gui_c_QTouchEvent_TouchPoint_setLastPos(QTouchEvent::TouchPoint* this_ptr, const QPointF* lastPos) {
  this_ptr->setLastPos(*lastPos);
}

void qt_gui_c_QTouchEvent_TouchPoint_setLastScenePos(QTouchEvent::TouchPoint* this_ptr, const QPointF* lastScenePos) {
  this_ptr->setLastScenePos(*lastScenePos);
}

void qt_gui_c_QTouchEvent_TouchPoint_setLastScreenPos(QTouchEvent::TouchPoint* this_ptr, const QPointF* lastScreenPos) {
  this_ptr->setLastScreenPos(*lastScreenPos);
}

void qt_gui_c_QTouchEvent_TouchPoint_setNormalizedPos(QTouchEvent::TouchPoint* this_ptr, const QPointF* normalizedPos) {
  this_ptr->setNormalizedPos(*normalizedPos);
}

void qt_gui_c_QTouchEvent_TouchPoint_setPos(QTouchEvent::TouchPoint* this_ptr, const QPointF* pos) {
  this_ptr->setPos(*pos);
}

void qt_gui_c_QTouchEvent_TouchPoint_setPressure(QTouchEvent::TouchPoint* this_ptr, double pressure) {
  this_ptr->setPressure(pressure);
}

void qt_gui_c_QTouchEvent_TouchPoint_setRawScreenPositions(QTouchEvent::TouchPoint* this_ptr, const QVector< QPointF >* positions) {
  this_ptr->setRawScreenPositions(*positions);
}

void qt_gui_c_QTouchEvent_TouchPoint_setRect(QTouchEvent::TouchPoint* this_ptr, const QRectF* rect) {
  this_ptr->setRect(*rect);
}

void qt_gui_c_QTouchEvent_TouchPoint_setRotation(QTouchEvent::TouchPoint* this_ptr, double angle) {
  this_ptr->setRotation(angle);
}

void qt_gui_c_QTouchEvent_TouchPoint_setScenePos(QTouchEvent::TouchPoint* this_ptr, const QPointF* scenePos) {
  this_ptr->setScenePos(*scenePos);
}

void qt_gui_c_QTouchEvent_TouchPoint_setSceneRect(QTouchEvent::TouchPoint* this_ptr, const QRectF* sceneRect) {
  this_ptr->setSceneRect(*sceneRect);
}

void qt_gui_c_QTouchEvent_TouchPoint_setScreenPos(QTouchEvent::TouchPoint* this_ptr, const QPointF* screenPos) {
  this_ptr->setScreenPos(*screenPos);
}

void qt_gui_c_QTouchEvent_TouchPoint_setScreenRect(QTouchEvent::TouchPoint* this_ptr, const QRectF* screenRect) {
  this_ptr->setScreenRect(*screenRect);
}

void qt_gui_c_QTouchEvent_TouchPoint_setStartNormalizedPos(QTouchEvent::TouchPoint* this_ptr, const QPointF* startNormalizedPos) {
  this_ptr->setStartNormalizedPos(*startNormalizedPos);
}

void qt_gui_c_QTouchEvent_TouchPoint_setStartPos(QTouchEvent::TouchPoint* this_ptr, const QPointF* startPos) {
  this_ptr->setStartPos(*startPos);
}

void qt_gui_c_QTouchEvent_TouchPoint_setStartScenePos(QTouchEvent::TouchPoint* this_ptr, const QPointF* startScenePos) {
  this_ptr->setStartScenePos(*startScenePos);
}

void qt_gui_c_QTouchEvent_TouchPoint_setStartScreenPos(QTouchEvent::TouchPoint* this_ptr, const QPointF* startScreenPos) {
  this_ptr->setStartScreenPos(*startScreenPos);
}

void qt_gui_c_QTouchEvent_TouchPoint_setUniqueId(QTouchEvent::TouchPoint* this_ptr, qint64 uid) {
  this_ptr->setUniqueId(uid);
}

void qt_gui_c_QTouchEvent_TouchPoint_setVelocity(QTouchEvent::TouchPoint* this_ptr, const QVector2D* v) {
  this_ptr->setVelocity(*v);
}

void qt_gui_c_QTouchEvent_TouchPoint_startNormalizedPos_to_output(const QTouchEvent::TouchPoint* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->startNormalizedPos());
}

void qt_gui_c_QTouchEvent_TouchPoint_startPos_to_output(const QTouchEvent::TouchPoint* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->startPos());
}

void qt_gui_c_QTouchEvent_TouchPoint_startScenePos_to_output(const QTouchEvent::TouchPoint* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->startScenePos());
}

void qt_gui_c_QTouchEvent_TouchPoint_startScreenPos_to_output(const QTouchEvent::TouchPoint* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->startScreenPos());
}

void qt_gui_c_QTouchEvent_TouchPoint_swap(QTouchEvent::TouchPoint* this_ptr, QTouchEvent::TouchPoint* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QTouchEvent_TouchPoint_uniqueId_to_output(const QTouchEvent::TouchPoint* this_ptr, QPointingDeviceUniqueId* output) {
  new(output) QPointingDeviceUniqueId(this_ptr->uniqueId());
}

QVector2D* qt_gui_c_QTouchEvent_TouchPoint_velocity_as_ptr(const QTouchEvent::TouchPoint* this_ptr) {
  return new QVector2D(this_ptr->velocity());
}

void qt_gui_c_QTouchEvent_delete(QTouchEvent* this_ptr) {
  delete this_ptr;
}

QTouchDevice* qt_gui_c_QTouchEvent_device(const QTouchEvent* this_ptr) {
  return this_ptr->device();
}

void qt_gui_c_QTouchEvent_setDevice(QTouchEvent* this_ptr, QTouchDevice* adevice) {
  this_ptr->setDevice(adevice);
}

void qt_gui_c_QTouchEvent_setTarget(QTouchEvent* this_ptr, QObject* atarget) {
  this_ptr->setTarget(atarget);
}

void qt_gui_c_QTouchEvent_setTouchPoints(QTouchEvent* this_ptr, const QList< QTouchEvent::TouchPoint >* atouchPoints) {
  this_ptr->setTouchPoints(*atouchPoints);
}

void qt_gui_c_QTouchEvent_setWindow(QTouchEvent* this_ptr, QWindow* awindow) {
  this_ptr->setWindow(awindow);
}

QObject* qt_gui_c_QTouchEvent_target(const QTouchEvent* this_ptr) {
  return this_ptr->target();
}

const QList< QTouchEvent::TouchPoint >* qt_gui_c_QTouchEvent_touchPoints(const QTouchEvent* this_ptr) {
  return &this_ptr->touchPoints();
}

QWindow* qt_gui_c_QTouchEvent_window(const QTouchEvent* this_ptr) {
  return this_ptr->window();
}


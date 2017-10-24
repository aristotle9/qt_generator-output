#include "qt_widgets_c_QScroller.h"

QObject* qt_widgets_c_QScroller_G_static_cast_QObject_ptr(QScroller* ptr) {
  return static_cast<QObject*>(ptr);
}

QScroller* qt_widgets_c_QScroller_G_static_cast_QScroller_ptr(QObject* ptr) {
  return static_cast<QScroller*>(ptr);
}

void qt_widgets_c_QScroller_activeScrollers_to_output(QList< QScroller* >* output) {
  new(output) QList< QScroller* >(QScroller::activeScrollers());
}

void qt_widgets_c_QScroller_ensureVisible_rect_xmargin_ymargin(QScroller* this_ptr, const QRectF* rect, double xmargin, double ymargin) {
  this_ptr->ensureVisible(*rect, xmargin, ymargin);
}

void qt_widgets_c_QScroller_ensureVisible_rect_xmargin_ymargin_scrollTime(QScroller* this_ptr, const QRectF* rect, double xmargin, double ymargin, int scrollTime) {
  this_ptr->ensureVisible(*rect, xmargin, ymargin, scrollTime);
}

void qt_widgets_c_QScroller_finalPosition_to_output(const QScroller* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->finalPosition());
}

bool qt_widgets_c_QScroller_handleInput_input_position(QScroller* this_ptr, QScroller::Input input, const QPointF* position) {
  return this_ptr->handleInput(input, *position);
}

bool qt_widgets_c_QScroller_handleInput_input_position_timestamp(QScroller* this_ptr, QScroller::Input input, const QPointF* position, qint64 timestamp) {
  return this_ptr->handleInput(input, *position, timestamp);
}

bool qt_widgets_c_QScroller_hasScroller(QObject* target) {
  return QScroller::hasScroller(target);
}

const QMetaObject* qt_widgets_c_QScroller_metaObject(const QScroller* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QScroller_pixelPerMeter_to_output(const QScroller* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->pixelPerMeter());
}

int qt_widgets_c_QScroller_qt_metacall(QScroller* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QScroller_qt_metacast(QScroller* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QScroller_resendPrepareEvent(QScroller* this_ptr) {
  this_ptr->resendPrepareEvent();
}

void qt_widgets_c_QScroller_scrollTo_pos(QScroller* this_ptr, const QPointF* pos) {
  this_ptr->scrollTo(*pos);
}

void qt_widgets_c_QScroller_scrollTo_pos_scrollTime(QScroller* this_ptr, const QPointF* pos, int scrollTime) {
  this_ptr->scrollTo(*pos, scrollTime);
}

QScrollerProperties* qt_widgets_c_QScroller_scrollerProperties_as_ptr(const QScroller* this_ptr) {
  return new QScrollerProperties(this_ptr->scrollerProperties());
}

QScroller* qt_widgets_c_QScroller_scroller_QObject_ptr(QObject* target) {
  return QScroller::scroller(target);
}

const QScroller* qt_widgets_c_QScroller_scroller_const_QObject_ptr(const QObject* target) {
  return QScroller::scroller(target);
}

void qt_widgets_c_QScroller_setScrollerProperties(QScroller* this_ptr, const QScrollerProperties* prop) {
  this_ptr->setScrollerProperties(*prop);
}

void qt_widgets_c_QScroller_setSnapPositionsX_first_interval(QScroller* this_ptr, double first, double interval) {
  this_ptr->setSnapPositionsX(first, interval);
}

void qt_widgets_c_QScroller_setSnapPositionsX_positions(QScroller* this_ptr, const QList< double >* positions) {
  this_ptr->setSnapPositionsX(*positions);
}

void qt_widgets_c_QScroller_setSnapPositionsY_first_interval(QScroller* this_ptr, double first, double interval) {
  this_ptr->setSnapPositionsY(first, interval);
}

void qt_widgets_c_QScroller_setSnapPositionsY_positions(QScroller* this_ptr, const QList< double >* positions) {
  this_ptr->setSnapPositionsY(*positions);
}

QScroller::State qt_widgets_c_QScroller_state(const QScroller* this_ptr) {
  return this_ptr->state();
}

void qt_widgets_c_QScroller_stop(QScroller* this_ptr) {
  this_ptr->stop();
}

QObject* qt_widgets_c_QScroller_target(const QScroller* this_ptr) {
  return this_ptr->target();
}

void qt_widgets_c_QScroller_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QScroller::trUtf8(s, c, n));
}

void qt_widgets_c_QScroller_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QScroller::tr(s, c, n));
}

void qt_widgets_c_QScroller_ungrabGesture(QObject* target) {
  QScroller::ungrabGesture(target);
}

void qt_widgets_c_QScroller_velocity_to_output(const QScroller* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->velocity());
}


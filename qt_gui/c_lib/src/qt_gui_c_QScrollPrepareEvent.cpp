#include "qt_gui_c_QScrollPrepareEvent.h"

QEvent* qt_gui_c_QScrollPrepareEvent_G_static_cast_QEvent_ptr(QScrollPrepareEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

QScrollPrepareEvent* qt_gui_c_QScrollPrepareEvent_G_static_cast_QScrollPrepareEvent_ptr(QEvent* ptr) {
  return static_cast<QScrollPrepareEvent*>(ptr);
}

void qt_gui_c_QScrollPrepareEvent_contentPosRange_to_output(const QScrollPrepareEvent* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->contentPosRange());
}

void qt_gui_c_QScrollPrepareEvent_contentPos_to_output(const QScrollPrepareEvent* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->contentPos());
}

void qt_gui_c_QScrollPrepareEvent_delete(QScrollPrepareEvent* this_ptr) {
  delete this_ptr;
}

QScrollPrepareEvent* qt_gui_c_QScrollPrepareEvent_new(const QPointF* startPos) {
  return new QScrollPrepareEvent(*startPos);
}

void qt_gui_c_QScrollPrepareEvent_setContentPos(QScrollPrepareEvent* this_ptr, const QPointF* pos) {
  this_ptr->setContentPos(*pos);
}

void qt_gui_c_QScrollPrepareEvent_setContentPosRange(QScrollPrepareEvent* this_ptr, const QRectF* rect) {
  this_ptr->setContentPosRange(*rect);
}

void qt_gui_c_QScrollPrepareEvent_setViewportSize(QScrollPrepareEvent* this_ptr, const QSizeF* size) {
  this_ptr->setViewportSize(*size);
}

void qt_gui_c_QScrollPrepareEvent_startPos_to_output(const QScrollPrepareEvent* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->startPos());
}

void qt_gui_c_QScrollPrepareEvent_viewportSize_to_output(const QScrollPrepareEvent* this_ptr, QSizeF* output) {
  new(output) QSizeF(this_ptr->viewportSize());
}


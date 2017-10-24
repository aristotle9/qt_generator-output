#include "qt_gui_c_QDropEvent.h"

QDropEvent* qt_gui_c_QDropEvent_G_static_cast_QDropEvent_ptr(QEvent* ptr) {
  return static_cast<QDropEvent*>(ptr);
}

QEvent* qt_gui_c_QDropEvent_G_static_cast_QEvent_ptr(QDropEvent* ptr) {
  return static_cast<QEvent*>(ptr);
}

void qt_gui_c_QDropEvent_acceptProposedAction(QDropEvent* this_ptr) {
  this_ptr->acceptProposedAction();
}

void qt_gui_c_QDropEvent_delete(QDropEvent* this_ptr) {
  delete this_ptr;
}

const QMimeData* qt_gui_c_QDropEvent_mimeData(const QDropEvent* this_ptr) {
  return this_ptr->mimeData();
}

const QPointF* qt_gui_c_QDropEvent_posF(const QDropEvent* this_ptr) {
  return &this_ptr->posF();
}

void qt_gui_c_QDropEvent_pos_to_output(const QDropEvent* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->pos());
}

void qt_gui_c_QDropEvent_setDropAction(QDropEvent* this_ptr, const Qt::DropAction* action) {
  this_ptr->setDropAction(*action);
}

QObject* qt_gui_c_QDropEvent_source(const QDropEvent* this_ptr) {
  return this_ptr->source();
}


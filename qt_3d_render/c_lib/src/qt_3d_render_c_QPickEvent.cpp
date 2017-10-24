#include "qt_3d_render_c_QPickEvent.h"

QObject* qt_3d_render_c_QPickEvent_G_static_cast_QObject_ptr(Qt3DRender::QPickEvent* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DRender::QPickEvent* qt_3d_render_c_QPickEvent_G_static_cast_Qt3DRender_QPickEvent_ptr(QObject* ptr) {
  return static_cast<Qt3DRender::QPickEvent*>(ptr);
}

Qt3DRender::QPickEvent::Buttons qt_3d_render_c_Qt3DRender_QPickEvent_button(const Qt3DRender::QPickEvent* this_ptr) {
  return this_ptr->button();
}

int qt_3d_render_c_Qt3DRender_QPickEvent_buttons(const Qt3DRender::QPickEvent* this_ptr) {
  return this_ptr->buttons();
}

void qt_3d_render_c_Qt3DRender_QPickEvent_delete(Qt3DRender::QPickEvent* this_ptr) {
  delete this_ptr;
}

float qt_3d_render_c_Qt3DRender_QPickEvent_distance(const Qt3DRender::QPickEvent* this_ptr) {
  return this_ptr->distance();
}

bool qt_3d_render_c_Qt3DRender_QPickEvent_isAccepted(const Qt3DRender::QPickEvent* this_ptr) {
  return this_ptr->isAccepted();
}

QVector3D* qt_3d_render_c_Qt3DRender_QPickEvent_localIntersection_as_ptr(const Qt3DRender::QPickEvent* this_ptr) {
  return new QVector3D(this_ptr->localIntersection());
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QPickEvent_metaObject(const Qt3DRender::QPickEvent* this_ptr) {
  return this_ptr->metaObject();
}

int qt_3d_render_c_Qt3DRender_QPickEvent_modifiers(const Qt3DRender::QPickEvent* this_ptr) {
  return this_ptr->modifiers();
}

Qt3DRender::QPickEvent* qt_3d_render_c_Qt3DRender_QPickEvent_new_no_args() {
  return new Qt3DRender::QPickEvent();
}

Qt3DRender::QPickEvent* qt_3d_render_c_Qt3DRender_QPickEvent_new_position_worldIntersection_localIntersection_distance(const QPointF* position, const QVector3D* worldIntersection, const QVector3D* localIntersection, float distance) {
  return new Qt3DRender::QPickEvent(*position, *worldIntersection, *localIntersection, distance);
}

Qt3DRender::QPickEvent* qt_3d_render_c_Qt3DRender_QPickEvent_new_position_worldIntersection_localIntersection_distance_button_buttons_modifiers(const QPointF* position, const QVector3D* worldIntersection, const QVector3D* localIntersection, float distance, Qt3DRender::QPickEvent::Buttons button, int buttons, int modifiers) {
  return new Qt3DRender::QPickEvent(*position, *worldIntersection, *localIntersection, distance, button, buttons, modifiers);
}

void qt_3d_render_c_Qt3DRender_QPickEvent_position_to_output(const Qt3DRender::QPickEvent* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->position());
}

int qt_3d_render_c_Qt3DRender_QPickEvent_qt_metacall(Qt3DRender::QPickEvent* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QPickEvent_qt_metacast(Qt3DRender::QPickEvent* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QPickEvent_setAccepted(Qt3DRender::QPickEvent* this_ptr, bool accepted) {
  this_ptr->setAccepted(accepted);
}

void qt_3d_render_c_Qt3DRender_QPickEvent_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QPickEvent::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QPickEvent_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QPickEvent::tr(s, c, n));
}

QVector3D* qt_3d_render_c_Qt3DRender_QPickEvent_worldIntersection_as_ptr(const Qt3DRender::QPickEvent* this_ptr) {
  return new QVector3D(this_ptr->worldIntersection());
}


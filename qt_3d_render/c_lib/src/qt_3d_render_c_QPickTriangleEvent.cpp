#include "qt_3d_render_c_QPickTriangleEvent.h"

Qt3DRender::QPickTriangleEvent* qt_3d_render_c_QPickTriangleEvent_G_dynamic_cast_Qt3DRender_QPickTriangleEvent_ptr(Qt3DRender::QPickEvent* ptr) {
  return dynamic_cast<Qt3DRender::QPickTriangleEvent*>(ptr);
}

QObject* qt_3d_render_c_QPickTriangleEvent_G_static_cast_QObject_ptr(Qt3DRender::QPickTriangleEvent* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DRender::QPickEvent* qt_3d_render_c_QPickTriangleEvent_G_static_cast_Qt3DRender_QPickEvent_ptr(Qt3DRender::QPickTriangleEvent* ptr) {
  return static_cast<Qt3DRender::QPickEvent*>(ptr);
}

Qt3DRender::QPickTriangleEvent* qt_3d_render_c_QPickTriangleEvent_G_static_cast_Qt3DRender_QPickTriangleEvent_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QPickTriangleEvent*>(ptr);
}

Qt3DRender::QPickTriangleEvent* qt_3d_render_c_QPickTriangleEvent_G_static_cast_Qt3DRender_QPickTriangleEvent_ptr_Qt3DRender_QPickEvent(Qt3DRender::QPickEvent* ptr) {
  return static_cast<Qt3DRender::QPickTriangleEvent*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QPickTriangleEvent_delete(Qt3DRender::QPickTriangleEvent* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QPickTriangleEvent_metaObject(const Qt3DRender::QPickTriangleEvent* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QPickTriangleEvent* qt_3d_render_c_Qt3DRender_QPickTriangleEvent_new_no_args() {
  return new Qt3DRender::QPickTriangleEvent();
}

Qt3DRender::QPickTriangleEvent* qt_3d_render_c_Qt3DRender_QPickTriangleEvent_new_position_worldIntersection_localIntersection_distance_triangleIndex_vertex1Index_vertex2Index_vertex3Index(const QPointF* position, const QVector3D* worldIntersection, const QVector3D* localIntersection, float distance, unsigned int triangleIndex, unsigned int vertex1Index, unsigned int vertex2Index, unsigned int vertex3Index) {
  return new Qt3DRender::QPickTriangleEvent(*position, *worldIntersection, *localIntersection, distance, triangleIndex, vertex1Index, vertex2Index, vertex3Index);
}

Qt3DRender::QPickTriangleEvent* qt_3d_render_c_Qt3DRender_QPickTriangleEvent_new_position_worldIntersection_localIntersection_distance_triangleIndex_vertex1Index_vertex2Index_vertex3Index_button_buttons_modifiers_uvw(const QPointF* position, const QVector3D* worldIntersection, const QVector3D* localIntersection, float distance, unsigned int triangleIndex, unsigned int vertex1Index, unsigned int vertex2Index, unsigned int vertex3Index, Qt3DRender::QPickEvent::Buttons button, int buttons, int modifiers, const QVector3D* uvw) {
  return new Qt3DRender::QPickTriangleEvent(*position, *worldIntersection, *localIntersection, distance, triangleIndex, vertex1Index, vertex2Index, vertex3Index, button, buttons, modifiers, *uvw);
}

int qt_3d_render_c_Qt3DRender_QPickTriangleEvent_qt_metacall(Qt3DRender::QPickTriangleEvent* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QPickTriangleEvent_qt_metacast(Qt3DRender::QPickTriangleEvent* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QPickTriangleEvent_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QPickTriangleEvent::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QPickTriangleEvent_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QPickTriangleEvent::tr(s, c, n));
}

unsigned int qt_3d_render_c_Qt3DRender_QPickTriangleEvent_triangleIndex(const Qt3DRender::QPickTriangleEvent* this_ptr) {
  return this_ptr->triangleIndex();
}

QVector3D* qt_3d_render_c_Qt3DRender_QPickTriangleEvent_uvw_as_ptr(const Qt3DRender::QPickTriangleEvent* this_ptr) {
  return new QVector3D(this_ptr->uvw());
}

unsigned int qt_3d_render_c_Qt3DRender_QPickTriangleEvent_vertex1Index(const Qt3DRender::QPickTriangleEvent* this_ptr) {
  return this_ptr->vertex1Index();
}

unsigned int qt_3d_render_c_Qt3DRender_QPickTriangleEvent_vertex2Index(const Qt3DRender::QPickTriangleEvent* this_ptr) {
  return this_ptr->vertex2Index();
}

unsigned int qt_3d_render_c_Qt3DRender_QPickTriangleEvent_vertex3Index(const Qt3DRender::QPickTriangleEvent* this_ptr) {
  return this_ptr->vertex3Index();
}


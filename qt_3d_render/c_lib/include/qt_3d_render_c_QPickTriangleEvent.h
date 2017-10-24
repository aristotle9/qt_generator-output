#ifndef QT_3D_RENDER_C_QPICKTRIANGLEEVENT_H
#define QT_3D_RENDER_C_QPICKTRIANGLEEVENT_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT Qt3DRender::QPickTriangleEvent* qt_3d_render_c_QPickTriangleEvent_G_dynamic_cast_Qt3DRender_QPickTriangleEvent_ptr(Qt3DRender::QPickEvent* ptr);
QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QPickTriangleEvent_G_static_cast_QObject_ptr(Qt3DRender::QPickTriangleEvent* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QPickEvent* qt_3d_render_c_QPickTriangleEvent_G_static_cast_Qt3DRender_QPickEvent_ptr(Qt3DRender::QPickTriangleEvent* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QPickTriangleEvent* qt_3d_render_c_QPickTriangleEvent_G_static_cast_Qt3DRender_QPickTriangleEvent_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QPickTriangleEvent* qt_3d_render_c_QPickTriangleEvent_G_static_cast_Qt3DRender_QPickTriangleEvent_ptr_Qt3DRender_QPickEvent(Qt3DRender::QPickEvent* ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QPickTriangleEvent_delete(Qt3DRender::QPickTriangleEvent* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QPickTriangleEvent_metaObject(const Qt3DRender::QPickTriangleEvent* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QPickTriangleEvent* qt_3d_render_c_Qt3DRender_QPickTriangleEvent_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QPickTriangleEvent* qt_3d_render_c_Qt3DRender_QPickTriangleEvent_new_position_worldIntersection_localIntersection_distance_triangleIndex_vertex1Index_vertex2Index_vertex3Index(const QPointF* position, const QVector3D* worldIntersection, const QVector3D* localIntersection, float distance, unsigned int triangleIndex, unsigned int vertex1Index, unsigned int vertex2Index, unsigned int vertex3Index);
QT_3D_RENDER_C_EXPORT Qt3DRender::QPickTriangleEvent* qt_3d_render_c_Qt3DRender_QPickTriangleEvent_new_position_worldIntersection_localIntersection_distance_triangleIndex_vertex1Index_vertex2Index_vertex3Index_button_buttons_modifiers_uvw(const QPointF* position, const QVector3D* worldIntersection, const QVector3D* localIntersection, float distance, unsigned int triangleIndex, unsigned int vertex1Index, unsigned int vertex2Index, unsigned int vertex3Index, Qt3DRender::QPickEvent::Buttons button, int buttons, int modifiers, const QVector3D* uvw);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QPickTriangleEvent_qt_metacall(Qt3DRender::QPickTriangleEvent* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QPickTriangleEvent_qt_metacast(Qt3DRender::QPickTriangleEvent* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QPickTriangleEvent_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QPickTriangleEvent_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT unsigned int qt_3d_render_c_Qt3DRender_QPickTriangleEvent_triangleIndex(const Qt3DRender::QPickTriangleEvent* this_ptr);
QT_3D_RENDER_C_EXPORT QVector3D* qt_3d_render_c_Qt3DRender_QPickTriangleEvent_uvw_as_ptr(const Qt3DRender::QPickTriangleEvent* this_ptr);
QT_3D_RENDER_C_EXPORT unsigned int qt_3d_render_c_Qt3DRender_QPickTriangleEvent_vertex1Index(const Qt3DRender::QPickTriangleEvent* this_ptr);
QT_3D_RENDER_C_EXPORT unsigned int qt_3d_render_c_Qt3DRender_QPickTriangleEvent_vertex2Index(const Qt3DRender::QPickTriangleEvent* this_ptr);
QT_3D_RENDER_C_EXPORT unsigned int qt_3d_render_c_Qt3DRender_QPickTriangleEvent_vertex3Index(const Qt3DRender::QPickTriangleEvent* this_ptr);

} // extern "C"

#endif // QT_3D_RENDER_C_QPICKTRIANGLEEVENT_H

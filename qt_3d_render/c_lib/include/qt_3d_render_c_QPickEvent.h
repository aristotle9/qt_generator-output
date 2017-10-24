#ifndef QT_3D_RENDER_C_QPICKEVENT_H
#define QT_3D_RENDER_C_QPICKEVENT_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QPickEvent_G_static_cast_QObject_ptr(Qt3DRender::QPickEvent* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QPickEvent* qt_3d_render_c_QPickEvent_G_static_cast_Qt3DRender_QPickEvent_ptr(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QPickEvent::Buttons qt_3d_render_c_Qt3DRender_QPickEvent_button(const Qt3DRender::QPickEvent* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QPickEvent_buttons(const Qt3DRender::QPickEvent* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QPickEvent_delete(Qt3DRender::QPickEvent* this_ptr);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QPickEvent_distance(const Qt3DRender::QPickEvent* this_ptr);
QT_3D_RENDER_C_EXPORT bool qt_3d_render_c_Qt3DRender_QPickEvent_isAccepted(const Qt3DRender::QPickEvent* this_ptr);
QT_3D_RENDER_C_EXPORT QVector3D* qt_3d_render_c_Qt3DRender_QPickEvent_localIntersection_as_ptr(const Qt3DRender::QPickEvent* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QPickEvent_metaObject(const Qt3DRender::QPickEvent* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QPickEvent_modifiers(const Qt3DRender::QPickEvent* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QPickEvent* qt_3d_render_c_Qt3DRender_QPickEvent_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QPickEvent* qt_3d_render_c_Qt3DRender_QPickEvent_new_position_worldIntersection_localIntersection_distance(const QPointF* position, const QVector3D* worldIntersection, const QVector3D* localIntersection, float distance);
QT_3D_RENDER_C_EXPORT Qt3DRender::QPickEvent* qt_3d_render_c_Qt3DRender_QPickEvent_new_position_worldIntersection_localIntersection_distance_button_buttons_modifiers(const QPointF* position, const QVector3D* worldIntersection, const QVector3D* localIntersection, float distance, Qt3DRender::QPickEvent::Buttons button, int buttons, int modifiers);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QPickEvent_position_to_output(const Qt3DRender::QPickEvent* this_ptr, QPointF* output);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QPickEvent_qt_metacall(Qt3DRender::QPickEvent* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QPickEvent_qt_metacast(Qt3DRender::QPickEvent* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QPickEvent_setAccepted(Qt3DRender::QPickEvent* this_ptr, bool accepted);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QPickEvent_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QPickEvent_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT QVector3D* qt_3d_render_c_Qt3DRender_QPickEvent_worldIntersection_as_ptr(const Qt3DRender::QPickEvent* this_ptr);

} // extern "C"

#endif // QT_3D_RENDER_C_QPICKEVENT_H

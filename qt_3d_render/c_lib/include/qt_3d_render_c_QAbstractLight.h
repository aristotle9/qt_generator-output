#ifndef QT_3D_RENDER_C_QABSTRACTLIGHT_H
#define QT_3D_RENDER_C_QABSTRACTLIGHT_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QAbstractLight_G_static_cast_QObject_ptr(Qt3DRender::QAbstractLight* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QComponent* qt_3d_render_c_QAbstractLight_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QAbstractLight* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QAbstractLight_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QAbstractLight* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAbstractLight* qt_3d_render_c_QAbstractLight_G_static_cast_Qt3DRender_QAbstractLight_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAbstractLight* qt_3d_render_c_QAbstractLight_G_static_cast_Qt3DRender_QAbstractLight_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAbstractLight* qt_3d_render_c_QAbstractLight_G_static_cast_Qt3DRender_QAbstractLight_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractLight_color_to_output(const Qt3DRender::QAbstractLight* this_ptr, QColor* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractLight_delete(Qt3DRender::QAbstractLight* this_ptr);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QAbstractLight_intensity(const Qt3DRender::QAbstractLight* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QAbstractLight_metaObject(const Qt3DRender::QAbstractLight* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QAbstractLight_qt_metacall(Qt3DRender::QAbstractLight* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QAbstractLight_qt_metacast(Qt3DRender::QAbstractLight* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractLight_setColor(Qt3DRender::QAbstractLight* this_ptr, const QColor* color);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractLight_setIntensity(Qt3DRender::QAbstractLight* this_ptr, float intensity);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractLight_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAbstractLight_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAbstractLight::Type qt_3d_render_c_Qt3DRender_QAbstractLight_type(const Qt3DRender::QAbstractLight* this_ptr);

} // extern "C"

#endif // QT_3D_RENDER_C_QABSTRACTLIGHT_H

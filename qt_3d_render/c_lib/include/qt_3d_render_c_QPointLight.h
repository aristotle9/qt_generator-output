#ifndef QT_3D_RENDER_C_QPOINTLIGHT_H
#define QT_3D_RENDER_C_QPOINTLIGHT_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT Qt3DRender::QPointLight* qt_3d_render_c_QPointLight_G_dynamic_cast_Qt3DRender_QPointLight_ptr(Qt3DRender::QAbstractLight* ptr);
QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QPointLight_G_static_cast_QObject_ptr(Qt3DRender::QPointLight* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QComponent* qt_3d_render_c_QPointLight_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QPointLight* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QPointLight_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QPointLight* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAbstractLight* qt_3d_render_c_QPointLight_G_static_cast_Qt3DRender_QAbstractLight_ptr(Qt3DRender::QPointLight* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QPointLight* qt_3d_render_c_QPointLight_G_static_cast_Qt3DRender_QPointLight_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QPointLight* qt_3d_render_c_QPointLight_G_static_cast_Qt3DRender_QPointLight_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QPointLight* qt_3d_render_c_QPointLight_G_static_cast_Qt3DRender_QPointLight_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QPointLight* qt_3d_render_c_QPointLight_G_static_cast_Qt3DRender_QPointLight_ptr_Qt3DRender_QAbstractLight(Qt3DRender::QAbstractLight* ptr);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QPointLight_constantAttenuation(const Qt3DRender::QPointLight* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QPointLight_delete(Qt3DRender::QPointLight* this_ptr);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QPointLight_linearAttenuation(const Qt3DRender::QPointLight* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QPointLight_metaObject(const Qt3DRender::QPointLight* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QPointLight* qt_3d_render_c_Qt3DRender_QPointLight_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QPointLight* qt_3d_render_c_Qt3DRender_QPointLight_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QPointLight_qt_metacall(Qt3DRender::QPointLight* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QPointLight_qt_metacast(Qt3DRender::QPointLight* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QPointLight_quadraticAttenuation(const Qt3DRender::QPointLight* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QPointLight_setConstantAttenuation(Qt3DRender::QPointLight* this_ptr, float value);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QPointLight_setLinearAttenuation(Qt3DRender::QPointLight* this_ptr, float value);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QPointLight_setQuadraticAttenuation(Qt3DRender::QPointLight* this_ptr, float value);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QPointLight_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QPointLight_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QPOINTLIGHT_H

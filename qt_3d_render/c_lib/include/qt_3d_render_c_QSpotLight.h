#ifndef QT_3D_RENDER_C_QSPOTLIGHT_H
#define QT_3D_RENDER_C_QSPOTLIGHT_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT Qt3DRender::QSpotLight* qt_3d_render_c_QSpotLight_G_dynamic_cast_Qt3DRender_QSpotLight_ptr(Qt3DRender::QAbstractLight* ptr);
QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QSpotLight_G_static_cast_QObject_ptr(Qt3DRender::QSpotLight* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QComponent* qt_3d_render_c_QSpotLight_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QSpotLight* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QSpotLight_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QSpotLight* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAbstractLight* qt_3d_render_c_QSpotLight_G_static_cast_Qt3DRender_QAbstractLight_ptr(Qt3DRender::QSpotLight* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QSpotLight* qt_3d_render_c_QSpotLight_G_static_cast_Qt3DRender_QSpotLight_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QSpotLight* qt_3d_render_c_QSpotLight_G_static_cast_Qt3DRender_QSpotLight_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QSpotLight* qt_3d_render_c_QSpotLight_G_static_cast_Qt3DRender_QSpotLight_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QSpotLight* qt_3d_render_c_QSpotLight_G_static_cast_Qt3DRender_QSpotLight_ptr_Qt3DRender_QAbstractLight(Qt3DRender::QAbstractLight* ptr);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QSpotLight_constantAttenuation(const Qt3DRender::QSpotLight* this_ptr);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QSpotLight_cutOffAngle(const Qt3DRender::QSpotLight* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QSpotLight_delete(Qt3DRender::QSpotLight* this_ptr);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QSpotLight_linearAttenuation(const Qt3DRender::QSpotLight* this_ptr);
QT_3D_RENDER_C_EXPORT QVector3D* qt_3d_render_c_Qt3DRender_QSpotLight_localDirection_as_ptr(const Qt3DRender::QSpotLight* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QSpotLight_metaObject(const Qt3DRender::QSpotLight* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QSpotLight* qt_3d_render_c_Qt3DRender_QSpotLight_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QSpotLight* qt_3d_render_c_Qt3DRender_QSpotLight_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QSpotLight_qt_metacall(Qt3DRender::QSpotLight* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QSpotLight_qt_metacast(Qt3DRender::QSpotLight* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT float qt_3d_render_c_Qt3DRender_QSpotLight_quadraticAttenuation(const Qt3DRender::QSpotLight* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QSpotLight_setConstantAttenuation(Qt3DRender::QSpotLight* this_ptr, float value);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QSpotLight_setCutOffAngle(Qt3DRender::QSpotLight* this_ptr, float cutOffAngle);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QSpotLight_setLinearAttenuation(Qt3DRender::QSpotLight* this_ptr, float value);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QSpotLight_setLocalDirection(Qt3DRender::QSpotLight* this_ptr, const QVector3D* localDirection);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QSpotLight_setQuadraticAttenuation(Qt3DRender::QSpotLight* this_ptr, float value);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QSpotLight_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QSpotLight_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QSPOTLIGHT_H

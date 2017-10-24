#ifndef QT_3D_RENDER_C_QENVIRONMENTLIGHT_H
#define QT_3D_RENDER_C_QENVIRONMENTLIGHT_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QEnvironmentLight_G_static_cast_QObject_ptr(Qt3DRender::QEnvironmentLight* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QComponent* qt_3d_render_c_QEnvironmentLight_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QEnvironmentLight* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QEnvironmentLight_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QEnvironmentLight* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QEnvironmentLight* qt_3d_render_c_QEnvironmentLight_G_static_cast_Qt3DRender_QEnvironmentLight_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QEnvironmentLight* qt_3d_render_c_QEnvironmentLight_G_static_cast_Qt3DRender_QEnvironmentLight_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QEnvironmentLight* qt_3d_render_c_QEnvironmentLight_G_static_cast_Qt3DRender_QEnvironmentLight_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QEnvironmentLight_delete(Qt3DRender::QEnvironmentLight* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAbstractTexture* qt_3d_render_c_Qt3DRender_QEnvironmentLight_irradiance(const Qt3DRender::QEnvironmentLight* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QEnvironmentLight_metaObject(const Qt3DRender::QEnvironmentLight* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QEnvironmentLight* qt_3d_render_c_Qt3DRender_QEnvironmentLight_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QEnvironmentLight* qt_3d_render_c_Qt3DRender_QEnvironmentLight_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QEnvironmentLight_qt_metacall(Qt3DRender::QEnvironmentLight* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QEnvironmentLight_qt_metacast(Qt3DRender::QEnvironmentLight* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QEnvironmentLight_setIrradiance(Qt3DRender::QEnvironmentLight* this_ptr, Qt3DRender::QAbstractTexture* irradiance);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QEnvironmentLight_setSpecular(Qt3DRender::QEnvironmentLight* this_ptr, Qt3DRender::QAbstractTexture* specular);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAbstractTexture* qt_3d_render_c_Qt3DRender_QEnvironmentLight_specular(const Qt3DRender::QEnvironmentLight* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QEnvironmentLight_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QEnvironmentLight_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QENVIRONMENTLIGHT_H

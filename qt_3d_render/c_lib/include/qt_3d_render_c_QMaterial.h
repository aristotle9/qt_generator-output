#ifndef QT_3D_RENDER_C_QMATERIAL_H
#define QT_3D_RENDER_C_QMATERIAL_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QMaterial_G_static_cast_QObject_ptr(Qt3DRender::QMaterial* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QComponent* qt_3d_render_c_QMaterial_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QMaterial* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QMaterial_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QMaterial* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QMaterial* qt_3d_render_c_QMaterial_G_static_cast_Qt3DRender_QMaterial_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QMaterial* qt_3d_render_c_QMaterial_G_static_cast_Qt3DRender_QMaterial_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QMaterial* qt_3d_render_c_QMaterial_G_static_cast_Qt3DRender_QMaterial_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QMaterial_addParameter(Qt3DRender::QMaterial* this_ptr, Qt3DRender::QParameter* parameter);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QMaterial_delete(Qt3DRender::QMaterial* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QEffect* qt_3d_render_c_Qt3DRender_QMaterial_effect(const Qt3DRender::QMaterial* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QMaterial_metaObject(const Qt3DRender::QMaterial* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QMaterial* qt_3d_render_c_Qt3DRender_QMaterial_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QMaterial* qt_3d_render_c_Qt3DRender_QMaterial_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QMaterial_parameters_to_output(const Qt3DRender::QMaterial* this_ptr, QVector< Qt3DRender::QParameter* >* output);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QMaterial_qt_metacall(Qt3DRender::QMaterial* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QMaterial_qt_metacast(Qt3DRender::QMaterial* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QMaterial_removeParameter(Qt3DRender::QMaterial* this_ptr, Qt3DRender::QParameter* parameter);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QMaterial_setEffect(Qt3DRender::QMaterial* this_ptr, Qt3DRender::QEffect* effect);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QMaterial_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QMaterial_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QMATERIAL_H

#ifndef QT_3D_RENDER_C_QEFFECT_H
#define QT_3D_RENDER_C_QEFFECT_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QEffect_G_static_cast_QObject_ptr(Qt3DRender::QEffect* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QEffect_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QEffect* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QEffect* qt_3d_render_c_QEffect_G_static_cast_Qt3DRender_QEffect_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QEffect* qt_3d_render_c_QEffect_G_static_cast_Qt3DRender_QEffect_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QEffect_addParameter(Qt3DRender::QEffect* this_ptr, Qt3DRender::QParameter* parameter);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QEffect_addTechnique(Qt3DRender::QEffect* this_ptr, Qt3DRender::QTechnique* t);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QEffect_delete(Qt3DRender::QEffect* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QEffect_metaObject(const Qt3DRender::QEffect* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QEffect* qt_3d_render_c_Qt3DRender_QEffect_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QEffect* qt_3d_render_c_Qt3DRender_QEffect_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QEffect_parameters_to_output(const Qt3DRender::QEffect* this_ptr, QVector< Qt3DRender::QParameter* >* output);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QEffect_qt_metacall(Qt3DRender::QEffect* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QEffect_qt_metacast(Qt3DRender::QEffect* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QEffect_removeParameter(Qt3DRender::QEffect* this_ptr, Qt3DRender::QParameter* parameter);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QEffect_removeTechnique(Qt3DRender::QEffect* this_ptr, Qt3DRender::QTechnique* t);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QEffect_techniques_to_output(const Qt3DRender::QEffect* this_ptr, QVector< Qt3DRender::QTechnique* >* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QEffect_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QEffect_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QEFFECT_H

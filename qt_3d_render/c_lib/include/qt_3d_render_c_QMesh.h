#ifndef QT_3D_RENDER_C_QMESH_H
#define QT_3D_RENDER_C_QMESH_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT Qt3DRender::QMesh* qt_3d_render_c_QMesh_G_dynamic_cast_Qt3DRender_QMesh_ptr(Qt3DRender::QGeometryRenderer* ptr);
QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QMesh_G_static_cast_QObject_ptr(Qt3DRender::QMesh* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QComponent* qt_3d_render_c_QMesh_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QMesh* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QMesh_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QMesh* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QGeometryRenderer* qt_3d_render_c_QMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(Qt3DRender::QMesh* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QMesh* qt_3d_render_c_QMesh_G_static_cast_Qt3DRender_QMesh_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QMesh* qt_3d_render_c_QMesh_G_static_cast_Qt3DRender_QMesh_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QMesh* qt_3d_render_c_QMesh_G_static_cast_Qt3DRender_QMesh_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QMesh* qt_3d_render_c_QMesh_G_static_cast_Qt3DRender_QMesh_ptr_Qt3DRender_QGeometryRenderer(Qt3DRender::QGeometryRenderer* ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QMesh_delete(Qt3DRender::QMesh* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QMesh_meshName_to_output(const Qt3DRender::QMesh* this_ptr, QString* output);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QMesh_metaObject(const Qt3DRender::QMesh* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QMesh* qt_3d_render_c_Qt3DRender_QMesh_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QMesh* qt_3d_render_c_Qt3DRender_QMesh_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QMesh_qt_metacall(Qt3DRender::QMesh* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QMesh_qt_metacast(Qt3DRender::QMesh* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QMesh_setMeshName(Qt3DRender::QMesh* this_ptr, const QString* meshName);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QMesh_setSource(Qt3DRender::QMesh* this_ptr, const QUrl* source);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QMesh_source_to_output(const Qt3DRender::QMesh* this_ptr, QUrl* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QMesh_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QMesh_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QMESH_H

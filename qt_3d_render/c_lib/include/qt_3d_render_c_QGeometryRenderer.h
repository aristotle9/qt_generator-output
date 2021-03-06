#ifndef QT_3D_RENDER_C_QGEOMETRYRENDERER_H
#define QT_3D_RENDER_C_QGEOMETRYRENDERER_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QGeometryRenderer_G_static_cast_QObject_ptr(Qt3DRender::QGeometryRenderer* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QComponent* qt_3d_render_c_QGeometryRenderer_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QGeometryRenderer* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QGeometryRenderer_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QGeometryRenderer* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QGeometryRenderer* qt_3d_render_c_QGeometryRenderer_G_static_cast_Qt3DRender_QGeometryRenderer_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QGeometryRenderer* qt_3d_render_c_QGeometryRenderer_G_static_cast_Qt3DRender_QGeometryRenderer_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QGeometryRenderer* qt_3d_render_c_QGeometryRenderer_G_static_cast_Qt3DRender_QGeometryRenderer_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGeometryRenderer_delete(Qt3DRender::QGeometryRenderer* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QGeometryRenderer_firstInstance(const Qt3DRender::QGeometryRenderer* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QGeometryRenderer_firstVertex(const Qt3DRender::QGeometryRenderer* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QGeometry* qt_3d_render_c_Qt3DRender_QGeometryRenderer_geometry(const Qt3DRender::QGeometryRenderer* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGeometryRenderer_geometryFactory_to_output(const Qt3DRender::QGeometryRenderer* this_ptr, QSharedPointer< Qt3DRender::QGeometryFactory >* output);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QGeometryRenderer_indexOffset(const Qt3DRender::QGeometryRenderer* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QGeometryRenderer_instanceCount(const Qt3DRender::QGeometryRenderer* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QGeometryRenderer_metaObject(const Qt3DRender::QGeometryRenderer* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QGeometryRenderer* qt_3d_render_c_Qt3DRender_QGeometryRenderer_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QGeometryRenderer* qt_3d_render_c_Qt3DRender_QGeometryRenderer_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT bool qt_3d_render_c_Qt3DRender_QGeometryRenderer_primitiveRestartEnabled(const Qt3DRender::QGeometryRenderer* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QGeometryRenderer::PrimitiveType qt_3d_render_c_Qt3DRender_QGeometryRenderer_primitiveType(const Qt3DRender::QGeometryRenderer* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QGeometryRenderer_qt_metacall(Qt3DRender::QGeometryRenderer* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QGeometryRenderer_qt_metacast(Qt3DRender::QGeometryRenderer* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QGeometryRenderer_restartIndexValue(const Qt3DRender::QGeometryRenderer* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGeometryRenderer_setFirstInstance(Qt3DRender::QGeometryRenderer* this_ptr, int firstInstance);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGeometryRenderer_setFirstVertex(Qt3DRender::QGeometryRenderer* this_ptr, int firstVertex);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGeometryRenderer_setGeometry(Qt3DRender::QGeometryRenderer* this_ptr, Qt3DRender::QGeometry* geometry);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGeometryRenderer_setGeometryFactory(Qt3DRender::QGeometryRenderer* this_ptr, const QSharedPointer< Qt3DRender::QGeometryFactory >* factory);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGeometryRenderer_setIndexOffset(Qt3DRender::QGeometryRenderer* this_ptr, int indexOffset);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGeometryRenderer_setInstanceCount(Qt3DRender::QGeometryRenderer* this_ptr, int instanceCount);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGeometryRenderer_setPrimitiveRestartEnabled(Qt3DRender::QGeometryRenderer* this_ptr, bool enabled);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGeometryRenderer_setPrimitiveType(Qt3DRender::QGeometryRenderer* this_ptr, Qt3DRender::QGeometryRenderer::PrimitiveType primitiveType);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGeometryRenderer_setRestartIndexValue(Qt3DRender::QGeometryRenderer* this_ptr, int index);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGeometryRenderer_setVertexCount(Qt3DRender::QGeometryRenderer* this_ptr, int vertexCount);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGeometryRenderer_setVerticesPerPatch(Qt3DRender::QGeometryRenderer* this_ptr, int verticesPerPatch);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGeometryRenderer_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGeometryRenderer_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QGeometryRenderer_vertexCount(const Qt3DRender::QGeometryRenderer* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QGeometryRenderer_verticesPerPatch(const Qt3DRender::QGeometryRenderer* this_ptr);

} // extern "C"

#endif // QT_3D_RENDER_C_QGEOMETRYRENDERER_H

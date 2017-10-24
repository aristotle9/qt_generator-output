#ifndef QT_3D_RENDER_C_QTECHNIQUE_H
#define QT_3D_RENDER_C_QTECHNIQUE_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QTechnique_G_static_cast_QObject_ptr(Qt3DRender::QTechnique* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QTechnique_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QTechnique* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QTechnique* qt_3d_render_c_QTechnique_G_static_cast_Qt3DRender_QTechnique_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QTechnique* qt_3d_render_c_QTechnique_G_static_cast_Qt3DRender_QTechnique_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTechnique_addFilterKey(Qt3DRender::QTechnique* this_ptr, Qt3DRender::QFilterKey* filterKey);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTechnique_addParameter(Qt3DRender::QTechnique* this_ptr, Qt3DRender::QParameter* p);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTechnique_addRenderPass(Qt3DRender::QTechnique* this_ptr, Qt3DRender::QRenderPass* pass);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTechnique_delete(Qt3DRender::QTechnique* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTechnique_filterKeys_to_output(const Qt3DRender::QTechnique* this_ptr, QVector< Qt3DRender::QFilterKey* >* output);
QT_3D_RENDER_C_EXPORT Qt3DRender::QGraphicsApiFilter* qt_3d_render_c_Qt3DRender_QTechnique_graphicsApiFilter(Qt3DRender::QTechnique* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QTechnique_metaObject(const Qt3DRender::QTechnique* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QTechnique* qt_3d_render_c_Qt3DRender_QTechnique_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QTechnique* qt_3d_render_c_Qt3DRender_QTechnique_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTechnique_parameters_to_output(const Qt3DRender::QTechnique* this_ptr, QVector< Qt3DRender::QParameter* >* output);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QTechnique_qt_metacall(Qt3DRender::QTechnique* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QTechnique_qt_metacast(Qt3DRender::QTechnique* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTechnique_removeFilterKey(Qt3DRender::QTechnique* this_ptr, Qt3DRender::QFilterKey* filterKey);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTechnique_removeParameter(Qt3DRender::QTechnique* this_ptr, Qt3DRender::QParameter* p);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTechnique_removeRenderPass(Qt3DRender::QTechnique* this_ptr, Qt3DRender::QRenderPass* pass);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTechnique_renderPasses_to_output(const Qt3DRender::QTechnique* this_ptr, QVector< Qt3DRender::QRenderPass* >* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTechnique_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QTechnique_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QTECHNIQUE_H

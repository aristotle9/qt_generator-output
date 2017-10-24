#ifndef QT_3D_RENDER_C_QSHADERPROGRAM_H
#define QT_3D_RENDER_C_QSHADERPROGRAM_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QShaderProgram_G_static_cast_QObject_ptr(Qt3DRender::QShaderProgram* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QShaderProgram_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QShaderProgram* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QShaderProgram* qt_3d_render_c_QShaderProgram_G_static_cast_Qt3DRender_QShaderProgram_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QShaderProgram* qt_3d_render_c_QShaderProgram_G_static_cast_Qt3DRender_QShaderProgram_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QShaderProgram_computeShaderCode_to_output(const Qt3DRender::QShaderProgram* this_ptr, QByteArray* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QShaderProgram_delete(Qt3DRender::QShaderProgram* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QShaderProgram_fragmentShaderCode_to_output(const Qt3DRender::QShaderProgram* this_ptr, QByteArray* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QShaderProgram_geometryShaderCode_to_output(const Qt3DRender::QShaderProgram* this_ptr, QByteArray* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QShaderProgram_loadSource_to_output(const QUrl* sourceUrl, QByteArray* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QShaderProgram_log_to_output(const Qt3DRender::QShaderProgram* this_ptr, QString* output);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QShaderProgram_metaObject(const Qt3DRender::QShaderProgram* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QShaderProgram* qt_3d_render_c_Qt3DRender_QShaderProgram_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QShaderProgram* qt_3d_render_c_Qt3DRender_QShaderProgram_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QShaderProgram_qt_metacall(Qt3DRender::QShaderProgram* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QShaderProgram_qt_metacast(Qt3DRender::QShaderProgram* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QShaderProgram_setComputeShaderCode(Qt3DRender::QShaderProgram* this_ptr, const QByteArray* computeShaderCode);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QShaderProgram_setFragmentShaderCode(Qt3DRender::QShaderProgram* this_ptr, const QByteArray* fragmentShaderCode);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QShaderProgram_setGeometryShaderCode(Qt3DRender::QShaderProgram* this_ptr, const QByteArray* geometryShaderCode);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QShaderProgram_setShaderCode(Qt3DRender::QShaderProgram* this_ptr, Qt3DRender::QShaderProgram::ShaderType type, const QByteArray* shaderCode);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QShaderProgram_setTessellationControlShaderCode(Qt3DRender::QShaderProgram* this_ptr, const QByteArray* tessellationControlShaderCode);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QShaderProgram_setTessellationEvaluationShaderCode(Qt3DRender::QShaderProgram* this_ptr, const QByteArray* tessellationEvaluationShaderCode);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QShaderProgram_setVertexShaderCode(Qt3DRender::QShaderProgram* this_ptr, const QByteArray* vertexShaderCode);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QShaderProgram_shaderCode_to_output(const Qt3DRender::QShaderProgram* this_ptr, Qt3DRender::QShaderProgram::ShaderType type, QByteArray* output);
QT_3D_RENDER_C_EXPORT Qt3DRender::QShaderProgram::Status qt_3d_render_c_Qt3DRender_QShaderProgram_status(const Qt3DRender::QShaderProgram* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QShaderProgram_tessellationControlShaderCode_to_output(const Qt3DRender::QShaderProgram* this_ptr, QByteArray* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QShaderProgram_tessellationEvaluationShaderCode_to_output(const Qt3DRender::QShaderProgram* this_ptr, QByteArray* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QShaderProgram_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QShaderProgram_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QShaderProgram_vertexShaderCode_to_output(const Qt3DRender::QShaderProgram* this_ptr, QByteArray* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QSHADERPROGRAM_H

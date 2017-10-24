#ifndef QT_3D_RENDER_C_QSTENCILTESTARGUMENTS_H
#define QT_3D_RENDER_C_QSTENCILTESTARGUMENTS_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QStencilTestArguments_G_static_cast_QObject_ptr(Qt3DRender::QStencilTestArguments* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QStencilTestArguments* qt_3d_render_c_QStencilTestArguments_G_static_cast_Qt3DRender_QStencilTestArguments_ptr(QObject* ptr);
QT_3D_RENDER_C_EXPORT unsigned int qt_3d_render_c_Qt3DRender_QStencilTestArguments_comparisonMask(const Qt3DRender::QStencilTestArguments* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QStencilTestArguments_delete(Qt3DRender::QStencilTestArguments* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QStencilTestArguments::StencilFaceMode qt_3d_render_c_Qt3DRender_QStencilTestArguments_faceMode(const Qt3DRender::QStencilTestArguments* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QStencilTestArguments_metaObject(const Qt3DRender::QStencilTestArguments* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QStencilTestArguments_qt_metacall(Qt3DRender::QStencilTestArguments* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QStencilTestArguments_qt_metacast(Qt3DRender::QStencilTestArguments* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QStencilTestArguments_referenceValue(const Qt3DRender::QStencilTestArguments* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QStencilTestArguments_setComparisonMask(Qt3DRender::QStencilTestArguments* this_ptr, unsigned int comparisonMask);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QStencilTestArguments_setReferenceValue(Qt3DRender::QStencilTestArguments* this_ptr, int referenceValue);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QStencilTestArguments_setStencilFunction(Qt3DRender::QStencilTestArguments* this_ptr, Qt3DRender::QStencilTestArguments::StencilFunction stencilFunction);
QT_3D_RENDER_C_EXPORT Qt3DRender::QStencilTestArguments::StencilFunction qt_3d_render_c_Qt3DRender_QStencilTestArguments_stencilFunction(const Qt3DRender::QStencilTestArguments* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QStencilTestArguments_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QStencilTestArguments_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QSTENCILTESTARGUMENTS_H

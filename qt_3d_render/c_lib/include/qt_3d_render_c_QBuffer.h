#ifndef QT_3D_RENDER_C_QBUFFER_H
#define QT_3D_RENDER_C_QBUFFER_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QBuffer_G_static_cast_QObject_ptr(Qt3DRender::QBuffer* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QBuffer_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QBuffer* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QBuffer* qt_3d_render_c_QBuffer_G_static_cast_Qt3DRender_QBuffer_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QBuffer* qt_3d_render_c_QBuffer_G_static_cast_Qt3DRender_QBuffer_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QBuffer::AccessType qt_3d_render_c_Qt3DRender_QBuffer_accessType(const Qt3DRender::QBuffer* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QBuffer_dataGenerator_to_output(const Qt3DRender::QBuffer* this_ptr, QSharedPointer< Qt3DRender::QBufferDataGenerator >* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QBuffer_data_to_output(const Qt3DRender::QBuffer* this_ptr, QByteArray* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QBuffer_delete(Qt3DRender::QBuffer* this_ptr);
QT_3D_RENDER_C_EXPORT bool qt_3d_render_c_Qt3DRender_QBuffer_isSyncData(const Qt3DRender::QBuffer* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QBuffer_metaObject(const Qt3DRender::QBuffer* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QBuffer* qt_3d_render_c_Qt3DRender_QBuffer_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QBuffer* qt_3d_render_c_Qt3DRender_QBuffer_new_ty(Qt3DRender::QBuffer::BufferType ty);
QT_3D_RENDER_C_EXPORT Qt3DRender::QBuffer* qt_3d_render_c_Qt3DRender_QBuffer_new_ty_parent(Qt3DRender::QBuffer::BufferType ty, Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QBuffer_qt_metacall(Qt3DRender::QBuffer* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QBuffer_qt_metacast(Qt3DRender::QBuffer* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QBuffer_setAccessType(Qt3DRender::QBuffer* this_ptr, Qt3DRender::QBuffer::AccessType access);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QBuffer_setData(Qt3DRender::QBuffer* this_ptr, const QByteArray* bytes);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QBuffer_setDataGenerator(Qt3DRender::QBuffer* this_ptr, const QSharedPointer< Qt3DRender::QBufferDataGenerator >* functor);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QBuffer_setSyncData(Qt3DRender::QBuffer* this_ptr, bool syncData);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QBuffer_setType(Qt3DRender::QBuffer* this_ptr, Qt3DRender::QBuffer::BufferType type);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QBuffer_setUsage(Qt3DRender::QBuffer* this_ptr, Qt3DRender::QBuffer::UsageType usage);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QBuffer_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QBuffer_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT Qt3DRender::QBuffer::BufferType qt_3d_render_c_Qt3DRender_QBuffer_type(const Qt3DRender::QBuffer* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QBuffer_updateData(Qt3DRender::QBuffer* this_ptr, int offset, const QByteArray* bytes);
QT_3D_RENDER_C_EXPORT Qt3DRender::QBuffer::UsageType qt_3d_render_c_Qt3DRender_QBuffer_usage(const Qt3DRender::QBuffer* this_ptr);

} // extern "C"

#endif // QT_3D_RENDER_C_QBUFFER_H

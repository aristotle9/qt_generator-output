#ifndef QT_3D_RENDER_C_QATTRIBUTE_H
#define QT_3D_RENDER_C_QATTRIBUTE_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QAttribute_G_static_cast_QObject_ptr(Qt3DRender::QAttribute* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QAttribute_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QAttribute* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAttribute* qt_3d_render_c_QAttribute_G_static_cast_Qt3DRender_QAttribute_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAttribute* qt_3d_render_c_QAttribute_G_static_cast_Qt3DRender_QAttribute_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAttribute::AttributeType qt_3d_render_c_Qt3DRender_QAttribute_attributeType(const Qt3DRender::QAttribute* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QBuffer* qt_3d_render_c_Qt3DRender_QAttribute_buffer(const Qt3DRender::QAttribute* this_ptr);
QT_3D_RENDER_C_EXPORT unsigned int qt_3d_render_c_Qt3DRender_QAttribute_byteOffset(const Qt3DRender::QAttribute* this_ptr);
QT_3D_RENDER_C_EXPORT unsigned int qt_3d_render_c_Qt3DRender_QAttribute_byteStride(const Qt3DRender::QAttribute* this_ptr);
QT_3D_RENDER_C_EXPORT unsigned int qt_3d_render_c_Qt3DRender_QAttribute_count(const Qt3DRender::QAttribute* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAttribute_defaultColorAttributeName_to_output(QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAttribute_defaultNormalAttributeName_to_output(QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAttribute_defaultPositionAttributeName_to_output(QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAttribute_defaultTangentAttributeName_to_output(QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAttribute_defaultTextureCoordinateAttributeName_to_output(QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAttribute_delete(Qt3DRender::QAttribute* this_ptr);
QT_3D_RENDER_C_EXPORT unsigned int qt_3d_render_c_Qt3DRender_QAttribute_divisor(const Qt3DRender::QAttribute* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QAttribute_metaObject(const Qt3DRender::QAttribute* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAttribute_name_to_output(const Qt3DRender::QAttribute* this_ptr, QString* output);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAttribute* qt_3d_render_c_Qt3DRender_QAttribute_new_buf_name_vertexBaseType_vertexSize_count(Qt3DRender::QBuffer* buf, const QString* name, Qt3DRender::QAttribute::VertexBaseType vertexBaseType, unsigned int vertexSize, unsigned int count);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAttribute* qt_3d_render_c_Qt3DRender_QAttribute_new_buf_name_vertexBaseType_vertexSize_count_offset(Qt3DRender::QBuffer* buf, const QString* name, Qt3DRender::QAttribute::VertexBaseType vertexBaseType, unsigned int vertexSize, unsigned int count, unsigned int offset);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAttribute* qt_3d_render_c_Qt3DRender_QAttribute_new_buf_name_vertexBaseType_vertexSize_count_offset_stride(Qt3DRender::QBuffer* buf, const QString* name, Qt3DRender::QAttribute::VertexBaseType vertexBaseType, unsigned int vertexSize, unsigned int count, unsigned int offset, unsigned int stride);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAttribute* qt_3d_render_c_Qt3DRender_QAttribute_new_buf_name_vertexBaseType_vertexSize_count_offset_stride_parent(Qt3DRender::QBuffer* buf, const QString* name, Qt3DRender::QAttribute::VertexBaseType vertexBaseType, unsigned int vertexSize, unsigned int count, unsigned int offset, unsigned int stride, Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAttribute* qt_3d_render_c_Qt3DRender_QAttribute_new_buf_vertexBaseType_vertexSize_count(Qt3DRender::QBuffer* buf, Qt3DRender::QAttribute::VertexBaseType vertexBaseType, unsigned int vertexSize, unsigned int count);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAttribute* qt_3d_render_c_Qt3DRender_QAttribute_new_buf_vertexBaseType_vertexSize_count_offset(Qt3DRender::QBuffer* buf, Qt3DRender::QAttribute::VertexBaseType vertexBaseType, unsigned int vertexSize, unsigned int count, unsigned int offset);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAttribute* qt_3d_render_c_Qt3DRender_QAttribute_new_buf_vertexBaseType_vertexSize_count_offset_stride(Qt3DRender::QBuffer* buf, Qt3DRender::QAttribute::VertexBaseType vertexBaseType, unsigned int vertexSize, unsigned int count, unsigned int offset, unsigned int stride);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAttribute* qt_3d_render_c_Qt3DRender_QAttribute_new_buf_vertexBaseType_vertexSize_count_offset_stride_parent(Qt3DRender::QBuffer* buf, Qt3DRender::QAttribute::VertexBaseType vertexBaseType, unsigned int vertexSize, unsigned int count, unsigned int offset, unsigned int stride, Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAttribute* qt_3d_render_c_Qt3DRender_QAttribute_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QAttribute* qt_3d_render_c_Qt3DRender_QAttribute_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QAttribute_qt_metacall(Qt3DRender::QAttribute* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QAttribute_qt_metacast(Qt3DRender::QAttribute* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAttribute_setAttributeType(Qt3DRender::QAttribute* this_ptr, Qt3DRender::QAttribute::AttributeType attributeType);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAttribute_setBuffer(Qt3DRender::QAttribute* this_ptr, Qt3DRender::QBuffer* buffer);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAttribute_setByteOffset(Qt3DRender::QAttribute* this_ptr, unsigned int byteOffset);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAttribute_setByteStride(Qt3DRender::QAttribute* this_ptr, unsigned int byteStride);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAttribute_setCount(Qt3DRender::QAttribute* this_ptr, unsigned int count);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAttribute_setDataSize(Qt3DRender::QAttribute* this_ptr, unsigned int size);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAttribute_setDataType(Qt3DRender::QAttribute* this_ptr, Qt3DRender::QAttribute::VertexBaseType type);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAttribute_setDivisor(Qt3DRender::QAttribute* this_ptr, unsigned int divisor);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAttribute_setName(Qt3DRender::QAttribute* this_ptr, const QString* name);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAttribute_setVertexBaseType(Qt3DRender::QAttribute* this_ptr, Qt3DRender::QAttribute::VertexBaseType type);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAttribute_setVertexSize(Qt3DRender::QAttribute* this_ptr, unsigned int size);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAttribute_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QAttribute_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT Qt3DRender::QAttribute::VertexBaseType qt_3d_render_c_Qt3DRender_QAttribute_vertexBaseType(const Qt3DRender::QAttribute* this_ptr);
QT_3D_RENDER_C_EXPORT unsigned int qt_3d_render_c_Qt3DRender_QAttribute_vertexSize(const Qt3DRender::QAttribute* this_ptr);

} // extern "C"

#endif // QT_3D_RENDER_C_QATTRIBUTE_H

#include "qt_3d_render_c_QAttribute.h"

QObject* qt_3d_render_c_QAttribute_G_static_cast_QObject_ptr(Qt3DRender::QAttribute* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QAttribute_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QAttribute* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QAttribute* qt_3d_render_c_QAttribute_G_static_cast_Qt3DRender_QAttribute_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QAttribute*>(ptr);
}

Qt3DRender::QAttribute* qt_3d_render_c_QAttribute_G_static_cast_Qt3DRender_QAttribute_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QAttribute*>(ptr);
}

Qt3DRender::QAttribute::AttributeType qt_3d_render_c_Qt3DRender_QAttribute_attributeType(const Qt3DRender::QAttribute* this_ptr) {
  return this_ptr->attributeType();
}

Qt3DRender::QBuffer* qt_3d_render_c_Qt3DRender_QAttribute_buffer(const Qt3DRender::QAttribute* this_ptr) {
  return this_ptr->buffer();
}

unsigned int qt_3d_render_c_Qt3DRender_QAttribute_byteOffset(const Qt3DRender::QAttribute* this_ptr) {
  return this_ptr->byteOffset();
}

unsigned int qt_3d_render_c_Qt3DRender_QAttribute_byteStride(const Qt3DRender::QAttribute* this_ptr) {
  return this_ptr->byteStride();
}

unsigned int qt_3d_render_c_Qt3DRender_QAttribute_count(const Qt3DRender::QAttribute* this_ptr) {
  return this_ptr->count();
}

void qt_3d_render_c_Qt3DRender_QAttribute_defaultColorAttributeName_to_output(QString* output) {
  new(output) QString(Qt3DRender::QAttribute::defaultColorAttributeName());
}

void qt_3d_render_c_Qt3DRender_QAttribute_defaultNormalAttributeName_to_output(QString* output) {
  new(output) QString(Qt3DRender::QAttribute::defaultNormalAttributeName());
}

void qt_3d_render_c_Qt3DRender_QAttribute_defaultPositionAttributeName_to_output(QString* output) {
  new(output) QString(Qt3DRender::QAttribute::defaultPositionAttributeName());
}

void qt_3d_render_c_Qt3DRender_QAttribute_defaultTangentAttributeName_to_output(QString* output) {
  new(output) QString(Qt3DRender::QAttribute::defaultTangentAttributeName());
}

void qt_3d_render_c_Qt3DRender_QAttribute_defaultTextureCoordinateAttributeName_to_output(QString* output) {
  new(output) QString(Qt3DRender::QAttribute::defaultTextureCoordinateAttributeName());
}

void qt_3d_render_c_Qt3DRender_QAttribute_delete(Qt3DRender::QAttribute* this_ptr) {
  delete this_ptr;
}

unsigned int qt_3d_render_c_Qt3DRender_QAttribute_divisor(const Qt3DRender::QAttribute* this_ptr) {
  return this_ptr->divisor();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QAttribute_metaObject(const Qt3DRender::QAttribute* this_ptr) {
  return this_ptr->metaObject();
}

void qt_3d_render_c_Qt3DRender_QAttribute_name_to_output(const Qt3DRender::QAttribute* this_ptr, QString* output) {
  new(output) QString(this_ptr->name());
}

Qt3DRender::QAttribute* qt_3d_render_c_Qt3DRender_QAttribute_new_buf_name_vertexBaseType_vertexSize_count(Qt3DRender::QBuffer* buf, const QString* name, Qt3DRender::QAttribute::VertexBaseType vertexBaseType, unsigned int vertexSize, unsigned int count) {
  return new Qt3DRender::QAttribute(buf, *name, vertexBaseType, vertexSize, count);
}

Qt3DRender::QAttribute* qt_3d_render_c_Qt3DRender_QAttribute_new_buf_name_vertexBaseType_vertexSize_count_offset(Qt3DRender::QBuffer* buf, const QString* name, Qt3DRender::QAttribute::VertexBaseType vertexBaseType, unsigned int vertexSize, unsigned int count, unsigned int offset) {
  return new Qt3DRender::QAttribute(buf, *name, vertexBaseType, vertexSize, count, offset);
}

Qt3DRender::QAttribute* qt_3d_render_c_Qt3DRender_QAttribute_new_buf_name_vertexBaseType_vertexSize_count_offset_stride(Qt3DRender::QBuffer* buf, const QString* name, Qt3DRender::QAttribute::VertexBaseType vertexBaseType, unsigned int vertexSize, unsigned int count, unsigned int offset, unsigned int stride) {
  return new Qt3DRender::QAttribute(buf, *name, vertexBaseType, vertexSize, count, offset, stride);
}

Qt3DRender::QAttribute* qt_3d_render_c_Qt3DRender_QAttribute_new_buf_name_vertexBaseType_vertexSize_count_offset_stride_parent(Qt3DRender::QBuffer* buf, const QString* name, Qt3DRender::QAttribute::VertexBaseType vertexBaseType, unsigned int vertexSize, unsigned int count, unsigned int offset, unsigned int stride, Qt3DCore::QNode* parent) {
  return new Qt3DRender::QAttribute(buf, *name, vertexBaseType, vertexSize, count, offset, stride, parent);
}

Qt3DRender::QAttribute* qt_3d_render_c_Qt3DRender_QAttribute_new_buf_vertexBaseType_vertexSize_count(Qt3DRender::QBuffer* buf, Qt3DRender::QAttribute::VertexBaseType vertexBaseType, unsigned int vertexSize, unsigned int count) {
  return new Qt3DRender::QAttribute(buf, vertexBaseType, vertexSize, count);
}

Qt3DRender::QAttribute* qt_3d_render_c_Qt3DRender_QAttribute_new_buf_vertexBaseType_vertexSize_count_offset(Qt3DRender::QBuffer* buf, Qt3DRender::QAttribute::VertexBaseType vertexBaseType, unsigned int vertexSize, unsigned int count, unsigned int offset) {
  return new Qt3DRender::QAttribute(buf, vertexBaseType, vertexSize, count, offset);
}

Qt3DRender::QAttribute* qt_3d_render_c_Qt3DRender_QAttribute_new_buf_vertexBaseType_vertexSize_count_offset_stride(Qt3DRender::QBuffer* buf, Qt3DRender::QAttribute::VertexBaseType vertexBaseType, unsigned int vertexSize, unsigned int count, unsigned int offset, unsigned int stride) {
  return new Qt3DRender::QAttribute(buf, vertexBaseType, vertexSize, count, offset, stride);
}

Qt3DRender::QAttribute* qt_3d_render_c_Qt3DRender_QAttribute_new_buf_vertexBaseType_vertexSize_count_offset_stride_parent(Qt3DRender::QBuffer* buf, Qt3DRender::QAttribute::VertexBaseType vertexBaseType, unsigned int vertexSize, unsigned int count, unsigned int offset, unsigned int stride, Qt3DCore::QNode* parent) {
  return new Qt3DRender::QAttribute(buf, vertexBaseType, vertexSize, count, offset, stride, parent);
}

Qt3DRender::QAttribute* qt_3d_render_c_Qt3DRender_QAttribute_new_no_args() {
  return new Qt3DRender::QAttribute();
}

Qt3DRender::QAttribute* qt_3d_render_c_Qt3DRender_QAttribute_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QAttribute(parent);
}

int qt_3d_render_c_Qt3DRender_QAttribute_qt_metacall(Qt3DRender::QAttribute* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QAttribute_qt_metacast(Qt3DRender::QAttribute* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QAttribute_setAttributeType(Qt3DRender::QAttribute* this_ptr, Qt3DRender::QAttribute::AttributeType attributeType) {
  this_ptr->setAttributeType(attributeType);
}

void qt_3d_render_c_Qt3DRender_QAttribute_setBuffer(Qt3DRender::QAttribute* this_ptr, Qt3DRender::QBuffer* buffer) {
  this_ptr->setBuffer(buffer);
}

void qt_3d_render_c_Qt3DRender_QAttribute_setByteOffset(Qt3DRender::QAttribute* this_ptr, unsigned int byteOffset) {
  this_ptr->setByteOffset(byteOffset);
}

void qt_3d_render_c_Qt3DRender_QAttribute_setByteStride(Qt3DRender::QAttribute* this_ptr, unsigned int byteStride) {
  this_ptr->setByteStride(byteStride);
}

void qt_3d_render_c_Qt3DRender_QAttribute_setCount(Qt3DRender::QAttribute* this_ptr, unsigned int count) {
  this_ptr->setCount(count);
}

void qt_3d_render_c_Qt3DRender_QAttribute_setDataSize(Qt3DRender::QAttribute* this_ptr, unsigned int size) {
  this_ptr->setDataSize(size);
}

void qt_3d_render_c_Qt3DRender_QAttribute_setDataType(Qt3DRender::QAttribute* this_ptr, Qt3DRender::QAttribute::VertexBaseType type) {
  this_ptr->setDataType(type);
}

void qt_3d_render_c_Qt3DRender_QAttribute_setDivisor(Qt3DRender::QAttribute* this_ptr, unsigned int divisor) {
  this_ptr->setDivisor(divisor);
}

void qt_3d_render_c_Qt3DRender_QAttribute_setName(Qt3DRender::QAttribute* this_ptr, const QString* name) {
  this_ptr->setName(*name);
}

void qt_3d_render_c_Qt3DRender_QAttribute_setVertexBaseType(Qt3DRender::QAttribute* this_ptr, Qt3DRender::QAttribute::VertexBaseType type) {
  this_ptr->setVertexBaseType(type);
}

void qt_3d_render_c_Qt3DRender_QAttribute_setVertexSize(Qt3DRender::QAttribute* this_ptr, unsigned int size) {
  this_ptr->setVertexSize(size);
}

void qt_3d_render_c_Qt3DRender_QAttribute_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QAttribute::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QAttribute_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QAttribute::tr(s, c, n));
}

Qt3DRender::QAttribute::VertexBaseType qt_3d_render_c_Qt3DRender_QAttribute_vertexBaseType(const Qt3DRender::QAttribute* this_ptr) {
  return this_ptr->vertexBaseType();
}

unsigned int qt_3d_render_c_Qt3DRender_QAttribute_vertexSize(const Qt3DRender::QAttribute* this_ptr) {
  return this_ptr->vertexSize();
}


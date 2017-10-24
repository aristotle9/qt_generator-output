#include "qt_3d_render_c_QBuffer.h"

QObject* qt_3d_render_c_QBuffer_G_static_cast_QObject_ptr(Qt3DRender::QBuffer* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QBuffer_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QBuffer* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QBuffer* qt_3d_render_c_QBuffer_G_static_cast_Qt3DRender_QBuffer_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QBuffer*>(ptr);
}

Qt3DRender::QBuffer* qt_3d_render_c_QBuffer_G_static_cast_Qt3DRender_QBuffer_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QBuffer*>(ptr);
}

Qt3DRender::QBuffer::AccessType qt_3d_render_c_Qt3DRender_QBuffer_accessType(const Qt3DRender::QBuffer* this_ptr) {
  return this_ptr->accessType();
}

void qt_3d_render_c_Qt3DRender_QBuffer_dataGenerator_to_output(const Qt3DRender::QBuffer* this_ptr, QSharedPointer< Qt3DRender::QBufferDataGenerator >* output) {
  new(output) QSharedPointer< Qt3DRender::QBufferDataGenerator >(this_ptr->dataGenerator());
}

void qt_3d_render_c_Qt3DRender_QBuffer_data_to_output(const Qt3DRender::QBuffer* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->data());
}

void qt_3d_render_c_Qt3DRender_QBuffer_delete(Qt3DRender::QBuffer* this_ptr) {
  delete this_ptr;
}

bool qt_3d_render_c_Qt3DRender_QBuffer_isSyncData(const Qt3DRender::QBuffer* this_ptr) {
  return this_ptr->isSyncData();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QBuffer_metaObject(const Qt3DRender::QBuffer* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QBuffer* qt_3d_render_c_Qt3DRender_QBuffer_new_no_args() {
  return new Qt3DRender::QBuffer();
}

Qt3DRender::QBuffer* qt_3d_render_c_Qt3DRender_QBuffer_new_ty(Qt3DRender::QBuffer::BufferType ty) {
  return new Qt3DRender::QBuffer(ty);
}

Qt3DRender::QBuffer* qt_3d_render_c_Qt3DRender_QBuffer_new_ty_parent(Qt3DRender::QBuffer::BufferType ty, Qt3DCore::QNode* parent) {
  return new Qt3DRender::QBuffer(ty, parent);
}

int qt_3d_render_c_Qt3DRender_QBuffer_qt_metacall(Qt3DRender::QBuffer* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QBuffer_qt_metacast(Qt3DRender::QBuffer* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QBuffer_setAccessType(Qt3DRender::QBuffer* this_ptr, Qt3DRender::QBuffer::AccessType access) {
  this_ptr->setAccessType(access);
}

void qt_3d_render_c_Qt3DRender_QBuffer_setData(Qt3DRender::QBuffer* this_ptr, const QByteArray* bytes) {
  this_ptr->setData(*bytes);
}

void qt_3d_render_c_Qt3DRender_QBuffer_setDataGenerator(Qt3DRender::QBuffer* this_ptr, const QSharedPointer< Qt3DRender::QBufferDataGenerator >* functor) {
  this_ptr->setDataGenerator(*functor);
}

void qt_3d_render_c_Qt3DRender_QBuffer_setSyncData(Qt3DRender::QBuffer* this_ptr, bool syncData) {
  this_ptr->setSyncData(syncData);
}

void qt_3d_render_c_Qt3DRender_QBuffer_setType(Qt3DRender::QBuffer* this_ptr, Qt3DRender::QBuffer::BufferType type) {
  this_ptr->setType(type);
}

void qt_3d_render_c_Qt3DRender_QBuffer_setUsage(Qt3DRender::QBuffer* this_ptr, Qt3DRender::QBuffer::UsageType usage) {
  this_ptr->setUsage(usage);
}

void qt_3d_render_c_Qt3DRender_QBuffer_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QBuffer::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QBuffer_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QBuffer::tr(s, c, n));
}

Qt3DRender::QBuffer::BufferType qt_3d_render_c_Qt3DRender_QBuffer_type(const Qt3DRender::QBuffer* this_ptr) {
  return this_ptr->type();
}

void qt_3d_render_c_Qt3DRender_QBuffer_updateData(Qt3DRender::QBuffer* this_ptr, int offset, const QByteArray* bytes) {
  this_ptr->updateData(offset, *bytes);
}

Qt3DRender::QBuffer::UsageType qt_3d_render_c_Qt3DRender_QBuffer_usage(const Qt3DRender::QBuffer* this_ptr) {
  return this_ptr->usage();
}


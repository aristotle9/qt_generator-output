#include "qt_core_c_QBuffer.h"

QBuffer* qt_core_c_QBuffer_G_dynamic_cast_QBuffer_ptr_QIODevice(QIODevice* ptr) {
  return dynamic_cast<QBuffer*>(ptr);
}

QBuffer* qt_core_c_QBuffer_G_dynamic_cast_QBuffer_ptr_QObject(QObject* ptr) {
  return dynamic_cast<QBuffer*>(ptr);
}

QBuffer* qt_core_c_QBuffer_G_static_cast_QBuffer_ptr_QIODevice(QIODevice* ptr) {
  return static_cast<QBuffer*>(ptr);
}

QBuffer* qt_core_c_QBuffer_G_static_cast_QBuffer_ptr_QObject(QObject* ptr) {
  return static_cast<QBuffer*>(ptr);
}

QIODevice* qt_core_c_QBuffer_G_static_cast_QIODevice_ptr(QBuffer* ptr) {
  return static_cast<QIODevice*>(ptr);
}

QObject* qt_core_c_QBuffer_G_static_cast_QObject_ptr(QBuffer* ptr) {
  return static_cast<QObject*>(ptr);
}

bool qt_core_c_QBuffer_atEnd(const QBuffer* this_ptr) {
  return this_ptr->atEnd();
}

QByteArray* qt_core_c_QBuffer_buffer(QBuffer* this_ptr) {
  return &this_ptr->buffer();
}

const QByteArray* qt_core_c_QBuffer_buffer_const(const QBuffer* this_ptr) {
  return &this_ptr->buffer();
}

bool qt_core_c_QBuffer_canReadLine(const QBuffer* this_ptr) {
  return this_ptr->canReadLine();
}

void qt_core_c_QBuffer_close(QBuffer* this_ptr) {
  this_ptr->close();
}

const QByteArray* qt_core_c_QBuffer_data(const QBuffer* this_ptr) {
  return &this_ptr->data();
}

void qt_core_c_QBuffer_delete(QBuffer* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_core_c_QBuffer_metaObject(const QBuffer* this_ptr) {
  return this_ptr->metaObject();
}

QBuffer* qt_core_c_QBuffer_new_buf(QByteArray* buf) {
  return new QBuffer(buf);
}

QBuffer* qt_core_c_QBuffer_new_buf_parent(QByteArray* buf, QObject* parent) {
  return new QBuffer(buf, parent);
}

QBuffer* qt_core_c_QBuffer_new_no_args() {
  return new QBuffer();
}

QBuffer* qt_core_c_QBuffer_new_parent(QObject* parent) {
  return new QBuffer(parent);
}

bool qt_core_c_QBuffer_open(QBuffer* this_ptr, unsigned int openMode) {
  return this_ptr->open(QFlags< QIODevice::OpenModeFlag >(openMode));
}

qint64 qt_core_c_QBuffer_pos(const QBuffer* this_ptr) {
  return this_ptr->pos();
}

bool qt_core_c_QBuffer_seek(QBuffer* this_ptr, qint64 off) {
  return this_ptr->seek(off);
}

void qt_core_c_QBuffer_setBuffer(QBuffer* this_ptr, QByteArray* a) {
  this_ptr->setBuffer(a);
}

void qt_core_c_QBuffer_setData_data(QBuffer* this_ptr, const QByteArray* data) {
  this_ptr->setData(*data);
}

void qt_core_c_QBuffer_setData_data_len(QBuffer* this_ptr, const char* data, int len) {
  this_ptr->setData(data, len);
}

qint64 qt_core_c_QBuffer_size(const QBuffer* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QBuffer_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QBuffer::trUtf8(s, c, n));
}

void qt_core_c_QBuffer_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QBuffer::tr(s, c, n));
}


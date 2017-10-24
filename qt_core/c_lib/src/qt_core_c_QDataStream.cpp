#include "qt_core_c_QDataStream.h"

void qt_core_c_QDataStream_abortTransaction(QDataStream* this_ptr) {
  this_ptr->abortTransaction();
}

bool qt_core_c_QDataStream_atEnd(const QDataStream* this_ptr) {
  return this_ptr->atEnd();
}

QDataStream::ByteOrder qt_core_c_QDataStream_byteOrder(const QDataStream* this_ptr) {
  return this_ptr->byteOrder();
}

bool qt_core_c_QDataStream_commitTransaction(QDataStream* this_ptr) {
  return this_ptr->commitTransaction();
}

void qt_core_c_QDataStream_constructor_QByteArray(const QByteArray* arg1, QDataStream* output) {
  new(output) QDataStream(*arg1);
}

void qt_core_c_QDataStream_constructor_QIODevice(QIODevice* arg1, QDataStream* output) {
  new(output) QDataStream(arg1);
}

void qt_core_c_QDataStream_constructor_no_args(QDataStream* output) {
  new(output) QDataStream();
}

void qt_core_c_QDataStream_destructor(QDataStream* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

QIODevice* qt_core_c_QDataStream_device(const QDataStream* this_ptr) {
  return this_ptr->device();
}

QDataStream::FloatingPointPrecision qt_core_c_QDataStream_floatingPointPrecision(const QDataStream* this_ptr) {
  return this_ptr->floatingPointPrecision();
}

QDataStream* qt_core_c_QDataStream_operator_shl_bool(QDataStream* this_ptr, bool i) {
  return &this_ptr->operator<<(i);
}

QDataStream* qt_core_c_QDataStream_operator_shl_char(QDataStream* this_ptr, const char* str) {
  return &this_ptr->operator<<(str);
}

QDataStream* qt_core_c_QDataStream_operator_shl_double(QDataStream* this_ptr, double f) {
  return &this_ptr->operator<<(f);
}

QDataStream* qt_core_c_QDataStream_operator_shl_float(QDataStream* this_ptr, float f) {
  return &this_ptr->operator<<(f);
}

QDataStream* qt_core_c_QDataStream_operator_shl_qint16(QDataStream* this_ptr, qint16 i) {
  return &this_ptr->operator<<(i);
}

QDataStream* qt_core_c_QDataStream_operator_shl_qint32(QDataStream* this_ptr, qint32 i) {
  return &this_ptr->operator<<(i);
}

QDataStream* qt_core_c_QDataStream_operator_shl_qint64(QDataStream* this_ptr, qint64 i) {
  return &this_ptr->operator<<(i);
}

QDataStream* qt_core_c_QDataStream_operator_shl_qint8(QDataStream* this_ptr, qint8 i) {
  return &this_ptr->operator<<(i);
}

QDataStream* qt_core_c_QDataStream_operator_shl_quint16(QDataStream* this_ptr, quint16 i) {
  return &this_ptr->operator<<(i);
}

QDataStream* qt_core_c_QDataStream_operator_shl_quint32(QDataStream* this_ptr, quint32 i) {
  return &this_ptr->operator<<(i);
}

QDataStream* qt_core_c_QDataStream_operator_shl_quint64(QDataStream* this_ptr, quint64 i) {
  return &this_ptr->operator<<(i);
}

QDataStream* qt_core_c_QDataStream_operator_shl_quint8(QDataStream* this_ptr, quint8 i) {
  return &this_ptr->operator<<(i);
}

QDataStream* qt_core_c_QDataStream_operator_shr_bool(QDataStream* this_ptr, bool* i) {
  return &this_ptr->operator>>(*i);
}

QDataStream* qt_core_c_QDataStream_operator_shr_char(QDataStream* this_ptr, char** str) {
  return &this_ptr->operator>>(*str);
}

QDataStream* qt_core_c_QDataStream_operator_shr_double(QDataStream* this_ptr, double* f) {
  return &this_ptr->operator>>(*f);
}

QDataStream* qt_core_c_QDataStream_operator_shr_float(QDataStream* this_ptr, float* f) {
  return &this_ptr->operator>>(*f);
}

QDataStream* qt_core_c_QDataStream_operator_shr_qint16(QDataStream* this_ptr, qint16* i) {
  return &this_ptr->operator>>(*i);
}

QDataStream* qt_core_c_QDataStream_operator_shr_qint32(QDataStream* this_ptr, qint32* i) {
  return &this_ptr->operator>>(*i);
}

QDataStream* qt_core_c_QDataStream_operator_shr_qint64(QDataStream* this_ptr, qint64* i) {
  return &this_ptr->operator>>(*i);
}

QDataStream* qt_core_c_QDataStream_operator_shr_qint8(QDataStream* this_ptr, qint8* i) {
  return &this_ptr->operator>>(*i);
}

QDataStream* qt_core_c_QDataStream_operator_shr_quint16(QDataStream* this_ptr, quint16* i) {
  return &this_ptr->operator>>(*i);
}

QDataStream* qt_core_c_QDataStream_operator_shr_quint32(QDataStream* this_ptr, quint32* i) {
  return &this_ptr->operator>>(*i);
}

QDataStream* qt_core_c_QDataStream_operator_shr_quint64(QDataStream* this_ptr, quint64* i) {
  return &this_ptr->operator>>(*i);
}

QDataStream* qt_core_c_QDataStream_operator_shr_quint8(QDataStream* this_ptr, quint8* i) {
  return &this_ptr->operator>>(*i);
}

QDataStream* qt_core_c_QDataStream_readBytes(QDataStream* this_ptr, char** arg1, unsigned int* len) {
  return &this_ptr->readBytes(*arg1, *len);
}

int qt_core_c_QDataStream_readRawData(QDataStream* this_ptr, char* arg1, int len) {
  return this_ptr->readRawData(arg1, len);
}

void qt_core_c_QDataStream_resetStatus(QDataStream* this_ptr) {
  this_ptr->resetStatus();
}

void qt_core_c_QDataStream_rollbackTransaction(QDataStream* this_ptr) {
  this_ptr->rollbackTransaction();
}

void qt_core_c_QDataStream_setByteOrder(QDataStream* this_ptr, QDataStream::ByteOrder arg1) {
  this_ptr->setByteOrder(arg1);
}

void qt_core_c_QDataStream_setDevice(QDataStream* this_ptr, QIODevice* arg1) {
  this_ptr->setDevice(arg1);
}

void qt_core_c_QDataStream_setFloatingPointPrecision(QDataStream* this_ptr, QDataStream::FloatingPointPrecision precision) {
  this_ptr->setFloatingPointPrecision(precision);
}

void qt_core_c_QDataStream_setStatus(QDataStream* this_ptr, QDataStream::Status status) {
  this_ptr->setStatus(status);
}

void qt_core_c_QDataStream_setVersion(QDataStream* this_ptr, int arg1) {
  this_ptr->setVersion(arg1);
}

int qt_core_c_QDataStream_skipRawData(QDataStream* this_ptr, int len) {
  return this_ptr->skipRawData(len);
}

void qt_core_c_QDataStream_startTransaction(QDataStream* this_ptr) {
  this_ptr->startTransaction();
}

QDataStream::Status qt_core_c_QDataStream_status(const QDataStream* this_ptr) {
  return this_ptr->status();
}

void qt_core_c_QDataStream_unsetDevice(QDataStream* this_ptr) {
  this_ptr->unsetDevice();
}

int qt_core_c_QDataStream_version(const QDataStream* this_ptr) {
  return this_ptr->version();
}

QDataStream* qt_core_c_QDataStream_writeBytes(QDataStream* this_ptr, const char* arg1, unsigned int len) {
  return &this_ptr->writeBytes(arg1, len);
}

int qt_core_c_QDataStream_writeRawData(QDataStream* this_ptr, const char* arg1, int len) {
  return this_ptr->writeRawData(arg1, len);
}


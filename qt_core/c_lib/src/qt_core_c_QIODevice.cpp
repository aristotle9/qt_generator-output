#include "qt_core_c_QIODevice.h"

QIODevice* qt_core_c_QIODevice_G_dynamic_cast_QIODevice_ptr(QObject* ptr) {
  return dynamic_cast<QIODevice*>(ptr);
}

QIODevice* qt_core_c_QIODevice_G_static_cast_QIODevice_ptr(QObject* ptr) {
  return static_cast<QIODevice*>(ptr);
}

QObject* qt_core_c_QIODevice_G_static_cast_QObject_ptr(QIODevice* ptr) {
  return static_cast<QObject*>(ptr);
}

bool qt_core_c_QIODevice_atEnd(const QIODevice* this_ptr) {
  return this_ptr->atEnd();
}

qint64 qt_core_c_QIODevice_bytesAvailable(const QIODevice* this_ptr) {
  return this_ptr->bytesAvailable();
}

qint64 qt_core_c_QIODevice_bytesToWrite(const QIODevice* this_ptr) {
  return this_ptr->bytesToWrite();
}

bool qt_core_c_QIODevice_canReadLine(const QIODevice* this_ptr) {
  return this_ptr->canReadLine();
}

void qt_core_c_QIODevice_close(QIODevice* this_ptr) {
  this_ptr->close();
}

void qt_core_c_QIODevice_commitTransaction(QIODevice* this_ptr) {
  this_ptr->commitTransaction();
}

int qt_core_c_QIODevice_currentReadChannel(const QIODevice* this_ptr) {
  return this_ptr->currentReadChannel();
}

int qt_core_c_QIODevice_currentWriteChannel(const QIODevice* this_ptr) {
  return this_ptr->currentWriteChannel();
}

void qt_core_c_QIODevice_delete(QIODevice* this_ptr) {
  delete this_ptr;
}

void qt_core_c_QIODevice_errorString_to_output(const QIODevice* this_ptr, QString* output) {
  new(output) QString(this_ptr->errorString());
}

bool qt_core_c_QIODevice_getChar(QIODevice* this_ptr, char* c) {
  return this_ptr->getChar(c);
}

bool qt_core_c_QIODevice_isOpen(const QIODevice* this_ptr) {
  return this_ptr->isOpen();
}

bool qt_core_c_QIODevice_isReadable(const QIODevice* this_ptr) {
  return this_ptr->isReadable();
}

bool qt_core_c_QIODevice_isSequential(const QIODevice* this_ptr) {
  return this_ptr->isSequential();
}

bool qt_core_c_QIODevice_isTextModeEnabled(const QIODevice* this_ptr) {
  return this_ptr->isTextModeEnabled();
}

bool qt_core_c_QIODevice_isTransactionStarted(const QIODevice* this_ptr) {
  return this_ptr->isTransactionStarted();
}

bool qt_core_c_QIODevice_isWritable(const QIODevice* this_ptr) {
  return this_ptr->isWritable();
}

const QMetaObject* qt_core_c_QIODevice_metaObject(const QIODevice* this_ptr) {
  return this_ptr->metaObject();
}

bool qt_core_c_QIODevice_open(QIODevice* this_ptr, unsigned int mode) {
  return this_ptr->open(QFlags< QIODevice::OpenModeFlag >(mode));
}

unsigned int qt_core_c_QIODevice_openMode(const QIODevice* this_ptr) {
  return uint(this_ptr->openMode());
}

qint64 qt_core_c_QIODevice_peek(QIODevice* this_ptr, char* data, qint64 maxlen) {
  return this_ptr->peek(data, maxlen);
}

void qt_core_c_QIODevice_peek_to_output(QIODevice* this_ptr, qint64 maxlen, QByteArray* output) {
  new(output) QByteArray(this_ptr->peek(maxlen));
}

qint64 qt_core_c_QIODevice_pos(const QIODevice* this_ptr) {
  return this_ptr->pos();
}

bool qt_core_c_QIODevice_putChar(QIODevice* this_ptr, char c) {
  return this_ptr->putChar(c);
}

qint64 qt_core_c_QIODevice_read(QIODevice* this_ptr, char* data, qint64 maxlen) {
  return this_ptr->read(data, maxlen);
}

void qt_core_c_QIODevice_readAll_to_output(QIODevice* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->readAll());
}

int qt_core_c_QIODevice_readChannelCount(const QIODevice* this_ptr) {
  return this_ptr->readChannelCount();
}

qint64 qt_core_c_QIODevice_readLine(QIODevice* this_ptr, char* data, qint64 maxlen) {
  return this_ptr->readLine(data, maxlen);
}

void qt_core_c_QIODevice_readLine_to_output_maxlen(QIODevice* this_ptr, qint64 maxlen, QByteArray* output) {
  new(output) QByteArray(this_ptr->readLine(maxlen));
}

void qt_core_c_QIODevice_readLine_to_output_no_args(QIODevice* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->readLine());
}

void qt_core_c_QIODevice_read_to_output(QIODevice* this_ptr, qint64 maxlen, QByteArray* output) {
  new(output) QByteArray(this_ptr->read(maxlen));
}

bool qt_core_c_QIODevice_reset(QIODevice* this_ptr) {
  return this_ptr->reset();
}

void qt_core_c_QIODevice_rollbackTransaction(QIODevice* this_ptr) {
  this_ptr->rollbackTransaction();
}

bool qt_core_c_QIODevice_seek(QIODevice* this_ptr, qint64 pos) {
  return this_ptr->seek(pos);
}

void qt_core_c_QIODevice_setCurrentReadChannel(QIODevice* this_ptr, int channel) {
  this_ptr->setCurrentReadChannel(channel);
}

void qt_core_c_QIODevice_setCurrentWriteChannel(QIODevice* this_ptr, int channel) {
  this_ptr->setCurrentWriteChannel(channel);
}

void qt_core_c_QIODevice_setTextModeEnabled(QIODevice* this_ptr, bool enabled) {
  this_ptr->setTextModeEnabled(enabled);
}

qint64 qt_core_c_QIODevice_size(const QIODevice* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QIODevice_startTransaction(QIODevice* this_ptr) {
  this_ptr->startTransaction();
}

void qt_core_c_QIODevice_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QIODevice::trUtf8(s, c, n));
}

void qt_core_c_QIODevice_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QIODevice::tr(s, c, n));
}

void qt_core_c_QIODevice_ungetChar(QIODevice* this_ptr, char c) {
  this_ptr->ungetChar(c);
}

bool qt_core_c_QIODevice_waitForBytesWritten(QIODevice* this_ptr, int msecs) {
  return this_ptr->waitForBytesWritten(msecs);
}

bool qt_core_c_QIODevice_waitForReadyRead(QIODevice* this_ptr, int msecs) {
  return this_ptr->waitForReadyRead(msecs);
}

int qt_core_c_QIODevice_writeChannelCount(const QIODevice* this_ptr) {
  return this_ptr->writeChannelCount();
}

qint64 qt_core_c_QIODevice_write_QByteArray(QIODevice* this_ptr, const QByteArray* data) {
  return this_ptr->write(*data);
}

qint64 qt_core_c_QIODevice_write_char(QIODevice* this_ptr, const char* data) {
  return this_ptr->write(data);
}

qint64 qt_core_c_QIODevice_write_char_qint64(QIODevice* this_ptr, const char* data, qint64 len) {
  return this_ptr->write(data, len);
}


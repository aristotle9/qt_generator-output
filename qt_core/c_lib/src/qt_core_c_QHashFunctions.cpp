#include "qt_core_c_QHashFunctions.h"

int qt_core_c_QHashFunctions_G_qGlobalQHashSeed() {
  return qGlobalQHashSeed();
}

unsigned int qt_core_c_QHashFunctions_G_qHashBits_p_size(const void* p, unsigned long size) {
  return qHashBits(p, size);
}

unsigned int qt_core_c_QHashFunctions_G_qHashBits_p_size_seed(const void* p, unsigned long size, unsigned int seed) {
  return qHashBits(p, size, seed);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_QBitArray(const QBitArray* key) {
  return qHash(*key);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_QBitArray_unsigned_int(const QBitArray* key, unsigned int seed) {
  return qHash(*key, seed);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_QByteArray(const QByteArray* key) {
  return qHash(*key);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_QByteArray_unsigned_int(const QByteArray* key, unsigned int seed) {
  return qHash(*key, seed);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_QChar(const QChar* key) {
  return qHash(*key);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_QChar_unsigned_int(const QChar* key, unsigned int seed) {
  return qHash(*key, seed);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_QLatin1String(const QLatin1String* key) {
  return qHash(*key);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_QLatin1String_unsigned_int(const QLatin1String* key, unsigned int seed) {
  return qHash(*key, seed);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_QString(const QString* key) {
  return qHash(*key);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_QStringRef(const QStringRef* key) {
  return qHash(*key);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_QStringRef_unsigned_int(const QStringRef* key, unsigned int seed) {
  return qHash(*key, seed);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_QString_unsigned_int(const QString* key, unsigned int seed) {
  return qHash(*key, seed);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_char(char key) {
  return qHash(key);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_char_unsigned_int(char key, unsigned int seed) {
  return qHash(key, seed);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_double(double key) {
  return qHash(key);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_double_unsigned_int(double key, unsigned int seed) {
  return qHash(key, seed);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_float(float key) {
  return qHash(key);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_float_unsigned_int(float key, unsigned int seed) {
  return qHash(key, seed);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_int(int key) {
  return qHash(key);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_int_unsigned_int(int key, unsigned int seed) {
  return qHash(key, seed);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_long(long key) {
  return qHash(key);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_long_unsigned_int(long key, unsigned int seed) {
  return qHash(key, seed);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_qint64(qint64 key) {
  return qHash(key);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_qint64_unsigned_int(qint64 key, unsigned int seed) {
  return qHash(key, seed);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_quint64(quint64 key) {
  return qHash(key);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_quint64_unsigned_int(quint64 key, unsigned int seed) {
  return qHash(key, seed);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_short(short key) {
  return qHash(key);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_short_unsigned_int(short key, unsigned int seed) {
  return qHash(key, seed);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_signed_char(signed char key) {
  return qHash(key);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_signed_char_unsigned_int(signed char key, unsigned int seed) {
  return qHash(key, seed);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_unsigned_char(unsigned char key) {
  return qHash(key);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_unsigned_char_unsigned_int(unsigned char key, unsigned int seed) {
  return qHash(key, seed);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_unsigned_int(unsigned int key) {
  return qHash(key);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_unsigned_int_unsigned_int(unsigned int key, unsigned int seed) {
  return qHash(key, seed);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_unsigned_long(unsigned long key) {
  return qHash(key);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_unsigned_long_unsigned_int(unsigned long key, unsigned int seed) {
  return qHash(key, seed);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_unsigned_short(unsigned short key) {
  return qHash(key);
}

unsigned int qt_core_c_QHashFunctions_G_qHash_unsigned_short_unsigned_int(unsigned short key, unsigned int seed) {
  return qHash(key, seed);
}

void qt_core_c_QHashFunctions_G_qSetGlobalQHashSeed(int newSeed) {
  qSetGlobalQHashSeed(newSeed);
}


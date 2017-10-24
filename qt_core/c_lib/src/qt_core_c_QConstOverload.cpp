#include "qt_core_c_QConstOverload.h"

int qt_core_c_QConstOverload_G_qEnvironmentVariableIntValue_varName(const char* varName) {
  return qEnvironmentVariableIntValue(varName);
}

int qt_core_c_QConstOverload_G_qEnvironmentVariableIntValue_varName_ok(const char* varName, bool* ok) {
  return qEnvironmentVariableIntValue(varName, ok);
}

bool qt_core_c_QConstOverload_G_qEnvironmentVariableIsEmpty(const char* varName) {
  return qEnvironmentVariableIsEmpty(varName);
}

bool qt_core_c_QConstOverload_G_qEnvironmentVariableIsSet(const char* varName) {
  return qEnvironmentVariableIsSet(varName);
}

void qt_core_c_QConstOverload_G_qFreeAligned(void* ptr) {
  qFreeAligned(ptr);
}

bool qt_core_c_QConstOverload_G_qFuzzyCompare_double_double(double p1, double p2) {
  return qFuzzyCompare(p1, p2);
}

bool qt_core_c_QConstOverload_G_qFuzzyCompare_float_float(float p1, float p2) {
  return qFuzzyCompare(p1, p2);
}

bool qt_core_c_QConstOverload_G_qFuzzyIsNull_d(double d) {
  return qFuzzyIsNull(d);
}

bool qt_core_c_QConstOverload_G_qFuzzyIsNull_f(float f) {
  return qFuzzyIsNull(f);
}

int qt_core_c_QConstOverload_G_qIntCast_double(double f) {
  return qIntCast(f);
}

int qt_core_c_QConstOverload_G_qIntCast_float(float f) {
  return qIntCast(f);
}

bool qt_core_c_QConstOverload_G_qIsNull_d(double d) {
  return qIsNull(d);
}

bool qt_core_c_QConstOverload_G_qIsNull_f(float f) {
  return qIsNull(f);
}

void* qt_core_c_QConstOverload_G_qMallocAligned(unsigned long size, unsigned long alignment) {
  return qMallocAligned(size, alignment);
}

void* qt_core_c_QConstOverload_G_qReallocAligned(void* ptr, unsigned long size, unsigned long oldsize, unsigned long alignment) {
  return qReallocAligned(ptr, size, oldsize, alignment);
}

qint64 qt_core_c_QConstOverload_G_qRound64_double(double d) {
  return qRound64(d);
}

qint64 qt_core_c_QConstOverload_G_qRound64_float(float d) {
  return qRound64(d);
}

int qt_core_c_QConstOverload_G_qRound_double(double d) {
  return qRound(d);
}

int qt_core_c_QConstOverload_G_qRound_float(float d) {
  return qRound(d);
}

void qt_core_c_QConstOverload_G_qTerminate() {
  qTerminate();
}

const char* qt_core_c_QConstOverload_G_qVersion() {
  return qVersion();
}

void qt_core_c_QConstOverload_G_qgetenv_to_output(const char* varName, QByteArray* output) {
  new(output) QByteArray(qgetenv(varName));
}

bool qt_core_c_QConstOverload_G_qputenv(const char* varName, const QByteArray* value) {
  return qputenv(varName, *value);
}

int qt_core_c_QConstOverload_G_qrand() {
  return qrand();
}

void qt_core_c_QConstOverload_G_qsrand(unsigned int seed) {
  qsrand(seed);
}

void qt_core_c_QConstOverload_G_qtTrId_to_output_id(const char* id, QString* output) {
  new(output) QString(qtTrId(id));
}

void qt_core_c_QConstOverload_G_qtTrId_to_output_id_n(const char* id, int n, QString* output) {
  new(output) QString(qtTrId(id, n));
}

void qt_core_c_QConstOverload_G_qt_error_string_to_output_errorCode(int errorCode, QString* output) {
  new(output) QString(qt_error_string(errorCode));
}

void qt_core_c_QConstOverload_G_qt_error_string_to_output_no_args(QString* output) {
  new(output) QString(qt_error_string());
}

bool qt_core_c_QConstOverload_G_qunsetenv(const char* varName) {
  return qunsetenv(varName);
}

void qt_core_c_QMacAutoReleasePool_constructor(QMacAutoReleasePool* output) {
  new(output) QMacAutoReleasePool();
}

void qt_core_c_QMacAutoReleasePool_destructor(QMacAutoReleasePool* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}


#ifndef QT_CORE_C_QCONSTOVERLOAD_H
#define QT_CORE_C_QCONSTOVERLOAD_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT int qt_core_c_QConstOverload_G_qEnvironmentVariableIntValue_varName(const char* varName);
QT_CORE_C_EXPORT int qt_core_c_QConstOverload_G_qEnvironmentVariableIntValue_varName_ok(const char* varName, bool* ok);
QT_CORE_C_EXPORT bool qt_core_c_QConstOverload_G_qEnvironmentVariableIsEmpty(const char* varName);
QT_CORE_C_EXPORT bool qt_core_c_QConstOverload_G_qEnvironmentVariableIsSet(const char* varName);
QT_CORE_C_EXPORT void qt_core_c_QConstOverload_G_qFreeAligned(void* ptr);
QT_CORE_C_EXPORT bool qt_core_c_QConstOverload_G_qFuzzyCompare_double_double(double p1, double p2);
QT_CORE_C_EXPORT bool qt_core_c_QConstOverload_G_qFuzzyCompare_float_float(float p1, float p2);
QT_CORE_C_EXPORT bool qt_core_c_QConstOverload_G_qFuzzyIsNull_d(double d);
QT_CORE_C_EXPORT bool qt_core_c_QConstOverload_G_qFuzzyIsNull_f(float f);
QT_CORE_C_EXPORT int qt_core_c_QConstOverload_G_qIntCast_double(double f);
QT_CORE_C_EXPORT int qt_core_c_QConstOverload_G_qIntCast_float(float f);
QT_CORE_C_EXPORT bool qt_core_c_QConstOverload_G_qIsNull_d(double d);
QT_CORE_C_EXPORT bool qt_core_c_QConstOverload_G_qIsNull_f(float f);
QT_CORE_C_EXPORT void* qt_core_c_QConstOverload_G_qMallocAligned(unsigned long size, unsigned long alignment);
QT_CORE_C_EXPORT void* qt_core_c_QConstOverload_G_qReallocAligned(void* ptr, unsigned long size, unsigned long oldsize, unsigned long alignment);
QT_CORE_C_EXPORT qint64 qt_core_c_QConstOverload_G_qRound64_double(double d);
QT_CORE_C_EXPORT qint64 qt_core_c_QConstOverload_G_qRound64_float(float d);
QT_CORE_C_EXPORT int qt_core_c_QConstOverload_G_qRound_double(double d);
QT_CORE_C_EXPORT int qt_core_c_QConstOverload_G_qRound_float(float d);
QT_CORE_C_EXPORT void qt_core_c_QConstOverload_G_qTerminate();
QT_CORE_C_EXPORT const char* qt_core_c_QConstOverload_G_qVersion();
QT_CORE_C_EXPORT void qt_core_c_QConstOverload_G_qgetenv_to_output(const char* varName, QByteArray* output);
QT_CORE_C_EXPORT bool qt_core_c_QConstOverload_G_qputenv(const char* varName, const QByteArray* value);
QT_CORE_C_EXPORT int qt_core_c_QConstOverload_G_qrand();
QT_CORE_C_EXPORT void qt_core_c_QConstOverload_G_qsrand(unsigned int seed);
QT_CORE_C_EXPORT void qt_core_c_QConstOverload_G_qtTrId_to_output_id(const char* id, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QConstOverload_G_qtTrId_to_output_id_n(const char* id, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QConstOverload_G_qt_error_string_to_output_errorCode(int errorCode, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QConstOverload_G_qt_error_string_to_output_no_args(QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QConstOverload_G_qunsetenv(const char* varName);
QT_CORE_C_EXPORT void qt_core_c_QMacAutoReleasePool_constructor(QMacAutoReleasePool* output);
QT_CORE_C_EXPORT void qt_core_c_QMacAutoReleasePool_destructor(QMacAutoReleasePool* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QCONSTOVERLOAD_H

#ifndef QT_CORE_C_QPROCESS_H
#define QT_CORE_C_QPROCESS_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QProcess* qt_core_c_QProcess_G_dynamic_cast_QProcess_ptr_QIODevice(QIODevice* ptr);
QT_CORE_C_EXPORT QProcess* qt_core_c_QProcess_G_dynamic_cast_QProcess_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QIODevice* qt_core_c_QProcess_G_static_cast_QIODevice_ptr(QProcess* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QProcess_G_static_cast_QObject_ptr(QProcess* ptr);
QT_CORE_C_EXPORT QProcess* qt_core_c_QProcess_G_static_cast_QProcess_ptr_QIODevice(QIODevice* ptr);
QT_CORE_C_EXPORT QProcess* qt_core_c_QProcess_G_static_cast_QProcess_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT void qt_core_c_QProcess_G_swap(QProcessEnvironment* value1, QProcessEnvironment* value2);
QT_CORE_C_EXPORT void qt_core_c_QProcess_arguments_to_output(const QProcess* this_ptr, QStringList* output);
QT_CORE_C_EXPORT bool qt_core_c_QProcess_atEnd(const QProcess* this_ptr);
QT_CORE_C_EXPORT qint64 qt_core_c_QProcess_bytesAvailable(const QProcess* this_ptr);
QT_CORE_C_EXPORT qint64 qt_core_c_QProcess_bytesToWrite(const QProcess* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QProcess_canReadLine(const QProcess* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QProcess_close(QProcess* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QProcess_closeReadChannel(QProcess* this_ptr, QProcess::ProcessChannel channel);
QT_CORE_C_EXPORT void qt_core_c_QProcess_closeWriteChannel(QProcess* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QProcess_delete(QProcess* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QProcess_environment_to_output(const QProcess* this_ptr, QStringList* output);
QT_CORE_C_EXPORT int qt_core_c_QProcess_execute_command(const QString* command);
QT_CORE_C_EXPORT int qt_core_c_QProcess_execute_program_arguments(const QString* program, const QStringList* arguments);
QT_CORE_C_EXPORT int qt_core_c_QProcess_exitCode(const QProcess* this_ptr);
QT_CORE_C_EXPORT QProcess::InputChannelMode qt_core_c_QProcess_inputChannelMode(const QProcess* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QProcess_isSequential(const QProcess* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QProcess_kill(QProcess* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QProcess_metaObject(const QProcess* this_ptr);
QT_CORE_C_EXPORT QProcess* qt_core_c_QProcess_new_no_args();
QT_CORE_C_EXPORT QProcess* qt_core_c_QProcess_new_parent(QObject* parent);
QT_CORE_C_EXPORT void qt_core_c_QProcess_nullDevice_to_output(QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QProcess_open_mode(QProcess* this_ptr, unsigned int mode);
QT_CORE_C_EXPORT bool qt_core_c_QProcess_open_no_args(QProcess* this_ptr);
QT_CORE_C_EXPORT QProcess::ProcessChannelMode qt_core_c_QProcess_processChannelMode(const QProcess* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QProcess_processEnvironment_to_output(const QProcess* this_ptr, QProcessEnvironment* output);
QT_CORE_C_EXPORT qint64 qt_core_c_QProcess_processId(const QProcess* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QProcess_program_to_output(const QProcess* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QProcess_readAllStandardError_to_output(QProcess* this_ptr, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QProcess_readAllStandardOutput_to_output(QProcess* this_ptr, QByteArray* output);
QT_CORE_C_EXPORT QProcess::ProcessChannel qt_core_c_QProcess_readChannel(const QProcess* this_ptr);
QT_CORE_C_EXPORT QProcess::ProcessChannelMode qt_core_c_QProcess_readChannelMode(const QProcess* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QProcess_setArguments(QProcess* this_ptr, const QStringList* arguments);
QT_CORE_C_EXPORT void qt_core_c_QProcess_setEnvironment(QProcess* this_ptr, const QStringList* environment);
QT_CORE_C_EXPORT void qt_core_c_QProcess_setInputChannelMode(QProcess* this_ptr, QProcess::InputChannelMode mode);
QT_CORE_C_EXPORT void qt_core_c_QProcess_setProcessChannelMode(QProcess* this_ptr, QProcess::ProcessChannelMode mode);
QT_CORE_C_EXPORT void qt_core_c_QProcess_setProcessEnvironment(QProcess* this_ptr, const QProcessEnvironment* environment);
QT_CORE_C_EXPORT void qt_core_c_QProcess_setProgram(QProcess* this_ptr, const QString* program);
QT_CORE_C_EXPORT void qt_core_c_QProcess_setReadChannel(QProcess* this_ptr, QProcess::ProcessChannel channel);
QT_CORE_C_EXPORT void qt_core_c_QProcess_setReadChannelMode(QProcess* this_ptr, QProcess::ProcessChannelMode mode);
QT_CORE_C_EXPORT void qt_core_c_QProcess_setStandardErrorFile_fileName(QProcess* this_ptr, const QString* fileName);
QT_CORE_C_EXPORT void qt_core_c_QProcess_setStandardErrorFile_fileName_mode(QProcess* this_ptr, const QString* fileName, unsigned int mode);
QT_CORE_C_EXPORT void qt_core_c_QProcess_setStandardInputFile(QProcess* this_ptr, const QString* fileName);
QT_CORE_C_EXPORT void qt_core_c_QProcess_setStandardOutputFile_fileName(QProcess* this_ptr, const QString* fileName);
QT_CORE_C_EXPORT void qt_core_c_QProcess_setStandardOutputFile_fileName_mode(QProcess* this_ptr, const QString* fileName, unsigned int mode);
QT_CORE_C_EXPORT void qt_core_c_QProcess_setStandardOutputProcess(QProcess* this_ptr, QProcess* destination);
QT_CORE_C_EXPORT void qt_core_c_QProcess_setWorkingDirectory(QProcess* this_ptr, const QString* dir);
QT_CORE_C_EXPORT bool qt_core_c_QProcess_startDetached_command(const QString* command);
QT_CORE_C_EXPORT bool qt_core_c_QProcess_startDetached_program_arguments(const QString* program, const QStringList* arguments);
QT_CORE_C_EXPORT bool qt_core_c_QProcess_startDetached_program_arguments_workingDirectory(const QString* program, const QStringList* arguments, const QString* workingDirectory);
QT_CORE_C_EXPORT bool qt_core_c_QProcess_startDetached_program_arguments_workingDirectory_pid(const QString* program, const QStringList* arguments, const QString* workingDirectory, qint64* pid);
QT_CORE_C_EXPORT void qt_core_c_QProcess_start_command(QProcess* this_ptr, const QString* command);
QT_CORE_C_EXPORT void qt_core_c_QProcess_start_command_mode(QProcess* this_ptr, const QString* command, unsigned int mode);
QT_CORE_C_EXPORT void qt_core_c_QProcess_start_mode(QProcess* this_ptr, unsigned int mode);
QT_CORE_C_EXPORT void qt_core_c_QProcess_start_no_args(QProcess* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QProcess_start_program_arguments(QProcess* this_ptr, const QString* program, const QStringList* arguments);
QT_CORE_C_EXPORT void qt_core_c_QProcess_start_program_arguments_mode(QProcess* this_ptr, const QString* program, const QStringList* arguments, unsigned int mode);
QT_CORE_C_EXPORT void qt_core_c_QProcess_systemEnvironment_to_output(QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QProcess_terminate(QProcess* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QProcess_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QProcess_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QProcess_waitForBytesWritten_msecs(QProcess* this_ptr, int msecs);
QT_CORE_C_EXPORT bool qt_core_c_QProcess_waitForBytesWritten_no_args(QProcess* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QProcess_waitForFinished_msecs(QProcess* this_ptr, int msecs);
QT_CORE_C_EXPORT bool qt_core_c_QProcess_waitForFinished_no_args(QProcess* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QProcess_waitForReadyRead_msecs(QProcess* this_ptr, int msecs);
QT_CORE_C_EXPORT bool qt_core_c_QProcess_waitForReadyRead_no_args(QProcess* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QProcess_waitForStarted_msecs(QProcess* this_ptr, int msecs);
QT_CORE_C_EXPORT bool qt_core_c_QProcess_waitForStarted_no_args(QProcess* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QProcess_workingDirectory_to_output(const QProcess* this_ptr, QString* output);

} // extern "C"

#endif // QT_CORE_C_QPROCESS_H

#include "qt_core_c_QProcess.h"

QProcess* qt_core_c_QProcess_G_dynamic_cast_QProcess_ptr_QIODevice(QIODevice* ptr) {
  return dynamic_cast<QProcess*>(ptr);
}

QProcess* qt_core_c_QProcess_G_dynamic_cast_QProcess_ptr_QObject(QObject* ptr) {
  return dynamic_cast<QProcess*>(ptr);
}

QIODevice* qt_core_c_QProcess_G_static_cast_QIODevice_ptr(QProcess* ptr) {
  return static_cast<QIODevice*>(ptr);
}

QObject* qt_core_c_QProcess_G_static_cast_QObject_ptr(QProcess* ptr) {
  return static_cast<QObject*>(ptr);
}

QProcess* qt_core_c_QProcess_G_static_cast_QProcess_ptr_QIODevice(QIODevice* ptr) {
  return static_cast<QProcess*>(ptr);
}

QProcess* qt_core_c_QProcess_G_static_cast_QProcess_ptr_QObject(QObject* ptr) {
  return static_cast<QProcess*>(ptr);
}

void qt_core_c_QProcess_G_swap(QProcessEnvironment* value1, QProcessEnvironment* value2) {
  swap(*value1, *value2);
}

void qt_core_c_QProcess_arguments_to_output(const QProcess* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->arguments());
}

bool qt_core_c_QProcess_atEnd(const QProcess* this_ptr) {
  return this_ptr->atEnd();
}

qint64 qt_core_c_QProcess_bytesAvailable(const QProcess* this_ptr) {
  return this_ptr->bytesAvailable();
}

qint64 qt_core_c_QProcess_bytesToWrite(const QProcess* this_ptr) {
  return this_ptr->bytesToWrite();
}

bool qt_core_c_QProcess_canReadLine(const QProcess* this_ptr) {
  return this_ptr->canReadLine();
}

void qt_core_c_QProcess_close(QProcess* this_ptr) {
  this_ptr->close();
}

void qt_core_c_QProcess_closeReadChannel(QProcess* this_ptr, QProcess::ProcessChannel channel) {
  this_ptr->closeReadChannel(channel);
}

void qt_core_c_QProcess_closeWriteChannel(QProcess* this_ptr) {
  this_ptr->closeWriteChannel();
}

void qt_core_c_QProcess_delete(QProcess* this_ptr) {
  delete this_ptr;
}

void qt_core_c_QProcess_environment_to_output(const QProcess* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->environment());
}

int qt_core_c_QProcess_execute_command(const QString* command) {
  return QProcess::execute(*command);
}

int qt_core_c_QProcess_execute_program_arguments(const QString* program, const QStringList* arguments) {
  return QProcess::execute(*program, *arguments);
}

int qt_core_c_QProcess_exitCode(const QProcess* this_ptr) {
  return this_ptr->exitCode();
}

QProcess::InputChannelMode qt_core_c_QProcess_inputChannelMode(const QProcess* this_ptr) {
  return this_ptr->inputChannelMode();
}

bool qt_core_c_QProcess_isSequential(const QProcess* this_ptr) {
  return this_ptr->isSequential();
}

void qt_core_c_QProcess_kill(QProcess* this_ptr) {
  this_ptr->kill();
}

const QMetaObject* qt_core_c_QProcess_metaObject(const QProcess* this_ptr) {
  return this_ptr->metaObject();
}

QProcess* qt_core_c_QProcess_new_no_args() {
  return new QProcess();
}

QProcess* qt_core_c_QProcess_new_parent(QObject* parent) {
  return new QProcess(parent);
}

void qt_core_c_QProcess_nullDevice_to_output(QString* output) {
  new(output) QString(QProcess::nullDevice());
}

bool qt_core_c_QProcess_open_mode(QProcess* this_ptr, unsigned int mode) {
  return this_ptr->open(QFlags< QIODevice::OpenModeFlag >(mode));
}

bool qt_core_c_QProcess_open_no_args(QProcess* this_ptr) {
  return this_ptr->open();
}

QProcess::ProcessChannelMode qt_core_c_QProcess_processChannelMode(const QProcess* this_ptr) {
  return this_ptr->processChannelMode();
}

void qt_core_c_QProcess_processEnvironment_to_output(const QProcess* this_ptr, QProcessEnvironment* output) {
  new(output) QProcessEnvironment(this_ptr->processEnvironment());
}

qint64 qt_core_c_QProcess_processId(const QProcess* this_ptr) {
  return this_ptr->processId();
}

void qt_core_c_QProcess_program_to_output(const QProcess* this_ptr, QString* output) {
  new(output) QString(this_ptr->program());
}

void qt_core_c_QProcess_readAllStandardError_to_output(QProcess* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->readAllStandardError());
}

void qt_core_c_QProcess_readAllStandardOutput_to_output(QProcess* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->readAllStandardOutput());
}

QProcess::ProcessChannel qt_core_c_QProcess_readChannel(const QProcess* this_ptr) {
  return this_ptr->readChannel();
}

QProcess::ProcessChannelMode qt_core_c_QProcess_readChannelMode(const QProcess* this_ptr) {
  return this_ptr->readChannelMode();
}

void qt_core_c_QProcess_setArguments(QProcess* this_ptr, const QStringList* arguments) {
  this_ptr->setArguments(*arguments);
}

void qt_core_c_QProcess_setEnvironment(QProcess* this_ptr, const QStringList* environment) {
  this_ptr->setEnvironment(*environment);
}

void qt_core_c_QProcess_setInputChannelMode(QProcess* this_ptr, QProcess::InputChannelMode mode) {
  this_ptr->setInputChannelMode(mode);
}

void qt_core_c_QProcess_setProcessChannelMode(QProcess* this_ptr, QProcess::ProcessChannelMode mode) {
  this_ptr->setProcessChannelMode(mode);
}

void qt_core_c_QProcess_setProcessEnvironment(QProcess* this_ptr, const QProcessEnvironment* environment) {
  this_ptr->setProcessEnvironment(*environment);
}

void qt_core_c_QProcess_setProgram(QProcess* this_ptr, const QString* program) {
  this_ptr->setProgram(*program);
}

void qt_core_c_QProcess_setReadChannel(QProcess* this_ptr, QProcess::ProcessChannel channel) {
  this_ptr->setReadChannel(channel);
}

void qt_core_c_QProcess_setReadChannelMode(QProcess* this_ptr, QProcess::ProcessChannelMode mode) {
  this_ptr->setReadChannelMode(mode);
}

void qt_core_c_QProcess_setStandardErrorFile_fileName(QProcess* this_ptr, const QString* fileName) {
  this_ptr->setStandardErrorFile(*fileName);
}

void qt_core_c_QProcess_setStandardErrorFile_fileName_mode(QProcess* this_ptr, const QString* fileName, unsigned int mode) {
  this_ptr->setStandardErrorFile(*fileName, QFlags< QIODevice::OpenModeFlag >(mode));
}

void qt_core_c_QProcess_setStandardInputFile(QProcess* this_ptr, const QString* fileName) {
  this_ptr->setStandardInputFile(*fileName);
}

void qt_core_c_QProcess_setStandardOutputFile_fileName(QProcess* this_ptr, const QString* fileName) {
  this_ptr->setStandardOutputFile(*fileName);
}

void qt_core_c_QProcess_setStandardOutputFile_fileName_mode(QProcess* this_ptr, const QString* fileName, unsigned int mode) {
  this_ptr->setStandardOutputFile(*fileName, QFlags< QIODevice::OpenModeFlag >(mode));
}

void qt_core_c_QProcess_setStandardOutputProcess(QProcess* this_ptr, QProcess* destination) {
  this_ptr->setStandardOutputProcess(destination);
}

void qt_core_c_QProcess_setWorkingDirectory(QProcess* this_ptr, const QString* dir) {
  this_ptr->setWorkingDirectory(*dir);
}

bool qt_core_c_QProcess_startDetached_command(const QString* command) {
  return QProcess::startDetached(*command);
}

bool qt_core_c_QProcess_startDetached_program_arguments(const QString* program, const QStringList* arguments) {
  return QProcess::startDetached(*program, *arguments);
}

bool qt_core_c_QProcess_startDetached_program_arguments_workingDirectory(const QString* program, const QStringList* arguments, const QString* workingDirectory) {
  return QProcess::startDetached(*program, *arguments, *workingDirectory);
}

bool qt_core_c_QProcess_startDetached_program_arguments_workingDirectory_pid(const QString* program, const QStringList* arguments, const QString* workingDirectory, qint64* pid) {
  return QProcess::startDetached(*program, *arguments, *workingDirectory, pid);
}

void qt_core_c_QProcess_start_command(QProcess* this_ptr, const QString* command) {
  this_ptr->start(*command);
}

void qt_core_c_QProcess_start_command_mode(QProcess* this_ptr, const QString* command, unsigned int mode) {
  this_ptr->start(*command, QFlags< QIODevice::OpenModeFlag >(mode));
}

void qt_core_c_QProcess_start_mode(QProcess* this_ptr, unsigned int mode) {
  this_ptr->start(QFlags< QIODevice::OpenModeFlag >(mode));
}

void qt_core_c_QProcess_start_no_args(QProcess* this_ptr) {
  this_ptr->start();
}

void qt_core_c_QProcess_start_program_arguments(QProcess* this_ptr, const QString* program, const QStringList* arguments) {
  this_ptr->start(*program, *arguments);
}

void qt_core_c_QProcess_start_program_arguments_mode(QProcess* this_ptr, const QString* program, const QStringList* arguments, unsigned int mode) {
  this_ptr->start(*program, *arguments, QFlags< QIODevice::OpenModeFlag >(mode));
}

void qt_core_c_QProcess_systemEnvironment_to_output(QStringList* output) {
  new(output) QStringList(QProcess::systemEnvironment());
}

void qt_core_c_QProcess_terminate(QProcess* this_ptr) {
  this_ptr->terminate();
}

void qt_core_c_QProcess_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QProcess::trUtf8(s, c, n));
}

void qt_core_c_QProcess_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QProcess::tr(s, c, n));
}

bool qt_core_c_QProcess_waitForBytesWritten_msecs(QProcess* this_ptr, int msecs) {
  return this_ptr->waitForBytesWritten(msecs);
}

bool qt_core_c_QProcess_waitForBytesWritten_no_args(QProcess* this_ptr) {
  return this_ptr->waitForBytesWritten();
}

bool qt_core_c_QProcess_waitForFinished_msecs(QProcess* this_ptr, int msecs) {
  return this_ptr->waitForFinished(msecs);
}

bool qt_core_c_QProcess_waitForFinished_no_args(QProcess* this_ptr) {
  return this_ptr->waitForFinished();
}

bool qt_core_c_QProcess_waitForReadyRead_msecs(QProcess* this_ptr, int msecs) {
  return this_ptr->waitForReadyRead(msecs);
}

bool qt_core_c_QProcess_waitForReadyRead_no_args(QProcess* this_ptr) {
  return this_ptr->waitForReadyRead();
}

bool qt_core_c_QProcess_waitForStarted_msecs(QProcess* this_ptr, int msecs) {
  return this_ptr->waitForStarted(msecs);
}

bool qt_core_c_QProcess_waitForStarted_no_args(QProcess* this_ptr) {
  return this_ptr->waitForStarted();
}

void qt_core_c_QProcess_workingDirectory_to_output(const QProcess* this_ptr, QString* output) {
  new(output) QString(this_ptr->workingDirectory());
}


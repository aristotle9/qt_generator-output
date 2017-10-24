#include "qt_core_c_QSysInfo.h"

void qt_core_c_QSysInfo_buildAbi_to_output(QString* output) {
  new(output) QString(QSysInfo::buildAbi());
}

void qt_core_c_QSysInfo_buildCpuArchitecture_to_output(QString* output) {
  new(output) QString(QSysInfo::buildCpuArchitecture());
}

void qt_core_c_QSysInfo_currentCpuArchitecture_to_output(QString* output) {
  new(output) QString(QSysInfo::currentCpuArchitecture());
}

void qt_core_c_QSysInfo_delete(QSysInfo* this_ptr) {
  delete this_ptr;
}

void qt_core_c_QSysInfo_kernelType_to_output(QString* output) {
  new(output) QString(QSysInfo::kernelType());
}

void qt_core_c_QSysInfo_kernelVersion_to_output(QString* output) {
  new(output) QString(QSysInfo::kernelVersion());
}

QSysInfo::MacVersion qt_core_c_QSysInfo_macVersion() {
  return QSysInfo::macVersion();
}

void qt_core_c_QSysInfo_machineHostName_to_output(QString* output) {
  new(output) QString(QSysInfo::machineHostName());
}

void qt_core_c_QSysInfo_prettyProductName_to_output(QString* output) {
  new(output) QString(QSysInfo::prettyProductName());
}

void qt_core_c_QSysInfo_productType_to_output(QString* output) {
  new(output) QString(QSysInfo::productType());
}

void qt_core_c_QSysInfo_productVersion_to_output(QString* output) {
  new(output) QString(QSysInfo::productVersion());
}

QSysInfo::WinVersion qt_core_c_QSysInfo_windowsVersion() {
  return QSysInfo::windowsVersion();
}


#ifndef QT_CORE_C_QSYSINFO_H
#define QT_CORE_C_QSYSINFO_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QSysInfo_buildAbi_to_output(QString* output);
QT_CORE_C_EXPORT void qt_core_c_QSysInfo_buildCpuArchitecture_to_output(QString* output);
QT_CORE_C_EXPORT void qt_core_c_QSysInfo_currentCpuArchitecture_to_output(QString* output);
QT_CORE_C_EXPORT void qt_core_c_QSysInfo_delete(QSysInfo* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSysInfo_kernelType_to_output(QString* output);
QT_CORE_C_EXPORT void qt_core_c_QSysInfo_kernelVersion_to_output(QString* output);
QT_CORE_C_EXPORT QSysInfo::MacVersion qt_core_c_QSysInfo_macVersion();
QT_CORE_C_EXPORT void qt_core_c_QSysInfo_machineHostName_to_output(QString* output);
QT_CORE_C_EXPORT void qt_core_c_QSysInfo_prettyProductName_to_output(QString* output);
QT_CORE_C_EXPORT void qt_core_c_QSysInfo_productType_to_output(QString* output);
QT_CORE_C_EXPORT void qt_core_c_QSysInfo_productVersion_to_output(QString* output);
QT_CORE_C_EXPORT QSysInfo::WinVersion qt_core_c_QSysInfo_windowsVersion();

} // extern "C"

#endif // QT_CORE_C_QSYSINFO_H

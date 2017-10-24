#ifndef QT_CORE_C_QLIBRARYINFO_H
#define QT_CORE_C_QLIBRARYINFO_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT const char* qt_core_c_QLibraryInfo_build();
QT_CORE_C_EXPORT void qt_core_c_QLibraryInfo_buildDate_to_output(QDate* output);
QT_CORE_C_EXPORT void qt_core_c_QLibraryInfo_delete(QLibraryInfo* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QLibraryInfo_isDebugBuild();
QT_CORE_C_EXPORT void qt_core_c_QLibraryInfo_licensedProducts_to_output(QString* output);
QT_CORE_C_EXPORT void qt_core_c_QLibraryInfo_licensee_to_output(QString* output);
QT_CORE_C_EXPORT void qt_core_c_QLibraryInfo_location_to_output(QLibraryInfo::LibraryLocation arg1, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QLibraryInfo_platformPluginArguments_to_output(const QString* platformName, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QLibraryInfo_version_to_output(QVersionNumber* output);

} // extern "C"

#endif // QT_CORE_C_QLIBRARYINFO_H

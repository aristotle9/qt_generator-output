#ifndef QT_CORE_C_QSTANDARDPATHS_H
#define QT_CORE_C_QSTANDARDPATHS_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QStandardPaths_displayName_to_output(QStandardPaths::StandardLocation type, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QStandardPaths_enableTestMode(bool testMode);
QT_CORE_C_EXPORT void qt_core_c_QStandardPaths_findExecutable_to_output_executableName(const QString* executableName, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QStandardPaths_findExecutable_to_output_executableName_paths(const QString* executableName, const QStringList* paths, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QStandardPaths_isTestModeEnabled();
QT_CORE_C_EXPORT void qt_core_c_QStandardPaths_locateAll_to_output_type_fileName(QStandardPaths::StandardLocation type, const QString* fileName, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QStandardPaths_locateAll_to_output_type_fileName_options(QStandardPaths::StandardLocation type, const QString* fileName, unsigned int options, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QStandardPaths_locate_to_output_type_fileName(QStandardPaths::StandardLocation type, const QString* fileName, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QStandardPaths_locate_to_output_type_fileName_options(QStandardPaths::StandardLocation type, const QString* fileName, unsigned int options, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QStandardPaths_setTestModeEnabled(bool testMode);
QT_CORE_C_EXPORT void qt_core_c_QStandardPaths_standardLocations_to_output(QStandardPaths::StandardLocation type, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QStandardPaths_writableLocation_to_output(QStandardPaths::StandardLocation type, QString* output);

} // extern "C"

#endif // QT_CORE_C_QSTANDARDPATHS_H

#include "qt_core_c_QStandardPaths.h"

void qt_core_c_QStandardPaths_displayName_to_output(QStandardPaths::StandardLocation type, QString* output) {
  new(output) QString(QStandardPaths::displayName(type));
}

void qt_core_c_QStandardPaths_enableTestMode(bool testMode) {
  QStandardPaths::enableTestMode(testMode);
}

void qt_core_c_QStandardPaths_findExecutable_to_output_executableName(const QString* executableName, QString* output) {
  new(output) QString(QStandardPaths::findExecutable(*executableName));
}

void qt_core_c_QStandardPaths_findExecutable_to_output_executableName_paths(const QString* executableName, const QStringList* paths, QString* output) {
  new(output) QString(QStandardPaths::findExecutable(*executableName, *paths));
}

bool qt_core_c_QStandardPaths_isTestModeEnabled() {
  return QStandardPaths::isTestModeEnabled();
}

void qt_core_c_QStandardPaths_locateAll_to_output_type_fileName(QStandardPaths::StandardLocation type, const QString* fileName, QStringList* output) {
  new(output) QStringList(QStandardPaths::locateAll(type, *fileName));
}

void qt_core_c_QStandardPaths_locateAll_to_output_type_fileName_options(QStandardPaths::StandardLocation type, const QString* fileName, unsigned int options, QStringList* output) {
  new(output) QStringList(QStandardPaths::locateAll(type, *fileName, QFlags< QStandardPaths::LocateOption >(options)));
}

void qt_core_c_QStandardPaths_locate_to_output_type_fileName(QStandardPaths::StandardLocation type, const QString* fileName, QString* output) {
  new(output) QString(QStandardPaths::locate(type, *fileName));
}

void qt_core_c_QStandardPaths_locate_to_output_type_fileName_options(QStandardPaths::StandardLocation type, const QString* fileName, unsigned int options, QString* output) {
  new(output) QString(QStandardPaths::locate(type, *fileName, QFlags< QStandardPaths::LocateOption >(options)));
}

void qt_core_c_QStandardPaths_setTestModeEnabled(bool testMode) {
  QStandardPaths::setTestModeEnabled(testMode);
}

void qt_core_c_QStandardPaths_standardLocations_to_output(QStandardPaths::StandardLocation type, QStringList* output) {
  new(output) QStringList(QStandardPaths::standardLocations(type));
}

void qt_core_c_QStandardPaths_writableLocation_to_output(QStandardPaths::StandardLocation type, QString* output) {
  new(output) QString(QStandardPaths::writableLocation(type));
}


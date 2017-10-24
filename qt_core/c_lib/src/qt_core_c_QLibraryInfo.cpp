#include "qt_core_c_QLibraryInfo.h"

const char* qt_core_c_QLibraryInfo_build() {
  return QLibraryInfo::build();
}

void qt_core_c_QLibraryInfo_buildDate_to_output(QDate* output) {
  new(output) QDate(QLibraryInfo::buildDate());
}

void qt_core_c_QLibraryInfo_delete(QLibraryInfo* this_ptr) {
  delete this_ptr;
}

bool qt_core_c_QLibraryInfo_isDebugBuild() {
  return QLibraryInfo::isDebugBuild();
}

void qt_core_c_QLibraryInfo_licensedProducts_to_output(QString* output) {
  new(output) QString(QLibraryInfo::licensedProducts());
}

void qt_core_c_QLibraryInfo_licensee_to_output(QString* output) {
  new(output) QString(QLibraryInfo::licensee());
}

void qt_core_c_QLibraryInfo_location_to_output(QLibraryInfo::LibraryLocation arg1, QString* output) {
  new(output) QString(QLibraryInfo::location(arg1));
}

void qt_core_c_QLibraryInfo_platformPluginArguments_to_output(const QString* platformName, QStringList* output) {
  new(output) QStringList(QLibraryInfo::platformPluginArguments(*platformName));
}

void qt_core_c_QLibraryInfo_version_to_output(QVersionNumber* output) {
  new(output) QVersionNumber(QLibraryInfo::version());
}


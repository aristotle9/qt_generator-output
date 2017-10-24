#include "qt_core_c_QOperatingSystemVersion.h"

void qt_core_c_QOperatingSystemVersion_constructor_osType_vmajor(QOperatingSystemVersion::OSType osType, int vmajor, QOperatingSystemVersion* output) {
  new(output) QOperatingSystemVersion(osType, vmajor);
}

void qt_core_c_QOperatingSystemVersion_constructor_osType_vmajor_vminor(QOperatingSystemVersion::OSType osType, int vmajor, int vminor, QOperatingSystemVersion* output) {
  new(output) QOperatingSystemVersion(osType, vmajor, vminor);
}

void qt_core_c_QOperatingSystemVersion_constructor_osType_vmajor_vminor_vmicro(QOperatingSystemVersion::OSType osType, int vmajor, int vminor, int vmicro, QOperatingSystemVersion* output) {
  new(output) QOperatingSystemVersion(osType, vmajor, vminor, vmicro);
}

void qt_core_c_QOperatingSystemVersion_constructor_other(const QOperatingSystemVersion* other, QOperatingSystemVersion* output) {
  new(output) QOperatingSystemVersion(*other);
}

void qt_core_c_QOperatingSystemVersion_current_to_output(QOperatingSystemVersion* output) {
  new(output) QOperatingSystemVersion(QOperatingSystemVersion::current());
}

void qt_core_c_QOperatingSystemVersion_destructor(QOperatingSystemVersion* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

int qt_core_c_QOperatingSystemVersion_majorVersion(const QOperatingSystemVersion* this_ptr) {
  return this_ptr->majorVersion();
}

int qt_core_c_QOperatingSystemVersion_microVersion(const QOperatingSystemVersion* this_ptr) {
  return this_ptr->microVersion();
}

int qt_core_c_QOperatingSystemVersion_minorVersion(const QOperatingSystemVersion* this_ptr) {
  return this_ptr->minorVersion();
}

void qt_core_c_QOperatingSystemVersion_name_to_output(const QOperatingSystemVersion* this_ptr, QString* output) {
  new(output) QString(this_ptr->name());
}

int qt_core_c_QOperatingSystemVersion_segmentCount(const QOperatingSystemVersion* this_ptr) {
  return this_ptr->segmentCount();
}

QOperatingSystemVersion::OSType qt_core_c_QOperatingSystemVersion_type(const QOperatingSystemVersion* this_ptr) {
  return this_ptr->type();
}


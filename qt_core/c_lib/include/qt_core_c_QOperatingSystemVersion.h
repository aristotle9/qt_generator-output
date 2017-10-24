#ifndef QT_CORE_C_QOPERATINGSYSTEMVERSION_H
#define QT_CORE_C_QOPERATINGSYSTEMVERSION_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QOperatingSystemVersion_constructor_osType_vmajor(QOperatingSystemVersion::OSType osType, int vmajor, QOperatingSystemVersion* output);
QT_CORE_C_EXPORT void qt_core_c_QOperatingSystemVersion_constructor_osType_vmajor_vminor(QOperatingSystemVersion::OSType osType, int vmajor, int vminor, QOperatingSystemVersion* output);
QT_CORE_C_EXPORT void qt_core_c_QOperatingSystemVersion_constructor_osType_vmajor_vminor_vmicro(QOperatingSystemVersion::OSType osType, int vmajor, int vminor, int vmicro, QOperatingSystemVersion* output);
QT_CORE_C_EXPORT void qt_core_c_QOperatingSystemVersion_constructor_other(const QOperatingSystemVersion* other, QOperatingSystemVersion* output);
QT_CORE_C_EXPORT void qt_core_c_QOperatingSystemVersion_current_to_output(QOperatingSystemVersion* output);
QT_CORE_C_EXPORT void qt_core_c_QOperatingSystemVersion_destructor(QOperatingSystemVersion* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QOperatingSystemVersion_majorVersion(const QOperatingSystemVersion* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QOperatingSystemVersion_microVersion(const QOperatingSystemVersion* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QOperatingSystemVersion_minorVersion(const QOperatingSystemVersion* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QOperatingSystemVersion_name_to_output(const QOperatingSystemVersion* this_ptr, QString* output);
QT_CORE_C_EXPORT int qt_core_c_QOperatingSystemVersion_segmentCount(const QOperatingSystemVersion* this_ptr);
QT_CORE_C_EXPORT QOperatingSystemVersion::OSType qt_core_c_QOperatingSystemVersion_type(const QOperatingSystemVersion* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QOPERATINGSYSTEMVERSION_H

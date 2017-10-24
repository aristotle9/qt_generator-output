#ifndef QT_CORE_C_QTEMPORARYDIR_H
#define QT_CORE_C_QTEMPORARYDIR_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT bool qt_core_c_QTemporaryDir_autoRemove(const QTemporaryDir* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTemporaryDir_constructor_no_args(QTemporaryDir* output);
QT_CORE_C_EXPORT void qt_core_c_QTemporaryDir_constructor_templateName(const QString* templateName, QTemporaryDir* output);
QT_CORE_C_EXPORT void qt_core_c_QTemporaryDir_destructor(QTemporaryDir* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTemporaryDir_errorString_to_output(const QTemporaryDir* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QTemporaryDir_filePath_to_output(const QTemporaryDir* this_ptr, const QString* fileName, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QTemporaryDir_isValid(const QTemporaryDir* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTemporaryDir_path_to_output(const QTemporaryDir* this_ptr, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QTemporaryDir_remove(QTemporaryDir* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTemporaryDir_setAutoRemove(QTemporaryDir* this_ptr, bool b);

} // extern "C"

#endif // QT_CORE_C_QTEMPORARYDIR_H

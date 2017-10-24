#ifndef QT_CORE_C_QMIMETYPE_H
#define QT_CORE_C_QMIMETYPE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QMimeType_G_operator_shl_to_output(const QDebug* debug, const QMimeType* mime, QDebug* output);
QT_CORE_C_EXPORT unsigned int qt_core_c_QMimeType_G_qHash_key(const QMimeType* key);
QT_CORE_C_EXPORT unsigned int qt_core_c_QMimeType_G_qHash_key_seed(const QMimeType* key, unsigned int seed);
QT_CORE_C_EXPORT void qt_core_c_QMimeType_G_swap(QMimeType* value1, QMimeType* value2);
QT_CORE_C_EXPORT void qt_core_c_QMimeType_aliases_to_output(const QMimeType* this_ptr, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeType_allAncestors_to_output(const QMimeType* this_ptr, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeType_comment_to_output(const QMimeType* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeType_constructor_no_args(QMimeType* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeType_constructor_other(const QMimeType* other, QMimeType* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeType_destructor(QMimeType* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMimeType_filterString_to_output(const QMimeType* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeType_genericIconName_to_output(const QMimeType* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeType_globPatterns_to_output(const QMimeType* this_ptr, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeType_iconName_to_output(const QMimeType* this_ptr, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QMimeType_inherits(const QMimeType* this_ptr, const QString* mimeTypeName);
QT_CORE_C_EXPORT bool qt_core_c_QMimeType_isDefault(const QMimeType* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMimeType_isValid(const QMimeType* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMimeType_name_to_output(const QMimeType* this_ptr, QString* output);
QT_CORE_C_EXPORT QMimeType* qt_core_c_QMimeType_operator_assign(QMimeType* this_ptr, const QMimeType* other);
QT_CORE_C_EXPORT bool qt_core_c_QMimeType_operator_eq(const QMimeType* this_ptr, const QMimeType* other);
QT_CORE_C_EXPORT bool qt_core_c_QMimeType_operator_neq(const QMimeType* this_ptr, const QMimeType* other);
QT_CORE_C_EXPORT void qt_core_c_QMimeType_parentMimeTypes_to_output(const QMimeType* this_ptr, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeType_preferredSuffix_to_output(const QMimeType* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeType_suffixes_to_output(const QMimeType* this_ptr, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QMimeType_swap(QMimeType* this_ptr, QMimeType* other);

} // extern "C"

#endif // QT_CORE_C_QMIMETYPE_H

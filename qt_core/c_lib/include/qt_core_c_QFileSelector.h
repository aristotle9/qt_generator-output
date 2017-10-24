#ifndef QT_CORE_C_QFILESELECTOR_H
#define QT_CORE_C_QFILESELECTOR_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QFileSelector* qt_core_c_QFileSelector_G_dynamic_cast_QFileSelector_ptr(QObject* ptr);
QT_CORE_C_EXPORT QFileSelector* qt_core_c_QFileSelector_G_static_cast_QFileSelector_ptr(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QFileSelector_G_static_cast_QObject_ptr(QFileSelector* ptr);
QT_CORE_C_EXPORT void qt_core_c_QFileSelector_allSelectors_to_output(const QFileSelector* this_ptr, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QFileSelector_delete(QFileSelector* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QFileSelector_extraSelectors_to_output(const QFileSelector* this_ptr, QStringList* output);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QFileSelector_metaObject(const QFileSelector* this_ptr);
QT_CORE_C_EXPORT QFileSelector* qt_core_c_QFileSelector_new_no_args();
QT_CORE_C_EXPORT QFileSelector* qt_core_c_QFileSelector_new_parent(QObject* parent);
QT_CORE_C_EXPORT void qt_core_c_QFileSelector_select_to_output_QString(const QFileSelector* this_ptr, const QString* filePath, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QFileSelector_select_to_output_QUrl(const QFileSelector* this_ptr, const QUrl* filePath, QUrl* output);
QT_CORE_C_EXPORT void qt_core_c_QFileSelector_setExtraSelectors(QFileSelector* this_ptr, const QStringList* list);
QT_CORE_C_EXPORT void qt_core_c_QFileSelector_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QFileSelector_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QFILESELECTOR_H

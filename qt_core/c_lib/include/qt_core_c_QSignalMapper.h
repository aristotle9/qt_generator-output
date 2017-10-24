#ifndef QT_CORE_C_QSIGNALMAPPER_H
#define QT_CORE_C_QSIGNALMAPPER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QSignalMapper* qt_core_c_QSignalMapper_G_dynamic_cast_QSignalMapper_ptr(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QSignalMapper_G_static_cast_QObject_ptr(QSignalMapper* ptr);
QT_CORE_C_EXPORT QSignalMapper* qt_core_c_QSignalMapper_G_static_cast_QSignalMapper_ptr(QObject* ptr);
QT_CORE_C_EXPORT void qt_core_c_QSignalMapper_delete(QSignalMapper* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSignalMapper_map_no_args(QSignalMapper* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSignalMapper_map_sender(QSignalMapper* this_ptr, QObject* sender);
QT_CORE_C_EXPORT QObject* qt_core_c_QSignalMapper_mapping_id(const QSignalMapper* this_ptr, int id);
QT_CORE_C_EXPORT QObject* qt_core_c_QSignalMapper_mapping_object(const QSignalMapper* this_ptr, QObject* object);
QT_CORE_C_EXPORT QObject* qt_core_c_QSignalMapper_mapping_text(const QSignalMapper* this_ptr, const QString* text);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QSignalMapper_metaObject(const QSignalMapper* this_ptr);
QT_CORE_C_EXPORT QSignalMapper* qt_core_c_QSignalMapper_new_no_args();
QT_CORE_C_EXPORT QSignalMapper* qt_core_c_QSignalMapper_new_parent(QObject* parent);
QT_CORE_C_EXPORT void qt_core_c_QSignalMapper_removeMappings(QSignalMapper* this_ptr, QObject* sender);
QT_CORE_C_EXPORT void qt_core_c_QSignalMapper_setMapping_sender_id(QSignalMapper* this_ptr, QObject* sender, int id);
QT_CORE_C_EXPORT void qt_core_c_QSignalMapper_setMapping_sender_object(QSignalMapper* this_ptr, QObject* sender, QObject* object);
QT_CORE_C_EXPORT void qt_core_c_QSignalMapper_setMapping_sender_text(QSignalMapper* this_ptr, QObject* sender, const QString* text);
QT_CORE_C_EXPORT void qt_core_c_QSignalMapper_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QSignalMapper_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QSIGNALMAPPER_H

#ifndef QT_CORE_C_QHISTORYSTATE_H
#define QT_CORE_C_QHISTORYSTATE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QHistoryState* qt_core_c_QHistoryState_G_dynamic_cast_QHistoryState_ptr_QAbstractState(QAbstractState* ptr);
QT_CORE_C_EXPORT QHistoryState* qt_core_c_QHistoryState_G_dynamic_cast_QHistoryState_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QAbstractState* qt_core_c_QHistoryState_G_static_cast_QAbstractState_ptr(QHistoryState* ptr);
QT_CORE_C_EXPORT QHistoryState* qt_core_c_QHistoryState_G_static_cast_QHistoryState_ptr_QAbstractState(QAbstractState* ptr);
QT_CORE_C_EXPORT QHistoryState* qt_core_c_QHistoryState_G_static_cast_QHistoryState_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QHistoryState_G_static_cast_QObject_ptr(QHistoryState* ptr);
QT_CORE_C_EXPORT QAbstractState* qt_core_c_QHistoryState_defaultState(const QHistoryState* this_ptr);
QT_CORE_C_EXPORT QAbstractTransition* qt_core_c_QHistoryState_defaultTransition(const QHistoryState* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QHistoryState_delete(QHistoryState* this_ptr);
QT_CORE_C_EXPORT QHistoryState::HistoryType qt_core_c_QHistoryState_historyType(const QHistoryState* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QHistoryState_metaObject(const QHistoryState* this_ptr);
QT_CORE_C_EXPORT QHistoryState* qt_core_c_QHistoryState_new_no_args();
QT_CORE_C_EXPORT QHistoryState* qt_core_c_QHistoryState_new_parent(QState* parent);
QT_CORE_C_EXPORT QHistoryState* qt_core_c_QHistoryState_new_type(QHistoryState::HistoryType type);
QT_CORE_C_EXPORT QHistoryState* qt_core_c_QHistoryState_new_type_parent(QHistoryState::HistoryType type, QState* parent);
QT_CORE_C_EXPORT void qt_core_c_QHistoryState_setDefaultState(QHistoryState* this_ptr, QAbstractState* state);
QT_CORE_C_EXPORT void qt_core_c_QHistoryState_setDefaultTransition(QHistoryState* this_ptr, QAbstractTransition* transition);
QT_CORE_C_EXPORT void qt_core_c_QHistoryState_setHistoryType(QHistoryState* this_ptr, QHistoryState::HistoryType type);
QT_CORE_C_EXPORT void qt_core_c_QHistoryState_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QHistoryState_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QHISTORYSTATE_H

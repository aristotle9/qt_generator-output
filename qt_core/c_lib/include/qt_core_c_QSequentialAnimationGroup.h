#ifndef QT_CORE_C_QSEQUENTIALANIMATIONGROUP_H
#define QT_CORE_C_QSEQUENTIALANIMATIONGROUP_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QSequentialAnimationGroup* qt_core_c_QSequentialAnimationGroup_G_dynamic_cast_QSequentialAnimationGroup_ptr_QAbstractAnimation(QAbstractAnimation* ptr);
QT_CORE_C_EXPORT QSequentialAnimationGroup* qt_core_c_QSequentialAnimationGroup_G_dynamic_cast_QSequentialAnimationGroup_ptr_QAnimationGroup(QAnimationGroup* ptr);
QT_CORE_C_EXPORT QSequentialAnimationGroup* qt_core_c_QSequentialAnimationGroup_G_dynamic_cast_QSequentialAnimationGroup_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QAbstractAnimation* qt_core_c_QSequentialAnimationGroup_G_static_cast_QAbstractAnimation_ptr(QSequentialAnimationGroup* ptr);
QT_CORE_C_EXPORT QAnimationGroup* qt_core_c_QSequentialAnimationGroup_G_static_cast_QAnimationGroup_ptr(QSequentialAnimationGroup* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QSequentialAnimationGroup_G_static_cast_QObject_ptr(QSequentialAnimationGroup* ptr);
QT_CORE_C_EXPORT QSequentialAnimationGroup* qt_core_c_QSequentialAnimationGroup_G_static_cast_QSequentialAnimationGroup_ptr_QAbstractAnimation(QAbstractAnimation* ptr);
QT_CORE_C_EXPORT QSequentialAnimationGroup* qt_core_c_QSequentialAnimationGroup_G_static_cast_QSequentialAnimationGroup_ptr_QAnimationGroup(QAnimationGroup* ptr);
QT_CORE_C_EXPORT QSequentialAnimationGroup* qt_core_c_QSequentialAnimationGroup_G_static_cast_QSequentialAnimationGroup_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QPauseAnimation* qt_core_c_QSequentialAnimationGroup_addPause(QSequentialAnimationGroup* this_ptr, int msecs);
QT_CORE_C_EXPORT QAbstractAnimation* qt_core_c_QSequentialAnimationGroup_currentAnimation(const QSequentialAnimationGroup* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSequentialAnimationGroup_delete(QSequentialAnimationGroup* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QSequentialAnimationGroup_duration(const QSequentialAnimationGroup* this_ptr);
QT_CORE_C_EXPORT QPauseAnimation* qt_core_c_QSequentialAnimationGroup_insertPause(QSequentialAnimationGroup* this_ptr, int index, int msecs);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QSequentialAnimationGroup_metaObject(const QSequentialAnimationGroup* this_ptr);
QT_CORE_C_EXPORT QSequentialAnimationGroup* qt_core_c_QSequentialAnimationGroup_new_no_args();
QT_CORE_C_EXPORT QSequentialAnimationGroup* qt_core_c_QSequentialAnimationGroup_new_parent(QObject* parent);
QT_CORE_C_EXPORT void qt_core_c_QSequentialAnimationGroup_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QSequentialAnimationGroup_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QSEQUENTIALANIMATIONGROUP_H

#ifndef QT_CORE_C_QANIMATIONGROUP_H
#define QT_CORE_C_QANIMATIONGROUP_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QAnimationGroup* qt_core_c_QAnimationGroup_G_dynamic_cast_QAnimationGroup_ptr_QAbstractAnimation(QAbstractAnimation* ptr);
QT_CORE_C_EXPORT QAnimationGroup* qt_core_c_QAnimationGroup_G_dynamic_cast_QAnimationGroup_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QAbstractAnimation* qt_core_c_QAnimationGroup_G_static_cast_QAbstractAnimation_ptr(QAnimationGroup* ptr);
QT_CORE_C_EXPORT QAnimationGroup* qt_core_c_QAnimationGroup_G_static_cast_QAnimationGroup_ptr_QAbstractAnimation(QAbstractAnimation* ptr);
QT_CORE_C_EXPORT QAnimationGroup* qt_core_c_QAnimationGroup_G_static_cast_QAnimationGroup_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QAnimationGroup_G_static_cast_QObject_ptr(QAnimationGroup* ptr);
QT_CORE_C_EXPORT void qt_core_c_QAnimationGroup_addAnimation(QAnimationGroup* this_ptr, QAbstractAnimation* animation);
QT_CORE_C_EXPORT QAbstractAnimation* qt_core_c_QAnimationGroup_animationAt(const QAnimationGroup* this_ptr, int index);
QT_CORE_C_EXPORT int qt_core_c_QAnimationGroup_animationCount(const QAnimationGroup* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAnimationGroup_clear(QAnimationGroup* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAnimationGroup_delete(QAnimationGroup* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QAnimationGroup_indexOfAnimation(const QAnimationGroup* this_ptr, QAbstractAnimation* animation);
QT_CORE_C_EXPORT void qt_core_c_QAnimationGroup_insertAnimation(QAnimationGroup* this_ptr, int index, QAbstractAnimation* animation);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QAnimationGroup_metaObject(const QAnimationGroup* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAnimationGroup_removeAnimation(QAnimationGroup* this_ptr, QAbstractAnimation* animation);
QT_CORE_C_EXPORT QAbstractAnimation* qt_core_c_QAnimationGroup_takeAnimation(QAnimationGroup* this_ptr, int index);
QT_CORE_C_EXPORT void qt_core_c_QAnimationGroup_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QAnimationGroup_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QANIMATIONGROUP_H

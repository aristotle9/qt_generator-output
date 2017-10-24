#ifndef QT_CORE_C_QVARIANTANIMATION_H
#define QT_CORE_C_QVARIANTANIMATION_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QVariantAnimation* qt_core_c_QVariantAnimation_G_dynamic_cast_QVariantAnimation_ptr_QAbstractAnimation(QAbstractAnimation* ptr);
QT_CORE_C_EXPORT QVariantAnimation* qt_core_c_QVariantAnimation_G_dynamic_cast_QVariantAnimation_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT QAbstractAnimation* qt_core_c_QVariantAnimation_G_static_cast_QAbstractAnimation_ptr(QVariantAnimation* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QVariantAnimation_G_static_cast_QObject_ptr(QVariantAnimation* ptr);
QT_CORE_C_EXPORT QVariantAnimation* qt_core_c_QVariantAnimation_G_static_cast_QVariantAnimation_ptr_QAbstractAnimation(QAbstractAnimation* ptr);
QT_CORE_C_EXPORT QVariantAnimation* qt_core_c_QVariantAnimation_G_static_cast_QVariantAnimation_ptr_QObject(QObject* ptr);
QT_CORE_C_EXPORT void qt_core_c_QVariantAnimation_currentValue_to_output(const QVariantAnimation* this_ptr, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariantAnimation_delete(QVariantAnimation* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QVariantAnimation_duration(const QVariantAnimation* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QVariantAnimation_easingCurve_to_output(const QVariantAnimation* this_ptr, QEasingCurve* output);
QT_CORE_C_EXPORT void qt_core_c_QVariantAnimation_endValue_to_output(const QVariantAnimation* this_ptr, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariantAnimation_keyValueAt_to_output(const QVariantAnimation* this_ptr, double step, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariantAnimation_keyValues_to_output(const QVariantAnimation* this_ptr, QVector< QPair< double, QVariant > >* output);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QVariantAnimation_metaObject(const QVariantAnimation* this_ptr);
QT_CORE_C_EXPORT QVariantAnimation* qt_core_c_QVariantAnimation_new_no_args();
QT_CORE_C_EXPORT QVariantAnimation* qt_core_c_QVariantAnimation_new_parent(QObject* parent);
QT_CORE_C_EXPORT void qt_core_c_QVariantAnimation_setDuration(QVariantAnimation* this_ptr, int msecs);
QT_CORE_C_EXPORT void qt_core_c_QVariantAnimation_setEasingCurve(QVariantAnimation* this_ptr, const QEasingCurve* easing);
QT_CORE_C_EXPORT void qt_core_c_QVariantAnimation_setEndValue(QVariantAnimation* this_ptr, const QVariant* value);
QT_CORE_C_EXPORT void qt_core_c_QVariantAnimation_setKeyValueAt(QVariantAnimation* this_ptr, double step, const QVariant* value);
QT_CORE_C_EXPORT void qt_core_c_QVariantAnimation_setKeyValues(QVariantAnimation* this_ptr, const QVector< QPair< double, QVariant > >* values);
QT_CORE_C_EXPORT void qt_core_c_QVariantAnimation_setStartValue(QVariantAnimation* this_ptr, const QVariant* value);
QT_CORE_C_EXPORT void qt_core_c_QVariantAnimation_startValue_to_output(const QVariantAnimation* this_ptr, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QVariantAnimation_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QVariantAnimation_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QVARIANTANIMATION_H

#ifndef QT_WIDGETS_C_QSCROLLERPROPERTIES_H
#define QT_WIDGETS_C_QSCROLLERPROPERTIES_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT void qt_widgets_c_QScrollerProperties_delete(QScrollerProperties* this_ptr);
QT_WIDGETS_C_EXPORT QScrollerProperties* qt_widgets_c_QScrollerProperties_new_no_args();
QT_WIDGETS_C_EXPORT QScrollerProperties* qt_widgets_c_QScrollerProperties_new_sp(const QScrollerProperties* sp);
QT_WIDGETS_C_EXPORT QScrollerProperties* qt_widgets_c_QScrollerProperties_operator_assign(QScrollerProperties* this_ptr, const QScrollerProperties* sp);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QScrollerProperties_operator_eq(const QScrollerProperties* this_ptr, const QScrollerProperties* sp);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QScrollerProperties_operator_neq(const QScrollerProperties* this_ptr, const QScrollerProperties* sp);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScrollerProperties_scrollMetric_to_output(const QScrollerProperties* this_ptr, QScrollerProperties::ScrollMetric metric, QVariant* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScrollerProperties_setDefaultScrollerProperties(const QScrollerProperties* sp);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScrollerProperties_setScrollMetric(QScrollerProperties* this_ptr, QScrollerProperties::ScrollMetric metric, const QVariant* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QScrollerProperties_unsetDefaultScrollerProperties();

} // extern "C"

#endif // QT_WIDGETS_C_QSCROLLERPROPERTIES_H

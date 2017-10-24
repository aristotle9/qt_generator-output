#ifndef QT_WIDGETS_C_QSIZEPOLICY_H
#define QT_WIDGETS_C_QSIZEPOLICY_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT void qt_widgets_c_QSizePolicy_G_operator_shl_to_output(const QDebug* dbg, const QSizePolicy* arg2, QDebug* output);
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QSizePolicy_G_qHash_key(const QSizePolicy* key);
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QSizePolicy_G_qHash_key_seed(const QSizePolicy* key, unsigned int seed);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSizePolicy_constructor_horizontal_vertical(QSizePolicy::Policy horizontal, QSizePolicy::Policy vertical, QSizePolicy* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSizePolicy_constructor_horizontal_vertical_type(QSizePolicy::Policy horizontal, QSizePolicy::Policy vertical, QSizePolicy::ControlType type, QSizePolicy* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSizePolicy_constructor_no_args(QSizePolicy* output);
QT_WIDGETS_C_EXPORT QSizePolicy::ControlType qt_widgets_c_QSizePolicy_controlType(const QSizePolicy* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSizePolicy_convert_to_QVariant_to_output(const QSizePolicy* this_ptr, QVariant* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSizePolicy_destructor(QSizePolicy* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QSizePolicy_hasHeightForWidth(const QSizePolicy* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QSizePolicy_hasWidthForHeight(const QSizePolicy* this_ptr);
QT_WIDGETS_C_EXPORT QSizePolicy::Policy qt_widgets_c_QSizePolicy_horizontalPolicy(const QSizePolicy* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QSizePolicy_horizontalStretch(const QSizePolicy* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QSizePolicy_operator_eq(const QSizePolicy* this_ptr, const QSizePolicy* s);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QSizePolicy_operator_neq(const QSizePolicy* this_ptr, const QSizePolicy* s);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QSizePolicy_retainSizeWhenHidden(const QSizePolicy* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSizePolicy_setControlType(QSizePolicy* this_ptr, QSizePolicy::ControlType type);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSizePolicy_setHeightForWidth(QSizePolicy* this_ptr, bool b);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSizePolicy_setHorizontalPolicy(QSizePolicy* this_ptr, QSizePolicy::Policy d);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSizePolicy_setHorizontalStretch(QSizePolicy* this_ptr, int stretchFactor);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSizePolicy_setRetainSizeWhenHidden(QSizePolicy* this_ptr, bool retainSize);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSizePolicy_setVerticalPolicy(QSizePolicy* this_ptr, QSizePolicy::Policy d);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSizePolicy_setVerticalStretch(QSizePolicy* this_ptr, int stretchFactor);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSizePolicy_setWidthForHeight(QSizePolicy* this_ptr, bool b);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSizePolicy_transpose(QSizePolicy* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSizePolicy_transposed_to_output(const QSizePolicy* this_ptr, QSizePolicy* output);
QT_WIDGETS_C_EXPORT QSizePolicy::Policy qt_widgets_c_QSizePolicy_verticalPolicy(const QSizePolicy* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QSizePolicy_verticalStretch(const QSizePolicy* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QSIZEPOLICY_H

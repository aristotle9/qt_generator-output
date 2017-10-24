#ifndef QT_GUI_C_QTEXTOPTION_H
#define QT_GUI_C_QTEXTOPTION_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QTextOption_Tab_constructor_no_args(QTextOption::Tab* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextOption_Tab_constructor_pos_tabType(double pos, QTextOption::TabType tabType, QTextOption::Tab* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextOption_Tab_constructor_pos_tabType_delim(double pos, QTextOption::TabType tabType, const QChar* delim, QTextOption::Tab* output);
QT_GUI_C_EXPORT const QChar* qt_gui_c_QTextOption_Tab_delimiter(const QTextOption::Tab* this_ptr);
QT_GUI_C_EXPORT QChar* qt_gui_c_QTextOption_Tab_delimiter_mut(QTextOption::Tab* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextOption_Tab_destructor(QTextOption::Tab* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextOption_Tab_operator_eq(const QTextOption::Tab* this_ptr, const QTextOption::Tab* other);
QT_GUI_C_EXPORT bool qt_gui_c_QTextOption_Tab_operator_neq(const QTextOption::Tab* this_ptr, const QTextOption::Tab* other);
QT_GUI_C_EXPORT double qt_gui_c_QTextOption_Tab_position(const QTextOption::Tab* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextOption_Tab_set_delimiter(QTextOption::Tab* this_ptr, const QChar* value);
QT_GUI_C_EXPORT void qt_gui_c_QTextOption_Tab_set_position(QTextOption::Tab* this_ptr, double value);
QT_GUI_C_EXPORT void qt_gui_c_QTextOption_Tab_set_type(QTextOption::Tab* this_ptr, QTextOption::TabType value);
QT_GUI_C_EXPORT QTextOption::TabType qt_gui_c_QTextOption_Tab_type(const QTextOption::Tab* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextOption_constructor_no_args(QTextOption* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextOption_constructor_o(const QTextOption* o, QTextOption* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextOption_destructor(QTextOption* this_ptr);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QTextOption_flags(const QTextOption* this_ptr);
QT_GUI_C_EXPORT QTextOption* qt_gui_c_QTextOption_operator_assign(QTextOption* this_ptr, const QTextOption* o);
QT_GUI_C_EXPORT void qt_gui_c_QTextOption_setFlags(QTextOption* this_ptr, unsigned int flags);
QT_GUI_C_EXPORT void qt_gui_c_QTextOption_setTabArray(QTextOption* this_ptr, const QList< double >* tabStops);
QT_GUI_C_EXPORT void qt_gui_c_QTextOption_setTabStop(QTextOption* this_ptr, double tabStop);
QT_GUI_C_EXPORT void qt_gui_c_QTextOption_setTabs(QTextOption* this_ptr, const QList< QTextOption::Tab >* tabStops);
QT_GUI_C_EXPORT void qt_gui_c_QTextOption_setTextDirection(QTextOption* this_ptr, const Qt::LayoutDirection* aDirection);
QT_GUI_C_EXPORT void qt_gui_c_QTextOption_setUseDesignMetrics(QTextOption* this_ptr, bool b);
QT_GUI_C_EXPORT void qt_gui_c_QTextOption_setWrapMode(QTextOption* this_ptr, QTextOption::WrapMode wrap);
QT_GUI_C_EXPORT void qt_gui_c_QTextOption_tabArray_to_output(const QTextOption* this_ptr, QList< double >* output);
QT_GUI_C_EXPORT double qt_gui_c_QTextOption_tabStop(const QTextOption* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextOption_tabs_to_output(const QTextOption* this_ptr, QList< QTextOption::Tab >* output);
QT_GUI_C_EXPORT bool qt_gui_c_QTextOption_useDesignMetrics(const QTextOption* this_ptr);
QT_GUI_C_EXPORT QTextOption::WrapMode qt_gui_c_QTextOption_wrapMode(const QTextOption* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QTEXTOPTION_H

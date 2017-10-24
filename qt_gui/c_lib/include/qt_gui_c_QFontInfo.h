#ifndef QT_GUI_C_QFONTINFO_H
#define QT_GUI_C_QFONTINFO_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QFontInfo_G_swap(QFontInfo* value1, QFontInfo* value2);
QT_GUI_C_EXPORT bool qt_gui_c_QFontInfo_bold(const QFontInfo* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QFontInfo_constructor_QFont(const QFont* arg1, QFontInfo* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontInfo_constructor_QFontInfo(const QFontInfo* arg1, QFontInfo* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontInfo_destructor(QFontInfo* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QFontInfo_exactMatch(const QFontInfo* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QFontInfo_family_to_output(const QFontInfo* this_ptr, QString* output);
QT_GUI_C_EXPORT bool qt_gui_c_QFontInfo_fixedPitch(const QFontInfo* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QFontInfo_italic(const QFontInfo* this_ptr);
QT_GUI_C_EXPORT QFontInfo* qt_gui_c_QFontInfo_operator_assign(QFontInfo* this_ptr, const QFontInfo* arg1);
QT_GUI_C_EXPORT bool qt_gui_c_QFontInfo_overline(const QFontInfo* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QFontInfo_pixelSize(const QFontInfo* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QFontInfo_pointSize(const QFontInfo* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QFontInfo_pointSizeF(const QFontInfo* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QFontInfo_rawMode(const QFontInfo* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QFontInfo_strikeOut(const QFontInfo* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QFontInfo_styleName_to_output(const QFontInfo* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontInfo_swap(QFontInfo* this_ptr, QFontInfo* other);
QT_GUI_C_EXPORT bool qt_gui_c_QFontInfo_underline(const QFontInfo* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QFontInfo_weight(const QFontInfo* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QFONTINFO_H

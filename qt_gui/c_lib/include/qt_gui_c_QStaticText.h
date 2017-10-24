#ifndef QT_GUI_C_QSTATICTEXT_H
#define QT_GUI_C_QSTATICTEXT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QStaticText_G_swap(QStaticText* value1, QStaticText* value2);
QT_GUI_C_EXPORT void qt_gui_c_QStaticText_constructor_no_args(QStaticText* output);
QT_GUI_C_EXPORT void qt_gui_c_QStaticText_constructor_other(const QStaticText* other, QStaticText* output);
QT_GUI_C_EXPORT void qt_gui_c_QStaticText_constructor_text(const QString* text, QStaticText* output);
QT_GUI_C_EXPORT void qt_gui_c_QStaticText_destructor(QStaticText* this_ptr);
QT_GUI_C_EXPORT QStaticText* qt_gui_c_QStaticText_operator_assign(QStaticText* this_ptr, const QStaticText* arg1);
QT_GUI_C_EXPORT bool qt_gui_c_QStaticText_operator_eq(const QStaticText* this_ptr, const QStaticText* arg1);
QT_GUI_C_EXPORT bool qt_gui_c_QStaticText_operator_neq(const QStaticText* this_ptr, const QStaticText* arg1);
QT_GUI_C_EXPORT QStaticText::PerformanceHint qt_gui_c_QStaticText_performanceHint(const QStaticText* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QStaticText_prepare_matrix(QStaticText* this_ptr, const QTransform* matrix);
QT_GUI_C_EXPORT void qt_gui_c_QStaticText_prepare_matrix_font(QStaticText* this_ptr, const QTransform* matrix, const QFont* font);
QT_GUI_C_EXPORT void qt_gui_c_QStaticText_prepare_no_args(QStaticText* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QStaticText_setPerformanceHint(QStaticText* this_ptr, QStaticText::PerformanceHint performanceHint);
QT_GUI_C_EXPORT void qt_gui_c_QStaticText_setText(QStaticText* this_ptr, const QString* text);
QT_GUI_C_EXPORT void qt_gui_c_QStaticText_setTextFormat(QStaticText* this_ptr, const Qt::TextFormat* textFormat);
QT_GUI_C_EXPORT void qt_gui_c_QStaticText_setTextOption(QStaticText* this_ptr, const QTextOption* textOption);
QT_GUI_C_EXPORT void qt_gui_c_QStaticText_setTextWidth(QStaticText* this_ptr, double textWidth);
QT_GUI_C_EXPORT void qt_gui_c_QStaticText_size_to_output(const QStaticText* this_ptr, QSizeF* output);
QT_GUI_C_EXPORT void qt_gui_c_QStaticText_swap(QStaticText* this_ptr, QStaticText* other);
QT_GUI_C_EXPORT void qt_gui_c_QStaticText_textOption_to_output(const QStaticText* this_ptr, QTextOption* output);
QT_GUI_C_EXPORT double qt_gui_c_QStaticText_textWidth(const QStaticText* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QStaticText_text_to_output(const QStaticText* this_ptr, QString* output);

} // extern "C"

#endif // QT_GUI_C_QSTATICTEXT_H

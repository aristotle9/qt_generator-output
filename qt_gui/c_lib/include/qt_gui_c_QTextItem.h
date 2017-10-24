#ifndef QT_GUI_C_QTEXTITEM_H
#define QT_GUI_C_QTEXTITEM_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT double qt_gui_c_QTextItem_ascent(const QTextItem* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextItem_descent(const QTextItem* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextItem_destructor(QTextItem* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextItem_font_to_output(const QTextItem* this_ptr, QFont* output);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QTextItem_renderFlags(const QTextItem* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextItem_text_to_output(const QTextItem* this_ptr, QString* output);
QT_GUI_C_EXPORT double qt_gui_c_QTextItem_width(const QTextItem* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QTEXTITEM_H

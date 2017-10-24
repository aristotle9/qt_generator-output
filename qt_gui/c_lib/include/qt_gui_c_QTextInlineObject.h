#ifndef QT_GUI_C_QTEXTINLINEOBJECT_H
#define QT_GUI_C_QTEXTINLINEOBJECT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT double qt_gui_c_QTextInlineObject_ascent(const QTextInlineObject* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextInlineObject_constructor(QTextInlineObject* output);
QT_GUI_C_EXPORT double qt_gui_c_QTextInlineObject_descent(const QTextInlineObject* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextInlineObject_destructor(QTextInlineObject* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextInlineObject_formatIndex(const QTextInlineObject* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextInlineObject_format_to_output(const QTextInlineObject* this_ptr, QTextFormat* output);
QT_GUI_C_EXPORT double qt_gui_c_QTextInlineObject_height(const QTextInlineObject* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextInlineObject_isValid(const QTextInlineObject* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextInlineObject_rect_to_output(const QTextInlineObject* this_ptr, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextInlineObject_setAscent(QTextInlineObject* this_ptr, double a);
QT_GUI_C_EXPORT void qt_gui_c_QTextInlineObject_setDescent(QTextInlineObject* this_ptr, double d);
QT_GUI_C_EXPORT void qt_gui_c_QTextInlineObject_setWidth(QTextInlineObject* this_ptr, double w);
QT_GUI_C_EXPORT int qt_gui_c_QTextInlineObject_textPosition(const QTextInlineObject* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextInlineObject_width(const QTextInlineObject* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QTEXTINLINEOBJECT_H

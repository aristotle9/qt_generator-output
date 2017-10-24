#ifndef QT_GUI_C_QTEXTIMAGEFORMAT_H
#define QT_GUI_C_QTEXTIMAGEFORMAT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QTextCharFormat* qt_gui_c_QTextImageFormat_G_static_cast_QTextCharFormat_ptr(QTextImageFormat* ptr);
QT_GUI_C_EXPORT QTextFormat* qt_gui_c_QTextImageFormat_G_static_cast_QTextFormat_ptr(QTextImageFormat* ptr);
QT_GUI_C_EXPORT QTextImageFormat* qt_gui_c_QTextImageFormat_G_static_cast_QTextImageFormat_ptr_QTextCharFormat(QTextCharFormat* ptr);
QT_GUI_C_EXPORT QTextImageFormat* qt_gui_c_QTextImageFormat_G_static_cast_QTextImageFormat_ptr_QTextFormat(QTextFormat* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextImageFormat_constructor(QTextImageFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextImageFormat_destructor(QTextImageFormat* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextImageFormat_height(const QTextImageFormat* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextImageFormat_isValid(const QTextImageFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextImageFormat_name_to_output(const QTextImageFormat* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextImageFormat_setHeight(QTextImageFormat* this_ptr, double height);
QT_GUI_C_EXPORT void qt_gui_c_QTextImageFormat_setName(QTextImageFormat* this_ptr, const QString* name);
QT_GUI_C_EXPORT void qt_gui_c_QTextImageFormat_setWidth(QTextImageFormat* this_ptr, double width);
QT_GUI_C_EXPORT double qt_gui_c_QTextImageFormat_width(const QTextImageFormat* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QTEXTIMAGEFORMAT_H

#ifndef QT_WIDGETS_C_QLABEL_H
#define QT_WIDGETS_C_QLABEL_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QLabel* qt_widgets_c_QLabel_G_dynamic_cast_QLabel_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QLabel* qt_widgets_c_QLabel_G_dynamic_cast_QLabel_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QFrame* qt_widgets_c_QLabel_G_static_cast_QFrame_ptr(QLabel* ptr);
QT_WIDGETS_C_EXPORT QLabel* qt_widgets_c_QLabel_G_static_cast_QLabel_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QLabel* qt_widgets_c_QLabel_G_static_cast_QLabel_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QLabel* qt_widgets_c_QLabel_G_static_cast_QLabel_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QLabel* qt_widgets_c_QLabel_G_static_cast_QLabel_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QLabel_G_static_cast_QObject_ptr(QLabel* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QLabel_G_static_cast_QPaintDevice_ptr(QLabel* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QLabel_G_static_cast_QWidget_ptr(QLabel* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QLabel_buddy(const QLabel* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLabel_clear(QLabel* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLabel_delete(QLabel* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QLabel_hasScaledContents(const QLabel* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QLabel_hasSelectedText(const QLabel* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QLabel_heightForWidth(const QLabel* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QLabel_indent(const QLabel* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QLabel_margin(const QLabel* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QLabel_metaObject(const QLabel* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLabel_minimumSizeHint_to_output(const QLabel* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QMovie* qt_widgets_c_QLabel_movie(const QLabel* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QLabel_openExternalLinks(const QLabel* this_ptr);
QT_WIDGETS_C_EXPORT const QPicture* qt_widgets_c_QLabel_picture(const QLabel* this_ptr);
QT_WIDGETS_C_EXPORT const QPixmap* qt_widgets_c_QLabel_pixmap(const QLabel* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QLabel_qt_metacall(QLabel* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QLabel_qt_metacast(QLabel* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLabel_selectedText_to_output(const QLabel* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QLabel_selectionStart(const QLabel* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLabel_setBuddy(QLabel* this_ptr, QWidget* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLabel_setIndent(QLabel* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLabel_setMargin(QLabel* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLabel_setMovie(QLabel* this_ptr, QMovie* movie);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLabel_setNum_double(QLabel* this_ptr, double arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLabel_setNum_int(QLabel* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLabel_setOpenExternalLinks(QLabel* this_ptr, bool open);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLabel_setPicture(QLabel* this_ptr, const QPicture* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLabel_setPixmap(QLabel* this_ptr, const QPixmap* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLabel_setScaledContents(QLabel* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLabel_setSelection(QLabel* this_ptr, int arg1, int arg2);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLabel_setText(QLabel* this_ptr, const QString* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLabel_setTextFormat(QLabel* this_ptr, const Qt::TextFormat* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLabel_setWordWrap(QLabel* this_ptr, bool on);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLabel_sizeHint_to_output(const QLabel* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLabel_text_to_output(const QLabel* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLabel_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLabel_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QLabel_wordWrap(const QLabel* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QLABEL_H

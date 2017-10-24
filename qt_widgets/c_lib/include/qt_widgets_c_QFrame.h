#ifndef QT_WIDGETS_C_QFRAME_H
#define QT_WIDGETS_C_QFRAME_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QFrame* qt_widgets_c_QFrame_G_dynamic_cast_QFrame_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QFrame* qt_widgets_c_QFrame_G_static_cast_QFrame_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QFrame* qt_widgets_c_QFrame_G_static_cast_QFrame_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QFrame* qt_widgets_c_QFrame_G_static_cast_QFrame_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QFrame_G_static_cast_QObject_ptr(QFrame* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QFrame_G_static_cast_QPaintDevice_ptr(QFrame* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QFrame_G_static_cast_QWidget_ptr(QFrame* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFrame_delete(QFrame* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFrame_frameRect_to_output(const QFrame* this_ptr, QRect* output);
QT_WIDGETS_C_EXPORT QFrame::Shadow qt_widgets_c_QFrame_frameShadow(const QFrame* this_ptr);
QT_WIDGETS_C_EXPORT QFrame::Shape qt_widgets_c_QFrame_frameShape(const QFrame* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QFrame_frameStyle(const QFrame* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QFrame_frameWidth(const QFrame* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QFrame_lineWidth(const QFrame* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QFrame_metaObject(const QFrame* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QFrame_midLineWidth(const QFrame* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QFrame_qt_metacall(QFrame* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QFrame_qt_metacast(QFrame* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFrame_setFrameRect(QFrame* this_ptr, const QRect* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFrame_setFrameShadow(QFrame* this_ptr, QFrame::Shadow arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFrame_setFrameShape(QFrame* this_ptr, QFrame::Shape arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFrame_setFrameStyle(QFrame* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFrame_setLineWidth(QFrame* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFrame_setMidLineWidth(QFrame* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFrame_sizeHint_to_output(const QFrame* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFrame_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFrame_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QFRAME_H

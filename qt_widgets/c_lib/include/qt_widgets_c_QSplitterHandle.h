#ifndef QT_WIDGETS_C_QSPLITTERHANDLE_H
#define QT_WIDGETS_C_QSPLITTERHANDLE_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QSplitterHandle* qt_widgets_c_QSplitterHandle_G_dynamic_cast_QSplitterHandle_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QSplitterHandle_G_static_cast_QObject_ptr(QSplitterHandle* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QSplitterHandle_G_static_cast_QPaintDevice_ptr(QSplitterHandle* ptr);
QT_WIDGETS_C_EXPORT QSplitterHandle* qt_widgets_c_QSplitterHandle_G_static_cast_QSplitterHandle_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QSplitterHandle* qt_widgets_c_QSplitterHandle_G_static_cast_QSplitterHandle_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QSplitterHandle* qt_widgets_c_QSplitterHandle_G_static_cast_QSplitterHandle_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QSplitterHandle_G_static_cast_QWidget_ptr(QSplitterHandle* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplitterHandle_delete(QSplitterHandle* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QSplitterHandle_metaObject(const QSplitterHandle* this_ptr);
QT_WIDGETS_C_EXPORT QSplitterHandle* qt_widgets_c_QSplitterHandle_new(const Qt::Orientation* o, QSplitter* parent);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QSplitterHandle_opaqueResize(const QSplitterHandle* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QSplitterHandle_qt_metacall(QSplitterHandle* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QSplitterHandle_qt_metacast(QSplitterHandle* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplitterHandle_setOrientation(QSplitterHandle* this_ptr, const Qt::Orientation* o);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplitterHandle_sizeHint_to_output(const QSplitterHandle* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QSplitter* qt_widgets_c_QSplitterHandle_splitter(const QSplitterHandle* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplitterHandle_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QSplitterHandle_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QSPLITTERHANDLE_H

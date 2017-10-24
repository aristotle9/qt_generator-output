#ifndef QT_WIDGETS_C_QKEYSEQUENCEEDIT_H
#define QT_WIDGETS_C_QKEYSEQUENCEEDIT_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QKeySequenceEdit* qt_widgets_c_QKeySequenceEdit_G_dynamic_cast_QKeySequenceEdit_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QKeySequenceEdit* qt_widgets_c_QKeySequenceEdit_G_static_cast_QKeySequenceEdit_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QKeySequenceEdit* qt_widgets_c_QKeySequenceEdit_G_static_cast_QKeySequenceEdit_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QKeySequenceEdit* qt_widgets_c_QKeySequenceEdit_G_static_cast_QKeySequenceEdit_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QKeySequenceEdit_G_static_cast_QObject_ptr(QKeySequenceEdit* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QKeySequenceEdit_G_static_cast_QPaintDevice_ptr(QKeySequenceEdit* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QKeySequenceEdit_G_static_cast_QWidget_ptr(QKeySequenceEdit* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QKeySequenceEdit_clear(QKeySequenceEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QKeySequenceEdit_delete(QKeySequenceEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QKeySequenceEdit_keySequence_to_output(const QKeySequenceEdit* this_ptr, QKeySequence* output);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QKeySequenceEdit_metaObject(const QKeySequenceEdit* this_ptr);
QT_WIDGETS_C_EXPORT QKeySequenceEdit* qt_widgets_c_QKeySequenceEdit_new_keySequence(const QKeySequence* keySequence);
QT_WIDGETS_C_EXPORT QKeySequenceEdit* qt_widgets_c_QKeySequenceEdit_new_keySequence_parent(const QKeySequence* keySequence, QWidget* parent);
QT_WIDGETS_C_EXPORT QKeySequenceEdit* qt_widgets_c_QKeySequenceEdit_new_no_args();
QT_WIDGETS_C_EXPORT QKeySequenceEdit* qt_widgets_c_QKeySequenceEdit_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QKeySequenceEdit_qt_metacall(QKeySequenceEdit* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QKeySequenceEdit_qt_metacast(QKeySequenceEdit* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QKeySequenceEdit_setKeySequence(QKeySequenceEdit* this_ptr, const QKeySequence* keySequence);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QKeySequenceEdit_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QKeySequenceEdit_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QKEYSEQUENCEEDIT_H

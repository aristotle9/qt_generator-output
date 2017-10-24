#ifndef QT_WIDGETS_C_QFONTCOMBOBOX_H
#define QT_WIDGETS_C_QFONTCOMBOBOX_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QFontComboBox* qt_widgets_c_QFontComboBox_G_dynamic_cast_QFontComboBox_ptr_QComboBox(QComboBox* ptr);
QT_WIDGETS_C_EXPORT QFontComboBox* qt_widgets_c_QFontComboBox_G_dynamic_cast_QFontComboBox_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QComboBox* qt_widgets_c_QFontComboBox_G_static_cast_QComboBox_ptr(QFontComboBox* ptr);
QT_WIDGETS_C_EXPORT QFontComboBox* qt_widgets_c_QFontComboBox_G_static_cast_QFontComboBox_ptr_QComboBox(QComboBox* ptr);
QT_WIDGETS_C_EXPORT QFontComboBox* qt_widgets_c_QFontComboBox_G_static_cast_QFontComboBox_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QFontComboBox* qt_widgets_c_QFontComboBox_G_static_cast_QFontComboBox_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QFontComboBox* qt_widgets_c_QFontComboBox_G_static_cast_QFontComboBox_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QFontComboBox_G_static_cast_QObject_ptr(QFontComboBox* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QFontComboBox_G_static_cast_QPaintDevice_ptr(QFontComboBox* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QFontComboBox_G_static_cast_QWidget_ptr(QFontComboBox* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFontComboBox_currentFont_to_output(const QFontComboBox* this_ptr, QFont* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFontComboBox_delete(QFontComboBox* this_ptr);
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QFontComboBox_fontFilters(const QFontComboBox* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QFontComboBox_metaObject(const QFontComboBox* this_ptr);
QT_WIDGETS_C_EXPORT QFontComboBox* qt_widgets_c_QFontComboBox_new_no_args();
QT_WIDGETS_C_EXPORT QFontComboBox* qt_widgets_c_QFontComboBox_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QFontComboBox_qt_metacall(QFontComboBox* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QFontComboBox_qt_metacast(QFontComboBox* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFontComboBox_setCurrentFont(QFontComboBox* this_ptr, const QFont* f);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFontComboBox_setFontFilters(QFontComboBox* this_ptr, unsigned int filters);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFontComboBox_setWritingSystem(QFontComboBox* this_ptr, const QFontDatabase::WritingSystem* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFontComboBox_sizeHint_to_output(const QFontComboBox* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFontComboBox_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QFontComboBox_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QFONTCOMBOBOX_H

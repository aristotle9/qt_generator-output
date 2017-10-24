#ifndef QT_WIDGETS_C_QBUTTONGROUP_H
#define QT_WIDGETS_C_QBUTTONGROUP_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QButtonGroup* qt_widgets_c_QButtonGroup_G_static_cast_QButtonGroup_ptr(QObject* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QButtonGroup_G_static_cast_QObject_ptr(QButtonGroup* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QButtonGroup_addButton_arg1(QButtonGroup* this_ptr, QAbstractButton* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QButtonGroup_addButton_arg1_id(QButtonGroup* this_ptr, QAbstractButton* arg1, int id);
QT_WIDGETS_C_EXPORT QAbstractButton* qt_widgets_c_QButtonGroup_button(const QButtonGroup* this_ptr, int id);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QButtonGroup_buttons_to_output(const QButtonGroup* this_ptr, QList< QAbstractButton* >* output);
QT_WIDGETS_C_EXPORT QAbstractButton* qt_widgets_c_QButtonGroup_checkedButton(const QButtonGroup* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QButtonGroup_checkedId(const QButtonGroup* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QButtonGroup_delete(QButtonGroup* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QButtonGroup_exclusive(const QButtonGroup* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QButtonGroup_id(const QButtonGroup* this_ptr, QAbstractButton* button);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QButtonGroup_metaObject(const QButtonGroup* this_ptr);
QT_WIDGETS_C_EXPORT QButtonGroup* qt_widgets_c_QButtonGroup_new_no_args();
QT_WIDGETS_C_EXPORT QButtonGroup* qt_widgets_c_QButtonGroup_new_parent(QObject* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QButtonGroup_qt_metacall(QButtonGroup* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QButtonGroup_qt_metacast(QButtonGroup* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QButtonGroup_removeButton(QButtonGroup* this_ptr, QAbstractButton* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QButtonGroup_setExclusive(QButtonGroup* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QButtonGroup_setId(QButtonGroup* this_ptr, QAbstractButton* button, int id);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QButtonGroup_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QButtonGroup_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QBUTTONGROUP_H

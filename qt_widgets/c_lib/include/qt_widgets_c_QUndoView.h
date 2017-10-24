#ifndef QT_WIDGETS_C_QUNDOVIEW_H
#define QT_WIDGETS_C_QUNDOVIEW_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QUndoView* qt_widgets_c_QUndoView_G_dynamic_cast_QUndoView_ptr_QAbstractItemView(QAbstractItemView* ptr);
QT_WIDGETS_C_EXPORT QUndoView* qt_widgets_c_QUndoView_G_dynamic_cast_QUndoView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QUndoView* qt_widgets_c_QUndoView_G_dynamic_cast_QUndoView_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QUndoView* qt_widgets_c_QUndoView_G_dynamic_cast_QUndoView_ptr_QListView(QListView* ptr);
QT_WIDGETS_C_EXPORT QUndoView* qt_widgets_c_QUndoView_G_dynamic_cast_QUndoView_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QAbstractItemView* qt_widgets_c_QUndoView_G_static_cast_QAbstractItemView_ptr(QUndoView* ptr);
QT_WIDGETS_C_EXPORT QAbstractScrollArea* qt_widgets_c_QUndoView_G_static_cast_QAbstractScrollArea_ptr(QUndoView* ptr);
QT_WIDGETS_C_EXPORT QFrame* qt_widgets_c_QUndoView_G_static_cast_QFrame_ptr(QUndoView* ptr);
QT_WIDGETS_C_EXPORT QListView* qt_widgets_c_QUndoView_G_static_cast_QListView_ptr(QUndoView* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QUndoView_G_static_cast_QObject_ptr(QUndoView* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QUndoView_G_static_cast_QPaintDevice_ptr(QUndoView* ptr);
QT_WIDGETS_C_EXPORT QUndoView* qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QAbstractItemView(QAbstractItemView* ptr);
QT_WIDGETS_C_EXPORT QUndoView* qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
QT_WIDGETS_C_EXPORT QUndoView* qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QFrame(QFrame* ptr);
QT_WIDGETS_C_EXPORT QUndoView* qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QListView(QListView* ptr);
QT_WIDGETS_C_EXPORT QUndoView* qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QUndoView* qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QUndoView* qt_widgets_c_QUndoView_G_static_cast_QUndoView_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QUndoView_G_static_cast_QWidget_ptr(QUndoView* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoView_cleanIcon_to_output(const QUndoView* this_ptr, QIcon* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoView_delete(QUndoView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoView_emptyLabel_to_output(const QUndoView* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT QUndoGroup* qt_widgets_c_QUndoView_group(const QUndoView* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QUndoView_metaObject(const QUndoView* this_ptr);
QT_WIDGETS_C_EXPORT QUndoView* qt_widgets_c_QUndoView_new_group(QUndoGroup* group);
QT_WIDGETS_C_EXPORT QUndoView* qt_widgets_c_QUndoView_new_group_parent(QUndoGroup* group, QWidget* parent);
QT_WIDGETS_C_EXPORT QUndoView* qt_widgets_c_QUndoView_new_no_args();
QT_WIDGETS_C_EXPORT QUndoView* qt_widgets_c_QUndoView_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT QUndoView* qt_widgets_c_QUndoView_new_stack(QUndoStack* stack);
QT_WIDGETS_C_EXPORT QUndoView* qt_widgets_c_QUndoView_new_stack_parent(QUndoStack* stack, QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QUndoView_qt_metacall(QUndoView* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QUndoView_qt_metacast(QUndoView* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoView_setCleanIcon(QUndoView* this_ptr, const QIcon* icon);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoView_setEmptyLabel(QUndoView* this_ptr, const QString* label);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoView_setGroup(QUndoView* this_ptr, QUndoGroup* group);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoView_setStack(QUndoView* this_ptr, QUndoStack* stack);
QT_WIDGETS_C_EXPORT QUndoStack* qt_widgets_c_QUndoView_stack(const QUndoView* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoView_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoView_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QUNDOVIEW_H

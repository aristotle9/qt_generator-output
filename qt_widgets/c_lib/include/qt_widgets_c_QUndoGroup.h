#ifndef QT_WIDGETS_C_QUNDOGROUP_H
#define QT_WIDGETS_C_QUNDOGROUP_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QUndoGroup_G_static_cast_QObject_ptr(QUndoGroup* ptr);
QT_WIDGETS_C_EXPORT QUndoGroup* qt_widgets_c_QUndoGroup_G_static_cast_QUndoGroup_ptr(QObject* ptr);
QT_WIDGETS_C_EXPORT QUndoStack* qt_widgets_c_QUndoGroup_activeStack(const QUndoGroup* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoGroup_addStack(QUndoGroup* this_ptr, QUndoStack* stack);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QUndoGroup_canRedo(const QUndoGroup* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QUndoGroup_canUndo(const QUndoGroup* this_ptr);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QUndoGroup_createRedoAction_parent(const QUndoGroup* this_ptr, QObject* parent);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QUndoGroup_createRedoAction_parent_prefix(const QUndoGroup* this_ptr, QObject* parent, const QString* prefix);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QUndoGroup_createUndoAction_parent(const QUndoGroup* this_ptr, QObject* parent);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QUndoGroup_createUndoAction_parent_prefix(const QUndoGroup* this_ptr, QObject* parent, const QString* prefix);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoGroup_delete(QUndoGroup* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QUndoGroup_isClean(const QUndoGroup* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QUndoGroup_metaObject(const QUndoGroup* this_ptr);
QT_WIDGETS_C_EXPORT QUndoGroup* qt_widgets_c_QUndoGroup_new_no_args();
QT_WIDGETS_C_EXPORT QUndoGroup* qt_widgets_c_QUndoGroup_new_parent(QObject* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QUndoGroup_qt_metacall(QUndoGroup* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QUndoGroup_qt_metacast(QUndoGroup* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoGroup_redo(QUndoGroup* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoGroup_redoText_to_output(const QUndoGroup* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoGroup_removeStack(QUndoGroup* this_ptr, QUndoStack* stack);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoGroup_setActiveStack(QUndoGroup* this_ptr, QUndoStack* stack);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoGroup_stacks_to_output(const QUndoGroup* this_ptr, QList< QUndoStack* >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoGroup_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoGroup_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoGroup_undo(QUndoGroup* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoGroup_undoText_to_output(const QUndoGroup* this_ptr, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QUNDOGROUP_H

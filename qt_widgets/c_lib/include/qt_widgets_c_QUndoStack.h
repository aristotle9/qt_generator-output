#ifndef QT_WIDGETS_C_QUNDOSTACK_H
#define QT_WIDGETS_C_QUNDOSTACK_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QUndoStack_G_static_cast_QObject_ptr(QUndoStack* ptr);
QT_WIDGETS_C_EXPORT QUndoStack* qt_widgets_c_QUndoStack_G_static_cast_QUndoStack_ptr(QObject* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoStack_beginMacro(QUndoStack* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QUndoStack_canRedo(const QUndoStack* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QUndoStack_canUndo(const QUndoStack* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QUndoStack_cleanIndex(const QUndoStack* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoStack_clear(QUndoStack* this_ptr);
QT_WIDGETS_C_EXPORT const QUndoCommand* qt_widgets_c_QUndoStack_command(const QUndoStack* this_ptr, int index);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QUndoStack_count(const QUndoStack* this_ptr);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QUndoStack_createRedoAction_parent(const QUndoStack* this_ptr, QObject* parent);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QUndoStack_createRedoAction_parent_prefix(const QUndoStack* this_ptr, QObject* parent, const QString* prefix);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QUndoStack_createUndoAction_parent(const QUndoStack* this_ptr, QObject* parent);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QUndoStack_createUndoAction_parent_prefix(const QUndoStack* this_ptr, QObject* parent, const QString* prefix);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoStack_delete(QUndoStack* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoStack_endMacro(QUndoStack* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QUndoStack_index(const QUndoStack* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QUndoStack_isActive(const QUndoStack* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QUndoStack_isClean(const QUndoStack* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QUndoStack_metaObject(const QUndoStack* this_ptr);
QT_WIDGETS_C_EXPORT QUndoStack* qt_widgets_c_QUndoStack_new_no_args();
QT_WIDGETS_C_EXPORT QUndoStack* qt_widgets_c_QUndoStack_new_parent(QObject* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoStack_push(QUndoStack* this_ptr, QUndoCommand* cmd);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QUndoStack_qt_metacall(QUndoStack* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QUndoStack_qt_metacast(QUndoStack* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoStack_redo(QUndoStack* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoStack_redoText_to_output(const QUndoStack* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoStack_resetClean(QUndoStack* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoStack_setActive_active(QUndoStack* this_ptr, bool active);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoStack_setActive_no_args(QUndoStack* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoStack_setClean(QUndoStack* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoStack_setIndex(QUndoStack* this_ptr, int idx);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoStack_setUndoLimit(QUndoStack* this_ptr, int limit);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoStack_text_to_output(const QUndoStack* this_ptr, int idx, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoStack_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoStack_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoStack_undo(QUndoStack* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QUndoStack_undoLimit(const QUndoStack* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoStack_undoText_to_output(const QUndoStack* this_ptr, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QUNDOSTACK_H

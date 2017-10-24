#ifndef QT_WIDGETS_C_QUNDOCOMMAND_H
#define QT_WIDGETS_C_QUNDOCOMMAND_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoCommand_actionText_to_output(const QUndoCommand* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT const QUndoCommand* qt_widgets_c_QUndoCommand_child(const QUndoCommand* this_ptr, int index);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QUndoCommand_childCount(const QUndoCommand* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoCommand_delete(QUndoCommand* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QUndoCommand_id(const QUndoCommand* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QUndoCommand_isObsolete(const QUndoCommand* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QUndoCommand_mergeWith(QUndoCommand* this_ptr, const QUndoCommand* other);
QT_WIDGETS_C_EXPORT QUndoCommand* qt_widgets_c_QUndoCommand_new_no_args();
QT_WIDGETS_C_EXPORT QUndoCommand* qt_widgets_c_QUndoCommand_new_parent(QUndoCommand* parent);
QT_WIDGETS_C_EXPORT QUndoCommand* qt_widgets_c_QUndoCommand_new_text(const QString* text);
QT_WIDGETS_C_EXPORT QUndoCommand* qt_widgets_c_QUndoCommand_new_text_parent(const QString* text, QUndoCommand* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoCommand_redo(QUndoCommand* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoCommand_setObsolete(QUndoCommand* this_ptr, bool obsolete);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoCommand_setText(QUndoCommand* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoCommand_text_to_output(const QUndoCommand* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QUndoCommand_undo(QUndoCommand* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QUNDOCOMMAND_H

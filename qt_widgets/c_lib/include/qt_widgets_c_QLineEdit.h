#ifndef QT_WIDGETS_C_QLINEEDIT_H
#define QT_WIDGETS_C_QLINEEDIT_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QLineEdit* qt_widgets_c_QLineEdit_G_dynamic_cast_QLineEdit_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QLineEdit* qt_widgets_c_QLineEdit_G_static_cast_QLineEdit_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QLineEdit* qt_widgets_c_QLineEdit_G_static_cast_QLineEdit_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QLineEdit* qt_widgets_c_QLineEdit_G_static_cast_QLineEdit_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QLineEdit_G_static_cast_QObject_ptr(QLineEdit* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QLineEdit_G_static_cast_QPaintDevice_ptr(QLineEdit* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QLineEdit_G_static_cast_QWidget_ptr(QLineEdit* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_addAction_action_position(QLineEdit* this_ptr, QAction* action, QLineEdit::ActionPosition position);
QT_WIDGETS_C_EXPORT QAction* qt_widgets_c_QLineEdit_addAction_icon_position(QLineEdit* this_ptr, const QIcon* icon, QLineEdit::ActionPosition position);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_backspace(QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_clear(QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT QCompleter* qt_widgets_c_QLineEdit_completer(const QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_copy(const QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT QMenu* qt_widgets_c_QLineEdit_createStandardContextMenu(QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_cursorBackward_mark(QLineEdit* this_ptr, bool mark);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_cursorBackward_mark_steps(QLineEdit* this_ptr, bool mark, int steps);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_cursorForward_mark(QLineEdit* this_ptr, bool mark);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_cursorForward_mark_steps(QLineEdit* this_ptr, bool mark, int steps);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QLineEdit_cursorPosition(const QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QLineEdit_cursorPositionAt(QLineEdit* this_ptr, const QPoint* pos);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_cursorWordBackward(QLineEdit* this_ptr, bool mark);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_cursorWordForward(QLineEdit* this_ptr, bool mark);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_cut(QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_del(QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_delete(QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_deselect(QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_displayText_to_output(const QLineEdit* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QLineEdit_dragEnabled(const QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT QLineEdit::EchoMode qt_widgets_c_QLineEdit_echoMode(const QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_end(QLineEdit* this_ptr, bool mark);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QLineEdit_event(QLineEdit* this_ptr, QEvent* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_getTextMargins(const QLineEdit* this_ptr, int* left, int* top, int* right, int* bottom);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QLineEdit_hasAcceptableInput(const QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QLineEdit_hasFrame(const QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QLineEdit_hasSelectedText(const QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_home(QLineEdit* this_ptr, bool mark);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_inputMask_to_output(const QLineEdit* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_inputMethodQuery_to_output_arg1(const QLineEdit* this_ptr, const Qt::InputMethodQuery* arg1, QVariant* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_inputMethodQuery_to_output_property_argument(const QLineEdit* this_ptr, const Qt::InputMethodQuery* property, const QVariant* argument, QVariant* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_insert(QLineEdit* this_ptr, const QString* arg1);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QLineEdit_isClearButtonEnabled(const QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QLineEdit_isModified(const QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QLineEdit_isReadOnly(const QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QLineEdit_isRedoAvailable(const QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QLineEdit_isUndoAvailable(const QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QLineEdit_maxLength(const QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QLineEdit_metaObject(const QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_minimumSizeHint_to_output(const QLineEdit* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QLineEdit* qt_widgets_c_QLineEdit_new_arg1(const QString* arg1);
QT_WIDGETS_C_EXPORT QLineEdit* qt_widgets_c_QLineEdit_new_arg1_parent(const QString* arg1, QWidget* parent);
QT_WIDGETS_C_EXPORT QLineEdit* qt_widgets_c_QLineEdit_new_no_args();
QT_WIDGETS_C_EXPORT QLineEdit* qt_widgets_c_QLineEdit_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_paste(QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_placeholderText_to_output(const QLineEdit* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QLineEdit_qt_metacall(QLineEdit* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QLineEdit_qt_metacast(QLineEdit* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_redo(QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_selectAll(QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_selectedText_to_output(const QLineEdit* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QLineEdit_selectionStart(const QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_setClearButtonEnabled(QLineEdit* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_setCompleter(QLineEdit* this_ptr, QCompleter* completer);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_setCursorMoveStyle(QLineEdit* this_ptr, const Qt::CursorMoveStyle* style);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_setCursorPosition(QLineEdit* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_setDragEnabled(QLineEdit* this_ptr, bool b);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_setEchoMode(QLineEdit* this_ptr, QLineEdit::EchoMode arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_setFrame(QLineEdit* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_setInputMask(QLineEdit* this_ptr, const QString* inputMask);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_setMaxLength(QLineEdit* this_ptr, int arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_setModified(QLineEdit* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_setPlaceholderText(QLineEdit* this_ptr, const QString* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_setReadOnly(QLineEdit* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_setSelection(QLineEdit* this_ptr, int arg1, int arg2);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_setText(QLineEdit* this_ptr, const QString* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_setTextMargins_left_top_right_bottom(QLineEdit* this_ptr, int left, int top, int right, int bottom);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_setTextMargins_margins(QLineEdit* this_ptr, const QMargins* margins);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_setValidator(QLineEdit* this_ptr, const QValidator* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_sizeHint_to_output(const QLineEdit* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_textMargins_to_output(const QLineEdit* this_ptr, QMargins* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_text_to_output(const QLineEdit* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QLineEdit_undo(QLineEdit* this_ptr);
QT_WIDGETS_C_EXPORT const QValidator* qt_widgets_c_QLineEdit_validator(const QLineEdit* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QLINEEDIT_H

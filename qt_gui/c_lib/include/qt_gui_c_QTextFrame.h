#ifndef QT_GUI_C_QTEXTFRAME_H
#define QT_GUI_C_QTEXTFRAME_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QTextFrame* qt_gui_c_QTextFrame_G_dynamic_cast_QTextFrame_ptr(QTextObject* ptr);
QT_GUI_C_EXPORT QObject* qt_gui_c_QTextFrame_G_static_cast_QObject_ptr(QTextFrame* ptr);
QT_GUI_C_EXPORT QTextFrame* qt_gui_c_QTextFrame_G_static_cast_QTextFrame_ptr_QObject(QObject* ptr);
QT_GUI_C_EXPORT QTextFrame* qt_gui_c_QTextFrame_G_static_cast_QTextFrame_ptr_QTextObject(QTextObject* ptr);
QT_GUI_C_EXPORT QTextObject* qt_gui_c_QTextFrame_G_static_cast_QTextObject_ptr(QTextFrame* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrame_begin_to_output(const QTextFrame* this_ptr, QTextFrame::iterator* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrame_childFrames_to_output(const QTextFrame* this_ptr, QList< QTextFrame* >* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrame_delete(QTextFrame* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrame_end_to_output(const QTextFrame* this_ptr, QTextFrame::iterator* output);
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextFrame_firstCursorPosition_as_ptr(const QTextFrame* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextFrame_firstPosition(const QTextFrame* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrame_frameFormat_to_output(const QTextFrame* this_ptr, QTextFrameFormat* output);
QT_GUI_C_EXPORT bool qt_gui_c_QTextFrame_iterator_atEnd(const QTextFrame::iterator* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrame_iterator_constructor_no_args(QTextFrame::iterator* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrame_iterator_constructor_o(const QTextFrame::iterator* o, QTextFrame::iterator* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrame_iterator_currentBlock_to_output(const QTextFrame::iterator* this_ptr, QTextBlock* output);
QT_GUI_C_EXPORT QTextFrame* qt_gui_c_QTextFrame_iterator_currentFrame(const QTextFrame::iterator* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrame_iterator_destructor(QTextFrame::iterator* this_ptr);
QT_GUI_C_EXPORT QTextFrame::iterator* qt_gui_c_QTextFrame_iterator_operator_assign(QTextFrame::iterator* this_ptr, const QTextFrame::iterator* o);
QT_GUI_C_EXPORT QTextFrame::iterator* qt_gui_c_QTextFrame_iterator_operator_dec(QTextFrame::iterator* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrame_iterator_operator_dec_postfix_to_output(QTextFrame::iterator* this_ptr, int arg1, QTextFrame::iterator* output);
QT_GUI_C_EXPORT bool qt_gui_c_QTextFrame_iterator_operator_eq(const QTextFrame::iterator* this_ptr, const QTextFrame::iterator* o);
QT_GUI_C_EXPORT QTextFrame::iterator* qt_gui_c_QTextFrame_iterator_operator_inc(QTextFrame::iterator* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrame_iterator_operator_inc_postfix_to_output(QTextFrame::iterator* this_ptr, int arg1, QTextFrame::iterator* output);
QT_GUI_C_EXPORT bool qt_gui_c_QTextFrame_iterator_operator_neq(const QTextFrame::iterator* this_ptr, const QTextFrame::iterator* o);
QT_GUI_C_EXPORT QTextFrame* qt_gui_c_QTextFrame_iterator_parentFrame(const QTextFrame::iterator* this_ptr);
QT_GUI_C_EXPORT QTextCursor* qt_gui_c_QTextFrame_lastCursorPosition_as_ptr(const QTextFrame* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextFrame_lastPosition(const QTextFrame* this_ptr);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QTextFrame_metaObject(const QTextFrame* this_ptr);
QT_GUI_C_EXPORT QTextFrame* qt_gui_c_QTextFrame_new(QTextDocument* doc);
QT_GUI_C_EXPORT QTextFrame* qt_gui_c_QTextFrame_parentFrame(const QTextFrame* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextFrame_qt_metacall(QTextFrame* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QTextFrame_qt_metacast(QTextFrame* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrame_setFrameFormat(QTextFrame* this_ptr, const QTextFrameFormat* format);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrame_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextFrame_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_GUI_C_QTEXTFRAME_H

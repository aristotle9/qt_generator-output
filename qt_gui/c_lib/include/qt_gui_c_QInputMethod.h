#ifndef QT_GUI_C_QINPUTMETHOD_H
#define QT_GUI_C_QINPUTMETHOD_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QInputMethod* qt_gui_c_QInputMethod_G_static_cast_QInputMethod_ptr(QObject* ptr);
QT_GUI_C_EXPORT QObject* qt_gui_c_QInputMethod_G_static_cast_QObject_ptr(QInputMethod* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethod_anchorRectangle_to_output(const QInputMethod* this_ptr, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethod_commit(QInputMethod* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethod_cursorRectangle_to_output(const QInputMethod* this_ptr, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethod_hide(QInputMethod* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethod_inputItemClipRectangle_to_output(const QInputMethod* this_ptr, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethod_inputItemRectangle_to_output(const QInputMethod* this_ptr, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethod_inputItemTransform_to_output(const QInputMethod* this_ptr, QTransform* output);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethod_invokeAction(QInputMethod* this_ptr, QInputMethod::Action a, int cursorPosition);
QT_GUI_C_EXPORT bool qt_gui_c_QInputMethod_isAnimating(const QInputMethod* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QInputMethod_isVisible(const QInputMethod* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethod_keyboardRectangle_to_output(const QInputMethod* this_ptr, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethod_locale_to_output(const QInputMethod* this_ptr, QLocale* output);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QInputMethod_metaObject(const QInputMethod* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QInputMethod_qt_metacall(QInputMethod* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QInputMethod_qt_metacast(QInputMethod* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethod_queryFocusObject_to_output(const Qt::InputMethodQuery* query, const QVariant* argument, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethod_reset(QInputMethod* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethod_setInputItemRectangle(QInputMethod* this_ptr, const QRectF* rect);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethod_setInputItemTransform(QInputMethod* this_ptr, const QTransform* transform);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethod_setVisible(QInputMethod* this_ptr, bool visible);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethod_show(QInputMethod* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethod_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethod_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_GUI_C_QINPUTMETHOD_H

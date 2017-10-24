#ifndef QT_GUI_C_QINPUTMETHODEVENT_H
#define QT_GUI_C_QINPUTMETHODEVENT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QInputMethodEvent_Attribute_constructor_typ_s_l(QInputMethodEvent::AttributeType typ, int s, int l, QInputMethodEvent::Attribute* output);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethodEvent_Attribute_constructor_typ_s_l_val(QInputMethodEvent::AttributeType typ, int s, int l, const QVariant* val, QInputMethodEvent::Attribute* output);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethodEvent_Attribute_destructor(QInputMethodEvent::Attribute* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QInputMethodEvent_Attribute_length(const QInputMethodEvent::Attribute* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethodEvent_Attribute_set_length(QInputMethodEvent::Attribute* this_ptr, int value);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethodEvent_Attribute_set_start(QInputMethodEvent::Attribute* this_ptr, int value);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethodEvent_Attribute_set_type(QInputMethodEvent::Attribute* this_ptr, QInputMethodEvent::AttributeType value);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethodEvent_Attribute_set_value(QInputMethodEvent::Attribute* this_ptr, const QVariant* value);
QT_GUI_C_EXPORT int qt_gui_c_QInputMethodEvent_Attribute_start(const QInputMethodEvent::Attribute* this_ptr);
QT_GUI_C_EXPORT QInputMethodEvent::AttributeType qt_gui_c_QInputMethodEvent_Attribute_type(const QInputMethodEvent::Attribute* this_ptr);
QT_GUI_C_EXPORT const QVariant* qt_gui_c_QInputMethodEvent_Attribute_value(const QInputMethodEvent::Attribute* this_ptr);
QT_GUI_C_EXPORT QVariant* qt_gui_c_QInputMethodEvent_Attribute_value_mut(QInputMethodEvent::Attribute* this_ptr);
QT_GUI_C_EXPORT QEvent* qt_gui_c_QInputMethodEvent_G_static_cast_QEvent_ptr(QInputMethodEvent* ptr);
QT_GUI_C_EXPORT QInputMethodEvent* qt_gui_c_QInputMethodEvent_G_static_cast_QInputMethodEvent_ptr(QEvent* ptr);
QT_GUI_C_EXPORT const QList< QInputMethodEvent::Attribute >* qt_gui_c_QInputMethodEvent_attributes(const QInputMethodEvent* this_ptr);
QT_GUI_C_EXPORT const QString* qt_gui_c_QInputMethodEvent_commitString(const QInputMethodEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethodEvent_delete(QInputMethodEvent* this_ptr);
QT_GUI_C_EXPORT QInputMethodEvent* qt_gui_c_QInputMethodEvent_new_no_args();
QT_GUI_C_EXPORT QInputMethodEvent* qt_gui_c_QInputMethodEvent_new_other(const QInputMethodEvent* other);
QT_GUI_C_EXPORT QInputMethodEvent* qt_gui_c_QInputMethodEvent_new_preeditText_attributes(const QString* preeditText, const QList< QInputMethodEvent::Attribute >* attributes);
QT_GUI_C_EXPORT const QString* qt_gui_c_QInputMethodEvent_preeditString(const QInputMethodEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QInputMethodEvent_replacementLength(const QInputMethodEvent* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QInputMethodEvent_replacementStart(const QInputMethodEvent* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethodEvent_setCommitString_commitString(QInputMethodEvent* this_ptr, const QString* commitString);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethodEvent_setCommitString_commitString_replaceFrom(QInputMethodEvent* this_ptr, const QString* commitString, int replaceFrom);
QT_GUI_C_EXPORT void qt_gui_c_QInputMethodEvent_setCommitString_commitString_replaceFrom_replaceLength(QInputMethodEvent* this_ptr, const QString* commitString, int replaceFrom, int replaceLength);

} // extern "C"

#endif // QT_GUI_C_QINPUTMETHODEVENT_H

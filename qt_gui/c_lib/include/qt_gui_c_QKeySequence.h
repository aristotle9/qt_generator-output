#ifndef QT_GUI_C_QKEYSEQUENCE_H
#define QT_GUI_C_QKEYSEQUENCE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QDataStream* qt_gui_c_QKeySequence_G_operator_shl(QDataStream* in, const QKeySequence* ks);
QT_GUI_C_EXPORT void qt_gui_c_QKeySequence_G_operator_shl_to_output(const QDebug* arg1, const QKeySequence* arg2, QDebug* output);
QT_GUI_C_EXPORT QDataStream* qt_gui_c_QKeySequence_G_operator_shr(QDataStream* out, QKeySequence* ks);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QKeySequence_G_qHash_key(const QKeySequence* key);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QKeySequence_G_qHash_key_seed(const QKeySequence* key, unsigned int seed);
QT_GUI_C_EXPORT void qt_gui_c_QKeySequence_G_swap(QKeySequence* value1, QKeySequence* value2);
QT_GUI_C_EXPORT void qt_gui_c_QKeySequence_constructor_QKeySequence(const QKeySequence* ks, QKeySequence* output);
QT_GUI_C_EXPORT void qt_gui_c_QKeySequence_constructor_QKeySequence_StandardKey(QKeySequence::StandardKey key, QKeySequence* output);
QT_GUI_C_EXPORT void qt_gui_c_QKeySequence_constructor_QString(const QString* key, QKeySequence* output);
QT_GUI_C_EXPORT void qt_gui_c_QKeySequence_constructor_QString_QKeySequence_SequenceFormat(const QString* key, QKeySequence::SequenceFormat format, QKeySequence* output);
QT_GUI_C_EXPORT void qt_gui_c_QKeySequence_constructor_int(int k1, QKeySequence* output);
QT_GUI_C_EXPORT void qt_gui_c_QKeySequence_constructor_int_int(int k1, int k2, QKeySequence* output);
QT_GUI_C_EXPORT void qt_gui_c_QKeySequence_constructor_int_int_int(int k1, int k2, int k3, QKeySequence* output);
QT_GUI_C_EXPORT void qt_gui_c_QKeySequence_constructor_int_int_int_int(int k1, int k2, int k3, int k4, QKeySequence* output);
QT_GUI_C_EXPORT void qt_gui_c_QKeySequence_constructor_no_args(QKeySequence* output);
QT_GUI_C_EXPORT void qt_gui_c_QKeySequence_convert_to_QVariant_to_output(const QKeySequence* this_ptr, QVariant* output);
QT_GUI_C_EXPORT int qt_gui_c_QKeySequence_count(const QKeySequence* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QKeySequence_destructor(QKeySequence* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QKeySequence_fromString_to_output_str(const QString* str, QKeySequence* output);
QT_GUI_C_EXPORT void qt_gui_c_QKeySequence_fromString_to_output_str_format(const QString* str, QKeySequence::SequenceFormat format, QKeySequence* output);
QT_GUI_C_EXPORT bool qt_gui_c_QKeySequence_isEmpty(const QKeySequence* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QKeySequence_keyBindings_to_output(QKeySequence::StandardKey key, QList< QKeySequence >* output);
QT_GUI_C_EXPORT void qt_gui_c_QKeySequence_listFromString_to_output_str(const QString* str, QList< QKeySequence >* output);
QT_GUI_C_EXPORT void qt_gui_c_QKeySequence_listFromString_to_output_str_format(const QString* str, QKeySequence::SequenceFormat format, QList< QKeySequence >* output);
QT_GUI_C_EXPORT void qt_gui_c_QKeySequence_listToString_to_output_list(const QList< QKeySequence >* list, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QKeySequence_listToString_to_output_list_format(const QList< QKeySequence >* list, QKeySequence::SequenceFormat format, QString* output);
QT_GUI_C_EXPORT QKeySequence::SequenceMatch qt_gui_c_QKeySequence_matches(const QKeySequence* this_ptr, const QKeySequence* seq);
QT_GUI_C_EXPORT void qt_gui_c_QKeySequence_mnemonic_to_output(const QString* text, QKeySequence* output);
QT_GUI_C_EXPORT QKeySequence* qt_gui_c_QKeySequence_operator_assign(QKeySequence* this_ptr, const QKeySequence* other);
QT_GUI_C_EXPORT bool qt_gui_c_QKeySequence_operator_eq(const QKeySequence* this_ptr, const QKeySequence* other);
QT_GUI_C_EXPORT bool qt_gui_c_QKeySequence_operator_ge(const QKeySequence* this_ptr, const QKeySequence* other);
QT_GUI_C_EXPORT bool qt_gui_c_QKeySequence_operator_gt(const QKeySequence* this_ptr, const QKeySequence* other);
QT_GUI_C_EXPORT int qt_gui_c_QKeySequence_operator_index(const QKeySequence* this_ptr, unsigned int i);
QT_GUI_C_EXPORT bool qt_gui_c_QKeySequence_operator_le(const QKeySequence* this_ptr, const QKeySequence* other);
QT_GUI_C_EXPORT bool qt_gui_c_QKeySequence_operator_lt(const QKeySequence* this_ptr, const QKeySequence* ks);
QT_GUI_C_EXPORT bool qt_gui_c_QKeySequence_operator_neq(const QKeySequence* this_ptr, const QKeySequence* other);
QT_GUI_C_EXPORT void qt_gui_c_QKeySequence_swap(QKeySequence* this_ptr, QKeySequence* other);
QT_GUI_C_EXPORT void qt_gui_c_QKeySequence_toString_to_output_format(const QKeySequence* this_ptr, QKeySequence::SequenceFormat format, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QKeySequence_toString_to_output_no_args(const QKeySequence* this_ptr, QString* output);

} // extern "C"

#endif // QT_GUI_C_QKEYSEQUENCE_H

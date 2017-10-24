#ifndef QT_WIDGETS_C_QABSTRACTSPINBOX_H
#define QT_WIDGETS_C_QABSTRACTSPINBOX_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QAbstractSpinBox* qt_widgets_c_QAbstractSpinBox_G_dynamic_cast_QAbstractSpinBox_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QAbstractSpinBox* qt_widgets_c_QAbstractSpinBox_G_static_cast_QAbstractSpinBox_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QAbstractSpinBox* qt_widgets_c_QAbstractSpinBox_G_static_cast_QAbstractSpinBox_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QAbstractSpinBox* qt_widgets_c_QAbstractSpinBox_G_static_cast_QAbstractSpinBox_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QAbstractSpinBox_G_static_cast_QObject_ptr(QAbstractSpinBox* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QAbstractSpinBox_G_static_cast_QPaintDevice_ptr(QAbstractSpinBox* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QAbstractSpinBox_G_static_cast_QWidget_ptr(QAbstractSpinBox* ptr);
QT_WIDGETS_C_EXPORT QAbstractSpinBox::ButtonSymbols qt_widgets_c_QAbstractSpinBox_buttonSymbols(const QAbstractSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSpinBox_clear(QAbstractSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT QAbstractSpinBox::CorrectionMode qt_widgets_c_QAbstractSpinBox_correctionMode(const QAbstractSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSpinBox_delete(QAbstractSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAbstractSpinBox_event(QAbstractSpinBox* this_ptr, QEvent* event);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSpinBox_fixup(const QAbstractSpinBox* this_ptr, QString* input);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAbstractSpinBox_hasAcceptableInput(const QAbstractSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAbstractSpinBox_hasFrame(const QAbstractSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSpinBox_inputMethodQuery_to_output(const QAbstractSpinBox* this_ptr, const Qt::InputMethodQuery* arg1, QVariant* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSpinBox_interpretText(QAbstractSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAbstractSpinBox_isAccelerated(const QAbstractSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAbstractSpinBox_isGroupSeparatorShown(const QAbstractSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAbstractSpinBox_isReadOnly(const QAbstractSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAbstractSpinBox_keyboardTracking(const QAbstractSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QAbstractSpinBox_metaObject(const QAbstractSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSpinBox_minimumSizeHint_to_output(const QAbstractSpinBox* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT QAbstractSpinBox* qt_widgets_c_QAbstractSpinBox_new_no_args();
QT_WIDGETS_C_EXPORT QAbstractSpinBox* qt_widgets_c_QAbstractSpinBox_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QAbstractSpinBox_qt_metacall(QAbstractSpinBox* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QAbstractSpinBox_qt_metacast(QAbstractSpinBox* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSpinBox_selectAll(QAbstractSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSpinBox_setAccelerated(QAbstractSpinBox* this_ptr, bool on);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSpinBox_setButtonSymbols(QAbstractSpinBox* this_ptr, QAbstractSpinBox::ButtonSymbols bs);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSpinBox_setCorrectionMode(QAbstractSpinBox* this_ptr, QAbstractSpinBox::CorrectionMode cm);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSpinBox_setFrame(QAbstractSpinBox* this_ptr, bool arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSpinBox_setGroupSeparatorShown(QAbstractSpinBox* this_ptr, bool shown);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSpinBox_setKeyboardTracking(QAbstractSpinBox* this_ptr, bool kt);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSpinBox_setReadOnly(QAbstractSpinBox* this_ptr, bool r);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSpinBox_setSpecialValueText(QAbstractSpinBox* this_ptr, const QString* txt);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSpinBox_setWrapping(QAbstractSpinBox* this_ptr, bool w);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSpinBox_sizeHint_to_output(const QAbstractSpinBox* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSpinBox_specialValueText_to_output(const QAbstractSpinBox* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSpinBox_stepBy(QAbstractSpinBox* this_ptr, int steps);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSpinBox_stepDown(QAbstractSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSpinBox_stepUp(QAbstractSpinBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSpinBox_text_to_output(const QAbstractSpinBox* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSpinBox_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QAbstractSpinBox_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QAbstractSpinBox_wrapping(const QAbstractSpinBox* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QABSTRACTSPINBOX_H

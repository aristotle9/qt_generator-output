#ifndef QT_WIDGETS_C_QMESSAGEBOX_H
#define QT_WIDGETS_C_QMESSAGEBOX_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QMessageBox* qt_widgets_c_QMessageBox_G_dynamic_cast_QMessageBox_ptr_QDialog(QDialog* ptr);
QT_WIDGETS_C_EXPORT QMessageBox* qt_widgets_c_QMessageBox_G_dynamic_cast_QMessageBox_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QDialog* qt_widgets_c_QMessageBox_G_static_cast_QDialog_ptr(QMessageBox* ptr);
QT_WIDGETS_C_EXPORT QMessageBox* qt_widgets_c_QMessageBox_G_static_cast_QMessageBox_ptr_QDialog(QDialog* ptr);
QT_WIDGETS_C_EXPORT QMessageBox* qt_widgets_c_QMessageBox_G_static_cast_QMessageBox_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QMessageBox* qt_widgets_c_QMessageBox_G_static_cast_QMessageBox_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QMessageBox* qt_widgets_c_QMessageBox_G_static_cast_QMessageBox_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QMessageBox_G_static_cast_QObject_ptr(QMessageBox* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QMessageBox_G_static_cast_QPaintDevice_ptr(QMessageBox* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QMessageBox_G_static_cast_QWidget_ptr(QMessageBox* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_about(QWidget* parent, const QString* title, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_aboutQt_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_aboutQt_parent_title(QWidget* parent, const QString* title);
QT_WIDGETS_C_EXPORT QPushButton* qt_widgets_c_QMessageBox_addButton_button(QMessageBox* this_ptr, QMessageBox::StandardButton button);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_addButton_button_role(QMessageBox* this_ptr, QAbstractButton* button, QMessageBox::ButtonRole role);
QT_WIDGETS_C_EXPORT QPushButton* qt_widgets_c_QMessageBox_addButton_text_role(QMessageBox* this_ptr, const QString* text, QMessageBox::ButtonRole role);
QT_WIDGETS_C_EXPORT QAbstractButton* qt_widgets_c_QMessageBox_button(const QMessageBox* this_ptr, QMessageBox::StandardButton which);
QT_WIDGETS_C_EXPORT QMessageBox::ButtonRole qt_widgets_c_QMessageBox_buttonRole(const QMessageBox* this_ptr, QAbstractButton* button);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_buttonText_to_output(const QMessageBox* this_ptr, int button, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_buttons_to_output(const QMessageBox* this_ptr, QList< QAbstractButton* >* output);
QT_WIDGETS_C_EXPORT QCheckBox* qt_widgets_c_QMessageBox_checkBox(const QMessageBox* this_ptr);
QT_WIDGETS_C_EXPORT QAbstractButton* qt_widgets_c_QMessageBox_clickedButton(const QMessageBox* this_ptr);
QT_WIDGETS_C_EXPORT QMessageBox::StandardButton qt_widgets_c_QMessageBox_critical_QWidget_QString_QString(QWidget* parent, const QString* title, const QString* text);
QT_WIDGETS_C_EXPORT QMessageBox::StandardButton qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_QFlags_QMessageBox_StandardButton(QWidget* parent, const QString* title, const QString* text, unsigned int buttons);
QT_WIDGETS_C_EXPORT QMessageBox::StandardButton qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_QFlags_QMessageBox_StandardButton_QMessageBox_StandardButton(QWidget* parent, const QString* title, const QString* text, unsigned int buttons, QMessageBox::StandardButton defaultButton);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_QMessageBox_StandardButton_QMessageBox_StandardButton(QWidget* parent, const QString* title, const QString* text, QMessageBox::StandardButton button0, QMessageBox::StandardButton button1);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_QString(QWidget* parent, const QString* title, const QString* text, const QString* button0Text);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_QString_QString(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_QString_QString_QString(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text, const QString* button2Text);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_QString_QString_QString_int(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text, const QString* button2Text, int defaultButtonNumber);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_QString_QString_QString_int_int(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text, const QString* button2Text, int defaultButtonNumber, int escapeButtonNumber);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_int_int(QWidget* parent, const QString* title, const QString* text, int button0, int button1);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_critical_QWidget_QString_QString_int_int_int(QWidget* parent, const QString* title, const QString* text, int button0, int button1, int button2);
QT_WIDGETS_C_EXPORT QPushButton* qt_widgets_c_QMessageBox_defaultButton(const QMessageBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_delete(QMessageBox* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_detailedText_to_output(const QMessageBox* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT QAbstractButton* qt_widgets_c_QMessageBox_escapeButton(const QMessageBox* this_ptr);
QT_WIDGETS_C_EXPORT QMessageBox::Icon qt_widgets_c_QMessageBox_icon(const QMessageBox* this_ptr);
QT_WIDGETS_C_EXPORT QPixmap* qt_widgets_c_QMessageBox_iconPixmap_as_ptr(const QMessageBox* this_ptr);
QT_WIDGETS_C_EXPORT QMessageBox::StandardButton qt_widgets_c_QMessageBox_information_QWidget_QString_QString(QWidget* parent, const QString* title, const QString* text);
QT_WIDGETS_C_EXPORT QMessageBox::StandardButton qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QFlags_QMessageBox_StandardButton(QWidget* parent, const QString* title, const QString* text, unsigned int buttons);
QT_WIDGETS_C_EXPORT QMessageBox::StandardButton qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QFlags_QMessageBox_StandardButton_QMessageBox_StandardButton(QWidget* parent, const QString* title, const QString* text, unsigned int buttons, QMessageBox::StandardButton defaultButton);
QT_WIDGETS_C_EXPORT QMessageBox::StandardButton qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QMessageBox_StandardButton(QWidget* parent, const QString* title, const QString* text, QMessageBox::StandardButton button0);
QT_WIDGETS_C_EXPORT QMessageBox::StandardButton qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QMessageBox_StandardButton_QMessageBox_StandardButton(QWidget* parent, const QString* title, const QString* text, QMessageBox::StandardButton button0, QMessageBox::StandardButton button1);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QString(QWidget* parent, const QString* title, const QString* text, const QString* button0Text);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QString_QString(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QString_QString_QString(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text, const QString* button2Text);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QString_QString_QString_int(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text, const QString* button2Text, int defaultButtonNumber);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_information_QWidget_QString_QString_QString_QString_QString_int_int(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text, const QString* button2Text, int defaultButtonNumber, int escapeButtonNumber);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_information_QWidget_QString_QString_int(QWidget* parent, const QString* title, const QString* text, int button0);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_information_QWidget_QString_QString_int_int(QWidget* parent, const QString* title, const QString* text, int button0, int button1);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_information_QWidget_QString_QString_int_int_int(QWidget* parent, const QString* title, const QString* text, int button0, int button1, int button2);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_informativeText_to_output(const QMessageBox* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QMessageBox_metaObject(const QMessageBox* this_ptr);
QT_WIDGETS_C_EXPORT QMessageBox* qt_widgets_c_QMessageBox_new_no_args();
QT_WIDGETS_C_EXPORT QMessageBox* qt_widgets_c_QMessageBox_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_open(QMessageBox* this_ptr, QObject* receiver, const char* member);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_qt_metacall(QMessageBox* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QMessageBox_qt_metacast(QMessageBox* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT QMessageBox::StandardButton qt_widgets_c_QMessageBox_question_QWidget_QString_QString(QWidget* parent, const QString* title, const QString* text);
QT_WIDGETS_C_EXPORT QMessageBox::StandardButton qt_widgets_c_QMessageBox_question_QWidget_QString_QString_QFlags_QMessageBox_StandardButton(QWidget* parent, const QString* title, const QString* text, unsigned int buttons);
QT_WIDGETS_C_EXPORT QMessageBox::StandardButton qt_widgets_c_QMessageBox_question_QWidget_QString_QString_QFlags_QMessageBox_StandardButton_QMessageBox_StandardButton(QWidget* parent, const QString* title, const QString* text, unsigned int buttons, QMessageBox::StandardButton defaultButton);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_question_QWidget_QString_QString_QMessageBox_StandardButton_QMessageBox_StandardButton(QWidget* parent, const QString* title, const QString* text, QMessageBox::StandardButton button0, QMessageBox::StandardButton button1);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_question_QWidget_QString_QString_QString(QWidget* parent, const QString* title, const QString* text, const QString* button0Text);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_question_QWidget_QString_QString_QString_QString(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_question_QWidget_QString_QString_QString_QString_QString(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text, const QString* button2Text);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_question_QWidget_QString_QString_QString_QString_QString_int(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text, const QString* button2Text, int defaultButtonNumber);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_question_QWidget_QString_QString_QString_QString_QString_int_int(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text, const QString* button2Text, int defaultButtonNumber, int escapeButtonNumber);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_question_QWidget_QString_QString_int(QWidget* parent, const QString* title, const QString* text, int button0);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_question_QWidget_QString_QString_int_int(QWidget* parent, const QString* title, const QString* text, int button0, int button1);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_question_QWidget_QString_QString_int_int_int(QWidget* parent, const QString* title, const QString* text, int button0, int button1, int button2);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_removeButton(QMessageBox* this_ptr, QAbstractButton* button);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_setButtonText(QMessageBox* this_ptr, int button, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_setCheckBox(QMessageBox* this_ptr, QCheckBox* cb);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_setDefaultButton_QMessageBox_StandardButton(QMessageBox* this_ptr, QMessageBox::StandardButton button);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_setDefaultButton_QPushButton(QMessageBox* this_ptr, QPushButton* button);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_setDetailedText(QMessageBox* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_setEscapeButton_QAbstractButton(QMessageBox* this_ptr, QAbstractButton* button);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_setEscapeButton_QMessageBox_StandardButton(QMessageBox* this_ptr, QMessageBox::StandardButton button);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_setIcon(QMessageBox* this_ptr, QMessageBox::Icon arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_setIconPixmap(QMessageBox* this_ptr, const QPixmap* pixmap);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_setInformativeText(QMessageBox* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_setStandardButtons(QMessageBox* this_ptr, unsigned int buttons);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_setText(QMessageBox* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_setTextFormat(QMessageBox* this_ptr, const Qt::TextFormat* format);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_setWindowModality(QMessageBox* this_ptr, const Qt::WindowModality* windowModality);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_setWindowTitle(QMessageBox* this_ptr, const QString* title);
QT_WIDGETS_C_EXPORT QMessageBox::StandardButton qt_widgets_c_QMessageBox_standardButton(const QMessageBox* this_ptr, QAbstractButton* button);
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QMessageBox_standardButtons(const QMessageBox* this_ptr);
QT_WIDGETS_C_EXPORT QPixmap* qt_widgets_c_QMessageBox_standardIcon_as_ptr(QMessageBox::Icon icon);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_text_to_output(const QMessageBox* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QMessageBox_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT QMessageBox::StandardButton qt_widgets_c_QMessageBox_warning_QWidget_QString_QString(QWidget* parent, const QString* title, const QString* text);
QT_WIDGETS_C_EXPORT QMessageBox::StandardButton qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_QFlags_QMessageBox_StandardButton(QWidget* parent, const QString* title, const QString* text, unsigned int buttons);
QT_WIDGETS_C_EXPORT QMessageBox::StandardButton qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_QFlags_QMessageBox_StandardButton_QMessageBox_StandardButton(QWidget* parent, const QString* title, const QString* text, unsigned int buttons, QMessageBox::StandardButton defaultButton);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_QMessageBox_StandardButton_QMessageBox_StandardButton(QWidget* parent, const QString* title, const QString* text, QMessageBox::StandardButton button0, QMessageBox::StandardButton button1);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_QString(QWidget* parent, const QString* title, const QString* text, const QString* button0Text);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_QString_QString(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_QString_QString_QString(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text, const QString* button2Text);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_QString_QString_QString_int(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text, const QString* button2Text, int defaultButtonNumber);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_QString_QString_QString_int_int(QWidget* parent, const QString* title, const QString* text, const QString* button0Text, const QString* button1Text, const QString* button2Text, int defaultButtonNumber, int escapeButtonNumber);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_int_int(QWidget* parent, const QString* title, const QString* text, int button0, int button1);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QMessageBox_warning_QWidget_QString_QString_int_int_int(QWidget* parent, const QString* title, const QString* text, int button0, int button1, int button2);

} // extern "C"

#endif // QT_WIDGETS_C_QMESSAGEBOX_H

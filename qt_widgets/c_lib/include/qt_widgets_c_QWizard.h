#ifndef QT_WIDGETS_C_QWIZARD_H
#define QT_WIDGETS_C_QWIZARD_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QWizard* qt_widgets_c_QWizard_G_dynamic_cast_QWizard_ptr_QDialog(QDialog* ptr);
QT_WIDGETS_C_EXPORT QWizard* qt_widgets_c_QWizard_G_dynamic_cast_QWizard_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QDialog* qt_widgets_c_QWizard_G_static_cast_QDialog_ptr(QWizard* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QWizard_G_static_cast_QObject_ptr(QWizard* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QWizard_G_static_cast_QPaintDevice_ptr(QWizard* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QWizard_G_static_cast_QWidget_ptr(QWizard* ptr);
QT_WIDGETS_C_EXPORT QWizard* qt_widgets_c_QWizard_G_static_cast_QWizard_ptr_QDialog(QDialog* ptr);
QT_WIDGETS_C_EXPORT QWizard* qt_widgets_c_QWizard_G_static_cast_QWizard_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QWizard* qt_widgets_c_QWizard_G_static_cast_QWizard_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QWizard* qt_widgets_c_QWizard_G_static_cast_QWizard_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QWizard_addPage(QWizard* this_ptr, QWizardPage* page);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_back(QWizard* this_ptr);
QT_WIDGETS_C_EXPORT QAbstractButton* qt_widgets_c_QWizard_button(const QWizard* this_ptr, QWizard::WizardButton which);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_buttonText_to_output(const QWizard* this_ptr, QWizard::WizardButton which, QString* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QWizard_currentId(const QWizard* this_ptr);
QT_WIDGETS_C_EXPORT QWizardPage* qt_widgets_c_QWizard_currentPage(const QWizard* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_delete(QWizard* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_field_to_output(const QWizard* this_ptr, const QString* name, QVariant* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWizard_hasVisitedPage(const QWizard* this_ptr, int id);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QWizard_metaObject(const QWizard* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_next(QWizard* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QWizard_nextId(const QWizard* this_ptr);
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QWizard_options(const QWizard* this_ptr);
QT_WIDGETS_C_EXPORT QWizardPage* qt_widgets_c_QWizard_page(const QWizard* this_ptr, int id);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_pageIds_to_output(const QWizard* this_ptr, QList< int >* output);
QT_WIDGETS_C_EXPORT QPixmap* qt_widgets_c_QWizard_pixmap_as_ptr(const QWizard* this_ptr, QWizard::WizardPixmap which);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QWizard_qt_metacall(QWizard* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QWizard_qt_metacast(QWizard* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_removePage(QWizard* this_ptr, int id);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_restart(QWizard* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_setButton(QWizard* this_ptr, QWizard::WizardButton which, QAbstractButton* button);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_setButtonLayout(QWizard* this_ptr, const QList< QWizard::WizardButton >* layout);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_setButtonText(QWizard* this_ptr, QWizard::WizardButton which, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_setDefaultProperty(QWizard* this_ptr, const char* className, const char* property, const char* changedSignal);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_setField(QWizard* this_ptr, const QString* name, const QVariant* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_setOption_option(QWizard* this_ptr, QWizard::WizardOption option);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_setOption_option_on(QWizard* this_ptr, QWizard::WizardOption option, bool on);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_setOptions(QWizard* this_ptr, unsigned int options);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_setPage(QWizard* this_ptr, int id, QWizardPage* page);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_setPixmap(QWizard* this_ptr, QWizard::WizardPixmap which, const QPixmap* pixmap);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_setSideWidget(QWizard* this_ptr, QWidget* widget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_setStartId(QWizard* this_ptr, int id);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_setSubTitleFormat(QWizard* this_ptr, const Qt::TextFormat* format);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_setTitleFormat(QWizard* this_ptr, const Qt::TextFormat* format);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_setVisible(QWizard* this_ptr, bool visible);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_setWizardStyle(QWizard* this_ptr, QWizard::WizardStyle style);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QWizard_sideWidget(const QWizard* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_sizeHint_to_output(const QWizard* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QWizard_startId(const QWizard* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWizard_testOption(const QWizard* this_ptr, QWizard::WizardOption option);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWizard_validateCurrentPage(QWizard* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizard_visitedPages_to_output(const QWizard* this_ptr, QList< int >* output);
QT_WIDGETS_C_EXPORT QWizard::WizardStyle qt_widgets_c_QWizard_wizardStyle(const QWizard* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QWIZARD_H

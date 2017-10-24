#ifndef QT_WIDGETS_C_QWIZARDPAGE_H
#define QT_WIDGETS_C_QWIZARDPAGE_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QWizardPage* qt_widgets_c_QWizardPage_G_dynamic_cast_QWizardPage_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QWizardPage_G_static_cast_QObject_ptr(QWizardPage* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QWizardPage_G_static_cast_QPaintDevice_ptr(QWizardPage* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QWizardPage_G_static_cast_QWidget_ptr(QWizardPage* ptr);
QT_WIDGETS_C_EXPORT QWizardPage* qt_widgets_c_QWizardPage_G_static_cast_QWizardPage_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QWizardPage* qt_widgets_c_QWizardPage_G_static_cast_QWizardPage_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QWizardPage* qt_widgets_c_QWizardPage_G_static_cast_QWizardPage_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizardPage_buttonText_to_output(const QWizardPage* this_ptr, const QWizard::WizardButton* which, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizardPage_cleanupPage(QWizardPage* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizardPage_delete(QWizardPage* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizardPage_initializePage(QWizardPage* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWizardPage_isCommitPage(const QWizardPage* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWizardPage_isComplete(const QWizardPage* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWizardPage_isFinalPage(const QWizardPage* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QWizardPage_metaObject(const QWizardPage* this_ptr);
QT_WIDGETS_C_EXPORT QWizardPage* qt_widgets_c_QWizardPage_new_no_args();
QT_WIDGETS_C_EXPORT QWizardPage* qt_widgets_c_QWizardPage_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QWizardPage_nextId(const QWizardPage* this_ptr);
QT_WIDGETS_C_EXPORT QPixmap* qt_widgets_c_QWizardPage_pixmap_as_ptr(const QWizardPage* this_ptr, const QWizard::WizardPixmap* which);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QWizardPage_qt_metacall(QWizardPage* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QWizardPage_qt_metacast(QWizardPage* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizardPage_setButtonText(QWizardPage* this_ptr, const QWizard::WizardButton* which, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizardPage_setCommitPage(QWizardPage* this_ptr, bool commitPage);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizardPage_setFinalPage(QWizardPage* this_ptr, bool finalPage);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizardPage_setPixmap(QWizardPage* this_ptr, const QWizard::WizardPixmap* which, const QPixmap* pixmap);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizardPage_setSubTitle(QWizardPage* this_ptr, const QString* subTitle);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizardPage_setTitle(QWizardPage* this_ptr, const QString* title);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizardPage_subTitle_to_output(const QWizardPage* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizardPage_title_to_output(const QWizardPage* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizardPage_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QWizardPage_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QWizardPage_validatePage(QWizardPage* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QWIZARDPAGE_H

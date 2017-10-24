#include "qt_widgets_c_QFileDialog.h"

QFileDialog* qt_widgets_c_QFileDialog_G_dynamic_cast_QFileDialog_ptr_QDialog(QDialog* ptr) {
  return dynamic_cast<QFileDialog*>(ptr);
}

QFileDialog* qt_widgets_c_QFileDialog_G_dynamic_cast_QFileDialog_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QFileDialog*>(ptr);
}

QDialog* qt_widgets_c_QFileDialog_G_static_cast_QDialog_ptr(QFileDialog* ptr) {
  return static_cast<QDialog*>(ptr);
}

QFileDialog* qt_widgets_c_QFileDialog_G_static_cast_QFileDialog_ptr_QDialog(QDialog* ptr) {
  return static_cast<QFileDialog*>(ptr);
}

QFileDialog* qt_widgets_c_QFileDialog_G_static_cast_QFileDialog_ptr_QObject(QObject* ptr) {
  return static_cast<QFileDialog*>(ptr);
}

QFileDialog* qt_widgets_c_QFileDialog_G_static_cast_QFileDialog_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QFileDialog*>(ptr);
}

QFileDialog* qt_widgets_c_QFileDialog_G_static_cast_QFileDialog_ptr_QWidget(QWidget* ptr) {
  return static_cast<QFileDialog*>(ptr);
}

QObject* qt_widgets_c_QFileDialog_G_static_cast_QObject_ptr(QFileDialog* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QFileDialog_G_static_cast_QPaintDevice_ptr(QFileDialog* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QFileDialog_G_static_cast_QWidget_ptr(QFileDialog* ptr) {
  return static_cast<QWidget*>(ptr);
}

QFileDialog::AcceptMode qt_widgets_c_QFileDialog_acceptMode(const QFileDialog* this_ptr) {
  return this_ptr->acceptMode();
}

bool qt_widgets_c_QFileDialog_confirmOverwrite(const QFileDialog* this_ptr) {
  return this_ptr->confirmOverwrite();
}

void qt_widgets_c_QFileDialog_defaultSuffix_to_output(const QFileDialog* this_ptr, QString* output) {
  new(output) QString(this_ptr->defaultSuffix());
}

void qt_widgets_c_QFileDialog_delete(QFileDialog* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QFileDialog_directoryUrl_to_output(const QFileDialog* this_ptr, QUrl* output) {
  new(output) QUrl(this_ptr->directoryUrl());
}

void qt_widgets_c_QFileDialog_directory_to_output(const QFileDialog* this_ptr, QDir* output) {
  new(output) QDir(this_ptr->directory());
}

QFileDialog::FileMode qt_widgets_c_QFileDialog_fileMode(const QFileDialog* this_ptr) {
  return this_ptr->fileMode();
}

void qt_widgets_c_QFileDialog_getExistingDirectoryUrl_to_output_no_args(QUrl* output) {
  new(output) QUrl(QFileDialog::getExistingDirectoryUrl());
}

void qt_widgets_c_QFileDialog_getExistingDirectoryUrl_to_output_parent(QWidget* parent, QUrl* output) {
  new(output) QUrl(QFileDialog::getExistingDirectoryUrl(parent));
}

void qt_widgets_c_QFileDialog_getExistingDirectoryUrl_to_output_parent_caption(QWidget* parent, const QString* caption, QUrl* output) {
  new(output) QUrl(QFileDialog::getExistingDirectoryUrl(parent, *caption));
}

void qt_widgets_c_QFileDialog_getExistingDirectoryUrl_to_output_parent_caption_dir(QWidget* parent, const QString* caption, const QUrl* dir, QUrl* output) {
  new(output) QUrl(QFileDialog::getExistingDirectoryUrl(parent, *caption, *dir));
}

void qt_widgets_c_QFileDialog_getExistingDirectoryUrl_to_output_parent_caption_dir_options(QWidget* parent, const QString* caption, const QUrl* dir, unsigned int options, QUrl* output) {
  new(output) QUrl(QFileDialog::getExistingDirectoryUrl(parent, *caption, *dir, QFlags< QFileDialog::Option >(options)));
}

void qt_widgets_c_QFileDialog_getExistingDirectoryUrl_to_output_parent_caption_dir_options_supportedSchemes(QWidget* parent, const QString* caption, const QUrl* dir, unsigned int options, const QStringList* supportedSchemes, QUrl* output) {
  new(output) QUrl(QFileDialog::getExistingDirectoryUrl(parent, *caption, *dir, QFlags< QFileDialog::Option >(options), *supportedSchemes));
}

void qt_widgets_c_QFileDialog_getExistingDirectory_to_output_no_args(QString* output) {
  new(output) QString(QFileDialog::getExistingDirectory());
}

void qt_widgets_c_QFileDialog_getExistingDirectory_to_output_parent(QWidget* parent, QString* output) {
  new(output) QString(QFileDialog::getExistingDirectory(parent));
}

void qt_widgets_c_QFileDialog_getExistingDirectory_to_output_parent_caption(QWidget* parent, const QString* caption, QString* output) {
  new(output) QString(QFileDialog::getExistingDirectory(parent, *caption));
}

void qt_widgets_c_QFileDialog_getExistingDirectory_to_output_parent_caption_dir(QWidget* parent, const QString* caption, const QString* dir, QString* output) {
  new(output) QString(QFileDialog::getExistingDirectory(parent, *caption, *dir));
}

void qt_widgets_c_QFileDialog_getExistingDirectory_to_output_parent_caption_dir_options(QWidget* parent, const QString* caption, const QString* dir, unsigned int options, QString* output) {
  new(output) QString(QFileDialog::getExistingDirectory(parent, *caption, *dir, QFlags< QFileDialog::Option >(options)));
}

void qt_widgets_c_QFileDialog_getOpenFileName_to_output_no_args(QString* output) {
  new(output) QString(QFileDialog::getOpenFileName());
}

void qt_widgets_c_QFileDialog_getOpenFileName_to_output_parent(QWidget* parent, QString* output) {
  new(output) QString(QFileDialog::getOpenFileName(parent));
}

void qt_widgets_c_QFileDialog_getOpenFileName_to_output_parent_caption(QWidget* parent, const QString* caption, QString* output) {
  new(output) QString(QFileDialog::getOpenFileName(parent, *caption));
}

void qt_widgets_c_QFileDialog_getOpenFileName_to_output_parent_caption_dir(QWidget* parent, const QString* caption, const QString* dir, QString* output) {
  new(output) QString(QFileDialog::getOpenFileName(parent, *caption, *dir));
}

void qt_widgets_c_QFileDialog_getOpenFileName_to_output_parent_caption_dir_filter(QWidget* parent, const QString* caption, const QString* dir, const QString* filter, QString* output) {
  new(output) QString(QFileDialog::getOpenFileName(parent, *caption, *dir, *filter));
}

void qt_widgets_c_QFileDialog_getOpenFileName_to_output_parent_caption_dir_filter_selectedFilter(QWidget* parent, const QString* caption, const QString* dir, const QString* filter, QString* selectedFilter, QString* output) {
  new(output) QString(QFileDialog::getOpenFileName(parent, *caption, *dir, *filter, selectedFilter));
}

void qt_widgets_c_QFileDialog_getOpenFileName_to_output_parent_caption_dir_filter_selectedFilter_options(QWidget* parent, const QString* caption, const QString* dir, const QString* filter, QString* selectedFilter, unsigned int options, QString* output) {
  new(output) QString(QFileDialog::getOpenFileName(parent, *caption, *dir, *filter, selectedFilter, QFlags< QFileDialog::Option >(options)));
}

void qt_widgets_c_QFileDialog_getOpenFileNames_to_output_no_args(QStringList* output) {
  new(output) QStringList(QFileDialog::getOpenFileNames());
}

void qt_widgets_c_QFileDialog_getOpenFileNames_to_output_parent(QWidget* parent, QStringList* output) {
  new(output) QStringList(QFileDialog::getOpenFileNames(parent));
}

void qt_widgets_c_QFileDialog_getOpenFileNames_to_output_parent_caption(QWidget* parent, const QString* caption, QStringList* output) {
  new(output) QStringList(QFileDialog::getOpenFileNames(parent, *caption));
}

void qt_widgets_c_QFileDialog_getOpenFileNames_to_output_parent_caption_dir(QWidget* parent, const QString* caption, const QString* dir, QStringList* output) {
  new(output) QStringList(QFileDialog::getOpenFileNames(parent, *caption, *dir));
}

void qt_widgets_c_QFileDialog_getOpenFileNames_to_output_parent_caption_dir_filter(QWidget* parent, const QString* caption, const QString* dir, const QString* filter, QStringList* output) {
  new(output) QStringList(QFileDialog::getOpenFileNames(parent, *caption, *dir, *filter));
}

void qt_widgets_c_QFileDialog_getOpenFileNames_to_output_parent_caption_dir_filter_selectedFilter(QWidget* parent, const QString* caption, const QString* dir, const QString* filter, QString* selectedFilter, QStringList* output) {
  new(output) QStringList(QFileDialog::getOpenFileNames(parent, *caption, *dir, *filter, selectedFilter));
}

void qt_widgets_c_QFileDialog_getOpenFileNames_to_output_parent_caption_dir_filter_selectedFilter_options(QWidget* parent, const QString* caption, const QString* dir, const QString* filter, QString* selectedFilter, unsigned int options, QStringList* output) {
  new(output) QStringList(QFileDialog::getOpenFileNames(parent, *caption, *dir, *filter, selectedFilter, QFlags< QFileDialog::Option >(options)));
}

void qt_widgets_c_QFileDialog_getOpenFileUrl_to_output_no_args(QUrl* output) {
  new(output) QUrl(QFileDialog::getOpenFileUrl());
}

void qt_widgets_c_QFileDialog_getOpenFileUrl_to_output_parent(QWidget* parent, QUrl* output) {
  new(output) QUrl(QFileDialog::getOpenFileUrl(parent));
}

void qt_widgets_c_QFileDialog_getOpenFileUrl_to_output_parent_caption(QWidget* parent, const QString* caption, QUrl* output) {
  new(output) QUrl(QFileDialog::getOpenFileUrl(parent, *caption));
}

void qt_widgets_c_QFileDialog_getOpenFileUrl_to_output_parent_caption_dir(QWidget* parent, const QString* caption, const QUrl* dir, QUrl* output) {
  new(output) QUrl(QFileDialog::getOpenFileUrl(parent, *caption, *dir));
}

void qt_widgets_c_QFileDialog_getOpenFileUrl_to_output_parent_caption_dir_filter(QWidget* parent, const QString* caption, const QUrl* dir, const QString* filter, QUrl* output) {
  new(output) QUrl(QFileDialog::getOpenFileUrl(parent, *caption, *dir, *filter));
}

void qt_widgets_c_QFileDialog_getOpenFileUrl_to_output_parent_caption_dir_filter_selectedFilter(QWidget* parent, const QString* caption, const QUrl* dir, const QString* filter, QString* selectedFilter, QUrl* output) {
  new(output) QUrl(QFileDialog::getOpenFileUrl(parent, *caption, *dir, *filter, selectedFilter));
}

void qt_widgets_c_QFileDialog_getOpenFileUrl_to_output_parent_caption_dir_filter_selectedFilter_options(QWidget* parent, const QString* caption, const QUrl* dir, const QString* filter, QString* selectedFilter, unsigned int options, QUrl* output) {
  new(output) QUrl(QFileDialog::getOpenFileUrl(parent, *caption, *dir, *filter, selectedFilter, QFlags< QFileDialog::Option >(options)));
}

void qt_widgets_c_QFileDialog_getOpenFileUrl_to_output_parent_caption_dir_filter_selectedFilter_options_supportedSchemes(QWidget* parent, const QString* caption, const QUrl* dir, const QString* filter, QString* selectedFilter, unsigned int options, const QStringList* supportedSchemes, QUrl* output) {
  new(output) QUrl(QFileDialog::getOpenFileUrl(parent, *caption, *dir, *filter, selectedFilter, QFlags< QFileDialog::Option >(options), *supportedSchemes));
}

void qt_widgets_c_QFileDialog_getOpenFileUrls_to_output_no_args(QList< QUrl >* output) {
  new(output) QList< QUrl >(QFileDialog::getOpenFileUrls());
}

void qt_widgets_c_QFileDialog_getOpenFileUrls_to_output_parent(QWidget* parent, QList< QUrl >* output) {
  new(output) QList< QUrl >(QFileDialog::getOpenFileUrls(parent));
}

void qt_widgets_c_QFileDialog_getOpenFileUrls_to_output_parent_caption(QWidget* parent, const QString* caption, QList< QUrl >* output) {
  new(output) QList< QUrl >(QFileDialog::getOpenFileUrls(parent, *caption));
}

void qt_widgets_c_QFileDialog_getOpenFileUrls_to_output_parent_caption_dir(QWidget* parent, const QString* caption, const QUrl* dir, QList< QUrl >* output) {
  new(output) QList< QUrl >(QFileDialog::getOpenFileUrls(parent, *caption, *dir));
}

void qt_widgets_c_QFileDialog_getOpenFileUrls_to_output_parent_caption_dir_filter(QWidget* parent, const QString* caption, const QUrl* dir, const QString* filter, QList< QUrl >* output) {
  new(output) QList< QUrl >(QFileDialog::getOpenFileUrls(parent, *caption, *dir, *filter));
}

void qt_widgets_c_QFileDialog_getOpenFileUrls_to_output_parent_caption_dir_filter_selectedFilter(QWidget* parent, const QString* caption, const QUrl* dir, const QString* filter, QString* selectedFilter, QList< QUrl >* output) {
  new(output) QList< QUrl >(QFileDialog::getOpenFileUrls(parent, *caption, *dir, *filter, selectedFilter));
}

void qt_widgets_c_QFileDialog_getOpenFileUrls_to_output_parent_caption_dir_filter_selectedFilter_options(QWidget* parent, const QString* caption, const QUrl* dir, const QString* filter, QString* selectedFilter, unsigned int options, QList< QUrl >* output) {
  new(output) QList< QUrl >(QFileDialog::getOpenFileUrls(parent, *caption, *dir, *filter, selectedFilter, QFlags< QFileDialog::Option >(options)));
}

void qt_widgets_c_QFileDialog_getOpenFileUrls_to_output_parent_caption_dir_filter_selectedFilter_options_supportedSchemes(QWidget* parent, const QString* caption, const QUrl* dir, const QString* filter, QString* selectedFilter, unsigned int options, const QStringList* supportedSchemes, QList< QUrl >* output) {
  new(output) QList< QUrl >(QFileDialog::getOpenFileUrls(parent, *caption, *dir, *filter, selectedFilter, QFlags< QFileDialog::Option >(options), *supportedSchemes));
}

void qt_widgets_c_QFileDialog_getSaveFileName_to_output_no_args(QString* output) {
  new(output) QString(QFileDialog::getSaveFileName());
}

void qt_widgets_c_QFileDialog_getSaveFileName_to_output_parent(QWidget* parent, QString* output) {
  new(output) QString(QFileDialog::getSaveFileName(parent));
}

void qt_widgets_c_QFileDialog_getSaveFileName_to_output_parent_caption(QWidget* parent, const QString* caption, QString* output) {
  new(output) QString(QFileDialog::getSaveFileName(parent, *caption));
}

void qt_widgets_c_QFileDialog_getSaveFileName_to_output_parent_caption_dir(QWidget* parent, const QString* caption, const QString* dir, QString* output) {
  new(output) QString(QFileDialog::getSaveFileName(parent, *caption, *dir));
}

void qt_widgets_c_QFileDialog_getSaveFileName_to_output_parent_caption_dir_filter(QWidget* parent, const QString* caption, const QString* dir, const QString* filter, QString* output) {
  new(output) QString(QFileDialog::getSaveFileName(parent, *caption, *dir, *filter));
}

void qt_widgets_c_QFileDialog_getSaveFileName_to_output_parent_caption_dir_filter_selectedFilter(QWidget* parent, const QString* caption, const QString* dir, const QString* filter, QString* selectedFilter, QString* output) {
  new(output) QString(QFileDialog::getSaveFileName(parent, *caption, *dir, *filter, selectedFilter));
}

void qt_widgets_c_QFileDialog_getSaveFileName_to_output_parent_caption_dir_filter_selectedFilter_options(QWidget* parent, const QString* caption, const QString* dir, const QString* filter, QString* selectedFilter, unsigned int options, QString* output) {
  new(output) QString(QFileDialog::getSaveFileName(parent, *caption, *dir, *filter, selectedFilter, QFlags< QFileDialog::Option >(options)));
}

void qt_widgets_c_QFileDialog_getSaveFileUrl_to_output_no_args(QUrl* output) {
  new(output) QUrl(QFileDialog::getSaveFileUrl());
}

void qt_widgets_c_QFileDialog_getSaveFileUrl_to_output_parent(QWidget* parent, QUrl* output) {
  new(output) QUrl(QFileDialog::getSaveFileUrl(parent));
}

void qt_widgets_c_QFileDialog_getSaveFileUrl_to_output_parent_caption(QWidget* parent, const QString* caption, QUrl* output) {
  new(output) QUrl(QFileDialog::getSaveFileUrl(parent, *caption));
}

void qt_widgets_c_QFileDialog_getSaveFileUrl_to_output_parent_caption_dir(QWidget* parent, const QString* caption, const QUrl* dir, QUrl* output) {
  new(output) QUrl(QFileDialog::getSaveFileUrl(parent, *caption, *dir));
}

void qt_widgets_c_QFileDialog_getSaveFileUrl_to_output_parent_caption_dir_filter(QWidget* parent, const QString* caption, const QUrl* dir, const QString* filter, QUrl* output) {
  new(output) QUrl(QFileDialog::getSaveFileUrl(parent, *caption, *dir, *filter));
}

void qt_widgets_c_QFileDialog_getSaveFileUrl_to_output_parent_caption_dir_filter_selectedFilter(QWidget* parent, const QString* caption, const QUrl* dir, const QString* filter, QString* selectedFilter, QUrl* output) {
  new(output) QUrl(QFileDialog::getSaveFileUrl(parent, *caption, *dir, *filter, selectedFilter));
}

void qt_widgets_c_QFileDialog_getSaveFileUrl_to_output_parent_caption_dir_filter_selectedFilter_options(QWidget* parent, const QString* caption, const QUrl* dir, const QString* filter, QString* selectedFilter, unsigned int options, QUrl* output) {
  new(output) QUrl(QFileDialog::getSaveFileUrl(parent, *caption, *dir, *filter, selectedFilter, QFlags< QFileDialog::Option >(options)));
}

void qt_widgets_c_QFileDialog_getSaveFileUrl_to_output_parent_caption_dir_filter_selectedFilter_options_supportedSchemes(QWidget* parent, const QString* caption, const QUrl* dir, const QString* filter, QString* selectedFilter, unsigned int options, const QStringList* supportedSchemes, QUrl* output) {
  new(output) QUrl(QFileDialog::getSaveFileUrl(parent, *caption, *dir, *filter, selectedFilter, QFlags< QFileDialog::Option >(options), *supportedSchemes));
}

void qt_widgets_c_QFileDialog_history_to_output(const QFileDialog* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->history());
}

QFileIconProvider* qt_widgets_c_QFileDialog_iconProvider(const QFileDialog* this_ptr) {
  return this_ptr->iconProvider();
}

bool qt_widgets_c_QFileDialog_isNameFilterDetailsVisible(const QFileDialog* this_ptr) {
  return this_ptr->isNameFilterDetailsVisible();
}

bool qt_widgets_c_QFileDialog_isReadOnly(const QFileDialog* this_ptr) {
  return this_ptr->isReadOnly();
}

QAbstractItemDelegate* qt_widgets_c_QFileDialog_itemDelegate(const QFileDialog* this_ptr) {
  return this_ptr->itemDelegate();
}

void qt_widgets_c_QFileDialog_labelText_to_output(const QFileDialog* this_ptr, QFileDialog::DialogLabel label, QString* output) {
  new(output) QString(this_ptr->labelText(label));
}

const QMetaObject* qt_widgets_c_QFileDialog_metaObject(const QFileDialog* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QFileDialog_mimeTypeFilters_to_output(const QFileDialog* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->mimeTypeFilters());
}

void qt_widgets_c_QFileDialog_nameFilters_to_output(const QFileDialog* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->nameFilters());
}

QFileDialog* qt_widgets_c_QFileDialog_new_no_args() {
  return new QFileDialog();
}

QFileDialog* qt_widgets_c_QFileDialog_new_parent(QWidget* parent) {
  return new QFileDialog(parent);
}

QFileDialog* qt_widgets_c_QFileDialog_new_parent_caption(QWidget* parent, const QString* caption) {
  return new QFileDialog(parent, *caption);
}

QFileDialog* qt_widgets_c_QFileDialog_new_parent_caption_directory(QWidget* parent, const QString* caption, const QString* directory) {
  return new QFileDialog(parent, *caption, *directory);
}

QFileDialog* qt_widgets_c_QFileDialog_new_parent_caption_directory_filter(QWidget* parent, const QString* caption, const QString* directory, const QString* filter) {
  return new QFileDialog(parent, *caption, *directory, *filter);
}

void qt_widgets_c_QFileDialog_open(QFileDialog* this_ptr, QObject* receiver, const char* member) {
  this_ptr->open(receiver, member);
}

unsigned int qt_widgets_c_QFileDialog_options(const QFileDialog* this_ptr) {
  return uint(this_ptr->options());
}

QAbstractProxyModel* qt_widgets_c_QFileDialog_proxyModel(const QFileDialog* this_ptr) {
  return this_ptr->proxyModel();
}

int qt_widgets_c_QFileDialog_qt_metacall(QFileDialog* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QFileDialog_qt_metacast(QFileDialog* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

bool qt_widgets_c_QFileDialog_resolveSymlinks(const QFileDialog* this_ptr) {
  return this_ptr->resolveSymlinks();
}

bool qt_widgets_c_QFileDialog_restoreState(QFileDialog* this_ptr, const QByteArray* state) {
  return this_ptr->restoreState(*state);
}

void qt_widgets_c_QFileDialog_saveState_to_output(const QFileDialog* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->saveState());
}

void qt_widgets_c_QFileDialog_selectFile(QFileDialog* this_ptr, const QString* filename) {
  this_ptr->selectFile(*filename);
}

void qt_widgets_c_QFileDialog_selectMimeTypeFilter(QFileDialog* this_ptr, const QString* filter) {
  this_ptr->selectMimeTypeFilter(*filter);
}

void qt_widgets_c_QFileDialog_selectNameFilter(QFileDialog* this_ptr, const QString* filter) {
  this_ptr->selectNameFilter(*filter);
}

void qt_widgets_c_QFileDialog_selectUrl(QFileDialog* this_ptr, const QUrl* url) {
  this_ptr->selectUrl(*url);
}

void qt_widgets_c_QFileDialog_selectedFiles_to_output(const QFileDialog* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->selectedFiles());
}

void qt_widgets_c_QFileDialog_selectedMimeTypeFilter_to_output(const QFileDialog* this_ptr, QString* output) {
  new(output) QString(this_ptr->selectedMimeTypeFilter());
}

void qt_widgets_c_QFileDialog_selectedNameFilter_to_output(const QFileDialog* this_ptr, QString* output) {
  new(output) QString(this_ptr->selectedNameFilter());
}

void qt_widgets_c_QFileDialog_selectedUrls_to_output(const QFileDialog* this_ptr, QList< QUrl >* output) {
  new(output) QList< QUrl >(this_ptr->selectedUrls());
}

void qt_widgets_c_QFileDialog_setAcceptMode(QFileDialog* this_ptr, QFileDialog::AcceptMode mode) {
  this_ptr->setAcceptMode(mode);
}

void qt_widgets_c_QFileDialog_setConfirmOverwrite(QFileDialog* this_ptr, bool enabled) {
  this_ptr->setConfirmOverwrite(enabled);
}

void qt_widgets_c_QFileDialog_setDefaultSuffix(QFileDialog* this_ptr, const QString* suffix) {
  this_ptr->setDefaultSuffix(*suffix);
}

void qt_widgets_c_QFileDialog_setDirectoryUrl(QFileDialog* this_ptr, const QUrl* directory) {
  this_ptr->setDirectoryUrl(*directory);
}

void qt_widgets_c_QFileDialog_setDirectory_QDir(QFileDialog* this_ptr, const QDir* directory) {
  this_ptr->setDirectory(*directory);
}

void qt_widgets_c_QFileDialog_setDirectory_QString(QFileDialog* this_ptr, const QString* directory) {
  this_ptr->setDirectory(*directory);
}

void qt_widgets_c_QFileDialog_setFileMode(QFileDialog* this_ptr, QFileDialog::FileMode mode) {
  this_ptr->setFileMode(mode);
}

void qt_widgets_c_QFileDialog_setHistory(QFileDialog* this_ptr, const QStringList* paths) {
  this_ptr->setHistory(*paths);
}

void qt_widgets_c_QFileDialog_setIconProvider(QFileDialog* this_ptr, QFileIconProvider* provider) {
  this_ptr->setIconProvider(provider);
}

void qt_widgets_c_QFileDialog_setItemDelegate(QFileDialog* this_ptr, QAbstractItemDelegate* delegate) {
  this_ptr->setItemDelegate(delegate);
}

void qt_widgets_c_QFileDialog_setLabelText(QFileDialog* this_ptr, QFileDialog::DialogLabel label, const QString* text) {
  this_ptr->setLabelText(label, *text);
}

void qt_widgets_c_QFileDialog_setMimeTypeFilters(QFileDialog* this_ptr, const QStringList* filters) {
  this_ptr->setMimeTypeFilters(*filters);
}

void qt_widgets_c_QFileDialog_setNameFilter(QFileDialog* this_ptr, const QString* filter) {
  this_ptr->setNameFilter(*filter);
}

void qt_widgets_c_QFileDialog_setNameFilterDetailsVisible(QFileDialog* this_ptr, bool enabled) {
  this_ptr->setNameFilterDetailsVisible(enabled);
}

void qt_widgets_c_QFileDialog_setNameFilters(QFileDialog* this_ptr, const QStringList* filters) {
  this_ptr->setNameFilters(*filters);
}

void qt_widgets_c_QFileDialog_setOption_option(QFileDialog* this_ptr, QFileDialog::Option option) {
  this_ptr->setOption(option);
}

void qt_widgets_c_QFileDialog_setOption_option_on(QFileDialog* this_ptr, QFileDialog::Option option, bool on) {
  this_ptr->setOption(option, on);
}

void qt_widgets_c_QFileDialog_setOptions(QFileDialog* this_ptr, unsigned int options) {
  this_ptr->setOptions(QFlags< QFileDialog::Option >(options));
}

void qt_widgets_c_QFileDialog_setProxyModel(QFileDialog* this_ptr, QAbstractProxyModel* model) {
  this_ptr->setProxyModel(model);
}

void qt_widgets_c_QFileDialog_setReadOnly(QFileDialog* this_ptr, bool enabled) {
  this_ptr->setReadOnly(enabled);
}

void qt_widgets_c_QFileDialog_setResolveSymlinks(QFileDialog* this_ptr, bool enabled) {
  this_ptr->setResolveSymlinks(enabled);
}

void qt_widgets_c_QFileDialog_setSidebarUrls(QFileDialog* this_ptr, const QList< QUrl >* urls) {
  this_ptr->setSidebarUrls(*urls);
}

void qt_widgets_c_QFileDialog_setSupportedSchemes(QFileDialog* this_ptr, const QStringList* schemes) {
  this_ptr->setSupportedSchemes(*schemes);
}

void qt_widgets_c_QFileDialog_setViewMode(QFileDialog* this_ptr, QFileDialog::ViewMode mode) {
  this_ptr->setViewMode(mode);
}

void qt_widgets_c_QFileDialog_setVisible(QFileDialog* this_ptr, bool visible) {
  this_ptr->setVisible(visible);
}

void qt_widgets_c_QFileDialog_sidebarUrls_to_output(const QFileDialog* this_ptr, QList< QUrl >* output) {
  new(output) QList< QUrl >(this_ptr->sidebarUrls());
}

void qt_widgets_c_QFileDialog_supportedSchemes_to_output(const QFileDialog* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->supportedSchemes());
}

bool qt_widgets_c_QFileDialog_testOption(const QFileDialog* this_ptr, QFileDialog::Option option) {
  return this_ptr->testOption(option);
}

void qt_widgets_c_QFileDialog_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QFileDialog::trUtf8(s, c, n));
}

void qt_widgets_c_QFileDialog_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QFileDialog::tr(s, c, n));
}

QFileDialog::ViewMode qt_widgets_c_QFileDialog_viewMode(const QFileDialog* this_ptr) {
  return this_ptr->viewMode();
}


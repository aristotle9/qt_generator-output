//! Bindings for [QtCore](http://doc.qt.io/qt-5/qtcore-index.html) library.
//!
//! This crate was generated using [cpp_to_rust](https://github.com/rust-qt/cpp_to_rust).
//!
//! This is work in progress, so the API will significantly change in the future.
//! Some methods are missing, and some are inconvenient to use.
//! Some methods are unsafe even though they are not marked as unsafe.
//! Users must carefully track ownership of the objects, as usual Rust guarantees
//! do not take effect. This will hopefully improve in the future.
//! Please report any issues to the
//! [issue tracker](https://github.com/rust-qt/cpp_to_rust/issues).
//!
//! This crate was generated for **Qt 5.9.1**.
//! If Qt compatibility guarantees take effect, it should be compatible
//! with future minor releases and with past and future patch releases,
//! but API added in future releases won't be available. The crate is not compatible
//! with past minor Qt releases. If you need to use a Qt version incompatible with this crate,
//! use [qt_generator](https://github.com/rust-qt/cpp_to_rust/tree/master/qt_generator/qt_generator)
//! to generate crates for your Qt version.
//!
//! # Starting up
//!
//! Qt requires an application object to be constructed at the beginning of the application.
//! (Some classes may be used without it,
//! The application object needs `argc` and `argv` available in `main` function in C++.
//! It's a bit tricky to do it in Rust, where `argc` and `argv` are not available.
//! `CoreApplication::create_and_exit` is a convenience function that performs proper
//! creation of the application object and terminates the process with the appropriate return code
//! when the application exists:
//!
//! ```rust,no_run
//! extern crate qt_core;
//! use qt_core::core_application::CoreApplication;
//!
//! fn main() {
//!   CoreApplication::create_and_exit(|app| {
//!     // initialization goes here
//!     CoreApplication::exec()
//!   })
//! }
//! ```
//!
//! Note that if you use `qt_gui` or `qt_widgets` crates, you should use
//! `qt_gui::gui_application::GuiApplication` and `qt_widgets::application::Application`
//! respectively instead of `CoreApplication`.
//!
//! `CoreApplication::exec` starts the main event loop. After your initialization code finishes,
//! any other Rust code will only be executed by Qt if you bind it to a slot:
//!
//! ```rust
//! extern crate qt_core;
//! use qt_core::core_application::CoreApplication;
//! use qt_core::variant::Variant;
//! use qt_core::variant_animation::VariantAnimation;
//! use qt_core::connection::Signal;
//! use qt_core::slots::SlotVariantRef;
//!
//! fn main() {
//!   CoreApplication::create_and_exit(|app| {
//!     let slot1 = SlotVariantRef::new(|value| {
//!       println!("value_changed: {}",
//!                value.to_string().to_std_string());
//!     });
//!
//!     let mut animation = VariantAnimation::new();
//!     animation.signals().value_changed().connect(&slot1);
//!     animation
//!         .signals()
//!         .finished()
//!         .connect(&app.slots().quit());
//!     animation.set_start_value(&Variant::new0(1));
//!     animation.set_end_value(&Variant::new0(5));
//!     animation.set_duration(5000);
//!     animation.start(());
//!     CoreApplication::exec()
//!   })
//! }
//! ```
//!
//! # Naming
//!
//! Names of Qt's classes and methods are modified according to Rust's naming conventions.
//! `Q` prefix is removed.
//! Each of Qt's include files is converted to a submodule. Original C++ names are always
//! listed in the documentation, so you may search for the Rust equivalents by original names.
//!
//! # Types and ownership
//!
//! Qt crates use two ways of handling ownership of C++ objects.
//!
//! Value-like types (`QString`, `QVector`, etc.) are represented by owned struct types
//! (e.g. `qt_core::string::String`) in Rust. The value is stored in the memory Rust itself
//! reserves for the struct. `Drop` implementation of the type will call C++ destructor to
//! ensure proper de-initialization of the value. It's not possible to transfer ownership
//! of such object to C++ side.
//!
//! All other types are stored in C++ heap and handled using raw and smart pointers.
//! Raw pointer types (e.g. `*mut qt_core::object::Object`) are the same pointers as in C++.
//! There is no guarantee that the pointer is valid at any time, and the null pointer
//! indicates lack of an object.  There is also no information about ownership in raw pointers.
//! Some C++ functions may return a raw object and expect the caller to take ownership, while
//! other functions keep ownership and may delete the object at any time. As in C++, the caller
//! needs to refer to the function's documentation and handle ownership manually.
//!
//! When it's determined that the ownership of the object belongs to the caller
//! (e.g. in a constructor), the raw pointer `*mut T` is wrapped into `cpp_utils::CppBox<T>`.
//! This struct owns the object and will delete it when dropped. It allows to move the raw pointer
//! out in case you need to transfer the ownership back to C++ side.
//!
//! References (`&T` and `&mut T`) in Qt crates are not very different from raw pointers.
//! They appear in the same places references were used in C++, but they can't hold any guarantees
//! Rust usually enforces for references. Lifetimes of references are set trivially: all input
//! references must be valid for the same lifetime, and output references have the same lifetime
//! as input references. If there are no input references, output references have `'static`
//! lifetime.
//!
//! It should be expected that raw pointers will be replaced with `CppBox`es and references,
//! and references will hold their guarantees. However, this requires manual annotation of methods,
//! so it's not easy to make this improvement.

//!

pub extern crate libc;
pub extern crate cpp_utils;

#[allow(dead_code)]
mod ffi {
  include!(concat!(env!("OUT_DIR"), "/ffi.rs"));
}

mod type_sizes {
  include!(concat!(env!("OUT_DIR"), "/type_sizes.rs"));
}

/// Entities from `QAbstractAnimation` C++ header
pub mod abstract_animation;
/// Entities from `QAbstractEventDispatcher` C++ header
pub mod abstract_event_dispatcher;
/// Entities from `QAbstractItemModel` C++ header
pub mod abstract_item_model;
/// Entities from `QAbstractListModel` C++ header
pub mod abstract_list_model;
/// Entities from `QAbstractNativeEventFilter` C++ header
pub mod abstract_native_event_filter;
/// Entities from `QAbstractProxyModel` C++ header
pub mod abstract_proxy_model;
/// Entities from `QAbstractState` C++ header
pub mod abstract_state;
/// Entities from `QAbstractTableModel` C++ header
pub mod abstract_table_model;
/// Entities from `QAbstractTransition` C++ header
pub mod abstract_transition;
/// Entities from `QtAlgorithms` C++ header
pub mod algorithms;
/// Entities from `QAnimationDriver` C++ header
pub mod animation_driver;
/// Entities from `QAnimationGroup` C++ header
pub mod animation_group;
/// Entities from `QAssociativeIterable` C++ header
pub mod associative_iterable;
/// Entities from `QBasicMutex` C++ header
pub mod basic_mutex;
/// Entities from `QBasicTimer` C++ header
pub mod basic_timer;
/// Entities from `QBitArray` C++ header
pub mod bit_array;
/// Entities from `QBitRef` C++ header
pub mod bit_ref;
/// Entities from `QBuffer` C++ header
pub mod buffer;
/// Entities from `QByteArray` C++ header
pub mod byte_array;
/// Entities from `QByteArrayMatcher` C++ header
pub mod byte_array_matcher;
/// Entities from `QByteRef` C++ header
pub mod byte_ref;
/// Entities from `QChar` C++ header
pub mod char;
/// Entities from `QCharRef` C++ header
pub mod char_ref;
/// Entities from `QChildEvent` C++ header
pub mod child_event;
/// Entities from `QCollator` C++ header
pub mod collator;
/// Entities from `QCollatorSortKey` C++ header
pub mod collator_sort_key;
/// Entities from `QCommandLineOption` C++ header
pub mod command_line_option;
/// Entities from `QCommandLineParser` C++ header
pub mod command_line_parser;
/// Entities from `QConstOverload` C++ header
pub mod const_overload;
/// Entities from `QCoreApplication` C++ header
pub mod core_application;
/// Entities from `QCryptographicHash` C++ header
pub mod cryptographic_hash;
/// Entities from `QDataStream` C++ header
pub mod data_stream;
/// Entities from `QDate` C++ header
pub mod date;
/// Entities from `QDateTime` C++ header
pub mod date_time;
/// Entities from `QDeadlineTimer` C++ header
pub mod deadline_timer;
/// Entities from `QDebug` C++ header
pub mod debug;
/// Entities from `QDebugStateSaver` C++ header
pub mod debug_state_saver;
/// Entities from `QDeferredDeleteEvent` C++ header
pub mod deferred_delete_event;
/// Entities from `QDir` C++ header
pub mod dir;
/// Entities from `QDirIterator` C++ header
pub mod dir_iterator;
/// Entities from `QDynamicPropertyChangeEvent` C++ header
pub mod dynamic_property_change_event;
/// Entities from `QEasingCurve` C++ header
pub mod easing_curve;
/// Entities from `QElapsedTimer` C++ header
pub mod elapsed_timer;
/// Entities from `QEvent` C++ header
pub mod event;
/// Entities from `QEventLoop` C++ header
pub mod event_loop;
/// Entities from `QEventLoopLocker` C++ header
pub mod event_loop_locker;
/// Entities from `QEventTransition` C++ header
pub mod event_transition;
/// Entities from `QFactoryInterface` C++ header
pub mod factory_interface;
/// Entities from `QFile` C++ header
pub mod file;
/// Entities from `QFileDevice` C++ header
pub mod file_device;
/// Entities from `QFileInfo` C++ header
pub mod file_info;
/// Entities from `QFileSelector` C++ header
pub mod file_selector;
/// Entities from `QFileSystemWatcher` C++ header
pub mod file_system_watcher;
/// Entities from `QFinalState` C++ header
pub mod final_state;
/// Entities from `QFuture` C++ header
pub mod future;
/// Entities from `QGenericArgument` C++ header
pub mod generic_argument;
/// Entities from `QGenericReturnArgument` C++ header
pub mod generic_return_argument;
/// Entities from `QHash` C++ header
pub mod hash;
/// Entities from `QHashFunctions` C++ header
pub mod hash_functions;
/// Entities from `QHistoryState` C++ header
pub mod history_state;
/// Entities from `QIdentityProxyModel` C++ header
pub mod identity_proxy_model;
/// Entities from `QInternal` C++ header
pub mod internal;
/// Entities from `QIODevice` C++ header
pub mod io_device;
/// Entities from `QItemSelection` C++ header
pub mod item_selection;
/// Entities from `QItemSelectionModel` C++ header
pub mod item_selection_model;
/// Entities from `QItemSelectionRange` C++ header
pub mod item_selection_range;
/// Entities from `QJsonArray` C++ header
pub mod json_array;
/// Entities from `QJsonDocument` C++ header
pub mod json_document;
/// Entities from `QJsonObject` C++ header
pub mod json_object;
/// Entities from `QJsonParseError` C++ header
pub mod json_parse_error;
/// Entities from `QJsonValue` C++ header
pub mod json_value;
/// Entities from `QJsonValueRef` C++ header
pub mod json_value_ref;
/// Entities from `QLatin1Char` C++ header
pub mod latin1_char;
/// Entities from `QLatin1String` C++ header
pub mod latin1_string;
/// Entities from `QLibrary` C++ header
pub mod library;
/// Entities from `QLibraryInfo` C++ header
pub mod library_info;
/// Entities from `QLine` C++ header
pub mod line;
/// Entities from `QLineF` C++ header
pub mod line_f;
/// Entities from `QList` C++ header
pub mod list;
/// Entities from `QLocale` C++ header
pub mod locale;
/// Entities from `QLockFile` C++ header
pub mod lock_file;
/// Entities from `QLoggingCategory` C++ header
pub mod logging_category;
/// Entities from `QMap` C++ header
pub mod map;
/// Entities from `QMargins` C++ header
pub mod margins;
/// Entities from `QMarginsF` C++ header
pub mod margins_f;
/// Entities from `QtMath` C++ header
pub mod math;
/// Entities from `QMessageAuthenticationCode` C++ header
pub mod message_authentication_code;
/// Entities from `QMessageLogContext` C++ header
pub mod message_log_context;
/// Entities from `QMessageLogger` C++ header
pub mod message_logger;
/// Entities from `QMetaClassInfo` C++ header
pub mod meta_class_info;
/// Entities from `QMetaEnum` C++ header
pub mod meta_enum;
/// Entities from `QMetaMethod` C++ header
pub mod meta_method;
/// Entities from `QMetaObject` C++ header
pub mod meta_object;
/// Entities from `QMetaProperty` C++ header
pub mod meta_property;
/// Entities from `QMetaType` C++ header
pub mod meta_type;
/// Entities from `QMimeData` C++ header
pub mod mime_data;
/// Entities from `QMimeDatabase` C++ header
pub mod mime_database;
/// Entities from `QMimeType` C++ header
pub mod mime_type;
/// Entities from `QModelIndex` C++ header
pub mod model_index;
/// Entities from `QMutex` C++ header
pub mod mutex;
/// Entities from `QMutexLocker` C++ header
pub mod mutex_locker;
/// Entities from `QtNumeric` C++ header
pub mod numeric;
/// Entities from `QObject` C++ header
pub mod object;
/// Entities from `QObjectCleanupHandler` C++ header
pub mod object_cleanup_handler;
/// Entities from `QOperatingSystemVersion` C++ header
pub mod operating_system_version;
/// Entities from `QPair` C++ header
pub mod pair;
/// Entities from `QParallelAnimationGroup` C++ header
pub mod parallel_animation_group;
/// Entities from `QPauseAnimation` C++ header
pub mod pause_animation;
/// Entities from `QPersistentModelIndex` C++ header
pub mod persistent_model_index;
/// Entities from `QPluginLoader` C++ header
pub mod plugin_loader;
/// Entities from `QPoint` C++ header
pub mod point;
/// Entities from `QPointF` C++ header
pub mod point_f;
/// Entities from `QProcess` C++ header
pub mod process;
/// Entities from `QProcessEnvironment` C++ header
pub mod process_environment;
/// Entities from `QPropertyAnimation` C++ header
pub mod property_animation;
/// Entities from `Qt` C++ header
pub mod qt;
/// Entities from `QReadLocker` C++ header
pub mod read_locker;
/// Entities from `QReadWriteLock` C++ header
pub mod read_write_lock;
/// Entities from `QRect` C++ header
pub mod rect;
/// Entities from `QRectF` C++ header
pub mod rect_f;
/// Entities from `QRegExp` C++ header
pub mod reg_exp;
/// Entities from `QRegularExpression` C++ header
pub mod regular_expression;
/// Entities from `QRegularExpressionMatch` C++ header
pub mod regular_expression_match;
/// Entities from `QRegularExpressionMatchIterator` C++ header
pub mod regular_expression_match_iterator;
/// Entities from `QResource` C++ header
pub mod resource;
/// Entities from `QRunnable` C++ header
pub mod runnable;
/// Entities from `QSaveFile` C++ header
pub mod save_file;
/// Entities from `QSemaphore` C++ header
pub mod semaphore;
/// Entities from `QSequentialAnimationGroup` C++ header
pub mod sequential_animation_group;
/// Entities from `QSequentialIterable` C++ header
pub mod sequential_iterable;
/// Entities from `QSet` C++ header
pub mod set;
/// Entities from `QSettings` C++ header
pub mod settings;
/// Entities from `QSharedData` C++ header
pub mod shared_data;
/// Entities from `QSharedMemory` C++ header
pub mod shared_memory;
/// Entities from `QSignalBlocker` C++ header
pub mod signal_blocker;
/// Entities from `QSignalMapper` C++ header
pub mod signal_mapper;
/// Entities from `QSignalTransition` C++ header
pub mod signal_transition;
/// Entities from `QSize` C++ header
pub mod size;
/// Entities from `QSizeF` C++ header
pub mod size_f;
/// Binding Qt signals to Rust closures or extern functions.
///
/// Types in this module allow to connect Qt signals with certain argument types to a Rust closure.
///
/// There is one slot type for each distinct set of argument types present in this crate. However, if argument types were present in a dependency crate, the corresponding slot type is located in the dependency's `slots` module.
pub mod slots;
/// Entities from `QSocketNotifier` C++ header
pub mod socket_notifier;
/// Entities from `QSortFilterProxyModel` C++ header
pub mod sort_filter_proxy_model;
/// Entities from `QStandardPaths` C++ header
pub mod standard_paths;
/// Entities from `QState` C++ header
pub mod state;
/// Entities from `QStateMachine` C++ header
pub mod state_machine;
/// Entities from `QStaticByteArrayMatcherBase` C++ header
pub mod static_byte_array_matcher_base;
/// Entities from `QStaticPlugin` C++ header
pub mod static_plugin;
/// Entities from `QStorageInfo` C++ header
pub mod storage_info;
/// Entities from `QString` C++ header
pub mod string;
/// Entities from `QStringList` C++ header
pub mod string_list;
/// Entities from `QStringListModel` C++ header
pub mod string_list_model;
/// Entities from `QStringMatcher` C++ header
pub mod string_matcher;
/// Entities from `QStringRef` C++ header
pub mod string_ref;
/// Entities from `QSysInfo` C++ header
pub mod sys_info;
/// Entities from `QSystemSemaphore` C++ header
pub mod system_semaphore;
/// Entities from `QTemporaryDir` C++ header
pub mod temporary_dir;
/// Entities from `QTemporaryFile` C++ header
pub mod temporary_file;
/// Entities from `QTextBoundaryFinder` C++ header
pub mod text_boundary_finder;
/// Entities from `QTextCodec` C++ header
pub mod text_codec;
/// Entities from `QTextDecoder` C++ header
pub mod text_decoder;
/// Entities from `QTextEncoder` C++ header
pub mod text_encoder;
/// Entities from `QTextStream` C++ header
pub mod text_stream;
/// Entities from `QTextStreamManipulator` C++ header
pub mod text_stream_manipulator;
/// Entities from `QThread` C++ header
pub mod thread;
/// Entities from `QThreadPool` C++ header
pub mod thread_pool;
/// Entities from `QTime` C++ header
pub mod time;
/// Entities from `QTimeLine` C++ header
pub mod time_line;
/// Entities from `QTimeZone` C++ header
pub mod time_zone;
/// Entities from `QTimer` C++ header
pub mod timer;
/// Entities from `QTimerEvent` C++ header
pub mod timer_event;
/// Entities from `QTranslator` C++ header
pub mod translator;
/// Entities from `QUrl` C++ header
pub mod url;
/// Entities from `QUrlQuery` C++ header
pub mod url_query;
/// Entities from `QUrlTwoFlags` C++ header
pub mod url_two_flags;
/// Entities from `QUuid` C++ header
pub mod uuid;
/// Entities from `QVariant` C++ header
pub mod variant;
/// Entities from `QVariantAnimation` C++ header
pub mod variant_animation;
/// Entities from `QVector` C++ header
pub mod vector;
/// Entities from `QVersionNumber` C++ header
pub mod version_number;
/// Entities from `QWaitCondition` C++ header
pub mod wait_condition;
/// Entities from `QWriteLocker` C++ header
pub mod write_locker;
/// Entities from `QXmlStreamAttribute` C++ header
pub mod xml_stream_attribute;
/// Entities from `QXmlStreamAttributes` C++ header
pub mod xml_stream_attributes;
/// Entities from `QXmlStreamEntityDeclaration` C++ header
pub mod xml_stream_entity_declaration;
/// Entities from `QXmlStreamEntityResolver` C++ header
pub mod xml_stream_entity_resolver;
/// Entities from `QXmlStreamNamespaceDeclaration` C++ header
pub mod xml_stream_namespace_declaration;
/// Entities from `QXmlStreamNotationDeclaration` C++ header
pub mod xml_stream_notation_declaration;
/// Entities from `QXmlStreamReader` C++ header
pub mod xml_stream_reader;
/// Entities from `QXmlStreamStringRef` C++ header
pub mod xml_stream_string_ref;
/// Entities from `QXmlStreamWriter` C++ header
pub mod xml_stream_writer;


pub mod connection;
pub mod flags;
mod impl_arguments_compatible;

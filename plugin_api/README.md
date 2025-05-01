# Разработка плагинов для Аси

## Общая информация

Плагины для Аси представляют собой динамические библиотеки, которые загружаются во время выполнения. Поддерживаются плагины, написанные на C и Rust (на данный момент).

## Структура плагина

### Обязательные компоненты

Каждый плагин **обязан** содержать **глобальную** функцию с именем `plugin_info` следующей сигнатуры:

```rust
NativePluginInformation* plugin_info();
```

### Структуры данных

#### NativePluginInformation

```rust
pub struct NativePluginInformation {
    pub name: *const c_char,        // Фиксированное имя плагина (идентификатор)
    pub entrypoint: EntryPointCallback, // Функция входа в плагин
    pub options: *const PluginOption,   // Опции конфигурации плагина
}
```

#### EntryPointCallback

```c
void (*EntryPointCallback)(const char*, ApiCallbacksMap);
```

#### ApiCallbacksMap

```rust
pub struct ApiCallbacksMap {
    callbacks: *const ApiCallback,
    callbacks_len: c_uint,
}
```

#### ApiCallback

```rust
pub struct ApiCallback {
    name: *const c_char,    // Имя колбэка
    callback: *const c_void, // Указатель на функцию любой сигнатуры
}
```

#### PluginOption

```rust
pub struct PluginOption {
    name: *const c_char,    // Имя опции
    value: *const c_void,   // Значение опции (может быть любого типа)
}
```

## Взаимодействие с Асей

1. Ася автоматически загружает все плагины из директории `plugins`
2. При загрузке вызывается функция `plugin_info`
3. Через `ApiCallbacksMap` плагин получает доступ к API Аси
4. Опции плагина (`PluginOption`) используются для настройки взаимодействия с плагином
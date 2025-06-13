# Asya Plugin Development

## General Information

Asya plugins are dynamic libraries that are loaded at runtime. Currently, plugins written in C and Rust are supported.

## Plugin Structure

### Required Components

Each plugin **must** contain a **global** function named `plugin_info` with the following signature:

```c
NativePluginInformation* plugin_info();
```

### Data Structures

#### NativePluginInformation

```c
struct NativePluginInformation {
    const char* name;        // Fixed plugin name (identifier)
    EntryPointCallback entrypoint; // Plugin entry point function
    const PluginOption* options;   // Plugin configuration options
};
```

#### EntryPointCallback

```c
typedef void (*EntryPointCallback)(const char*, ApiCallbacksMap);
```

#### ApiCallbacksMap

```c
struct ApiCallbacksMap {
    const ApiCallback* callbacks;
    unsigned int callbacks_len;
};
```

#### ApiCallback

```c
struct ApiCallback {
    const char* name;    // Callback name
    const void* callback; // Pointer to a function of any signature
};
```

#### PluginOption

```c
struct PluginOption {
    const char* name;    // Option name
    const void* value;   // Option value (can be of any type)
};
```

## Interaction with Asya

1. Asya automatically loads all plugins from the `plugins` directory
2. The `plugin_info` function is called during loading
3. Through `ApiCallbacksMap`, the plugin gets access to Asya's API
4. Plugin options (`PluginOption`) are used to configure interaction with the plugin
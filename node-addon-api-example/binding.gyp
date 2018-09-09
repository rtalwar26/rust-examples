{
  "targets": [
    {
      "target_name": "addon",     
  
      "cflags!": [ "-fno-exceptions" ],
      "cflags_cc!": [ "-fno-exceptions" ],
      "sources": [ "addon.cc" ],
      "include_dirs": [
          ".",
        "<!@(node -p \"require('node-addon-api').include\")"
      ],
      "libraries":["-Wl,-rpath,<(module_root_dir)/libdouble_input.a"],
      'defines': [ 'NAPI_DISABLE_CPP_EXCEPTIONS' ],
    }
  ]
}
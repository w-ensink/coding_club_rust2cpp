
juce_add_plugin(RustBiquadPlugin
    PLUGIN_MANUFACTURER_CODE RuBi 
    PLUGIN_CODE BiFi           
    FORMATS AU VST3 Standalone                  
    PRODUCT_NAME "Rust Biquad")       

target_sources(RustBiquadPlugin
    PRIVATE
        PluginEditor.cpp
        PluginProcessor.cpp)

target_compile_definitions(RustBiquadPlugin
    PUBLIC
        JUCE_WEB_BROWSER=0  
        JUCE_USE_CURL=0    
        JUCE_VST3_CAN_REPLACE_VST2=0)

target_link_libraries(RustBiquadPlugin
    PRIVATE
        juce::juce_audio_utils
        biquad_rs
    PUBLIC
        juce::juce_recommended_config_flags
        juce::juce_recommended_lto_flags
        juce::juce_recommended_warning_flags)

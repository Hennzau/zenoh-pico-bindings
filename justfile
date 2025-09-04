z_info:
    cargo build --example z_info
    leaks --atExit -- ./target/debug/examples/z_info

bindgen:
    bindgen ~/Documents/zenoh-pico/include/zenoh-pico.h \
        -o ~/Documents/zenoh-pico-bindings/src/bindings.rs \
        --use-core -- \
        -I/Users/enzolevan/Documents/zenoh-pico/include \
        -DZENOH_MACOS \
        -DZ_FEATURE_MULTI_THREAD=1 -DZ_FEATURE_INTEREST=1 -DZ_FEATURE_UNSTABLE_API=0 \
        -DZ_FEATURE_PUBLICATION=1 -DZ_FEATURE_SUBSCRIPTION=1 -DZ_FEATURE_QUERY=1 -DZ_FEATURE_QUERYABLE=1 \
        -DZ_FEATURE_LIVELINESS=1 -DZ_FEATURE_MATCHING=1 -DZ_FEATURE_SCOUTING=1 -DZ_FEATURE_PERIODIC_TASKS=0 \
        -DZ_FEATURE_ADVANCED_PUBLICATION=0 -DZ_FEATURE_ADVANCED_SUBSCRIPTION=0 \
        -DZ_FEATURE_UNICAST_TRANSPORT=1 -DZ_FEATURE_MULTICAST_TRANSPORT=1 \
        -DZ_FEATURE_RAWETH_TRANSPORT=0 -DZ_FEATURE_LOCAL_SUBSCRIBER=0 -DFRAG_MAX_SIZE=300000 -DBATCH_UNICAST_SIZE=65535 \
        -DBATCH_MULTICAST_SIZE=8192 -DZ_FEATURE_UNICAST_PEER=1

cargo ndk -o ./target/jniLibs build

cp -rf jniLibs/* ../android/app/src/main/jniLibs/
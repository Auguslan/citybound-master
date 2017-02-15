set -ex

echo "before deploy reached!"

main() {
    case $TRAVIS_OS_NAME in
        linux)
            cargo build --release
            cp target/release/citybound ./citybound-linux
            ;;
        osx)
            cd ci/bundling/osx/citybound
            xcodebuild -scheme citybound archive DSTROOT="../out"
            cd ../../../..
            ditto -ck --rsrc --sequesterRsrc --keepParent ci/bundling/osx/out/Applications/citybound.app citybound.app.zip
            ;;
    esac
}

main
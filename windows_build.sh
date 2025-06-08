docker build -t asya-windows-builder .
mkdir target/windows-cross-build 2> /dev/null;
docker run --rm -v $(pwd)/target/windows-cross-build:/output asya-windows-builder /bin/sh -c "cp /asya.exe /output/"
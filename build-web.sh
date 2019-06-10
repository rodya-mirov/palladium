docker build -t palladium-build -f ./build.dockerfile . && \
    ctr=$(docker create -t -i palladium-build) && \
    rm -rf built/* && \
    docker cp $ctr:target/deploy ./built && \
    mv ./built/deploy/* ./built/ && rm -r ./built/deploy && \
    docker rm -fv $ctr
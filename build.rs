fn main() {
    tonic_build::compile_protos("proto/etcdserverpb/rpc.proto").unwrap();
    tonic_build::compile_protos("proto/mvccpb/kv.proto").unwrap();
    tonic_build::compile_protos("proto/v3lockpb/v3lock.proto").unwrap();
}

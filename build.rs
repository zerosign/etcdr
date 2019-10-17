fn main() {
    let proto_files = [
        "proto/etcd/etcdserver/etcdserverpb/rpc.proto",
        "proto/etcd/v3election/v3electionpb/v3election.proto",
        "proto/etcd/v3lock/v3lockpb/v3lock.proto",
    ];

    let include_paths = ["proto"];

    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .format(true)
        .compile(&proto_files, &include_paths)
        .unwrap();
}

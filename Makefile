.PHONY: prepare

# manual updates by referencing to master
# (TODO: @zerosign, able to fetch by tag or just use git submodule instead)
prepare:

	mkdir -p proto/{google,gogoproto,etcdserverpb,mvccpb,v3lockpb,v3electionpb,authpb}
	curl -Lo proto/authpb/auth.proto https://raw.githubusercontent.com/etcd-io/etcd/master/auth/authpb/auth.proto
	curl -Lo proto/google/descriptor.proto	https://raw.githubusercontent.com/protocolbuffers/protobuf/master/src/google/protobuf/descriptor.proto
	curl -Lo proto/google/annotations.proto https://raw.githubusercontent.com/googleapis/googleapis/master/google/api/annotations.proto
	curl -Lo proto/gogoproto/gogo.proto https://raw.githubusercontent.com/gogo/protobuf/master/gogoproto/gogo.proto
	curl -Lo proto/etcdserverpb/rpc.proto https://raw.githubusercontent.com/etcd-io/etcd/master/etcdserver/etcdserverpb/rpc.proto
	curl -Lo proto/mvccpb/kv.proto https://raw.githubusercontent.com/etcd-io/etcd/master/mvcc/mvccpb/kv.proto
	curl -Lo proto/v3lockpb/v3lock.proto https://raw.githubusercontent.com/etcd-io/etcd/master/etcdserver/api/v3lock/v3lockpb/v3lock.proto
	curl -Lo proto/v3electionpb/v3election.proto https://raw.githubusercontent.com/etcd-io/etcd/master/etcdserver/api/v3election/v3electionpb/v3election.proto
